#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        let next_d = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Robot { d: next_d, ..self }
    }

    pub fn turn_left(self) -> Self {
        let next_d = match self.d {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Robot { d: next_d, ..self }
    }

    pub fn advance(self) -> Self {
        let mut next_x = self.x;
        let mut next_y = self.y;
        match self.d {
            Direction::North => next_y += 1,
            Direction::East => next_x += 1,
            Direction::South => next_y -= 1,
            Direction::West => next_x -= 1,
        };
        Robot {
            x: next_x,
            y: next_y,
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |accu, c| match c {
            'R' => accu.turn_right(),
            'L' => accu.turn_left(),
            'A' => accu.advance(),
            _ => accu,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
