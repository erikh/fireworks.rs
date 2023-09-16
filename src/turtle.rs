use crate::screen::Grid;

// logo represent
pub trait Turtle {
    fn update(&mut self, grid: &mut Grid);
    fn draw(&mut self, grid: &mut Grid);

    fn tick(&mut self, grid: &mut Grid) {
        self.draw(grid);
        self.update(grid);
    }

    fn finished(&self) -> bool {
        false
    }
}
