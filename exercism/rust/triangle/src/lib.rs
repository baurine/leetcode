use std::ops::Add;

pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T: Copy + PartialEq + PartialOrd + Add<Output = T>> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let mut sorted_sides = sides;
        // sort() need T implement Ord trait, but float doesn't implement Ord trait
        // sorted_sides.sort();
        sorted_sides.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let [a, b, c] = sorted_sides;
        if a + b == b || a + b < c {
            return None;
        }
        return Some(Triangle { a, b, c });
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.b == self.c
    }
}
