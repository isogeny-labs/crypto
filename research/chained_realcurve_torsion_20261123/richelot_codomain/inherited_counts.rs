//! Exact count/Weil functions selected from published census B.

fn add9(a: u8, b: u8) -> u8 {
    (a % 3 + b % 3) % 3 + 3 * ((a / 3 + b / 3) % 3)
}

fn mul9(a: u8, b: u8) -> u8 {
    (a % 3 * (b % 3) + 2 * (a / 3) * (b / 3)) % 3 + 3 * ((a % 3 * (b / 3) + a / 3 * (b % 3)) % 3)
}

pub(crate) fn point_counts(f: &[u8], extension: bool, infinities: u8) -> (i128, Vec<u8>) {
    let q = if extension { 9 } else { 3 };
    let mut fibres = Vec::new();
    for x in 0..q {
        let mut fx = 0;
        for &coefficient in f.iter().rev() {
            fx = if extension {
                add9(mul9(fx, x), coefficient)
            } else {
                (fx * x + coefficient) % 3
            };
        }
        let count = (0..q)
            .filter(|&y| {
                let square = if extension { mul9(y, y) } else { y * y % 3 };
                square == fx
            })
            .count() as u8;
        fibres.push(count);
    }
    let total = i128::from(infinities) + fibres.iter().map(|&x| i128::from(x)).sum::<i128>();
    (total, fibres)
}

pub(crate) fn weil(n1: i128, n2: i128) -> (i128, i128) {
    let t = 4 - n1;
    let numerator = t * t - 10 + n2;
    assert_eq!(numerator % 2, 0);
    (t, numerator / 2)
}
