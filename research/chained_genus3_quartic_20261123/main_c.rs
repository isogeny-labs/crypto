// Fixed F5 finite correctness population. No Python science or timing claim.
#[derive(Clone)]
struct Field {
    degree: usize,
    reduction: Vec<usize>,
    q: usize,
    plus: Vec<usize>,
    times: Vec<usize>,
    roots: Vec<usize>,
}

impl Field {
    fn digits(&self, mut a: usize) -> Vec<usize> {
        let mut out = vec![0; self.degree];
        for value in &mut out {
            *value = a % 5;
            a /= 5;
        }
        assert_eq!(a, 0);
        out
    }

    fn encode(values: &[usize]) -> usize {
        values.iter().rev().fold(0, |a, &b| a * 5 + b)
    }

    fn add_reference(&self, a: usize, b: usize) -> usize {
        let aa = self.digits(a);
        let bb = self.digits(b);
        Self::encode(
            &aa.iter()
                .zip(bb)
                .map(|(x, y)| (x + y) % 5)
                .collect::<Vec<_>>(),
        )
    }

    fn product_reference(&self, a: usize, b: usize) -> usize {
        let aa = self.digits(a);
        let bb = self.digits(b);
        let mut product = vec![0; 2 * self.degree - 1];
        for (i, x) in aa.iter().enumerate() {
            for (j, y) in bb.iter().enumerate() {
                product[i + j] = (product[i + j] + x * y) % 5;
            }
        }
        for k in (self.degree..product.len()).rev() {
            let coefficient = product[k];
            for (i, &r) in self.reduction.iter().enumerate() {
                product[k - self.degree + i] = (product[k - self.degree + i] + coefficient * r) % 5;
            }
            product[k] = 0;
        }
        Self::encode(&product[..self.degree])
    }

    fn horner_product(&self, a: usize, b: usize) -> usize {
        let aa = self.digits(a);
        let bb = self.digits(b);
        let mut accum = vec![0; self.degree];
        for &coefficient in bb.iter().rev() {
            let top = accum[self.degree - 1];
            let mut shifted = vec![0; self.degree];
            shifted[1..].copy_from_slice(&accum[..self.degree - 1]);
            for (i, value) in shifted.iter_mut().enumerate() {
                *value = (*value + top * self.reduction[i] + coefficient * aa[i]) % 5;
            }
            accum = shifted;
        }
        Self::encode(&accum)
    }

    fn new(degree: usize, reduction: Vec<usize>) -> Self {
        assert_eq!(reduction.len(), degree);
        let q = 5usize.pow(degree as u32);
        let mut field = Self {
            degree,
            reduction,
            q,
            plus: vec![0; q * q],
            times: vec![0; q * q],
            roots: vec![0; q],
        };
        for a in 0..q {
            for b in 0..q {
                field.plus[a * q + b] = field.add_reference(a, b);
                field.times[a * q + b] = field.product_reference(a, b);
            }
        }
        for a in 0..q {
            let square = field.mul(a, a);
            field.roots[square] += 1;
        }
        field
    }

    fn add(&self, a: usize, b: usize) -> usize {
        self.plus[a * self.q + b]
    }

    fn mul(&self, a: usize, b: usize) -> usize {
        self.times[a * self.q + b]
    }

    fn neg(&self, a: usize) -> usize {
        self.mul(4, a)
    }

    fn sub(&self, a: usize, b: usize) -> usize {
        self.add(a, self.neg(b))
    }

    fn square(&self, a: usize) -> usize {
        self.mul(a, a)
    }

    fn pow(&self, mut a: usize, mut exponent: usize) -> usize {
        let mut out = 1;
        while exponent != 0 {
            if exponent & 1 == 1 {
                out = self.mul(out, a);
            }
            a = self.square(a);
            exponent >>= 1;
        }
        out
    }

    fn check(&self) {
        for a in 0..self.q {
            assert_eq!(self.pow(a, self.q), a);
            assert_eq!(self.add(a, self.neg(a)), 0);
            if a != 0 {
                assert_eq!(self.mul(a, self.pow(a, self.q - 2)), 1);
            }
            for b in 0..self.q {
                assert_eq!(self.mul(a, b), self.horner_product(a, b));
                assert_eq!(self.add(a, b), self.add(b, a));
                assert_eq!(self.mul(a, b), self.mul(b, a));
                assert_eq!(
                    self.pow(self.add(a, b), 5),
                    self.add(self.pow(a, 5), self.pow(b, 5))
                );
                assert_eq!(
                    self.pow(self.mul(a, b), 5),
                    self.mul(self.pow(a, 5), self.pow(b, 5))
                );
                for c in 0..self.q {
                    assert_eq!(self.add(self.add(a, b), c), self.add(a, self.add(b, c)));
                    assert_eq!(self.mul(self.mul(a, b), c), self.mul(a, self.mul(b, c)));
                    assert_eq!(
                        self.mul(a, self.add(b, c)),
                        self.add(self.mul(a, b), self.mul(a, c))
                    );
                }
            }
        }
        println!(
            "FIELD\tq={}\tdegree={}\treduction={:?}\tpairs={}\ttriples={}\tPASS",
            self.q,
            self.degree,
            self.reduction,
            self.q * self.q,
            self.q * self.q * self.q
        );
    }
}

fn trim(poly: &mut Vec<usize>) {
    while poly.last() == Some(&0) {
        poly.pop();
    }
}

fn inverse5(a: usize) -> usize {
    assert_ne!(a % 5, 0);
    (1..5).find(|x| x * a % 5 == 1).unwrap()
}

fn remainder(mut a: Vec<usize>, b: &[usize]) -> Vec<usize> {
    assert!(!b.is_empty());
    trim(&mut a);
    let leading_inverse = inverse5(*b.last().unwrap());
    while a.len() >= b.len() {
        let shift = a.len() - b.len();
        let scale = a[a.len() - 1] * leading_inverse % 5;
        for (i, &v) in b.iter().enumerate() {
            a[i + shift] = (a[i + shift] + 5 - scale * v % 5) % 5;
        }
        trim(&mut a);
    }
    a
}

fn gcd_degree(mut a: Vec<usize>, mut b: Vec<usize>) -> usize {
    while !b.is_empty() {
        let next = remainder(a, &b);
        a = b;
        b = next;
    }
    a.len() - 1
}

fn sylvester_nullity(a: &[usize], b: &[usize]) -> usize {
    let m = a.len() - 1;
    let n = b.len() - 1;
    let size = m + n;
    let mut matrix = vec![vec![0; size]; size];
    for (shift, row) in matrix[..n].iter_mut().enumerate() {
        row[shift..shift + a.len()].copy_from_slice(a);
    }
    for (shift, row) in matrix[n..].iter_mut().enumerate() {
        row[shift..shift + b.len()].copy_from_slice(b);
    }
    let mut rank = 0;
    for column in 0..size {
        let pivot = (rank..size).find(|&row| matrix[row][column] != 0);
        if let Some(pivot) = pivot {
            matrix.swap(rank, pivot);
            let inv = inverse5(matrix[rank][column]);
            for value in &mut matrix[rank] {
                *value = *value * inv % 5;
            }
            let pivot_row = matrix[rank].clone();
            for row in matrix.iter_mut().skip(rank + 1) {
                let scale = row[column];
                for (entry, &p) in row.iter_mut().zip(&pivot_row) {
                    *entry = (*entry + 5 - scale * p % 5) % 5;
                }
            }
            rank += 1;
        }
    }
    size - rank
}

fn smoothness(a: usize, b: usize, c: usize) -> usize {
    let g = [c, b, a];
    let d = [1, 2, 0, 1, 0];
    let mut h = vec![0; 5];
    for (i, &x) in g.iter().enumerate() {
        for (j, &y) in g.iter().enumerate() {
            h[i + j] = (h[i + j] + x * y) % 5;
        }
    }
    for (value, &v) in h.iter_mut().zip(&d) {
        *value = (*value + 5 - v) % 5;
    }
    trim(&mut h);
    assert_eq!(h.len(), if a == 0 { 4 } else { 5 });
    let mut derivative: Vec<_> = h
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, &x)| i * x % 5)
        .collect();
    trim(&mut derivative);
    let degree = gcd_degree(h.clone(), derivative.clone());
    assert_eq!(degree, sylvester_nullity(&h, &derivative));
    degree
}

fn polynomial(field: &Field, coefficients: &[usize], x: usize) -> usize {
    coefficients
        .iter()
        .rev()
        .fold(0, |acc, &c| field.add(field.mul(acc, x), c))
}

fn elliptic_count(field: &Field) -> usize {
    let mut points = 1;
    for x in 0..field.q {
        let d = polynomial(field, &[1, 2, 0, 1], x);
        for v in 0..field.q {
            points += usize::from(field.square(v) == d);
        }
    }
    points
}

fn counts(field: &Field, a: usize, b: usize, c: usize, kappa: usize) -> usize {
    let mut direct = 0;
    let mut fibre = 0;
    let inverse_kappa = inverse5(kappa);
    for x in 0..field.q {
        let g = polynomial(field, &[c, b, a], x);
        let d = polynomial(field, &[1, 2, 0, 1], x);
        for z in 0..field.q {
            let z2 = field.square(z);
            let first = field.mul(field.square(kappa), field.square(z2));
            let cross = field.mul(field.mul(2, kappa), field.mul(g, z2));
            let expanded = field.sub(field.add(field.add(first, cross), field.square(g)), d);
            direct += usize::from(expanded == 0);
        }
        for v in 0..field.q {
            if field.square(v) == d {
                let rhs = field.mul(inverse_kappa, field.sub(v, g));
                fibre += field.roots[rhs];
            }
        }
    }
    for z in 0..field.q {
        let left = field.add(field.mul(kappa, field.square(z)), a);
        direct += usize::from(field.square(left) == 0);
    }
    fibre += field.roots[field.mul(inverse_kappa, field.neg(a))];
    assert_eq!(direct, fibre, "independent projective counts");
    direct
}

fn weil(counts: [usize; 3]) -> [i64; 7] {
    let s1 = 6 - counts[0] as i64;
    let s2 = 26 - counts[1] as i64;
    let s3 = 126 - counts[2] as i64;
    let a1 = -s1;
    let numerator2 = s1 * s1 - s2;
    let numerator3 = s1 * s1 * s1 - 3 * s1 * s2 + 2 * s3;
    assert_eq!(numerator2 % 2, 0);
    assert_eq!(numerator3 % 6, 0);
    let a2 = numerator2 / 2;
    let a3 = -numerator3 / 6;
    [125, 25 * a1, 5 * a2, a3, a2, a1, 1]
}

fn elliptic_factor(mut poly: [i64; 7]) -> Option<[i64; 5]> {
    let mut quotient = [0; 5];
    for k in (2..7).rev() {
        let scale = poly[k];
        quotient[k - 2] = scale;
        for (j, &coefficient) in [5, 1, 1].iter().enumerate() {
            poly[k - 2 + j] -= scale * coefficient;
        }
    }
    if poly == [0; 7] {
        Some(quotient)
    } else {
        None
    }
}

fn main() {
    println!("PROFILE\tfixed250-F5-bielliptic-quartics-A\tunsigned\tno timing or DLP claim");
    assert!((0usize..5).all(|x| x * x % 5 != 2));
    assert!((0usize..5).all(|x| (x * x * x + 2 * x + 1) % 5 != 0));
    let fields = [
        Field::new(1, vec![0]),
        Field::new(2, vec![2, 0]),
        Field::new(3, vec![4, 3, 0]),
    ];
    for (index, field) in fields.iter().enumerate() {
        field.check();
        assert_eq!(elliptic_count(field), [7, 35, 112][index]);
    }
    assert_eq!(smoothness(0, 0, 0), 0);
    assert!(smoothness(0, 1, 1) > 0);
    for kappa in [1, 2] {
        let field = &fields[0];
        let x = 0;
        let y = 1;
        let z = 0;
        let v = field.add(field.mul(kappa, field.square(z)), field.add(x, y));
        let d = field.mul(
            y,
            field.add(
                field.add(field.pow(x, 3), field.mul(2, field.mul(x, field.square(y)))),
                field.pow(y, 3),
            ),
        );
        assert_eq!(field.sub(field.square(v), d), 0);
        let dx = field.sub(
            field.mul(field.mul(2, v), y),
            field.mul(
                y,
                field.add(field.mul(3, field.square(x)), field.mul(2, field.square(y))),
            ),
        );
        let dy = field.sub(
            field.mul(field.mul(2, v), field.add(x, field.mul(2, y))),
            field.add(
                field.add(field.pow(x, 3), field.mul(x, field.square(y))),
                field.mul(4, field.pow(y, 3)),
            ),
        );
        let dz = field.mul(field.mul(4, kappa), field.mul(z, v));
        assert_eq!([dx, dy, dz], [0, 0, 0]);
    }
    println!("CONTROLS\tg=0 smooth\tg=x+1 singular at[0:1:0]\tPASS");
    let mut smooth = 0;
    let mut singular = 0;
    let mut phi_count = 0;
    let mut cube_count = 0;
    let mut first_phi = None;
    let mut first_cube = None;
    println!("ROW_HEADER\ta\tb\tc\tkappa\tstatus\tN1,N2,N3\tP_ascending\tquotient_ascending\tJ_order\tPhi3cube\tPEcube\tmutation");
    for a in 0..5 {
        for b in 0..5 {
            for c in 0..5 {
                let gcd = smoothness(a, b, c);
                if gcd != 0 {
                    for kappa in [1, 2] {
                        singular += 1;
                        println!("SINGULAR\t{a}\t{b}\t{c}\t{kappa}\tgcd_degree={gcd}");
                    }
                    continue;
                }
                let mut paired = [[0; 3]; 2];
                for (k_index, kappa) in [1, 2].into_iter().enumerate() {
                    let ns = std::array::from_fn(|index| counts(&fields[index], a, b, c, kappa));
                    paired[k_index] = ns;
                    let poly = weil(ns);
                    let quotient = elliptic_factor(poly).expect("fixed elliptic factor");
                    let mut changed = ns;
                    changed[2] += 3;
                    assert!(elliptic_factor(weil(changed)).is_none());
                    assert_eq!(elliptic_factor(weil(ns)), Some(quotient));
                    let phi = poly.map(|x| x.rem_euclid(2)) == [1, 1, 0, 1, 0, 1, 1];
                    let cube = poly == [125, 75, 90, 31, 18, 3, 1];
                    if phi {
                        phi_count += 1;
                        first_phi.get_or_insert([a, b, c, kappa]);
                    }
                    if cube {
                        cube_count += 1;
                        first_cube.get_or_insert([a, b, c, kappa]);
                        assert_eq!(ns, [9, 53, 84]);
                    }
                    smooth += 1;
                    let order: i64 = poly.iter().sum();
                    assert!(order > 0);
                    println!(
                        "ROW\t{a}\t{b}\t{c}\t{kappa}\tsmooth\t{ns:?}\t{poly:?}\t{quotient:?}\t{order}\t{phi}\t{cube}\tPASS"
                    );
                }
                assert_eq!(paired[0][0] + paired[1][0], 14);
                assert_eq!(paired[0][1], paired[1][1]);
                assert_eq!(paired[0][2] + paired[1][2], 224);
            }
        }
    }
    assert_eq!(smooth + singular, 250);
    println!(
        "SUMMARY\tequations=250\tsmooth={smooth}\tsingular={singular}\tPhi3cube={phi_count}\tPEcube={cube_count}\tfirst_phi={first_phi:?}\tfirst_cube={first_cube:?}\tPASS"
    );
}
