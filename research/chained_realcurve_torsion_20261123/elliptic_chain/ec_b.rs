#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: u32,
    pub y: u32,
    pub infinity: bool,
}

pub const IDENTITY: Point = Point {
    x: 0,
    y: 0,
    infinity: true,
};

pub fn on_curve(p: Point) -> bool {
    if p.infinity {
        return p == IDENTITY;
    }
    p.x < 3 && p.y < 3 && p.y * p.y % 3 == (p.x * p.x * p.x + 2 * p.x + 1) % 3
}

pub fn points() -> Vec<Point> {
    let mut result = vec![IDENTITY];
    for x in 0..3 {
        for y in 0..3 {
            let p = Point {
                x,
                y,
                infinity: false,
            };
            if on_curve(p) {
                result.push(p);
            }
        }
    }
    assert_eq!(result.len(), 7);
    result
}

pub fn add(p: Point, q: Point) -> Point {
    assert!(on_curve(p) && on_curve(q));
    if p.infinity {
        return q;
    }
    if q.infinity {
        return p;
    }
    if p.x == q.x && (p.y + q.y).is_multiple_of(3) {
        return IDENTITY;
    }
    let (numerator, denominator) = if p == q {
        ((3 * p.x * p.x + 2) % 3, 2 * p.y % 3)
    } else {
        ((q.y + 3 - p.y) % 3, (q.x + 3 - p.x) % 3)
    };
    assert_ne!(denominator, 0);
    let slope = numerator * denominator % 3;
    let x = (slope * slope + 6 - p.x - q.x) % 3;
    let y = (slope * ((p.x + 3 - x) % 3) + 3 - p.y) % 3;
    let result = Point {
        x,
        y,
        infinity: false,
    };
    assert!(on_curve(result));
    result
}

pub fn mul(p: Point, mut n: u32) -> Point {
    let mut result = IDENTITY;
    let mut base = p;
    while n != 0 {
        if n & 1 == 1 {
            result = add(result, base);
        }
        n >>= 1;
        if n != 0 {
            base = add(base, base);
        }
    }
    result
}
