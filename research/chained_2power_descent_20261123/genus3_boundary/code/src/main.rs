use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Point([u8; 6]);

impl Point {
    fn decode(mut code: u16) -> Self {
        let mut digits = [0; 6];
        for x in &mut digits {
            *x = (code % 4) as u8;
            code /= 4;
        }
        Self(digits)
    }

    fn code(self) -> u16 {
        self.0
            .iter()
            .rev()
            .fold(0, |acc, &x| 4 * acc + u16::from(x))
    }

    fn add(self, rhs: Self) -> Self {
        let mut digits = [0; 6];
        for (i, x) in digits.iter_mut().enumerate() {
            *x = (self.0[i] + rhs.0[i]) % 4;
        }
        Self(digits)
    }

    fn double(self) -> Self {
        self.add(self)
    }

    fn frobenius(self) -> Self {
        let mut digits = [0; 6];
        for i in 0..3 {
            let (a, b) = (self.0[2 * i], self.0[2 * i + 1]);
            digits[2 * i] = 3 * b % 4;
            digits[2 * i + 1] = (a + 3 * b) % 4;
        }
        Self(digits)
    }

    fn two_bits(self) -> Option<u8> {
        let mut out = 0;
        for (i, &x) in self.0.iter().enumerate() {
            if x % 2 != 0 {
                return None;
            }
            out |= (x / 2) << i;
        }
        Some(out)
    }
}

fn pairing4(a: Point, b: Point) -> u8 {
    let mut out = 0_i32;
    for i in 0..3 {
        out += i32::from(a.0[2 * i]) * i32::from(b.0[2 * i + 1]);
        out -= i32::from(a.0[2 * i + 1]) * i32::from(b.0[2 * i]);
    }
    out.rem_euclid(4) as u8
}

fn pairing2(a: u8, b: u8) -> u8 {
    let mut out = 0;
    for i in 0..3 {
        out ^= ((a >> (2 * i)) & 1) * ((b >> (2 * i + 1)) & 1);
        out ^= ((a >> (2 * i + 1)) & 1) * ((b >> (2 * i)) & 1);
    }
    out
}

fn frobenius2(a: u8) -> u8 {
    let mut out = 0;
    for i in 0..3 {
        let (x, y) = ((a >> (2 * i)) & 1, (a >> (2 * i + 1)) & 1);
        out |= y << (2 * i);
        out |= (x ^ y) << (2 * i + 1);
    }
    out
}

fn image_bits(bits: u64, image: impl Fn(u8) -> u8) -> u64 {
    let mut out = 0;
    for v in 0..64 {
        if bits & (1_u64 << v) != 0 {
            out |= 1_u64 << image(v);
        }
    }
    out
}

fn lagrangians() -> BTreeSet<u64> {
    let mut out = BTreeSet::new();
    for a in 1_u8..64 {
        for b in (a + 1)..64 {
            if pairing2(a, b) != 0 {
                continue;
            }
            for c in (b + 1)..64 {
                if c == a ^ b || pairing2(a, c) != 0 || pairing2(b, c) != 0 {
                    continue;
                }
                let mut span = 0_u64;
                for v in [0, a, b, a ^ b, c, a ^ c, b ^ c, a ^ b ^ c] {
                    span |= 1_u64 << v;
                }
                assert_eq!(span.count_ones(), 8);
                out.insert(span);
            }
        }
    }
    out
}

fn kernel(corrupt_graph: bool) -> BTreeSet<Point> {
    let mut out = BTreeSet::new();
    for x in 0..2 {
        for y in 0..2 {
            for a in 0..4 {
                for b in 0..4 {
                    let (c, d) = if corrupt_graph {
                        (a, b)
                    } else {
                        ((a + 2 * b) % 4, (2 * a + 3 * b) % 4)
                    };
                    out.insert(Point([2 * x, 2 * y, a, b, c, d]));
                }
            }
        }
    }
    out
}

fn nonisotropic_pair(points: &BTreeSet<Point>) -> Option<(Point, Point, u8)> {
    for &a in points {
        for &b in points {
            let pairing = pairing4(a, b);
            if pairing != 0 {
                return Some((a, b, pairing));
            }
        }
    }
    None
}

fn main() {
    println!("study=genus3_boundary_first_A; exact finite module only");
    println!("encoding=base4_little_endian_6coords; form=sum(x_even*y_odd-x_odd*y_even)");
    println!("F_block=[[0,3],[1,3]]; M=[[1,2],[2,3]]; modulus=4");
    let mut affine = 0;
    for x in 0_u32..5 {
        for y in 0_u32..5 {
            if y * y % 5 == (x * x * x + 2 * x + 1) % 5 {
                println!("curve_affine x={x} y={y}");
                affine += 1;
            }
        }
    }
    assert_eq!(affine + 1, 7);
    let discriminant = (5 - 16 * ((4 * 2_u32.pow(3) + 27) % 5) % 5) % 5;
    assert_ne!(discriminant, 0);
    println!(
        "curve=E/F5:y^2=x^3+2x+1; affine={affine}; total=7; trace=-1; discriminant={discriminant}"
    );

    let zero = Point([0; 6]);
    for code in 0..4096 {
        let p = Point::decode(code);
        assert_eq!(p.code(), code);
        assert_eq!(p.frobenius().frobenius().frobenius(), p);
        assert_eq!(p.frobenius().frobenius().add(p.frobenius()).add(p), zero);
    }
    for i in 0..6 {
        for j in 0..6 {
            let (mut x, mut y) = ([0; 6], [0; 6]);
            x[i] = 1;
            y[j] = 1;
            assert_eq!(
                pairing4(Point(x), Point(y)),
                pairing4(Point(x).frobenius(), Point(y).frobenius())
            );
        }
    }
    let fixed: Vec<_> = (0..64).filter(|&x| frobenius2(x) == x).collect();
    assert_eq!(fixed, vec![0]);
    println!("ambient=4096; F_order3_all_points=pass; charpoly_all_points=pass; symplectic_basis36=pass; fixed_two_bits={fixed:?}");

    let k = kernel(false);
    assert_eq!(k.len(), 64);
    assert_eq!(nonisotropic_pair(&k), None);
    let doubled: BTreeSet<_> = k.iter().map(|&x| x.double()).collect();
    let k2: BTreeSet<_> = k.iter().copied().filter(|&x| x.double() == zero).collect();
    assert_eq!(doubled.len(), 4);
    assert_eq!(k2.len(), 16);
    for &x in &k {
        assert!(k.contains(&x.frobenius()));
        assert_eq!(x.double().double(), zero);
        for &y in &k {
            assert!(k.contains(&x.add(y)));
        }
    }
    let perp: BTreeSet<_> = (0..4096)
        .map(Point::decode)
        .filter(|&x| k.iter().all(|&y| pairing4(x, y) == 0))
        .collect();
    assert_eq!(perp, k);
    for &p in &k {
        let order = if p == zero {
            1
        } else if p.double() == zero {
            2
        } else {
            4
        };
        println!(
            "K point={} F={} order={order}",
            p.code(),
            p.frobenius().code()
        );
    }
    println!(
        "K_perp_complete_ids={:?}",
        perp.iter().map(|x| x.code()).collect::<Vec<_>>()
    );
    println!(
        "doubled_complete_ids={:?}",
        doubled.iter().map(|x| x.code()).collect::<Vec<_>>()
    );
    let k2_bits = k2
        .iter()
        .fold(0_u64, |b, p| b | (1_u64 << p.two_bits().unwrap()));
    let doubled_bits = doubled
        .iter()
        .fold(0_u64, |b, p| b | (1_u64 << p.two_bits().unwrap()));

    let ls = lagrangians();
    assert_eq!(ls.len(), 135);
    let mut stable = 0;
    let mut compatible = Vec::new();
    for &l in &ls {
        let f = image_bits(l, frobenius2);
        assert!(ls.contains(&f));
        let invariant = f == l;
        stable += usize::from(invariant);
        let first = l & k2_bits == l && l & doubled_bits == doubled_bits;
        if first {
            compatible.push(l);
        }
        println!("L bits={l:016x} F={f:016x} stable={invariant} first={first}");
    }
    assert_eq!(stable, 0);
    assert_eq!(compatible.len(), 3);
    for &l in &compatible {
        let f = image_bits(l, frobenius2);
        assert_ne!(f, l);
        assert!(compatible.contains(&f));
        assert_eq!(image_bits(image_bits(f, frobenius2), frobenius2), l);
        println!("compatible_first bits={l:016x} F={f:016x}");
    }
    let stable_identity = ls
        .iter()
        .filter(|&&l| image_bits(l, std::convert::identity) == l)
        .count();
    assert_eq!(stable_identity, 135);
    let bad = kernel(true);
    let witness = nonisotropic_pair(&bad).expect("identity-graph control must fail isotropy");
    println!(
        "corrupt_M_identity rejected_pair=({},{}),pairing={}",
        witness.0.code(),
        witness.1.code(),
        witness.2
    );
    assert_eq!(kernel(false), k);
    assert_eq!(nonisotropic_pair(&kernel(false)), None);
    println!("restored_M=pass; identity_F_stable_lagrangians={stable_identity}");
    println!("RESULT K=64 K_two=16 doubled=4 orthogonal_equals_K=true lagrangians=135 F_stable=0 compatible_first=3 cycle=3");
    println!("SCOPE product_PPAV_module; no_indecomposable_genus3_Jacobian_or_explicit_divisor_map_or_DLP_cost_claim");
}
