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
        // 1. Глобальный фон
        .insert_resource(ClearColor(Color::BLACK))
        
        // 2. Настройка плагинов
        .add_plugins(
            DefaultPlugins
                // Настройка рендеринга (WebGL2 для iPhone XR)
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::GL),
                        ..default()
                    }),
                    ..default()
                })
                // Настройка окна (Канвас и частота обновления)
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#bevy-canvas".into()),
                        fit_canvas_to_parent: true,
                        // Убираем лимит 60 FPS для плавности на мониторах >60Гц
                        present_mode: bevy::window::PresentMode::AutoNoVsync, 
                        ..default()
                    }),
                    ..default()
                })
                // Отключаем аудио, чтобы не блокировать старт на iOS
                .disable::<bevy::audio::AudioPlugin>(), 
        )

        // 3. Состояния и системы
        .init_state::<GameState>()
        .add_systems(Startup, (setup_loading_camera, setup_game_scene))
        
        // Установка скорости времени один раз при старте игры
        .add_systems(OnEnter(GameState::InGame), set_global_time_speed)
        
        .add_systems(Update, (
            hide_loading_screen.run_if(in_state(GameState::Loading)),
            rotate_cube_system.run_if(in_state(GameState::InGame)),
        ))
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
            // Вызываем JS несколько раз, пока состояние не сменится
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
    touches: Res<Touches>, // Добавляем ресурс касаний
    windows: Query<&Window>, // Нужно для получения ширины экрана
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MyCube>>,
    mut current_rotation_speed: Local<f32>,
) {
    let mut rotation_factor = 0.0;
    let window = windows.single();
    let width = window.width();

    // --- Управление клавиатурой (ПК) ---
    if keyboard_input.pressed(KeyCode::KeyA) {
        rotation_factor += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation_factor -= 1.0;
    }

    // --- Управление тапами (iPhone) ---
    for touch in touches.iter() {
        let x = touch.position().x;
        if x < width / 2.0 {
            // Левая часть экрана
            rotation_factor += 1.0;
        } else {
            // Правая часть экрана
            rotation_factor -= 1.0;
        }
    }

    // Плавное сглаживание скорости (Lerp)
    let target_speed = rotation_factor * 9.0; 
    let smoothing = 0.2;
    *current_rotation_speed += (target_speed - *current_rotation_speed) * smoothing;

    for mut transform in &mut query {
        transform.rotate_y(*current_rotation_speed * time.delta_secs());
    }
}

fn set_global_time_speed(mut time: ResMut<Time<Virtual>>) {
    // Теперь во всей игре время всегда будет идти в 2 раза быстрее
    time.set_relative_speed(1.0); 
}