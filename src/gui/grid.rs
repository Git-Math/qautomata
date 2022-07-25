use bevy::prelude::*;

pub struct GridPlugin;

const GRID_SIZE: usize = 50;
const CELL_SIZE: f32 = 32.0;
const SPACE_BETWEEN_CELLS: f32 = 1.0;
const CELL_WITH_SPACE_SIZE: f32 = CELL_SIZE + SPACE_BETWEEN_CELLS;
const CAMERA_X: f32 = CELL_WITH_SPACE_SIZE * (GRID_SIZE as f32) / 2.0;
const CAMERA_Y: f32 = CAMERA_X * -1.0;

const DEAD_CELL: Color = Color::rgba(1.0, 1.0, 1.0, 1.0);
const GREY: Color = Color::rgba(0.5, 0.5, 0.5, 1.0);

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .insert_resource(ClearColor(GREY));
    }
}

fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_xyz(CAMERA_X, CAMERA_Y, 0.0);
    commands.spawn_bundle(camera);

    let mut grid_entities: [[Entity; GRID_SIZE]; GRID_SIZE] = [[Entity::from_raw(0); GRID_SIZE]; GRID_SIZE];


    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            println!("{}, {}", x, y);
            grid_entities[x][y] = commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.0, 0.0, 0.0, 1.0),
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

    /*commands.entity(grid_entities[3][0]).insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(0.0, 0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(CELL_SIZE * 1.0, CELL_SIZE * 1.0)),
            ..default()
        },
        transform: Transform {
            translation: coordinates_to_vec(3 as f32, 0 as f32),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..default()
        },
        ..default()
    });*/
}

// Universe coordinates: [0,0] [0,1]
//                       [1,0] [1,1]
// Gui coordinates: [1,0] [1,1]
//                  [0,0] [0,1]
// So we need to do a conversion when computing the vector
fn coordinates_to_vec(x: f32, y: f32) -> Vec3 {
    Vec3::new(y * CELL_WITH_SPACE_SIZE, x * CELL_WITH_SPACE_SIZE * -1.0, 0.0)
}
