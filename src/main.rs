//use bevy::prelude::Component;
use bevy::prelude::*;

use bevy::ecs as bevy_ecs;
use bevy_atmosphere::*;
use bevy_ecs::prelude::Component;
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use bevy_mod_picking::*;

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins);
    //#[cfg(target_arch = "wasm32")]
    //    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.add_plugin(NoCameraPlayerPlugin);
    app.add_plugins(DefaultPickingPlugins);
    app.add_plugin(DebugCursorPickingPlugin);
    app.insert_resource(bevy_atmosphere::AtmosphereMat::default()); // Default Earth sky
    app.add_plugin(bevy_atmosphere::AtmospherePlugin {
        dynamic: false, // Set to false since we aren't changing the sky's appearance
        sky_radius: 10.0,
    });
    app.add_system(hello).insert_resource(MovementSettings {
        sensitivity: 0.00015, // default: 0.00012
        speed: 12.0,          // default: 12.0
    });
    app.add_system(levitate_system);
    app.add_system(sink_system);
    app.add_system(react);
    app.add_startup_system(setup).run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.7, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert(DownComponent { height: 20. })
        .insert_bundle(PickableBundle::default());
    // // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCam)
        .insert_bundle(PickingCameraBundle::default());

    commands.spawn_bundle(UiCameraBundle::default());
    //        .insert(SkyboxCamera);
}

#[derive(Component)]
pub struct CubeComponent {
    height: f32,
}

#[derive(Component)]
pub struct DownComponent {
    height: f32,
}

fn hello(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(2.0, 0.5, 0.0)),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(CubeComponent { height: 20. });
}

fn levitate_system(
    mut commands: Commands,
    mut boxes: Query<(&mut Transform, &mut CubeComponent, Entity)>,
) {
    for (mut transform, mut cube_comp, entity) in boxes.iter_mut() {
        if cube_comp.height > 0. {
            cube_comp.height -= 0.1;
            transform.translation.y += 0.1;
        } else {
            commands.entity(entity).remove::<CubeComponent>();
            commands
                .entity(entity)
                .insert(DownComponent { height: 20. });
        }
    }
}

fn sink_system(
    mut commands: Commands,
    mut boxes: Query<(&mut Transform, &mut DownComponent, Entity)>,
) {
    for (mut transform, mut cube_comp, entity) in boxes.iter_mut() {
        if cube_comp.height > 0. {
            cube_comp.height -= 0.1;
            transform.translation.y -= 0.1;
        } else {
            commands.entity(entity).remove::<DownComponent>();
            commands
                .entity(entity)
                .insert(CubeComponent { height: 20. });
        }
    }
}

fn react(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    mut asset_server: Res<AssetServer>,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Clicked(entity) => {
                commands.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Clicked!",
                        TextStyle {
                            //https://github.com/gilescope/cyberpunkfonts/raw/master/audiowide-mono/Audiowide-Mono-Latest.ttf
                            font: asset_server.load("font/Audiowide-Mono-Latest.ttf"), //
                            ..Default::default()
                        },
                        TextAlignment {
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                });
                println!("CLICKED");
            }
            _ => {}
        }
    }
}
