use bevy::prelude::*;
// Добавляем импорты для настройки графического бэкенда
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Loading,
    InGame,
}

#[derive(Component)]
struct MyCube;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::GL),
                        ..default()
                    }),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#bevy-canvas".into()),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                // ВЫЗЫВАЕМ ЗДЕСЬ (внутри add_plugins)
                .disable::<bevy::audio::AudioPlugin>(), 
        )
        .init_state::<GameState>()
        .add_systems(Startup, (setup_loading_camera, setup_game_scene))
        .add_systems(Update, hide_loading_screen.run_if(in_state(GameState::Loading)))
        .add_systems(Update, rotate_cube_system.run_if(in_state(GameState::InGame)))
        .run();
}

// --- Остальная часть кода остается без изменений ---

fn hide_loading_screen(
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<Entity, With<MyCube>>, 
) {
    if !query.is_empty() {
        #[cfg(target_arch = "wasm32")]
        {
            hide_loader_in_html();
        }
        next_state.set(GameState::InGame);
    }
}

fn setup_loading_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = hideBevyLoader)]
    fn hide_loader_in_html();
}

fn setup_game_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        MyCube,
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.7, 0.6),
            ..default()
        })),
    ));
    
    commands.spawn((
        PointLight { 
            intensity: 500_000.0, 
            shadows_enabled: true,
            ..default() 
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn rotate_cube_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MyCube>>, 
) {
    for mut transform in &mut query {
        let mut rotation_factor = 0.0;
        if keyboard_input.pressed(KeyCode::KeyA) { rotation_factor += 1.0; }
        if keyboard_input.pressed(KeyCode::KeyD) { rotation_factor -= 1.0; }
        transform.rotate_y(rotation_factor * time.delta_secs() * 2.0);
    }
}
