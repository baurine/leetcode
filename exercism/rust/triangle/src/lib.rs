use std::ops::Add;

#[derive(PartialEq)]
enum TriangleKind {
    Equilateral,
    Isosceles,
    Scalene,
}

pub struct Triangle<T> {
    kind: TriangleKind,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Copy + PartialEq + PartialOrd + Add<Output = T>> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let mut sorted_sides = sides;
        // sorted_sides.sort(); // sort() need T implement Ord trait, but float doesn't implement Ord trait
        sorted_sides.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if sorted_sides[0] + sorted_sides[1] == sorted_sides[1]
            || sorted_sides[0] + sorted_sides[1] < sorted_sides[2]
        {
            return None;
        }
        if sorted_sides[0] == sorted_sides[1] && sorted_sides[1] == sorted_sides[2] {
            return Some(Triangle {
                kind: TriangleKind::Equilateral,
                _marker: std::marker::PhantomData,
            });
        }
        if sorted_sides[0] == sorted_sides[1] || sorted_sides[1] == sorted_sides[2] {
            return Some(Triangle {
                kind: TriangleKind::Isosceles,
                _marker: std::marker::PhantomData,
            });
        }
        return Some(Triangle {
            kind: TriangleKind::Scalene,
            _marker: std::marker::PhantomData,
        });
    }

    pub fn is_equilateral(&self) -> bool {
        self.kind == TriangleKind::Equilateral
    }

    pub fn is_scalene(&self) -> bool {
        self.kind == TriangleKind::Scalene
    }

    pub fn is_isosceles(&self) -> bool {
        self.is_equilateral() || self.kind == TriangleKind::Isosceles
    }
}
