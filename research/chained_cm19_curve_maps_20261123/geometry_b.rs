use std::collections::BTreeMap;

pub type Point = [u8; 3];
pub type Matrix = [[u8; 3]; 3];
pub type Form = BTreeMap<[u8; 3], u8>;
pub const ID: Matrix = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
pub const DECK: Matrix = [[1, 0, 0], [0, 1, 0], [0, 0, 6]];

pub fn add(a: u8, b: u8) -> u8 {
    (a + b) % 7
}

pub fn sub(a: u8, b: u8) -> u8 {
    (a + 7 - b) % 7
}

pub fn mul(a: u8, b: u8) -> u8 {
    a * b % 7
}

pub fn pow(mut a: u8, mut n: u32) -> u8 {
    let mut out = 1;
    while n != 0 {
        if n & 1 != 0 {
            out = mul(out, a);
        }
        a = mul(a, a);
        n >>= 1;
    }
    out
}

pub fn inv(a: u8) -> u8 {
    assert_ne!(a, 0);
    let out = pow(a, 5);
    assert_eq!(mul(a, out), 1);
    out
}

pub fn dot(a: Point, b: Point) -> u8 {
    a.into_iter().zip(b).fold(0, |n, (x, y)| add(n, mul(x, y)))
}

pub fn mv(a: Matrix, p: Point) -> Point {
    a.map(|row| dot(row, p))
}

pub fn mm(a: Matrix, b: Matrix) -> Matrix {
    std::array::from_fn(|i| std::array::from_fn(|j| dot(a[i], [b[0][j], b[1][j], b[2][j]])))
}

pub fn mscale(a: Matrix, t: u8) -> Matrix {
    a.map(|row| row.map(|x| mul(x, t)))
}

pub fn det(a: Matrix) -> u8 {
    add(
        sub(
            mul(a[0][0], sub(mul(a[1][1], a[2][2]), mul(a[1][2], a[2][1]))),
            mul(a[0][1], sub(mul(a[1][0], a[2][2]), mul(a[1][2], a[2][0]))),
        ),
        mul(a[0][2], sub(mul(a[1][0], a[2][1]), mul(a[1][1], a[2][0]))),
    )
}

pub fn inverse(a: Matrix) -> Option<Matrix> {
    let determinant = det(a);
    if determinant == 0 {
        return None;
    }
    let out = std::array::from_fn(|i| {
        std::array::from_fn(|j| {
            let rows: Vec<_> = (0..3).filter(|&r| r != j).collect();
            let cols: Vec<_> = (0..3).filter(|&c| c != i).collect();
            let minor = sub(
                mul(a[rows[0]][cols[0]], a[rows[1]][cols[1]]),
                mul(a[rows[0]][cols[1]], a[rows[1]][cols[0]]),
            );
            mul(
                mul(minor, if (i + j) & 1 == 0 { 1 } else { 6 }),
                inv(determinant),
            )
        })
    });
    assert_eq!(mm(a, out), ID);
    assert_eq!(mm(out, a), ID);
    Some(out)
}

pub fn canonical(p: Point) -> Point {
    let first = p.into_iter().find(|&x| x != 0).expect("projective nonzero");
    p.map(|x| mul(x, inv(first)))
}

pub fn canonical_matrix(a: Matrix) -> Matrix {
    let first = a
        .into_iter()
        .flatten()
        .find(|&x| x != 0)
        .expect("projective matrix nonzero");
    mscale(a, inv(first))
}

pub fn frame_basis(frame: [Point; 4]) -> Option<Matrix> {
    let a = std::array::from_fn(|i| [frame[0][i], frame[1][i], frame[2][i]]);
    let coordinates = mv(inverse(a)?, frame[3]);
    if coordinates.contains(&0) {
        return None;
    }
    Some(std::array::from_fn(|i| {
        std::array::from_fn(|j| mul(a[i][j], coordinates[j]))
    }))
}

pub fn term(exponents: [u8; 3], coefficient: u8) -> Form {
    if coefficient == 0 {
        Form::new()
    } else {
        Form::from([(exponents, coefficient)])
    }
}

pub fn fadd(a: &Form, b: &Form) -> Form {
    let mut out = a.clone();
    for (&key, &value) in b {
        let entry = out.entry(key).or_default();
        *entry = add(*entry, value);
    }
    out.retain(|_, value| *value != 0);
    out
}

pub fn fscale(a: &Form, t: u8) -> Form {
    a.iter()
        .filter_map(|(&key, &value)| {
            let product = mul(value, t);
            (product != 0).then_some((key, product))
        })
        .collect()
}

pub fn fsub(a: &Form, b: &Form) -> Form {
    fadd(a, &fscale(b, 6))
}

pub fn fmul(a: &Form, b: &Form) -> Form {
    let mut out = Form::new();
    for (&aa, &x) in a {
        for (&bb, &y) in b {
            let key = std::array::from_fn(|i| aa[i] + bb[i]);
            let entry = out.entry(key).or_default();
            *entry = add(*entry, mul(x, y));
        }
    }
    out.retain(|_, value| *value != 0);
    out
}

pub fn fpow(a: &Form, exponent: u8) -> Form {
    (0..exponent).fold(term([0, 0, 0], 1), |out, _| fmul(&out, a))
}

pub fn evaluate(f: &Form, p: Point) -> u8 {
    f.iter().fold(0, |out, (&powers, &coefficient)| {
        add(
            out,
            p.into_iter()
                .zip(powers)
                .fold(coefficient, |value, (x, n)| mul(value, pow(x, n as u32))),
        )
    })
}

pub fn transform(f: &Form, a: Matrix) -> Form {
    let rows: Vec<_> = a
        .into_iter()
        .map(|row| {
            row.into_iter()
                .enumerate()
                .fold(Form::new(), |out, (i, value)| {
                    let mut key = [0; 3];
                    key[i] = 1;
                    fadd(&out, &term(key, value))
                })
        })
        .collect();
    f.iter()
        .fold(Form::new(), |out, (&exponents, &coefficient)| {
            let product = rows
                .iter()
                .zip(exponents)
                .fold(term([0, 0, 0], coefficient), |acc, (row, exponent)| {
                    fmul(&acc, &fpow(row, exponent))
                });
            fadd(&out, &product)
        })
}

pub fn coefficients(f: &Form) -> Vec<u8> {
    let mut out = Vec::new();
    for i in 0..=4 {
        for j in 0..=4 - i {
            out.push(*f.get(&[i, j, 4 - i - j]).unwrap_or(&0));
        }
    }
    assert_eq!(out.len(), 15);
    out
}

pub fn curve() -> Form {
    let x = term([1, 0, 0], 1);
    let y = term([0, 1, 0], 1);
    let z = term([0, 0, 1], 1);
    let g = fadd(
        &fadd(&fscale(&fpow(&x, 2), 6), &fscale(&fmul(&x, &y), 3)),
        &fpow(&y, 2),
    );
    let v = fadd(&fpow(&z, 2), &g);
    let d = fmul(
        &y,
        &fadd(
            &fadd(&fpow(&x, 3), &fscale(&fmul(&x, &fpow(&y, 2)), 2)),
            &fscale(&fpow(&y, 3), 6),
        ),
    );
    fsub(&fpow(&v, 2), &d)
}

pub fn proportional(f: &Form, g: &Form) -> Option<u8> {
    let (&key, &coefficient) = g.first_key_value().unwrap();
    let scale = mul(*f.get(&key).unwrap_or(&0), inv(coefficient));
    if scale != 0 && *f == fscale(g, scale) {
        Some(scale)
    } else {
        None
    }
}

pub fn reduce_w2(mut f: Form, coefficients: [u8; 5]) -> Form {
    while let Some((&key, &value)) = f.iter().find(|(key, _)| key[1] >= 2) {
        assert_eq!(key[2], 0);
        f.remove(&key);
        for (power, coefficient) in coefficients.into_iter().enumerate() {
            f = fadd(
                &f,
                &term(
                    [key[0] + power as u8, key[1] - 2, 0],
                    mul(value, coefficient),
                ),
            );
        }
    }
    f
}

pub fn eval_poly(coefficients: [u8; 5], x: u8) -> u8 {
    coefficients
        .into_iter()
        .rev()
        .fold(0, |out, value| add(mul(out, x), value))
}
