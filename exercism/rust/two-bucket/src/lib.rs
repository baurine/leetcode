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

// 判断两个桶能否实现目标容量，通过扩展欧几里德算法，对 bucket_1_cap 依次乘以 1..n，然后对 bucket_2_cap 取模
// 如果有一个余数等于 goal，则可以
// 如果没有余数等于 goal，则不可以
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
        // 如果余数开始出现循环，则说明这两个桶已经不可能实现目标容量
        if i > 1 && remain == first_remain {
            return false;
        }
    }
    false
}

// 扩展欧几里德算法的实现
pub fn ext_euclid(bucket_1_cap: u8, bucket_2_cap: u8, goal: u8) -> (u8, u8, u8) {
    let mut moves = 0_u8;
    let mut bucket_1_cur = 0_u8;
    let mut bucket_2_cur = 0_u8;
    let (goal_bucket_cap, other_bucket) = loop {
        if bucket_1_cur == 0 {
            // 如果桶 1 当前容量为 0，则填充之
            bucket_1_cur = bucket_1_cap;
        } else if bucket_2_cur == 0 && bucket_2_cap == goal {
            // 处理特殊情况
            // 如果桶 2 当前容量为 0，且它的最大容量和目标容量一样，则填充桶 2
            bucket_2_cur = bucket_2_cap;
        } else if bucket_1_cur < bucket_2_cap - bucket_2_cur {
            // 如果桶 1 当前容量小于桶 2 的剩余空间，则把桶 1 的所有水倒入桶 2
            // 如此操作后，桶 1 为 0，桶 2 为原来的容量加上桶 1 的容量
            bucket_2_cur += bucket_1_cur;
            bucket_1_cur = 0;
        } else {
            // 如果桶 1 当前容量大于等于桶 2 的剩余空间，则用桶 1 的一部分水倒满桶 2
            // 如此操作后，桶 1 还剩余一部分容量，桶 2 则被倒满
            bucket_1_cur -= bucket_2_cap - bucket_2_cur;
            bucket_2_cur = bucket_2_cap;
        }
        moves += 1;
        // 判断两个桶是否达到目标容量
        if bucket_1_cur == goal {
            break (bucket_1_cap, bucket_2_cur);
        }
        if bucket_2_cur == goal {
            break (bucket_2_cap, bucket_1_cur);
        }
        if bucket_2_cur == bucket_2_cap {
            // 如果桶 2 被倒满，则把桶 2 清空
            bucket_2_cur = 0;
            moves += 1;
        }
        // 如此反复操作，直到达到目标
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
    // 如果 start_bucket 为桶 2，则把桶 2 和桶 1 进行交换，把桶 2 放在左边，把桶 1 放到右边
    // 即 solve(bucket_1, bucket_2, goal, &Bucket::One) == solve(bucket_2, bucket_1, goal, &Bucket::Two)
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
