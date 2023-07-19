use macroquad::prelude::*;
use std::time::{Duration, Instant};

mod input;

fn window_conf() -> Conf {
    Conf { 
        window_title: "NOM".into(),
        window_width: 600,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let tile_data_str = load_string("assets/data/tiles.yaml").await.expect("Could not load tile data!");
    let player_data_str = load_string("assets/data/player.yaml").await.expect("Could not load player data!");

    let mut game_data = nom_data::GameData::new();
    let _ = game_data.add_entities_from_str(player_data_str);
    let tiles = game_data.add_entities_from_str(tile_data_str);
    game_data.tiles = tiles;

    let mut backend = macroquad_sprites::MacroquadBackend::new();

    backend.load_atlas(
        "ascii",
        "assets/sprites/ascii.png",
        16,
        16,
        None
    ).await
    .expect("Could not load sprites!");

    backend.load_font("default",  "assets/ui/04B_03.ttf").await
        .expect("Could not find fonts!");

    let mut main_camera = Camera2D {
        zoom: Vec2::new(2. / screen_width(), 2. / screen_height()),
        ..Default::default()
    };

    let mut world = rogalik::storage::World::new();
    world.insert_resource(game_data);

    let mut graphics_state = nom_graphics::GraphicsState::new(
        &mut world
    );
    let manager = nom_game::init(&mut world);

    loop {
        let frame_start = Instant::now();
        nom_game::game_step(&mut world, &manager);
        clear_background(BLACK);
        update_camera(&mut main_camera, &world);
        set_camera(&main_camera);
        backend.set_bounds(&main_camera);
        nom_graphics::graphics::update(&world, &mut graphics_state, &backend);

        set_default_camera();
        nom_graphics::ui::ui_update(
            &mut world,
            &backend,
            &input::get_ui_state(&main_camera)
        );
        next_frame().await;

        // temp to save some cpu cycles
        std::thread::sleep(Duration::from_millis(16).saturating_sub(frame_start.elapsed()));   
    }
}

fn update_camera(
    camera: &mut Camera2D,
    world: &rogalik::storage::World
) {
    let Some(board) = world.get_resource::<nom_game::Board>() else { return };
    let tile_position = rogalik::math::vectors::Vector2I::new(
        nom_game::globals::BOARD_WIDTH as i32,
        (board.shift - nom_game::globals::BOARD_LENGTH / 2) as i32 
    );
    let v = nom_graphics::graphics::tile_to_world(tile_position);
    let v = nom_graphics::graphics::move_towards(
        rogalik::math::vectors::Vector2F::new(camera.target.x, camera.target.y),
        rogalik::math::vectors::Vector2F::new(0.5 * v.x, v.y),
        2.
    );
    camera.target = Vec2::new(v.x, v.y);
}