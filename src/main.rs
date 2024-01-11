use bevy::{
    pbr::{DefaultOpaqueRendererMethod, DirectionalLightShadowMap},
    prelude::*,
};
use network::NetworkPlugin;
use systems::{
    camera::CameraPlugin,
    menu::{MenuEvent, MenuPlugin},
    player::PlayerPlugin,
    BasePlugins,
};
use ui::{button::build_button, UiPlugin};

mod systems;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(DefaultOpaqueRendererMethod::deferred())
        .insert_resource(AmbientLight {
            color: Color::rgb(0.2, 0.2, 0.2),
            brightness: 1.0 / 3.0,
        })
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(BasePlugins)
        .add_plugins((
            PlayerPlugin,
            CameraPlugin,
            NetworkPlugin,
            UiPlugin::new().with_event::<MenuEvent>(),
            MenuPlugin,
        ))
        .add_systems(Startup, (spawn_floor, spawn_light, setup_test_ui))
        .run();
}

fn spawn_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 8.0, 0.0)),
        ..default()
    });
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(20.0))),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}

fn setup_test_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                padding: UiRect::top(Val::Px(60.0)),
                row_gap: Val::Px(30.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Geostationary",
                    TextStyle {
                        font: Handle::default(),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
            build_button()
                .with_text("Play")
                .with_callback(Box::new(|| {
                    println!("Button pressed!");
                }))
                .build(parent);
            build_button()
                .with_text("Quit")
                .with_event(MenuEvent::Quit)
                .build(parent);
        });
}
