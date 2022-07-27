use bevy::core::FixedTimestep;
use bevy::prelude::*;

use crate::universe::types::*;

const GRID_SIZE: usize = 10;
const CELL_SIZE: f32 = 32.0;
const SPACE_BETWEEN_CELLS: f32 = 1.0;
const CELL_WITH_SPACE_SIZE: f32 = CELL_SIZE + SPACE_BETWEEN_CELLS;
const SHIFT_Y: f32 = CELL_WITH_SPACE_SIZE * (GRID_SIZE as f32) / 2.0;
const SHIFT_X: f32 = SHIFT_Y * -1.0;


const DEAD_CELL_COLOR: Color = Color::rgba(1.0, 1.0, 1.0, 1.0);
const LIVING_CELL_BASE_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 1.0);
const GREY: Color = Color::rgba(0.5, 0.5, 0.5, 1.0);

struct GridEntities([[Entity; GRID_SIZE]; GRID_SIZE]);

pub struct GridPlugin {
    pub universe: Universe,
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .insert_resource(ClearColor(GREY))
            .insert_resource(self.universe.clone())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(2.0))
                    .with_system(step),
            );
    }
}

fn setup(mut commands: Commands, universe: Res<Universe>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(camera);

    let mut grid_entities: GridEntities =
        GridEntities([[Entity::from_raw(0); GRID_SIZE]; GRID_SIZE]);

    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            println!("{}, {}", x, y);
            grid_entities.0[x][y] = commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: if universe.state[0].living_cells.contains_key(&Coordinates {
                            x: x as i32,
                            y: y as i32,
                        }) {
                            LIVING_CELL_BASE_COLOR
                        } else {
                            DEAD_CELL_COLOR
                        },
                        custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                        ..default()
                    },
                    transform: Transform {
                        translation: coordinates_to_vec(x as f32, y as f32),
                        scale: Vec3::new(1.0, 1.0, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .id();
        }
    }

    commands.insert_resource(grid_entities);
}

fn step(mut commands: Commands, grid_entities: Res<GridEntities>, universe: ResMut<Universe>) {
    commands
        .entity(grid_entities.0[3][0])
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.0, 0.0, 0.0, 1.0),
                custom_size: Some(Vec2::new(CELL_SIZE * 1.0, CELL_SIZE * 1.0)),
                ..default()
            },
            transform: Transform {
                translation: coordinates_to_vec(3 as f32, 0 as f32),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        });
}

// Universe coordinates: [0,0] [0,1]
//                       [1,0] [1,1]
// Gui coordinates: [1,0] [1,1]
//                  [0,0] [0,1]
// So we need to do a conversion when computing the vector
// We add the shift so that the center of the grid is 0,0
fn coordinates_to_vec(x: f32, y: f32) -> Vec3 {
    Vec3::new(
        y * CELL_WITH_SPACE_SIZE + SHIFT_X,
        x * CELL_WITH_SPACE_SIZE * -1.0 + SHIFT_Y,
        0.0,
    )
}
