use std::{
    collections::HashSet,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;
use pathfinding::undirected::connected_components;
use rand::{seq::IteratorRandom, Rng};

use crate::TILE_SIZE;

// TODO Make this a generic on the plugin or otherwise configurable
pub const GRID_SIZE_X: usize = 65;
pub const GRID_SIZE_Y: usize = 36;

#[derive(Resource)]
pub struct Grid<T> {
    entities: [[Option<Vec<Entity>>; GRID_SIZE_Y]; GRID_SIZE_X],
    _marker: PhantomData<T>,
}

#[derive(Resource)]
pub struct ConnectedComponents<T> {
    pub components: Vec<HashSet<GridLocation>>,
    _marker: PhantomData<T>,
}

#[derive(Component, Eq, PartialEq, Hash, Clone, Debug, Deref, DerefMut)]
pub struct GridLocation(IVec2);

#[derive(Debug)]
pub enum GridLocationError {
    InvalidLocation,
}

impl GridLocation {
    pub fn get_location(&self) -> IVec2 {
        self.0
    }
    pub fn try_set_location(&mut self, new_location: IVec2) -> Result<(), GridLocationError> {
        if Grid::<()>::valid_index(&GridLocation(new_location)) {
            self.0 = new_location;
            Ok(())
        } else {
            Err(GridLocationError::InvalidLocation)
        }
    }
}

/// Entities with this component will have their translation locked to the grid
#[derive(Component)]
pub struct LockToGrid;

#[derive(Event)]
pub struct DirtyGridEvent<T>(pub GridLocation, PhantomData<T>);

#[derive(Default)]
pub struct GridPlugin<T> {
    _marker: PhantomData<T>,
}

impl<T: Component> Plugin for GridPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid<T>>()
            .init_resource::<ConnectedComponents<T>>()
            .add_systems(PostUpdate, update_connected_components::<T>)
            .add_event::<DirtyGridEvent<T>>()
            // TODO move_on_grid / GridLocation change detection
            .add_systems(Startup, first_dirty_event::<T>)
            .add_systems(
                PreUpdate,
                (
                    add_to_grid::<T>,
                    lock_to_grid::<T>.after(update_in_grid::<T>),
                    update_in_grid::<T>.after(add_to_grid::<T>),
                    remove_from_grid::<T>,
                    resolve_connected_components::<T>,
                ),
            );
    }
}

fn lock_to_grid<T: Component>(
    grid: Res<Grid<T>>,
    mut positions: Query<
        (Entity, &mut Transform),
        (With<LockToGrid>, With<T>, Changed<GridLocation>),
    >,
) {
    for (entity, mut position) in &mut positions {
        if let Some(location) = grid.find_in_grid(entity) {
            position.translation.x = location.x as f32 * TILE_SIZE;
            position.translation.y = location.y as f32 * TILE_SIZE;
        }
    }
}

// Forces some sane initializations of connected components
fn first_dirty_event<T: Component>(mut dirty: EventWriter<DirtyGridEvent<T>>) {
    dirty.send(DirtyGridEvent::<T>(GridLocation::new(0, 0), PhantomData));
}

#[derive(Component)]
struct ConnectedTask<T> {
    task: Task<ConnectedComponents<T>>,
}

fn resolve_connected_components<T: Component>(
    mut commands: Commands,
    mut connected: ResMut<ConnectedComponents<T>>,
    // Should maybe be a resource?
    mut tasks: Query<(Entity, &mut ConnectedTask<T>)>,
) {
    for (task_entity, mut task) in &mut tasks {
        if let Some(result) = future::block_on(future::poll_once(&mut task.task)) {
            //TODO is there a way to make bevy auto remove these or not panic or something
            commands.entity(task_entity).despawn_recursive();
            *connected = result;
        }
    }
}

fn update_connected_components<T: Component>(
    mut commands: Commands,
    grid: Res<Grid<T>>,
    mut events: EventReader<DirtyGridEvent<T>>,
    // Should maybe be a resource?
    current_tasks: Query<Entity, With<ConnectedTask<T>>>,
) {
    if !events.is_empty() {
        events.clear();
        for task in &current_tasks {
            commands.entity(task).despawn_recursive();
        }

        let thread_pool = AsyncComputeTaskPool::get();
        let grid = Box::new(grid.clone());

        let task = thread_pool.spawn(async move {
            let starts = all_points()
                .into_iter()
                .filter(|point| !grid.occupied(point))
                .collect::<Vec<_>>();

            ConnectedComponents::<T> {
                components: connected_components::connected_components(&starts, |p| {
                    neumann_neighbors(&grid, p)
                }),
                ..default()
            }
        });

        commands.spawn(ConnectedTask { task });
    }
}

fn remove_from_grid<T: Component>(
    mut grid: ResMut<Grid<T>>,
    mut query: RemovedComponents<T>,
    mut dirty: EventWriter<DirtyGridEvent<T>>,
) {
    for removed_entity in query.read() {
        // Search for entity
        let removed = grid.iter().find(|(entity, _)| *entity == removed_entity);
        if let Some((_, location)) = removed {
            dirty.send(DirtyGridEvent::<T>(location.clone(), PhantomData));
            grid[&location] = None;
        }
    }
}

impl<T> Grid<T> {
    pub fn force_update(&mut self, entity: Entity, new_location: &GridLocation) {
        if let Some(previous) = self.find_in_grid(entity) {
            self[&previous] = None;
        }
        if Grid::<()>::valid_index(new_location) {
            if let Some(ref mut existing) = &mut self[new_location] {
                if !existing.contains(&entity) {
                    existing.push(entity);
                }
            } else {
                self[new_location] = Some(vec![entity]);
            }
        }
    }
}

fn update_in_grid<T: Component>(
    mut grid: ResMut<Grid<T>>,
    query: Query<(Entity, &GridLocation), (Changed<GridLocation>, With<T>)>,
    mut dirty: EventWriter<DirtyGridEvent<T>>,
) {
    for (entity, location) in &query {
        if Grid::<()>::valid_index(location)
            && grid[location]
                .as_ref()
                .map(|vec| !vec.contains(&entity))
                .unwrap_or(true)
        {
            grid.force_update(entity, location);
            dirty.send(DirtyGridEvent::<T>(location.clone(), PhantomData));
        }
    }
}

fn add_to_grid<T: Component>(
    mut grid: ResMut<Grid<T>>,
    query: Query<(Entity, &GridLocation), Added<T>>,
    mut dirty: EventWriter<DirtyGridEvent<T>>,
) {
    for (entity, location) in &query {
        if Grid::<()>::valid_index(location) {
            if let Some(ref mut existing) = &mut grid[location] {
                if !existing.contains(&entity) {
                    dirty.send(DirtyGridEvent::<T>(location.clone(), PhantomData));
                    existing.push(entity);
                }
            } else {
                dirty.send(DirtyGridEvent::<T>(location.clone(), PhantomData));
                grid[location] = Some(vec![entity]);
            }
        }
    }
}

fn all_points() -> Vec<GridLocation> {
    (0..GRID_SIZE_X)
        .flat_map(|x| (0..GRID_SIZE_Y).map(move |y| GridLocation::new(x as u32, y as u32)))
        .collect()
}

impl<T> Default for ConnectedComponents<T> {
    fn default() -> Self {
        Self {
            components: Default::default(),
            _marker: Default::default(),
        }
    }
}

impl<T> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            entities: self.entities.clone(),
            _marker: self._marker,
        }
    }
}

// https://github.com/rust-lang/rust/issues/44796#issuecomment-967747810
const INIT: Option<Vec<Entity>> = None;
const INIT_INNER: [Option<Vec<Entity>>; GRID_SIZE_Y] = [INIT; GRID_SIZE_Y];

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self {
            entities: [INIT_INNER; GRID_SIZE_X],
            _marker: Default::default(),
        }
    }
}

impl GridLocation {
    pub fn new(x: u32, y: u32) -> Self {
        GridLocation(IVec2::new(x as i32, y as i32))
    }

    /*
    pub fn from_world_position(position: Vec2) -> Option<Self> {
        let position = position + Vec2::splat(0.5);
        let location = GridLocation(IVec2::new(position.x as i32, position.y as i32));
        if Grid::<()>::valid_index(&location) {
            Some(location)
        } else {
            None
        }
    }
    */
}

impl From<IVec2> for GridLocation {
    fn from(value: IVec2) -> Self {
        GridLocation(value)
    }
}

impl<T> Grid<T> {
    pub fn occupied(&self, location: &GridLocation) -> bool {
        Grid::<T>::valid_index(location) && self[location].is_some()
    }

    pub fn valid_index(location: &GridLocation) -> bool {
        location.x >= 0
            && location.y >= 0
            && location.x < GRID_SIZE_X as i32
            && location.y < GRID_SIZE_Y as i32
    }

    pub fn find_in_grid(&self, to_find: Entity) -> Option<GridLocation> {
        for (entity, location) in self.iter() {
            if entity == to_find {
                return Some(location);
            }
        }
        None
    }
}

impl<T> Grid<T> {
    pub fn iter(&self) -> impl Iterator<Item = (Entity, GridLocation)> + '_ {
        self.entities
            .iter()
            .flatten()
            .enumerate()
            .filter_map(|(i, cell)| {
                cell.as_ref().map(|entities| {
                    entities.iter().map(move |&entity| {
                        (
                            entity,
                            GridLocation::new(
                                i as u32 / GRID_SIZE_Y as u32,
                                i as u32 % GRID_SIZE_Y as u32,
                            ),
                        )
                    })
                })
            })
            .flatten()
    }
}

impl<T> Index<&GridLocation> for Grid<T> {
    type Output = Option<Vec<Entity>>;

    fn index(&self, index: &GridLocation) -> &Self::Output {
        &self.entities[index.x as usize][index.y as usize]
    }
}

impl<T> IndexMut<&GridLocation> for Grid<T> {
    fn index_mut(&mut self, index: &GridLocation) -> &mut Self::Output {
        &mut self.entities[index.x as usize][index.y as usize]
    }
}

impl<T> ConnectedComponents<T> {
    #[allow(unused)]
    pub fn point_to_component(&self, start: &GridLocation) -> Option<&HashSet<GridLocation>> {
        self.components
            .iter()
            .find(|component| component.contains(start))
    }

    pub fn is_in_same_component(&self, start: &GridLocation, end: &GridLocation) -> bool {
        self.point_to_component(start) == self.point_to_component(end)
    }

    pub fn get_random_point_in_same_component<R>(
        &self,
        start: &GridLocation,
        rng: &mut R,
    ) -> Option<GridLocation>
    where
        R: Rng + ?Sized,
    {
        self.point_to_component(start)
            .and_then(|component| component.iter().choose(rng).cloned())
    }
}

pub fn neumann_neighbors<T>(grid: &Grid<T>, location: &GridLocation) -> Vec<GridLocation> {
    let (x, y) = (location.x as u32, location.y as u32);

    let mut successors = Vec::new();
    if let Some(left) = x.checked_sub(1) {
        let location = GridLocation::new(left, y);
        if !grid.occupied(&location) {
            successors.push(location);
        }
    }
    if let Some(down) = y.checked_sub(1) {
        let location = GridLocation::new(x, down);
        if !grid.occupied(&location) {
            successors.push(location);
        }
    }
    if x + 1 < GRID_SIZE_X as u32 {
        let right = x + 1;
        let location = GridLocation::new(right, y);
        if !grid.occupied(&location) {
            successors.push(location);
        }
    }
    if y + 1 < GRID_SIZE_Y as u32 {
        let up = y + 1;
        let location = GridLocation::new(x, up);
        if !grid.occupied(&location) {
            successors.push(location);
        }
    }
    successors
}
