use crate::{cell::Cell, firework::Firework, turtle::Turtle};
use crossterm::{
    cursor::MoveTo,
    style::{Print, SetForegroundColor},
    ExecutableCommand,
};
use std::{cell::RefCell, io::stdout, rc::Rc};

pub type Grid = Vec<Vec<Cell>>;

pub type RcTurtle = Rc<RefCell<Firework>>;

pub struct Screen {
    turtles: Vec<RcTurtle>,
    grid: Grid,
    last_draw_grid: Option<Grid>,
}

impl Screen {
    pub fn new(lines: usize, cols: usize) -> Self {
        let mut grid = Vec::new();
        for _ in 0..lines {
            let mut inner = Vec::new();
            for _ in 0..cols {
                inner.push(Cell::default());
            }
            grid.push(inner);
        }

        let mut turtles = Vec::new();
        for _ in 0..10 {
            turtles.push(Rc::new(RefCell::new(Firework::new(lines, cols))));
        }

        Self {
            grid,
            turtles,
            last_draw_grid: None,
        }
    }

    pub fn draw(&mut self) -> std::io::Result<()> {
        let new_grid = self.grid.clone();
        let last_draw_grid = self.last_draw_grid.clone().unwrap_or_default();
        let is_last_draw_grid = self.last_draw_grid.is_some();
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if !is_last_draw_grid || self.grid[y][x] != last_draw_grid[y][x] {
                    stdout()
                        .execute(MoveTo(x as u16, y as u16))?
                        .execute(SetForegroundColor(self.grid[y][x].color()))?
                        .execute(Print(&format!("{}", self.grid[y][x].character())))?;
                }
            }
        }
        self.last_draw_grid = Some(new_grid);

        Ok(())
    }

    pub fn add_turtle(&mut self, turtle: RcTurtle) {
        self.turtles.push(turtle);
    }

    pub fn tick(&mut self) {
        let mut new = Vec::new();
        for turtle in &self.turtles {
            if !turtle.borrow().finished() {
                turtle.borrow_mut().tick(&mut self.grid);
                new.push(turtle.clone());
            }
        }
        self.turtles = new;
    }
}
