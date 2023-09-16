use crate::{cell::Cell, screen::Grid};
use crossterm::style::Color;

type Coords = (i32, i32);

const UP: Coords = (0, -1);
const DOWN: Coords = (0, 1);
const LEFT: Coords = (-1, 0);
const RIGHT: Coords = (1, 0);
const UP_LEFT: Coords = (-1, -1);
const UP_RIGHT: Coords = (1, -1);
const DOWN_LEFT: Coords = (-1, 1);
const DOWN_RIGHT: Coords = (1, 1);

#[derive(Debug, Clone)]
pub enum Bearing {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Bearing {
    pub fn all_bearings() -> [Self; 8] {
        [
            Self::Up,
            Self::Down,
            Self::Left,
            Self::Right,
            Self::UpLeft,
            Self::UpRight,
            Self::DownLeft,
            Self::DownRight,
        ]
    }

    pub fn to_coords(&self) -> Coords {
        match self {
            Self::Up => UP,
            Self::Down => DOWN,
            Self::Left => LEFT,
            Self::Right => RIGHT,
            Self::UpLeft => UP_LEFT,
            Self::UpRight => UP_RIGHT,
            Self::DownLeft => DOWN_LEFT,
            Self::DownRight => DOWN_RIGHT,
        }
    }
}

pub struct Direction {
    x: i32,
    y: i32,
    bearing: Bearing,
    distance: i32,
    eraseto: i32,
    flare: bool,
    colors: Vec<Color>,
}

impl Direction {
    pub fn new(
        x: i32,
        y: i32,
        bearing: Bearing,
        distance: i32,
        flare: bool,
        colors: Vec<Color>,
    ) -> Self {
        Self {
            x,
            y,
            bearing,
            distance,
            eraseto: 0,
            flare,
            colors,
        }
    }

    pub fn spread(&mut self, distance: i32) {
        self.distance += distance;
        self.eraseto += distance;
    }

    fn write(&mut self, grid: &mut Grid, distance: i32, f: impl Fn(&mut Cell)) {
        let mut x = self.x;
        let mut y = self.y;
        let coords = self.bearing.to_coords();

        for _ in 0..distance {
            x += coords.0;
            y += coords.1;

            if x < 0 || y < 0 {
                continue;
            }

            let y: usize = y.try_into().unwrap();
            let x: usize = x.try_into().unwrap();

            if y < grid.len() && x < grid[y].len() {
                f(&mut grid[y][x])
            }
        }
    }

    pub fn clear(&mut self, grid: &mut Grid) {
        self.write(grid, self.distance, |cell| cell.set_empty());
    }

    pub fn draw(&mut self, grid: &mut Grid) {
        let flare = self.flare.clone();
        let bearing = self.bearing.clone();
        let colors = self.colors.clone();
        self.write(grid, self.distance, {
            |cell| {
                if flare {
                    cell.set_color(colors[rand::random::<usize>() % colors.len()]);
                    cell.set_explosion();
                } else if matches!(bearing, Bearing::Up) {
                    cell.set_color(colors[rand::random::<usize>() % colors.len()]);
                    cell.set_rising();
                }
            }
        });
        self.erase(grid);
    }

    pub fn erase(&mut self, grid: &mut Grid) {
        self.write(grid, self.eraseto, |cell| cell.set_empty());
    }
}
