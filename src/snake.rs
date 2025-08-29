use egui::{Color32, Painter, Pos2, Rect, Stroke, Ui, StrokeKind};
use rand::prelude::*;
use rand::{thread_rng, rngs::ThreadRng};
use serde::{Deserialize, Serialize};


#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SnakeGame {
    grid_size: (usize, usize),
    cell_size: f32,
    snake: Vec<(usize, usize)>,
    dir: Direction,
    next_dir: Direction,
    food: (usize, usize),
    game_over: bool,
    timer: f32,
    speed: f32,
    #[serde(skip)]
    rng: ThreadRng,
}

impl Default for SnakeGame {
    fn default() -> Self {
        let grid_size = (20, 20);
        let mut rng = thread_rng();
        let snake = vec![(10, 10), (9, 10)];
        let food = Self::random_food(&snake, grid_size, &mut rng);

        Self {
            grid_size,
            cell_size: 20.0,
            snake,
            dir: Direction::Right,
            next_dir: Direction::Right,
            food,
            game_over: false,
            timer: 0.0,
            speed: 0.15, // seconds between moves
            rng,
        }
    }
}

impl SnakeGame {
    pub fn ui(&mut self, ui: &mut Ui, dt: f32) {
        use egui::Key;

        // Keyboard input
        if ui.input(|i| i.key_pressed(Key::ArrowUp)) && self.dir != Direction::Down {
            self.next_dir = Direction::Up;
        }
        if ui.input(|i| i.key_pressed(Key::ArrowDown)) && self.dir != Direction::Up {
            self.next_dir = Direction::Down;
        }
        if ui.input(|i| i.key_pressed(Key::ArrowLeft)) && self.dir != Direction::Right {
            self.next_dir = Direction::Left;
        }
        if ui.input(|i| i.key_pressed(Key::ArrowRight)) && self.dir != Direction::Left {
            self.next_dir = Direction::Right;
        }

        // Layout
        let desired_size = egui::Vec2::new(
            self.grid_size.0 as f32 * self.cell_size,
            self.grid_size.1 as f32 * self.cell_size,
        );
        let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
        let painter = ui.painter_at(rect);

        // Update game logic
        if !self.game_over {
            self.timer += dt;
            if self.timer >= self.speed {
                self.timer = 0.0;
                self.step();
            }
        }

        // Draw game
        self.draw(&painter, rect);
    }

    fn step(&mut self) {
        if self.game_over {
            return;
        }

        self.dir = self.next_dir;
        let (dx, dy) = match self.dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let (head_x, head_y) = self.snake[0];
        let new_head = (
            (head_x as isize + dx) as usize,
            (head_y as isize + dy) as usize,
        );

        // Check bounds
        if new_head.0 >= self.grid_size.0 || new_head.1 >= self.grid_size.1 {
            self.game_over = true;
            return;
        }

        // Check self-collision
        if self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        self.snake.insert(0, new_head);

        if new_head == self.food {
            self.food = Self::random_food(&self.snake, self.grid_size, &mut self.rng);
        } else {
            self.snake.pop();
        }
    }

    fn draw(&self, painter: &Painter, rect: Rect) {
        let cell = self.cell_size;

        // Draw background
        painter.rect_filled(rect, 0.0, Color32::BLACK);

        // Draw food
        self.draw_cell(painter, rect, self.food, Color32::RED);

        // Draw snake
        for (i, &pos) in self.snake.iter().enumerate() {
            let color = if i == 0 { Color32::LIGHT_GREEN } else { Color32::GREEN };
            self.draw_cell(painter, rect, pos, color);
        }

        // Game over message
        if self.game_over {
            let center = rect.center();
            painter.text(
                center,
                egui::Align2::CENTER_CENTER,
                "Game Over! Press R to restart.",
                egui::TextStyle::Heading.resolve(&painter.ctx().style()),
                Color32::WHITE,
            );
        }
    }

    fn draw_cell(&self, painter: &Painter, rect: Rect, pos: (usize, usize), color: Color32) {
        let cell = self.cell_size;
        let x = rect.left() + pos.0 as f32 * cell;
        let y = rect.top() + pos.1 as f32 * cell;
        let r = Rect::from_min_size(Pos2::new(x, y), egui::vec2(cell, cell));
        painter.rect_filled(r.shrink(1.0), 2.0, color);
        painter.rect_stroke(r.shrink(1.0), 2.0, Stroke::new(0.5, Color32::DARK_GRAY), StrokeKind::Inside);
    }

    fn random_food(
        snake: &[(usize, usize)],
        grid: (usize, usize),
        rng: &mut ThreadRng,
    ) -> (usize, usize) {
        let mut empty = vec![];
        for x in 0..grid.0 {
            for y in 0..grid.1 {
                if !snake.contains(&(x, y)) {
                    empty.push((x, y));
                }
            }
        }
        *empty.choose(rng).unwrap_or(&(0, 0))
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
