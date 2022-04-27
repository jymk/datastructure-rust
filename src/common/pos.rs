#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pos {
    Left,
    Right,
}

impl Default for Pos {
    fn default() -> Self {
        Pos::Left
    }
}
