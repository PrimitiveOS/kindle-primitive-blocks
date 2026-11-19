use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use blocks_core::{Game, SIZE};
use primitive_app::{App, Change};
use primitive_ui::{
    back_button, header, screen_rect, Canvas, Input, Rect, TextStyle, BLACK, BODY, CAPTION, PAPER,
};

pub struct View {
    pub game: Game,
    path: Option<PathBuf>,
    selected: Option<usize>,
    confirm: bool,
    notice: Option<String>,
}

impl View {
    pub fn open(path: PathBuf) -> Self {
        let game = fs::read(&path)
            .ok()
            .and_then(|bytes| serde_json::from_slice::<Game>(&bytes).ok())
            .map(Game::restored)
            .unwrap_or_else(|| Game::new(random_seed()));
        Self::new(game, Some(path))
    }

    pub fn preview(seed: u64) -> Self {
        Self::new(Game::new(seed), None)
    }

    fn new(game: Game, path: Option<PathBuf>) -> Self {
        Self {
            game,
            path,
            selected: None,
            confirm: false,
            notice: None,
        }
    }

    fn draw_board(&self, canvas: &mut Canvas, width: i32) {
        let board_active = !self.confirm && !self.game.over();
        for (y, row) in self.game.board().iter().enumerate() {
            for (x, &occupied) in row.iter().enumerate() {
                let bounds = board_cell(width, x, y);
                let pressed = board_active && canvas.control_pressed(bounds, false);
                let ink = match (occupied, pressed) {
                    (true, true) => PAPER,
                    (true, false) => 30,
                    (false, true) => BLACK,
                    (false, false) => 247,
                };
                canvas.fill(bounds.inset(1), ink);
                if occupied {
                    canvas.fill(bounds.inset(canvas.px(19)), 85);
                }
            }
        }
        for boundary in 0..=SIZE {
            let band = 136 + boundary as i32 * 110;
            let size = if boundary % 3 == 0 { 5 } else { 3 };
            let ink = if boundary % 3 == 0 { BLACK } else { 115 };
            canvas.fill(screen_rect(width, band, 286, size, 994), ink);
            canvas.fill(
                screen_rect(width, 136, 286 + boundary as i32 * 110, 994, size),
                ink,
            );
        }
    }

    fn draw_score(&self, canvas: &mut Canvas, width: i32) {
        for (x, (heading, value)) in [
            (40, ("Score", self.game.score())),
            (650, ("Best", self.game.best())),
        ] {
            canvas.card(screen_rect(width, x, 167, 574, 102));
            canvas.text(screen_rect(width, x + 25, 191, 230, 55), heading, CAPTION);
            canvas.text(
                screen_rect(width, x + 250, 190, 298, 55),
                &value.to_string(),
                BODY.bold().right(),
            );
        }
    }

    fn draw_pieces(&self, canvas: &mut Canvas, width: i32) {
        let pieces_active = !self.confirm && !self.game.over();
        for i in 0..3 {
            let bounds = piece_card(width, i);
            let shape = self.game.piece(i);
            let pressed = pieces_active && shape.is_some() && canvas.control_pressed(bounds, false);
            let selected = (self.selected == Some(i)) ^ pressed;
            canvas.panel(
                bounds,
                22,
                if selected { 5 } else { 2 },
                if selected { BLACK } else { PAPER },
            );
            if let Some(shape) = shape {
                let length = canvas.px(29);
                let shape_width = (shape.iter().map(|(x, _)| x).max().unwrap_or(&0) + 1) as i32;
                let shape_height = (shape.iter().map(|(_, y)| y).max().unwrap_or(&0) + 1) as i32;
                let start_x = bounds.x + (bounds.w - shape_width * length) / 2;
                let start_y = bounds.y + (bounds.h - shape_height * length) / 2;
                for &(x, y) in shape {
                    let tile = Rect::new(
                        start_x + x as i32 * length,
                        start_y + y as i32 * length,
                        length,
                        length,
                    );
                    canvas.fill(tile.inset(1), if selected { PAPER } else { BLACK });
                }
            }
        }
    }

    fn tap(&mut self, x: f32, y: f32) -> Change {
        let width = primitive_app::WIDTH as i32;
        if self.confirm {
            self.confirm = false;
            if confirm_button(width).contains(x, y) {
                self.game.restart();
                self.selected = None;
                self.notice = None;
                self.persist();
            }
            return Change::Full;
        }
        if back_button(width).contains(x, y) {
            return Change::Exit;
        }
        if new_button(width).contains(x, y) {
            if self.game.over() || self.game.score() == 0 {
                self.game.restart();
                self.selected = None;
                self.notice = None;
                self.persist();
            } else {
                self.confirm = true;
            }
            return Change::Full;
        }
        if self.game.over() {
            return Change::None;
        }
        if let Some(i) =
            (0..3).find(|&i| piece_card(width, i).contains(x, y) && self.game.piece(i).is_some())
        {
            self.selected = (self.selected != Some(i)).then_some(i);
            self.notice = None;
            return Change::Partial;
        }
        let board = screen_rect(width, 136, 286, 990, 990);
        if board.contains(x, y) {
            let col = ((x - board.x as f32) * SIZE as f32 / board.w as f32) as usize;
            let row = ((y - board.y as f32) * SIZE as f32 / board.h as f32) as usize;
            self.notice = match self.selected {
                Some(piece) => match self.game.place(piece, col, row) {
                    Some(placed) => {
                        self.selected = None;
                        if placed.cleared == 0 {
                            Some(format!("+{} points", placed.points))
                        } else {
                            let label = if placed.cleared == 1 { "area" } else { "areas" };
                            Some(format!(
                                "{} {label} cleared  ·  +{} points",
                                placed.cleared, placed.points
                            ))
                        }
                    }
                    None => Some("That piece doesn't fit here.".into()),
                },
                None => Some("Choose a piece first.".into()),
            };
            self.persist();
            return Change::Partial;
        }
        Change::None
    }

    fn persist(&mut self) {
        if self.save().is_err() {
            self.notice = Some("Could not save the game.".into());
        }
    }
}

impl App for View {
    fn draw(&self, canvas: &mut Canvas) {
        let width = canvas.bounds().w;
        header(canvas, "Blocks", "Done");
        self.draw_score(canvas, width);
        self.draw_board(canvas, width);
        self.draw_pieces(canvas, width);
        let guidance = if self.game.over() {
            "No pieces fit. Start a new game to play again."
        } else if let Some(notice) = &self.notice {
            notice
        } else if self.selected.is_some() {
            "Tap the upper-left square for the piece."
        } else {
            "Choose a piece."
        };
        canvas.text(
            screen_rect(width, 40, 1284, 1184, 48),
            guidance,
            CAPTION.center(),
        );
        canvas.button(new_button(width), "New game", self.game.over(), false);

        if self.confirm {
            canvas.popup(screen_rect(width, 190, 580, 884, 375));
            canvas.text(
                screen_rect(width, 238, 632, 788, 70),
                "Start a new game?",
                TextStyle::new(50).bold(),
            );
            canvas.text(
                screen_rect(width, 238, 720, 788, 65),
                "Your best score stays saved.",
                CAPTION,
            );
            canvas.button(
                screen_rect(width, 238, 808, 360, 100),
                "Cancel",
                false,
                false,
            );
            canvas.button(confirm_button(width), "New game", true, false);
        }
    }

    fn input(&mut self, input: Input) -> Change {
        match input {
            Input::Tap(x, y) => self.tap(x, y),
            Input::PageBack if self.confirm => {
                self.confirm = false;
                Change::Full
            }
            Input::PageBack => Change::Exit,
            _ => Change::None,
        }
    }

    fn save(&mut self) -> io::Result<()> {
        if let Some(path) = &self.path {
            primitive_app::save(path, &serde_json::to_vec(&self.game)?)?;
        }
        Ok(())
    }
}

fn board_cell(width: i32, x: usize, y: usize) -> Rect {
    screen_rect(width, 136 + x as i32 * 110, 286 + y as i32 * 110, 110, 110)
}

fn piece_card(width: i32, index: usize) -> Rect {
    screen_rect(width, 40 + index as i32 * 400, 1340, 384, 138)
}

fn new_button(width: i32) -> Rect {
    screen_rect(width, 412, 1520, 440, 100)
}

fn confirm_button(width: i32) -> Rect {
    screen_rect(width, 666, 808, 360, 100)
}

fn random_seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64
}
