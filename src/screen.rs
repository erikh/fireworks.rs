use crate::{cell::Cell, turtle::Turtle};
use std::{cell::RefCell, rc::Rc};

pub type Grid = Vec<Vec<Cell>>;

type RcTurtle<'a> = Rc<RefCell<&'a mut dyn Turtle>>;

pub struct Screen<'a> {
    turtles: Vec<RcTurtle<'a>>,
    grid: Grid,
}

impl<'a> Screen<'a> {
    pub fn new(lines: usize, cols: usize) -> Self {
        let mut grid = Vec::new();
        for _ in 0..lines {
            let mut inner = Vec::new();
            for _ in 0..cols {
                inner.push(Cell::default());
            }
            grid.push(inner);
        }

        Self {
            grid,
            turtles: Vec::new(),
        }
    }

    pub fn add_turtle(&mut self, turtle: RcTurtle<'a>) {
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
