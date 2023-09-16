use crate::screen::Grid;

// logo represent
pub trait Turtle {
    fn update(&mut self);
    fn draw(&mut self, screen: &mut Grid);

    fn tick(&mut self, screen: &mut Grid) {
        self.draw(screen);
        self.update();
    }

    fn finished(&self) -> bool {
        false
    }
}
