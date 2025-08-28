use {comfy::*, fences::*, std::process::exit};

simple_game!("Fences", GameState, config, setup, update);

/// State of the Cell wrt the puzzle
enum CellState {
    /// Inside the Fences
    _In,
    /// Outside the Fences
    _Out,
    /// Undecided
    None,
}

struct Fence(Option<bool>);

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        resolution: ResolutionConfig::Logical(600, 600 * 4 / 3),
        min_resolution: ResolutionConfig::Logical(100, 100 * 4 / 3),
        ..config
    }
}

fn setup(state: &mut GameState, _c: &mut EngineContext) {
    let board = &state.board;

    let z = 0;
    let _thickness = 0.1;

    for (idx, task) in board.tasks_iter() {
        let p = vec2(idx.1 as f32, -(idx.0 as f32));
        let size = splat(state.cell_size);
        commands().spawn((
            CellState::None,
            Transform::position(p),
            // Sprite::new("1px".to_string(), size, z, state.cell_colors.0),
        ));
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
        commands().spawn((
            Fence(None),
            Transform::position(p),
            // Sprite::new("1px".to_string(), size, z, state.fence_colors.0),
        ));
    }
}

pub struct GameState {
    pub fonts: Vec<FontHandle>,
    pub font_size: f32,
    pub cell_size: f32,
    pub cell_colors: (Color, Color, Color),
    pub fence_size: f32,
    pub fence_colors: (Color, Color, Color),
    pub board: Board,
}

use fences::solver::Idx;
impl GameState {
    pub fn new(_c: &mut EngineState) -> Self {
        let config = dbg!(game_config());
        config.resolution;
        let board = "2#33".parse().unwrap();
        Self {
            fonts: vec![
                load_font_from_bytes(include_bytes!("../assets/fonts/Orbitron-Black.ttf")),
                load_font_from_bytes(include_bytes!("../assets/fonts/Orbitron-Regular.ttf")),
            ],
            font_size: 32.0,
            cell_size: 2.,
            cell_colors: (GRAY, GREEN, BEIGE),
            fence_colors: (BROWN, GREEN, BEIGE),
            fence_size: 0.2,
            board,
        }
    }
    #[inline]
    fn idx2p(&self, idx: Idx) -> Vec2 {
        let (rows, cols) = self.board.size();
        Vec2 {
            x: (idx.1 as f32 - rows as f32 / 2.) * self.cell_size,
            y: (cols as f32 / 2. - idx.0 as f32) * self.cell_size,
        }
    }
}

struct SmallGlobalState(u8);
static STATE: Lazy<AtomicRefCell<SmallGlobalState>> =
    Lazy::new(|| AtomicRefCell::new(SmallGlobalState(0)));
fn update(state: &mut GameState, _c: &mut EngineContext) {
    clear_background(Color::rgb8(13, 2, 8));
    let board = &mut String::new();
    egui::Window::new("Font Controls")
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(-100.0, -160.0))
        .show(egui(), |ui| {
            ui.add(egui::Slider::new(&mut state.font_size, 12.0..=80.0));
            ui.checkbox(&mut game_config_mut().bloom_enabled, "Bloom Enabled");
            ui.add(egui::TextEdit::singleline(board).hint_text("New Board"));
        });
    if let Ok(board) = board.parse() {
        state.board = board;
    }
    let mut timer = STATE.borrow_mut();
    if cooldowns().can_use("Die", 88.) {
        println!("Die1");
        timer.0 += 1;
    } else if timer.0 > 2 {
        println!("Die");
        exit(0)
    }
    hint_display_system(state);
}

fn hint_display_system(state: &mut GameState) {
    state.board.tasks_iter().for_each(|(idx, v)| {
        let p = state.idx2p(idx);
        if let Some(v) = v {
            draw_text_pro_experimental(
                simple_styled_text(&format!("{v}")),
                p - splat(state.cell_size / 3.),
                GREEN,
                TextAlign::BottomLeft,
                state.font_size,
                state.fonts[0],
                100,
            );
        };

        draw_rect(p, splat(state.cell_size), state.cell_colors.0, 0);
        draw_rect_outline(p, splat(state.cell_size), 0.2, RED, 10);
    })
}
