use crossterm::style::Color;

const EXPLOSION_CHARS: &[char] = &['*', '\'', '.', '`', '#', '^', '@', '$', ' '];
const RISING_CHARS: &[char] = &['|', '^', '.', ',', '*', '$', ')', '(', '\\', '/', ' '];

#[derive(Debug, Clone)]
pub struct Cell {
    character: char,
    color: Color,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            character: ' ',
            color: Color::Reset,
        }
    }
}

impl Cell {
    pub fn set_empty(&mut self) {
        self.character = ' ';
    }

    pub fn set_rising(&mut self) {
        self.character = RISING_CHARS[rand::random::<usize>() % RISING_CHARS.len()]
    }

    pub fn set_explosion(&mut self) {
        self.character = EXPLOSION_CHARS[rand::random::<usize>() % EXPLOSION_CHARS.len()]
    }

    pub fn set_character(&mut self, character: char) {
        self.character = character
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color
    }
}
