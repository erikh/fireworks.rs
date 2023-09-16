use crossterm::style::Color;

const EXPLOSION_CHARS: &[char] = &['*', '\'', '.', '`', '#', '^', '@', '$', ' '];
const RISING_CHARS: &[char] = &['|', '^', '.', ',', '*', '$', ')', '(', '\\', '/', ' '];

#[derive(Debug, Clone, Eq)]
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

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.character == other.character && self.color == other.color
    }
}

impl Cell {
    pub fn color(&self) -> Color {
        self.color
    }

    pub fn character(&self) -> char {
        self.character
    }

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
