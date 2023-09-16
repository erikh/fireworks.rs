use crate::{
    direction::{Bearing, Direction},
    screen::Grid,
    turtle::Turtle,
};
use crossterm::style::Color;

const BRIGHT_WHITE: Color = Color::Rgb {
    r: 255,
    g: 255,
    b: 255,
};

const WHITE: Color = Color::Rgb {
    r: 128,
    g: 128,
    b: 128,
};

const RED: Color = Color::Rgb {
    r: 255,
    g: 32,
    b: 32,
};

const BRIGHT_RED: Color = Color::Rgb {
    r: 255,
    g: 64,
    b: 64,
};

const BLUE: Color = Color::Rgb {
    b: 255,
    g: 32,
    r: 32,
};

const BRIGHT_BLUE: Color = Color::Rgb {
    b: 255,
    g: 64,
    r: 64,
};

const GREEN: Color = Color::Rgb {
    g: 255,
    b: 32,
    r: 32,
};

const BRIGHT_GREEN: Color = Color::Rgb {
    g: 255,
    b: 64,
    r: 64,
};

const CYAN: Color = Color::Rgb {
    g: 128,
    b: 255,
    r: 32,
};

const BRIGHT_CYAN: Color = Color::Rgb {
    g: 128,
    b: 255,
    r: 64,
};

const MAGENTA: Color = Color::Rgb {
    r: 128,
    b: 255,
    g: 32,
};

const BRIGHT_MAGENTA: Color = Color::Rgb {
    r: 128,
    b: 255,
    g: 64,
};

const YELLOW: Color = Color::Rgb {
    r: 128,
    g: 255,
    b: 32,
};

const BRIGHT_YELLOW: Color = Color::Rgb {
    r: 128,
    g: 255,
    b: 64,
};

lazy_static::lazy_static! {
    static ref COLORS: Vec<Vec<Color>> = vec![
        vec![WHITE, BRIGHT_WHITE],
        vec![RED, BRIGHT_RED, WHITE, BRIGHT_WHITE],
        vec![BLUE, BRIGHT_BLUE, WHITE, BRIGHT_WHITE],
        vec![GREEN, BRIGHT_GREEN, WHITE, BRIGHT_WHITE],
        vec![CYAN, BRIGHT_CYAN, WHITE, BRIGHT_WHITE],
        vec![MAGENTA, BRIGHT_MAGENTA, WHITE, BRIGHT_WHITE],
        vec![YELLOW, BRIGHT_YELLOW, WHITE, BRIGHT_WHITE],
    ];
}

const MAX_ITERATIONS: usize = 10;
const MAX_DRAW_ITERATIONS: usize = 30;

pub struct Firework {
    detonated: bool,
    trail: usize,
    embers: Vec<Direction>,
    speed: usize,
    total_iterations: usize,
    iterations_since_last_animation: usize,
    max_iterations_before_detonation: usize,
    base_x: usize,
    lines: usize,
    colors: Vec<Color>,
    is_finished: bool,
}

impl Firework {
    pub fn new(lines: usize, cols: usize) -> Self {
        let base_x = rand::random::<usize>() % (cols - 1);
        let speed = (rand::random::<usize>() % 3) + 1;
        let colors = &COLORS[rand::random::<usize>() % COLORS.len()];
        Self {
            colors: colors.clone(),
            speed,
            base_x,
            lines,
            embers: vec![Direction::new(
                base_x.try_into().unwrap(),
                lines.try_into().unwrap(),
                Bearing::Up,
                speed.try_into().unwrap(),
                false,
                colors.clone(),
            )],
            max_iterations_before_detonation: (lines as f64 / speed as f64 * 0.8 * 2.0).ceil()
                as usize,
            trail: 1,
            iterations_since_last_animation: 0,
            is_finished: false,
            detonated: false,
            total_iterations: 0,
        }
    }

    fn create_embers(&mut self) {
        let mut embers = Vec::new();

        for bearing in Bearing::all_bearings() {
            embers.push(Direction::new(
                self.base_x.try_into().unwrap(),
                (self.lines - self.trail).try_into().unwrap(),
                bearing,
                (rand::random::<i32>() % 10).abs() + 1,
                true,
                self.colors.clone(),
            ))
        }

        self.embers = embers
    }
}

impl Turtle for Firework {
    fn finished(&self) -> bool {
        self.is_finished
    }

    fn draw(&mut self, grid: &mut Grid) {
        if self.iterations_since_last_animation < MAX_DRAW_ITERATIONS {
            for ember in &mut self.embers {
                ember.draw(grid)
            }
        } else if !self.is_finished {
            for ember in &mut self.embers {
                ember.clear(grid)
            }

            self.embers = Vec::new();
            self.is_finished = true;
        }
    }

    fn update(&mut self, grid: &mut Grid) {
        if self.detonated {
            for ember in &mut self.embers {
                ember.spread(self.speed.try_into().unwrap());
                self.iterations_since_last_animation += 1;
            }
        } else if self.trail < self.lines + self.speed {
            if self.iterations_since_last_animation == MAX_ITERATIONS {
                self.embers[0].spread(self.speed.try_into().unwrap());
                self.trail += self.speed;
                self.iterations_since_last_animation = 0;
            } else if rand::random::<usize>()
                % (MAX_ITERATIONS - self.iterations_since_last_animation)
                == 0
            {
                self.embers[0].spread(self.speed.try_into().unwrap());
                self.trail += self.speed;
                self.iterations_since_last_animation = 0;
            } else {
                self.iterations_since_last_animation = 1;
            }

            if rand::random::<usize>()
                % (self.max_iterations_before_detonation - self.total_iterations)
                == 0
            {
                self.embers[0].clear(grid);
                self.detonated = true;
                self.iterations_since_last_animation = 0;
                self.create_embers();
            }
        } else {
            self.is_finished = true;
        }
    }
}
