#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(ChessPosition { rank, file }),
            _ => None,
        }
    }

    pub fn can_attack(&self, other: &ChessPosition) -> bool {
        self.rank == other.rank
            || self.file == other.file
            || (self.rank - other.rank).abs() == (self.file - other.file).abs()
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { pos: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.pos.can_attack(&other.pos)
    }
}
