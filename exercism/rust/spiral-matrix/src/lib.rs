#[derive(Clone, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

fn change_dir(dir: &Dir) -> Dir {
    match dir {
        Dir::Right => Dir::Down,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Up,
        Dir::Up => Dir::Right,
    }
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix: Vec<Vec<u32>> = Vec::new();

    let mut i = 1;
    while i <= size {
        matrix.push(vec![0; size as usize]);
        i += 1;
    }

    let mut pos = Pos { x: 1, y: 1 };
    let mut dir = Dir::Right;
    i = 1;
    while i <= size * size {
        matrix[pos.y - 1][pos.x - 1] = i;

        // detect next pos
        let mut next_pos = pos.clone();
        // println!("cloned next_pos: {:?}", next_pos);
        match &dir {
            Dir::Right => next_pos.x += 1,
            Dir::Down => next_pos.y += 1,
            Dir::Left => next_pos.x -= 1,
            Dir::Up => next_pos.y -= 1,
        };
        if next_pos.x > size as usize
            || next_pos.x == 0
            || next_pos.y > size as usize
            || next_pos.y == 0
            || matrix[next_pos.y - 1][next_pos.x - 1] > 0
        {
            // if next pos is invalid, change dir
            dir = change_dir(&dir);
            match &dir {
                Dir::Right => pos.x += 1,
                Dir::Down => pos.y += 1,
                Dir::Left => pos.x -= 1,
                Dir::Up => pos.y -= 1,
            };
        } else {
            // if next pos is valid, use it
            pos = next_pos;
        }
        // println!("{}: cur dir: {:?}, next pos - {:?}", size, dir, pos);

        i += 1;
    }

    matrix
}
