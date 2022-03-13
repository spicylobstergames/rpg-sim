use std::f32::consts::TAU;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<NpcBundle>("NPC")
        .add_system(RandomWalk::update)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 0.6;
    commands.spawn_bundle(camera);

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("world.ldtk"),
        ..Default::default()
    });
}

#[derive(Bundle, LdtkEntity)]
pub struct NpcBundle {
    random_walk: RandomWalk,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Component)]
pub struct RandomWalk {
    angle: f32,
    random_timeout: f32,
}

impl Default for RandomWalk {
    fn default() -> Self {
        Self {
            angle: rand::random::<f32>()*TAU,
            random_timeout: -(1.5 + rand::random::<f32>() * 3.),
        }
    }
}

impl RandomWalk {
    fn dir(&self) -> Vec2 {
        Vec2::new(self.angle.cos(), self.angle.sin())
    }

    pub fn update(mut q_random_walk: Query<(&mut RandomWalk, &mut Transform)>, time: Res<Time>) {
        for (mut random_walk, mut transform) in q_random_walk.iter_mut() {
            if random_walk.random_timeout >= 0. { //Waiting
                random_walk.random_timeout -= time.delta_seconds();
                if random_walk.random_timeout < 0. { //Finished waiting
                    random_walk.random_timeout = -(1.5 + rand::random::<f32>() * 3.);

                    random_walk.angle = rand::random::<f32>()*TAU;
                }
            } else { //Walking
                transform.translation += (random_walk.dir() * 0.4).extend(0.);
                random_walk.angle += (rand::random::<f32>() - 0.5) * 0.2;

                random_walk.random_timeout += time.delta_seconds();
                if random_walk.random_timeout > 0. { //Finished walking
                    random_walk.random_timeout = 1.5 + rand::random::<f32>() * 3.;
                }
            }
        }
    }
}