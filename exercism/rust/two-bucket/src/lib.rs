use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

pub fn is_solvable(cap_1: u8, cap_2: u8, goal: u8) -> bool {
    if cap_1 == goal || cap_2 == goal {
        return true;
    }
    let mut remains: HashSet<u8> = HashSet::new();
    for i in 1.. {
        let remain = i * cap_1 % cap_2;
        if remain == goal {
            return true;
        }
        if remains.contains(&remain) {
            return false;
        }
        remains.insert(remain);
    }
    false
}

pub fn ext_euclid(cap_1: u8, cap_2: u8, goal: u8) -> (u8, u8, u8) {
    let mut moves = 0_u8;
    let mut cur_1 = 0_u8;
    let mut cur_2 = 0_u8;
    let (goal_cap, other_liter) = loop {
        if cur_1 == 0 {
            cur_1 = cap_1;
            moves += 1;
        } else if cur_2 == 0 && cap_2 == goal {
            cur_2 = cap_2;
            moves += 1;
        } else if cur_1 < cap_2 - cur_2 {
            cur_2 += cur_1;
            cur_1 = 0;
            moves += 1;
        } else {
            cur_1 = cur_1 - (cap_2 - cur_2);
            cur_2 = cap_2;
            moves += 1;
        }
        if cur_1 == goal {
            break (cap_1, cur_2);
        }
        if cur_2 == goal {
            break (cap_2, cur_1);
        }
        if cur_2 == cap_2 {
            cur_2 = 0;
            moves += 1;
        }
    };
    (moves, goal_cap, other_liter)
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    if start_bucket == &Bucket::One && !is_solvable(capacity_1, capacity_2, goal) {
        return None;
    }
    if start_bucket == &Bucket::Two && !is_solvable(capacity_2, capacity_1, goal) {
        return None;
    }
    let (moves, goal_cap, other_liter) = if start_bucket == &Bucket::One {
        ext_euclid(capacity_1, capacity_2, goal)
    } else {
        ext_euclid(capacity_2, capacity_1, goal)
    };
    Some(BucketStats {
        moves,
        goal_bucket: if goal_cap == capacity_1 {
            Bucket::One
        } else {
            Bucket::Two
        },
        other_bucket: other_liter,
    })
}
