//! Independent complete balanced-class replay with the existing odd-model
//! Cantor engine. Inputs/operators are frozen producer fixtures only.
//! No producer Balanced, tower, Richelot or elliptic-map implementation.
use isogeny_algos::field::{Field, Rng};
use isogeny_algos::fpr::{FpR, ER};
use isogeny_algos::poly;
use std::collections::HashSet;

#[path = "legacy_jacobian.rs"]
mod legacy;

const T0: [u64; 7] = [31, 75, 74, 54, 71, 55, 1];
const CLASSES: &[u8] = include_bytes!("../fixtures/full-class/t0-classes.bin");
const NEGATION: &[u8] = include_bytes!("../fixtures/full-class/t0-negation.bin");
const DOUBLING: &[u8] = include_bytes!("../fixtures/full-class/t0-doubling.bin");
const PAIRS: &[u8] = include_bytes!("../fixtures/full-class/t0-pairs-37-7.bin");
type Input = (Vec<ER>, Vec<ER>, u8);

struct Flat<'a>(&'a FpR);
impl legacy::Fld for Flat<'_> {
    type E = ER;
    fn zero(&self) -> ER {
        self.0.zero()
    }
    fn one(&self) -> ER {
        self.0.one()
    }
    fn add(&self, a: &ER, b: &ER) -> ER {
        self.0.add(*a, *b)
    }
    fn sub(&self, a: &ER, b: &ER) -> ER {
        self.0.sub(*a, *b)
    }
    fn neg(&self, a: &ER) -> ER {
        self.0.neg(*a)
    }
    fn mul(&self, a: &ER, b: &ER) -> ER {
        self.0.mul(*a, *b)
    }
    fn inv(&self, a: &ER) -> ER {
        assert_ne!(*a, self.0.zero());
        self.0.inv(*a)
    }
}

fn header(
    bytes: &[u8],
    magic: &[u8; 8],
    curve: &[u64; 7],
    width: usize,
) -> Result<usize, &'static str> {
    if bytes.len() < 18 || &bytes[..8] != magic || bytes[8] != 101 {
        return Err("wrong schema or field");
    }
    if curve
        .iter()
        .zip(&bytes[9..16])
        .any(|(a, b)| *a != u64::from(*b))
    {
        return Err("wrong actual curve");
    }
    let count = usize::from(u16::from_le_bytes([bytes[16], bytes[17]]));
    if count != 12317 || bytes.len() != 18 + width * count {
        return Err("wrong corpus length");
    }
    Ok(count)
}

fn decode_classes(bytes: &[u8], curve: &[u64; 7]) -> Result<Vec<[u8; 5]>, &'static str> {
    let count = header(bytes, b"CT27CLS1", curve, 5)?;
    let mut out = Vec::with_capacity(count);
    let mut seen = HashSet::new();
    for row in bytes[18..].chunks_exact(5) {
        let row: [u8; 5] = row.try_into().unwrap();
        let degree = row[0] / 3;
        let plus = row[0] % 3;
        if degree > 2 || plus > 2 - degree || row[1..].iter().any(|c| *c >= 101) {
            return Err("invalid balanced degree/plus/coefficient");
        }
        if (degree == 0 && row[1..].iter().any(|c| *c != 0))
            || (degree < 2 && (row[2] != 0 || row[4] != 0))
        {
            return Err("noncanonical padding");
        }
        if !seen.insert(row) {
            return Err("duplicate encoded class");
        }
        out.push(row);
    }
    Ok(out)
}

fn decode_indices(
    bytes: &[u8],
    magic: &[u8; 8],
    curve: &[u64; 7],
) -> Result<Vec<usize>, &'static str> {
    let count = header(bytes, magic, curve, 2)?;
    bytes[18..]
        .chunks_exact(2)
        .map(|row| {
            let index = usize::from(u16::from_le_bytes([row[0], row[1]]));
            if index < count {
                Ok(index)
            } else {
                Err("operator index outside corpus")
            }
        })
        .collect()
}

#[test]
fn existing_cantor_replays_every_balanced_t0_class() {
    let f = FpR::from_modulus(101, &[33, 46, 2, 18, 56, 12, 1]);
    let lift = |p: &[u64]| p.iter().map(|c| f.from_u64(*c)).collect::<Vec<_>>();
    let curve = lift(&T0);
    assert_eq!(
        poly::gcd(&f, &curve, &poly::derivative(&f, &curve)),
        vec![f.one()]
    );
    let encoded = decode_classes(CLASSES, &T0).unwrap();
    let negation = decode_indices(NEGATION, b"CT27NEG1", &T0).unwrap();
    let doubling = decode_indices(DOUBLING, b"CT27DBL1", &T0).unwrap();
    let pairs = decode_indices(PAIRS, b"CT27ADD1", &T0).unwrap();
    let mut degrees = [0_usize; 3];
    let mut infinity = [0_usize; 5];
    let inputs: Vec<Input> = encoded
        .iter()
        .map(|row| {
            let degree = usize::from(row[0] / 3);
            let plus = row[0] % 3;
            degrees[degree] += 1;
            infinity[usize::try_from(2 * i32::from(plus) + degree as i32).unwrap()] += 1;
            let u = match degree {
                0 => vec![f.one()],
                1 => vec![f.from_u64(u64::from(row[1])), f.one()],
                2 => vec![
                    f.from_u64(u64::from(row[1])),
                    f.from_u64(u64::from(row[2])),
                    f.one(),
                ],
                _ => unreachable!(),
            };
            let mut v = vec![f.from_u64(u64::from(row[3])), f.from_u64(u64::from(row[4]))];
            poly::trim(&f, &mut v);
            assert!(poly::rem(&f, &poly::sub(&f, &curve, &poly::mul(&f, &v, &v)), &u).is_empty());
            (u, v, plus)
        })
        .collect();
    assert_eq!(degrees, [3, 236, 12078]);
    assert_eq!(infinity, [1, 118, 12079, 118, 1]);
    let mut roots = poly::roots(&f, &curve, &mut Rng::new(0x0027_ca17));
    roots.sort();
    assert_eq!(roots.len(), 6);
    let a = *roots
        .iter()
        .find(|a| {
            inputs
                .iter()
                .all(|(u, _, _)| poly::eval(&f, u, **a) != f.zero())
        })
        .expect("a branch chart must avoid every rational support");
    let l = poly::eval(&f, &poly::derivative(&f, &curve), a);
    assert_ne!(l, f.zero());
    // Same existing monic chart as the kernel replay,without assuming
    // that all finite divisors have degree2 or infinity correction zero.
    let mut h = vec![];
    let az1 = vec![f.one(), a];
    let mut power = vec![f.one()];
    for (i, c) in curve.iter().enumerate() {
        let mut term = vec![f.zero(); 6 - i];
        term.extend(poly::scale(&f, &power, *c));
        h = poly::add(&f, &h, &term);
        power = poly::mul(&f, &power, &az1);
    }
    assert_eq!(h.len(), 6);
    assert_eq!(h[5], l);
    let quintic = h
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let scale = if i <= 4 {
                f.pow(l, (4 - i) as u128)
            } else {
                f.inv(l)
            };
            f.mul(*c, scale)
        })
        .collect::<Vec<_>>();
    assert_eq!(quintic.last(), Some(&f.one()));
    assert_eq!(
        poly::gcd(&f, &quintic, &poly::derivative(&f, &quintic)),
        vec![f.one()]
    );
    let field = Flat(&f);
    let j = legacy::Hyp {
        f: &field,
        h: quintic.clone(),
        g: 2,
    };
    for k in 1..=7 {
        let w = f.from_u64(k);
        let x = f.add(a, f.div(l, w));
        assert_eq!(
            legacy::peval(&field, &quintic, &w),
            f.div(f.mul(f.pow(w, 6), poly::eval(&f, &curve, x)), f.sq(l))
        );
    }
    // The two sextic infinities map to (w0,V=±L²). Their sum is
    // linearly equivalent to 2W;Q is the PLUS infinity minus W.
    let q = legacy::Div {
        u: vec![f.zero(), f.one()],
        v: vec![f.sq(l)],
    };
    assert!(j.is_valid(&q));
    let mut images = Vec::with_capacity(inputs.len());
    let mut seen = HashSet::new();
    for (i, (u, v, plus)) in inputs.iter().enumerate() {
        let degree = u.len() - 1;
        let ua = poly::eval(&f, u, a);
        assert_ne!(ua, f.zero(), "all-class denominator row{i}");
        let finite = if degree == 0 {
            j.identity()
        } else {
            let new_u = if degree == 1 {
                vec![f.div(l, ua), f.one()]
            } else {
                vec![
                    f.div(f.sq(l), ua),
                    f.div(f.mul(l, f.add(f.mul(f.from_u64(2), a), u[1])), ua),
                    f.one(),
                ]
            };
            let v1 = v.get(1).copied().unwrap_or(f.zero());
            let new_v = poly::rem(
                &f,
                &vec![f.zero(), f.zero(), v1, f.div(poly::eval(&f, v, a), l)],
                &new_u,
            );
            legacy::Div { u: new_u, v: new_v }
        };
        assert!(j.is_valid(&finite));
        let coefficient = 2 * i64::from(*plus) + degree as i64 - 2;
        let correction = j.mul(&q, u128::from(coefficient.unsigned_abs()));
        let correction = if coefficient < 0 {
            j.neg(&correction)
        } else {
            correction
        };
        let image = j.add(&finite, &correction);
        assert!(
            j.is_valid(&image) && image.u.len() <= 3,
            "reduced class row{i}"
        );
        assert!(
            seen.insert((image.u.clone(), image.v.clone())),
            "distinct class row{i}"
        );
        images.push(image);
    }
    let identities = inputs
        .iter()
        .enumerate()
        .filter(|(_, (u, _, n))| u.len() == 1 && *n == 1)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    assert_eq!(identities, [239]);
    assert_eq!(images[239], j.identity());
    for (i, d) in images.iter().enumerate() {
        assert_eq!(
            j.sub(&j.identity(), d),
            images[negation[i]],
            "old Cantor negation row{i}"
        );
        assert_eq!(j.mul(d, 2), images[doubling[i]], "old Cantor double row{i}");
        assert_eq!(
            j.add(d, &images[(37 * i + 7) % images.len()]),
            images[pairs[i]],
            "old Cantor paired sum row{i}"
        );
    }
    println!("PASS second EXISTING Cantor engine on ALL{} T0 balanced classes:curve/quintic validity,distinctness,identity239,all negations/doubles/predetermined paired additions;degree counts{degrees:?},infinity corrections{infinity:?};checked chart a={a:?},L={l:?}. No producer Balanced/tower/map import,no independent isogeny evaluation or exhaustive151M-pair claim. Global Picard bridge remains written;no timing claim.",images.len());
}

#[test]
fn full_class_fixtures_reject_wrong_models_and_malformed_rows() {
    let mut wrong = CLASSES.to_vec();
    wrong[8] = 103;
    assert!(decode_classes(&wrong, &T0).is_err());
    let mut wrong = CLASSES.to_vec();
    wrong[9] = 32;
    assert!(decode_classes(&wrong, &T0).is_err());
    assert!(decode_classes(&CLASSES[..CLASSES.len() - 1], &T0).is_err());
    let mut wrong = CLASSES.to_vec();
    wrong[18] = 7;
    assert!(decode_classes(&wrong, &T0).is_err());
    let mut wrong = CLASSES.to_vec();
    let empty = encoded_empty_row(&wrong);
    wrong[18 + 5 * empty + 3] = 1;
    assert!(decode_classes(&wrong, &T0).is_err());
    let mut wrong = DOUBLING.to_vec();
    wrong[18..20].copy_from_slice(&u16::MAX.to_le_bytes());
    assert!(decode_indices(&wrong, b"CT27DBL1", &T0).is_err());
    assert!(decode_indices(NEGATION, b"CT27DBL1", &T0).is_err());
}

fn encoded_empty_row(bytes: &[u8]) -> usize {
    bytes[18..]
        .chunks_exact(5)
        .position(|row| row[0] < 3)
        .unwrap()
}
