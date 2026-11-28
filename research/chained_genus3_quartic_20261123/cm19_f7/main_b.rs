mod field_b;
use field_b::Field;
use std::collections::BTreeMap;

type Form = BTreeMap<[usize; 3], usize>;

fn monomial(exponents: [usize; 3], coefficient: usize) -> Form {
    BTreeMap::from([(exponents, coefficient % 7)])
}

fn plus(a: &Form, b: &Form) -> Form {
    let mut out = a.clone();
    for (&key, &value) in b {
        let entry = out.entry(key).or_default();
        *entry = (*entry + value) % 7;
    }
    out.retain(|_, value| *value != 0);
    out
}

fn scale(a: &Form, coefficient: usize) -> Form {
    a.iter()
        .filter_map(|(&key, &value)| {
            let v = value * coefficient % 7;
            (v != 0).then_some((key, v))
        })
        .collect()
}

fn product(a: &Form, b: &Form) -> Form {
    let mut out = BTreeMap::new();
    for (&aa, &x) in a {
        for (&bb, &y) in b {
            let key = std::array::from_fn(|i| aa[i] + bb[i]);
            let entry = out.entry(key).or_default();
            *entry = (*entry + x * y) % 7;
        }
    }
    out.retain(|_, value| *value != 0);
    out
}

fn primary_identity() {
    let u = monomial([1, 0, 0], 1);
    let v = monomial([0, 1, 0], 1);
    let w = monomial([0, 0, 1], 1);
    let x = plus(&v, &scale(&w, 3));
    let y = w;
    let z = scale(&u, 2);
    let x2 = product(&x, &x);
    let y2 = product(&y, &y);
    let g = plus(
        &plus(&product(&z, &z), &scale(&x2, 6)),
        &plus(&scale(&product(&x, &y), 3), &y2),
    );
    let d = product(
        &y,
        &plus(
            &plus(&product(&x2, &x), &scale(&product(&x, &y2), 2)),
            &scale(&product(&y2, &y), 6),
        ),
    );
    let transformed = plus(&product(&g, &g), &scale(&d, 6));
    let inv9 = (1usize..7).find(|&x| 9 * x % 7 == 1).unwrap();
    let inv3 = (1usize..7).find(|&x| 3 * x % 7 == 1).unwrap();
    let signed = |a: i64| a.rem_euclid(7) as usize;
    let raw = [
        ([4, 0, 0], 1),
        ([0, 4, 0], inv9),
        ([2, 2, 0], 2 * inv3 % 7),
        ([0, 2, 2], signed(-190)),
        ([2, 0, 2], signed(-570)),
        ([0, 3, 1], 152 * inv9 % 7),
        ([2, 1, 1], signed(-152)),
        ([0, 0, 4], signed(-1083)),
    ];
    let primary: Form = raw.into_iter().collect();
    assert_eq!(transformed, scale(&primary, 2));
    for i in 0..5 {
        for j in 0..5 - i {
            let key = [i, j, 4 - i - j];
            let actual = *transformed.get(&key).unwrap_or(&0);
            let expected = 2 * primary.get(&key).unwrap_or(&0) % 7;
            assert_eq!(actual, expected);
            println!("COEFFICIENT\t{key:?}\ttransformed={actual}\t2F={expected}");
        }
    }
    let changed = plus(&primary, &monomial([0, 0, 4], 1));
    assert_ne!(transformed, scale(&changed, 2));
    assert_eq!(transformed, scale(&primary, 2));
    println!("PRIMARY_IDENTITY\tall15 coefficients\tactual W4 mutation rejects\tPASS");
}

fn trim(poly: &mut Vec<usize>) {
    while poly.last() == Some(&0) {
        poly.pop();
    }
}

fn inv7(a: usize) -> usize {
    (1..7).find(|x| x * a % 7 == 1).unwrap()
}

fn rem(mut a: Vec<usize>, b: &[usize]) -> Vec<usize> {
    trim(&mut a);
    while a.len() >= b.len() {
        let shift = a.len() - b.len();
        let factor = a[a.len() - 1] * inv7(*b.last().unwrap()) % 7;
        for (i, &v) in b.iter().enumerate() {
            a[i + shift] = (a[i + shift] + 7 - factor * v % 7) % 7;
        }
        trim(&mut a);
    }
    a
}

fn smooth(poly: &[usize]) -> usize {
    let mut derivative: Vec<_> = poly
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, &v)| i * v % 7)
        .collect();
    trim(&mut derivative);
    let mut a = poly.to_vec();
    let mut b = derivative.clone();
    while !b.is_empty() {
        let next = rem(a, &b);
        a = b;
        b = next;
    }
    let degree = a.len() - 1;
    let m = poly.len() - 1;
    let n = derivative.len() - 1;
    let size = m + n;
    let mut matrix = vec![vec![0; size]; size];
    for (i, row) in matrix[..n].iter_mut().enumerate() {
        row[i..i + poly.len()].copy_from_slice(poly);
    }
    for (i, row) in matrix[n..].iter_mut().enumerate() {
        row[i..i + derivative.len()].copy_from_slice(&derivative);
    }
    let mut rank = 0;
    for column in 0..size {
        if let Some(pivot) = (rank..size).find(|&r| matrix[r][column] != 0) {
            matrix.swap(rank, pivot);
            let inv = inv7(matrix[rank][column]);
            for value in &mut matrix[rank] {
                *value = *value * inv % 7;
            }
            let pivot_row = matrix[rank].clone();
            for row in matrix.iter_mut().skip(rank + 1) {
                let factor = row[column];
                for (entry, &value) in row.iter_mut().zip(&pivot_row) {
                    *entry = (*entry + 7 - factor * value % 7) % 7;
                }
            }
            rank += 1;
        }
    }
    assert_eq!(degree, size - rank);
    degree
}

fn poly(field: &Field, coefficients: &[usize], x: usize) -> usize {
    coefficients
        .iter()
        .rev()
        .fold(0, |a, &b| field.add(field.mul(a, x), b))
}

fn valid(field: &Field, point: [usize; 3], kappa: usize) -> bool {
    let [x, y, z] = point;
    let g = field.add(
        field.add(field.mul(6, field.square(x)), field.mul(3, field.mul(x, y))),
        field.square(y),
    );
    let v = field.add(field.mul(kappa, field.square(z)), g);
    let d = field.mul(
        y,
        field.add(
            field.add(field.pow(x, 3), field.mul(2, field.mul(x, field.square(y)))),
            field.mul(6, field.pow(y, 3)),
        ),
    );
    field.square(v) == d
}

fn counts(field: &Field, kappa: usize) -> (usize, usize, Vec<[usize; 3]>) {
    let mut direct = 0;
    let mut fibre = 0;
    let mut elliptic = 1;
    let mut points = Vec::new();
    let mut roots = vec![0; field.q];
    for z in 0..field.q {
        roots[field.square(z)] += 1;
    }
    for x in 0..field.q {
        let g = poly(field, &[1, 3, 6], x);
        let d = poly(field, &[6, 2, 0, 1], x);
        for z in 0..field.q {
            let z2 = field.square(z);
            let expanded = field.sub(
                field.add(
                    field.add(
                        field.mul(field.square(kappa), field.square(z2)),
                        field.mul(field.mul(2, kappa), field.mul(g, z2)),
                    ),
                    field.square(g),
                ),
                d,
            );
            if expanded == 0 {
                direct += 1;
                if field.q == 7 {
                    points.push([x, 1, z]);
                }
            }
        }
        for v in 0..field.q {
            if field.square(v) == d {
                elliptic += 1;
                fibre += roots[field.mul(inv7(kappa), field.sub(v, g))];
            }
        }
    }
    for z in 0..field.q {
        if field.square(field.add(field.mul(kappa, field.square(z)), 6)) == 0 {
            direct += 1;
            if field.q == 7 {
                points.push([1, 0, z]);
            }
        }
    }
    fibre += roots[field.mul(inv7(kappa), 1)];
    assert_eq!(direct, fibre);
    for &point in &points {
        assert!(valid(field, point, kappa));
    }
    (direct, elliptic, points)
}

fn weil(ns: [usize; 3]) -> [i64; 7] {
    let s1 = 8 - ns[0] as i64;
    let s2 = 50 - ns[1] as i64;
    let s3 = 344 - ns[2] as i64;
    let second = s1 * s1 - s2;
    let third = s1 * s1 * s1 - 3 * s1 * s2 + 2 * s3;
    assert_eq!(second % 2, 0);
    assert_eq!(third % 6, 0);
    let a1 = -s1;
    let a2 = second / 2;
    let a3 = -third / 6;
    [343, 49 * a1, 7 * a2, a3, a2, a1, 1]
}

fn factor(mut polynomial: [i64; 7]) -> Option<[i64; 5]> {
    let mut quotient = [0; 5];
    for k in (2..7).rev() {
        let scale = polynomial[k];
        quotient[k - 2] = scale;
        for (j, &coefficient) in [7, 3, 1].iter().enumerate() {
            polynomial[k - 2 + j] -= scale * coefficient;
        }
    }
    (polynomial == [0; 7]).then_some(quotient)
}

fn main() {
    println!("PROFILE\tsingle-primary-CM19-F7-source-A\tunsigned\tno timing/DLP claim");
    primary_identity();
    let g = [1, 3, 6];
    let mut h = vec![0; 5];
    for (i, &a) in g.iter().enumerate() {
        for (j, &b) in g.iter().enumerate() {
            h[i + j] = (h[i + j] + a * b) % 7;
        }
    }
    for (a, b) in h.iter_mut().zip([6, 2, 0, 1, 0]) {
        *a = (*a + 7 - b) % 7;
    }
    assert_eq!(h, [2, 4, 0, 0, 1]);
    assert_eq!(smooth(&h), 0);
    assert_eq!(smooth(&[0, 0, 1]), 1);
    assert_eq!(smooth(&h), 0);
    println!("SMOOTH\th={h:?}\tgcd/Sylvester=0\tx2 negative rejects\tPASS");
    assert!((0usize..7).all(|x| x * x % 7 != 3));
    assert!((0usize..7).all(|x| (x * x * x + 2 * x + 1) % 7 != 0));
    let fields = [
        Field::new(7, 1, vec![0]),
        Field::new(7, 2, vec![3, 0]),
        Field::new(7, 3, vec![6, 5, 0]),
    ];
    let mut all_counts = [[0; 3]; 2];
    let mut prime_points = Vec::new();
    for (index, field) in fields.iter().enumerate() {
        field.check();
        for (kindex, kappa) in [1, 3].into_iter().enumerate() {
            let (n, e, points) = counts(field, kappa);
            assert_eq!(e, [11, 55, 308][index]);
            all_counts[kindex][index] = n;
            println!(
                "COUNT\tq={}\tkappa={kappa}\tC={n}\tE={e}\tdirect=fibre\tpointsF7={points:?}",
                field.q
            );
            if index == 0 && kappa == 1 {
                prime_points = points;
            }
        }
    }
    for (kindex, ns) in all_counts.into_iter().enumerate() {
        let p = weil(ns);
        let quotient = factor(p).expect("actual elliptic quotient");
        let mut changed = ns;
        changed[2] += 3;
        assert!(factor(weil(changed)).is_none());
        assert_eq!(factor(weil(ns)), Some(quotient));
        println!("WEIL\tkappa={}\tcounts={ns:?}\tP={p:?}\tquotient={quotient:?}\tJorder={}\tmutant=reject/restored=pass", [1, 3][kindex], p.iter().sum::<i64>());
    }
    assert_eq!(all_counts[0], [17, 65, 236]);
    assert_eq!(all_counts[0][0] + all_counts[1][0], 22);
    assert_eq!(all_counts[0][1], all_counts[1][1]);
    assert_eq!(all_counts[0][2] + all_counts[1][2], 616);
    let p = weil(all_counts[0]);
    assert_eq!(p, [343, 441, 336, 153, 48, 9, 1]);
    assert_eq!(p.map(|a| a.rem_euclid(2)), [1, 1, 0, 1, 0, 1, 1]);
    assert_eq!(p.iter().sum::<i64>(), 1331);
    let extension = &fields[1];
    let gamma = (0..49).find(|&a| extension.square(a) == 3).unwrap();
    let gamma_inverse = extension.pow(gamma, 47);
    assert_eq!(extension.pow(gamma, 7), extension.neg(gamma));
    assert_eq!(extension.pow(gamma, 49), gamma);
    let mut images = Vec::new();
    let mut nonfixed = 0;
    for point in prime_points {
        let image = [point[0], point[1], extension.mul(point[2], gamma_inverse)];
        assert!(valid(extension, image, 3));
        assert_eq!(extension.mul(image[2], gamma), point[2]);
        let frob = image.map(|a| extension.pow(a, 7));
        let involution = [image[0], image[1], extension.neg(image[2])];
        assert_eq!(frob, involution);
        assert_eq!(frob == image, point[2] == 0);
        nonfixed += usize::from(frob != image);
        assert!(!images.contains(&image));
        images.push(image);
        println!("TWIST_POINT\tsource={point:?}\timage={image:?}\tFrob={frob:?}\troundtrip=PASS");
    }
    assert_eq!(images.len(), 17);
    assert_eq!(nonfixed, 16);
    println!("TWIST\tgamma={gamma}\tactual_images=17\tnonfixed=16\tfixed_branch=1\tPASS");
    println!("SUMMARY\tgenuine_smooth_genus3=PASS\tPE7cube=PASS\tPhi3cube=PASS\tJorder=1331\tpolarized_module=OPEN\tactual_J4_kernel_target_maps=OPEN\tDLP_cost=UNMEASURED");
}
