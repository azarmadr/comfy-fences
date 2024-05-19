use comfy::*;
use fences::*;

simple_game!("Fences", GameState, setup, update);

fn setup(state: &mut GameState, _c: &mut EngineContext) {
    game_config_mut().bloom_enabled = true;
}

pub struct GameState {
    pub fonts: Vec<FontHandle>,
    pub size: f32,
    pub font_size: f32,
    pub cell_size: f32,
    pub cell_colors: (Color, Color, Color),
    pub fence_size: f32,
    pub fence_colors: (Color, Color, Color),
    pub board: Board,
}

impl GameState {
    pub fn new(_c: &mut EngineState) -> Self {
        Self {
            fonts: vec![
                load_font_from_bytes(include_bytes!("../assets/fonts/Orbitron-Black.ttf")),
                load_font_from_bytes(include_bytes!("../assets/fonts/Orbitron-Regular.ttf")),
            ],
            font_size: 32.0,
            cell_size: 0.9,
            cell_colors: (GRAY, GREEN, BEIGE),
            fence_colors: (BLACK, GREEN, BEIGE),
            fence_size: 0.2,
            size: 1.,
            board: "2#31  ".parse().unwrap(),
        }
    }
}

fn update(state: &mut GameState, _c: &mut EngineContext) {
    let board = &state.board;
    clear_background(Color::rgb8(13, 2, 8));

    egui::Window::new("Font Controls")
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(-100.0, -160.0))
        .show(egui(), |ui| {
            ui.add(egui::Slider::new(&mut state.font_size, 12.0..=80.0));
        });

    let z = 0;
    let _thickness = 0.1;

    for (idx, task) in board.tasks_iter() {
        let p = vec2(idx.1 as f32, -(idx.0 as f32));
        draw_rect(p, splat(state.cell_size), state.cell_colors.0, z);
        draw_text_pro_experimental(
            simple_styled_text(&if let Some(v) = task {
                format!("{v}")
            } else {
                " ".to_string()
            }),
            p,
            BLACK,
            TextAlign::Center,
            state.font_size,
            state.fonts[0],
            100,
        );
    }

    for ((dir, row, col), _) in board.fences_iter() {
        let mut p = vec2(col as f32, -(row as f32));
        if dir == 0 {
            p.y += state.cell_size / 2.
        } else {
            p.x -= state.cell_size / 2.
        }
        let mut size = vec2(state.cell_size, state.fence_size);
        if dir == 1 {
            std::mem::swap(&mut size.x, &mut size.y)
        }
        draw_rect(p, size, state.fence_colors.0, z);
    }

    /*
    draw_circle(vec2(0.0, 0.0), size / 2.0, RED, z);
    draw_circle_outline(vec2(0.0, 2.0), size / 2.0, thickness, RED, z);

    draw_rect(vec2(2.0, 0.0), splat(size), GREEN, z);
    draw_rect_outline(vec2(2.0, 2.0), splat(size), thickness, GREEN, z);
    */
}
