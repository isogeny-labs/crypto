//! Exact rational coefficient search, independent integer identity replay.
//! Standalone native tool; imports the already verified integer S4 constructor.
#[path = "../../../../examples/quartic_twist_s4_integer.rs"]
mod integer;
use integer::{Exponent, ZPoly};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Q {
    n: i128,
    d: i128,
}
fn gcd(mut a: i128, mut b: i128) -> i128 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
impl Q {
    fn new(mut n: i128, mut d: i128) -> Self {
        assert_ne!(d, 0);
        if d < 0 {
            n = n.checked_neg().unwrap();
            d = d.checked_neg().unwrap();
        }
        let g = gcd(n, d);
        Self { n: n / g, d: d / g }
    }
    fn z(n: i128) -> Self {
        Self { n, d: 1 }
    }
    fn add(self, b: Self) -> Self {
        let g = gcd(self.d, b.d);
        Self::new(
            self.n
                .checked_mul(b.d / g)
                .unwrap()
                .checked_add(b.n.checked_mul(self.d / g).unwrap())
                .unwrap(),
            self.d.checked_mul(b.d / g).unwrap(),
        )
    }
    fn neg(self) -> Self {
        Self {
            n: self.n.checked_neg().unwrap(),
            d: self.d,
        }
    }
    fn sub(self, b: Self) -> Self {
        self.add(b.neg())
    }
    fn mul(self, b: Self) -> Self {
        let g = gcd(self.n, b.d);
        let h = gcd(b.n, self.d);
        Self::new(
            (self.n / g).checked_mul(b.n / h).unwrap(),
            (self.d / h).checked_mul(b.d / g).unwrap(),
        )
    }
    fn div(self, b: Self) -> Self {
        self.mul(Self::new(b.d, b.n))
    }
}
fn monomials(weight: u32) -> Vec<Exponent> {
    let mut out = Vec::new();
    for a in 0..=weight {
        for b in 0..=weight / 2 {
            for c in 0..=weight / 3 {
                for d in 0..=weight / 4 {
                    if a + 2 * b + 3 * c + 4 * d == weight {
                        out.push([a as u8, b as u8, c as u8, d as u8, 0]);
                    }
                }
            }
        }
    }
    out
}
fn discriminant() -> ZPoly {
    let [a, b, c, d, _] = std::array::from_fn(ZPoly::var);
    d.pow(3)
        .scale(256)
        .sub(&a.mul(&c).mul(&d.pow(2)).scale(192))
        .sub(&b.pow(2).mul(&d.pow(2)).scale(128))
        .add(&b.mul(&c.pow(2)).mul(&d).scale(144))
        .sub(&c.pow(4).scale(27))
        .add(&a.pow(2).mul(&b).mul(&d.pow(2)).scale(144))
        .sub(&a.pow(2).mul(&c.pow(2)).mul(&d).scale(6))
        .sub(&a.mul(&b.pow(2)).mul(&c).mul(&d).scale(80))
        .add(&a.mul(&b).mul(&c.pow(3)).scale(18))
        .add(&b.pow(4).mul(&d).scale(16))
        .sub(&b.pow(3).mul(&c.pow(2)).scale(4))
        .sub(&a.pow(4).mul(&d.pow(2)).scale(27))
        .add(&a.pow(3).mul(&b).mul(&c).mul(&d).scale(18))
        .sub(&a.pow(3).mul(&c.pow(3)).scale(4))
        .sub(&a.pow(2).mul(&b.pow(3)).mul(&d).scale(4))
        .add(&a.pow(2).mul(&b.pow(2)).mul(&c.pow(2)))
}
fn poly_json(p: &ZPoly) -> String {
    format!(
        "[{}]",
        p.0.iter()
            .map(|(e, c)| format!("[{:?},{c}]", e))
            .collect::<Vec<_>>()
            .join(",")
    )
}
fn solve(mut a: Vec<Vec<Q>>, cols: usize) -> Option<Vec<Q>> {
    let mut pivots = Vec::new();
    let mut row = 0;
    for col in 0..cols {
        let Some(pivot) = (row..a.len()).find(|&r| a[r][col].n != 0) else {
            continue;
        };
        a.swap(row, pivot);
        let divisor = a[row][col];
        for j in col..=cols {
            a[row][j] = a[row][j].div(divisor);
        }
        for r in 0..a.len() {
            if r != row {
                let factor = a[r][col];
                if factor.n != 0 {
                    for j in col..=cols {
                        a[r][j] = a[r][j].sub(factor.mul(a[row][j]));
                    }
                }
            }
        }
        pivots.push(col);
        row += 1;
        if row == a.len() {
            break;
        }
    }
    if a.iter()
        .any(|r| r[..cols].iter().all(|x| x.n == 0) && r[cols].n != 0)
    {
        return None;
    }
    let mut x = vec![Q::z(0); cols];
    for (r, &c) in pivots.iter().enumerate() {
        x[c] = a[r][cols];
    }
    Some(x)
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let square = args.len() == 3 && args[1] == "--square";
    assert!(args.len() == 2 || square);
    let out = Path::new(&args[if square { 2 } else { 1 }]);
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let (raw, _) = integer::derive();
    let h = integer::derive_full(&raw);
    let mut cs: [ZPoly; 4] = std::array::from_fn(|_| ZPoly::default());
    for (&e, &c) in &h.0 {
        let mut f = e;
        f[4] = 0;
        cs[e[4] as usize].add_term(f, c);
    }
    let [a, b, _c, d, _] = std::array::from_fn(ZPoly::var);
    let defect = d.scale(64).sub(&b.scale(4).sub(&a.pow(2)).pow(2));
    let target = if square {
        defect.pow(3)
    } else {
        discriminant()
    };
    let mut descriptors = Vec::new();
    let mut columns = Vec::new();
    for (i, c) in cs.iter().enumerate() {
        for e in monomials(3 * i as u32) {
            descriptors.push((i, e));
            columns.push(c.mul(&ZPoly(BTreeMap::from([(e, 1)]))));
        }
    }
    let mut support = BTreeSet::new();
    for g in columns.iter().chain([&target]) {
        support.extend(g.0.keys().copied());
    }
    let matrix = support
        .iter()
        .map(|e| {
            let mut row: Vec<_> = columns
                .iter()
                .map(|g| Q::z(*g.0.get(e).unwrap_or(&0)))
                .collect();
            row.push(Q::z(*target.0.get(e).unwrap_or(&0)));
            row
        })
        .collect();
    let solution = solve(matrix, columns.len());
    let target_key = if square {
        "square_defect_cube"
    } else {
        "discriminant"
    };
    let mut json=format!("{{\n\"schema\":\"pure-zero-s4-certificate/v1\",\n\"coefficient_weights\":[12,9,6,3],\n\"equations\":{},\n\"unknowns\":{},\n\"h4\":{},\n\"coefficients\":[{}],\n\"{}\":{},\n",support.len(),columns.len(),poly_json(&h),cs.iter().map(poly_json).collect::<Vec<_>>().join(","),target_key,poly_json(&target));
    if square {
        json.push_str(&format!(
            "\"square_defect\":{},\n\"target_power\":3,\n",
            poly_json(&defect)
        ));
    }
    if let Some(solution) = solution {
        let denominator = solution
            .iter()
            .fold(1i128, |d, q| (d / gcd(d, q.d)).checked_mul(q.d).unwrap());
        let mut multipliers: [ZPoly; 4] = std::array::from_fn(|_| ZPoly::default());
        for ((i, e), q) in descriptors.iter().zip(&solution) {
            multipliers[*i].add_term(*e, q.n.checked_mul(denominator / q.d).unwrap());
        }
        let rhs = cs
            .iter()
            .zip(&multipliers)
            .fold(ZPoly::default(), |s, (c, a)| s.add(&c.mul(a)));
        let lhs = target.scale(denominator);
        assert_eq!(lhs, rhs, "independent integer identity replay failed");
        let mutation = lhs.add(&ZPoly::constant(1));
        assert_ne!(mutation, rhs, "constant mutation accepted");
        json.push_str(&format!("\"status\":\"integer-identity-pass\",\n\"denominator\":{denominator},\n\"multipliers\":[{}],\n\"integer_remainder_terms\":{},\n\"constant_mutation_rejected\":true\n}}\n",multipliers.iter().map(poly_json).collect::<Vec<_>>().join(","),lhs.sub(&rhs).0.len()));
    } else {
        json.push_str("\"status\":\"no-representation-in-declared-linear-space\",\n\"scope\":\"No radical nonmembership or mathematical impossibility conclusion\"\n}\n");
    }
    std::fs::write(out.join("certificate.json"), json).unwrap();
    println!(
        "Exact search complete; inspect certificate.json. No curve/group bridge or cost claim."
    );
}
