//! "Twin" 3d object

use bevy::{
    color::palettes::basic::*,
    input::{gestures::RotationGesture, touch::TouchPhase},
    log::{Level, LogPlugin},
    prelude::*,
    window::{AppLifecycle, WindowMode},
    winit::WinitSettings,
};

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(LogPlugin {
                level: Level::DEBUG,
                filter: "wgpu=error,naga=info,bevy_render=info,bevy_ecs=trace".to_string(),
                ..Default::default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }),
    )
    .insert_resource(WinitSettings::mobile())
    .add_systems(Startup, setup_scene)
    .add_systems(
        Update,
        (
            touch_camera,
            button_handler,
            handle_lifetime.run_if(any_with_component::<AudioSink>),
        ),
    )
    .run();
}

fn touch_camera(
    window: Query<&Window>,
    mut touch_inputs: MessageReader<TouchInput>,
    mut camera_transform: Single<&mut Transform, With<Camera3d>>,
    mut last_position: Local<Option<Vec2>>,
    mut rotation_gestures: MessageReader<RotationGesture>,
) {
    let Ok(window) = window.single() else {
        return;
    };

    for touch_input in touch_inputs.read() {
        if touch_input.phase == TouchPhase::Started {
            *last_position = None;
        }
        if let Some(last_position) = *last_position {
            **camera_transform = Transform::from_xyz(
                camera_transform.translation.x
                    + (touch_input.position.x - last_position.x) / window.width() * 5.0,
                camera_transform.translation.y,
                camera_transform.translation.z
                    + (touch_input.position.y - last_position.y) / window.height() * 5.0,
            )
            .looking_at(Vec3::ZERO, Vec3::Y);
        }
        *last_position = Some(touch_input.position);
    }

    for rotation_gesture in rotation_gestures.read() {
        let forward = camera_transform.forward();
        camera_transform.rotate_axis(forward, rotation_gesture.0 / 10.0);
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    device: Res<bevy::render::renderer::RenderDevice>,
) {
    bevy::log::info!("Configured wgpu adapter Limits: {:#?}", device.limits());
    bevy::log::info!("Configured wgpu adapter Features: {:#?}", device.features());

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.2, 0.1))),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.4, 0.3))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5).mesh().ico(4).unwrap())),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.4, 0.8))),
        Transform::from_xyz(1.5, 1.5, 1.5),
    ));

    commands.spawn((
        PointLight {
            intensity: 1_000_000.0,
            #[cfg(not(target_os = "android"))]
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        #[cfg(target_os = "android")]
        Msaa::Off,
    ));

    // Test ui
    commands
        .spawn((
            Button,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                left: px(50),
                right: px(50),
                bottom: px(50),
                ..default()
            },
        ))
        .with_child((
            Text::new("Test Button"),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextColor::BLACK,
            TextLayout::new_with_justify(Justify::Center),
        ));
}

fn button_handler(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BLUE.into();
            }
            Interaction::Hovered => {
                *color = GRAY.into();
            }
            Interaction::None => {
                *color = WHITE.into();
            }
        }
    }
}

fn handle_lifetime(mut app_lifecycle_reader: MessageReader<AppLifecycle>) {
    for app_lifecycle in app_lifecycle_reader.read() {
        match app_lifecycle {
            AppLifecycle::Idle | AppLifecycle::WillSuspend | AppLifecycle::WillResume => {}
            AppLifecycle::Suspended => {}
            AppLifecycle::Running => {}
        }
    }
}
