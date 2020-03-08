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

pub fn is_solvable(bucket_1_cap: u8, bucket_2_cap: u8, goal: u8) -> bool {
    if bucket_1_cap == goal || bucket_2_cap == goal {
        return true;
    }
    let first_remain: u8 = bucket_1_cap % bucket_2_cap;
    for i in 1.. {
        let remain = i * bucket_1_cap % bucket_2_cap;
        if remain == goal {
            return true;
        }
        if i > 1 && remain == first_remain {
            return false;
        }
    }
    false
}

pub fn ext_euclid(bucket_1_cap: u8, bucket_2_cap: u8, goal: u8) -> (u8, u8, u8) {
    let mut moves = 0_u8;
    let mut bucket_1_cur = 0_u8;
    let mut bucket_2_cur = 0_u8;
    let (goal_bucket_cap, other_bucket) = loop {
        if bucket_1_cur == 0 {
            bucket_1_cur = bucket_1_cap;
        } else if bucket_2_cur == 0 && bucket_2_cap == goal {
            bucket_2_cur = bucket_2_cap;
        } else if bucket_1_cur < bucket_2_cap - bucket_2_cur {
            bucket_2_cur += bucket_1_cur;
            bucket_1_cur = 0;
        } else {
            bucket_1_cur = bucket_1_cur - (bucket_2_cap - bucket_2_cur);
            bucket_2_cur = bucket_2_cap;
        }
        moves += 1;
        if bucket_1_cur == goal {
            break (bucket_1_cap, bucket_2_cur);
        }
        if bucket_2_cur == goal {
            break (bucket_2_cap, bucket_1_cur);
        }
        if bucket_2_cur == bucket_2_cap {
            bucket_2_cur = 0;
            moves += 1;
        }
    };
    (moves, goal_bucket_cap, other_bucket)
}

/// Solve the bucket problem
pub fn solve(
    bucket_1_cap: u8,
    bucket_2_cap: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    if start_bucket == &Bucket::One && !is_solvable(bucket_1_cap, bucket_2_cap, goal) {
        return None;
    }
    if start_bucket == &Bucket::Two && !is_solvable(bucket_2_cap, bucket_1_cap, goal) {
        return None;
    }
    let (moves, goal_bucket_cap, other_bucket) = if start_bucket == &Bucket::One {
        ext_euclid(bucket_1_cap, bucket_2_cap, goal)
    } else {
        ext_euclid(bucket_2_cap, bucket_1_cap, goal)
    };
    Some(BucketStats {
        moves,
        goal_bucket: if goal_bucket_cap == bucket_1_cap {
            Bucket::One
        } else {
            Bucket::Two
        },
        other_bucket,
    })
}
