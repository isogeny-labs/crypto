//! Exact pair-image degree certificate for `EXP-C25519-PAIRIMAGE-GATE-001`.
//!
//! This constructs the product of every unordered-pair S3 factor over the
//! fixed Curve25519-family miniature and checks its trace-block decomposition
//! and squarefreeness. It is an algebraic gate, not an ECDLP solver.

use std::collections::BTreeMap;

use crypto_lib::cryptanalysis::ic_framework::subgroup_semaev::primitive_root_of_unity;
use sha2::{Digest, Sha256};

const P: u64 = 1_048_573;
const A: u64 = 63_700;
const B: u64 = 872_826;
const LO: u64 = 349_833;
const SCALE: u64 = 3;

#[inline]
fn addm(a: u64, b: u64) -> u64 {
    (a + b) % P
}

#[inline]
fn subm(a: u64, b: u64) -> u64 {
    (a + P - b % P) % P
}

#[inline]
fn mulm(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}

fn powm(mut a: u64, mut e: u64) -> u64 {
    let mut out = 1;
    while e != 0 {
        if e & 1 == 1 {
            out = mulm(out, a);
        }
        a = mulm(a, a);
        e >>= 1;
    }
    out
}

fn invm(a: u64) -> u64 {
    assert_ne!(a, 0);
    powm(a, P - 2)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Poly(Vec<u64>);

impl Poly {
    fn new(mut c: Vec<u64>) -> Self {
        while c.last() == Some(&0) {
            c.pop();
        }
        Self(c)
    }

    fn one() -> Self {
        Self(vec![1])
    }

    fn degree(&self) -> Option<usize> {
        self.0.len().checked_sub(1)
    }

    fn is_zero(&self) -> bool {
        self.0.is_empty()
    }

    fn mul(&self, rhs: &Self) -> Self {
        if self.is_zero() || rhs.is_zero() {
            return Self(Vec::new());
        }
        let mut out = vec![0; self.0.len() + rhs.0.len() - 1];
        for (i, &a) in self.0.iter().enumerate() {
            for (j, &b) in rhs.0.iter().enumerate() {
                out[i + j] = addm(out[i + j], mulm(a, b));
            }
        }
        Self::new(out)
    }

    fn derivative(&self) -> Self {
        if self.0.len() < 2 {
            return Self(Vec::new());
        }
        Self::new(
            self.0
                .iter()
                .enumerate()
                .skip(1)
                .map(|(i, &c)| mulm(c, i as u64 % P))
                .collect(),
        )
    }

    fn div_rem(&self, divisor: &Self) -> (Self, Self) {
        assert!(!divisor.is_zero());
        let Some(dd) = divisor.degree() else {
            unreachable!()
        };
        let Some(mut rd) = self.degree() else {
            return (Self(Vec::new()), Self(Vec::new()));
        };
        if rd < dd {
            return (Self(Vec::new()), self.clone());
        }
        let mut rem = self.0.clone();
        let mut quo = vec![0; rd - dd + 1];
        let inv_lead = invm(divisor.0[dd]);
        while rd >= dd && !rem.is_empty() {
            let shift = rd - dd;
            let q = mulm(rem[rd], inv_lead);
            quo[shift] = q;
            for j in 0..=dd {
                rem[shift + j] = subm(rem[shift + j], mulm(q, divisor.0[j]));
            }
            while rem.last() == Some(&0) {
                rem.pop();
            }
            let Some(next) = rem.len().checked_sub(1) else {
                break;
            };
            rd = next;
        }
        (Self::new(quo), Self::new(rem))
    }

    fn monic(&self) -> Self {
        let Some(d) = self.degree() else {
            return self.clone();
        };
        let k = invm(self.0[d]);
        Self::new(self.0.iter().map(|&c| mulm(c, k)).collect())
    }

    fn gcd(mut a: Self, mut b: Self) -> Self {
        while !b.is_zero() {
            let r = a.div_rem(&b).1;
            a = b;
            b = r;
        }
        a.monic()
    }
}

fn roots_of_unity(d: u64) -> Result<Vec<u64>, String> {
    let zeta = primitive_root_of_unity(P, d)
        .ok_or_else(|| format!("no primitive {d}-th root modulo {P}"))?;
    let roots: Vec<u64> = (0..d).map(|i| powm(zeta, i)).collect();
    let unique: std::collections::BTreeSet<_> = roots.iter().copied().collect();
    if unique.len() != d as usize || roots.iter().any(|&w| powm(w, d) != 1) {
        return Err(format!("invalid mu_{d} enumeration"));
    }
    Ok(roots)
}

fn x_of(w: u64) -> u64 {
    addm(LO, mulm(SCALE, w))
}

/// S3(x,y,U), constant coefficient first.
fn s3_factor(x: u64, y: u64) -> Poly {
    let sum = addm(x, y);
    let product = mulm(x, y);
    let a = mulm(subm(x, y), subm(x, y));
    let beta = addm(mulm(sum, addm(product, A)), addm(B, B));
    let b = subm(0, addm(beta, beta));
    let c = subm(
        mulm(subm(product, A), subm(product, A)),
        mulm(mulm(4, B), sum),
    );
    Poly::new(vec![c, b, a])
}

fn coefficient_sha256(poly: &Poly) -> String {
    let mut h = Sha256::new();
    for &c in &poly.0 {
        h.update(c.to_le_bytes());
    }
    format!("{:x}", h.finalize())
}

#[derive(serde::Serialize)]
struct BlockRecord {
    trace: u64,
    factor_count: usize,
    degree: usize,
    expected_degree: usize,
}

#[derive(serde::Serialize)]
struct OrderRecord {
    d: u64,
    roots: usize,
    pair_factors: usize,
    quadratic_factors: usize,
    linear_factors: usize,
    constant_factors: usize,
    zero_factors: usize,
    degree: usize,
    expected_degree: usize,
    repeated_gcd_degree: usize,
    squarefree: bool,
    coefficient_encoding: &'static str,
    coefficient_sha256: String,
    trace_blocks: Vec<BlockRecord>,
    block_product_equal: bool,
    every_factor_divides: bool,
    pass: bool,
}

fn certify(d: u64) -> Result<OrderRecord, String> {
    let roots = roots_of_unity(d)?;
    let mut all_factors = Vec::new();
    let mut blocks: BTreeMap<u64, Vec<Poly>> = BTreeMap::new();
    let mut quadratic = 0;
    let mut linear = 0;
    let mut constant = 0;
    let mut zero = 0;

    for i in 0..roots.len() {
        for j in i..roots.len() {
            let factor = s3_factor(x_of(roots[i]), x_of(roots[j]));
            match factor.degree() {
                None => zero += 1,
                Some(0) => constant += 1,
                Some(1) => linear += 1,
                Some(2) => quadratic += 1,
                Some(k) => return Err(format!("unexpected S3 factor degree {k}")),
            }
            let product = mulm(roots[i], roots[j]);
            let z = powm(product, d.div_ceil(2));
            if mulm(z, z) != product || powm(z, d) != 1 {
                return Err(format!(
                    "canonical pair scale failed at d={d}, i={i}, j={j}"
                ));
            }
            let q = mulm(roots[i], invm(z));
            let trace = addm(q, invm(q));
            blocks.entry(trace).or_default().push(factor.clone());
            all_factors.push(factor);
        }
    }

    let h = all_factors.iter().fold(Poly::one(), |acc, f| acc.mul(f));
    let repeated = Poly::gcd(h.clone(), h.derivative());
    let repeated_degree = repeated.degree().unwrap_or(0);
    let every_factor_divides = all_factors.iter().all(|f| h.div_rem(f).1.is_zero());

    let mut block_product = Poly::one();
    let mut block_records = Vec::new();
    for (trace, factors) in blocks {
        let block = factors.iter().fold(Poly::one(), |acc, f| acc.mul(f));
        let expected = if trace == 2 {
            d as usize
        } else {
            2 * d as usize
        };
        block_records.push(BlockRecord {
            trace,
            factor_count: factors.len(),
            degree: block.degree().unwrap_or(0),
            expected_degree: expected,
        });
        block_product = block_product.mul(&block);
    }
    let block_product_equal = block_product == h;
    let expected_degree = (d * d) as usize;
    let block_shapes_ok = block_records.len() == d.div_ceil(2) as usize
        && block_records.iter().all(|b| b.degree == b.expected_degree)
        && block_records.iter().map(|b| b.degree).sum::<usize>() == expected_degree;
    let degree = h.degree().unwrap_or(0);
    let pass = roots.len() == d as usize
        && all_factors.len() == (d * (d + 1) / 2) as usize
        && quadratic == (d * (d - 1) / 2) as usize
        && linear == d as usize
        && constant == 0
        && zero == 0
        && degree == expected_degree
        && repeated_degree == 0
        && block_shapes_ok
        && block_product_equal
        && every_factor_divides;

    Ok(OrderRecord {
        d,
        roots: roots.len(),
        pair_factors: all_factors.len(),
        quadratic_factors: quadratic,
        linear_factors: linear,
        constant_factors: constant,
        zero_factors: zero,
        degree,
        expected_degree,
        repeated_gcd_degree: repeated_degree,
        squarefree: repeated_degree == 0,
        coefficient_encoding: "u64-le constant-first",
        coefficient_sha256: coefficient_sha256(&h.monic()),
        trace_blocks: block_records,
        block_product_equal,
        every_factor_divides,
        pass,
    })
}

fn run() -> Result<(), String> {
    let orders: Vec<OrderRecord> = [7, 9, 19]
        .into_iter()
        .map(certify)
        .collect::<Result<_, _>>()?;
    let pass = orders.iter().all(|r| r.pass);
    println!(
        "{}",
        serde_json::json!({
            "schema": "curve25519.pair-image-degree-gate/v1",
            "study_id": "EXP-C25519-PAIRIMAGE-GATE-001",
            "revision": option_env!("GIT_COMMIT_HASH"),
            "p": P,
            "curve": {"a": A, "b": B},
            "factor_base": {"lo": LO, "scale": SCALE},
            "orders": orders,
            "decision": if pass {
                "close_explicit_pair_image; require_implicit_o_d2_common_root_algorithm"
            } else {
                "retain_exception_for_compression_derivation"
            },
            "claim_boundary": "exact finite algebraic degree gate; no asymptotic lower bound or ECDLP speed claim",
            "status": if pass { "pass" } else { "exception" }
        })
    );
    if pass {
        Ok(())
    } else {
        Err("one or more pair-image degree gates failed".into())
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("curve25519_pair_image_degree: {e}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polynomial_multiply_divide_gcd_controls() {
        let f = Poly::new(vec![3, 5, 1]);
        let g = Poly::new(vec![7, 1]);
        let product = f.mul(&g);
        assert_eq!(product.div_rem(&f), (g.clone(), Poly(Vec::new())));
        assert_eq!(product.div_rem(&g), (f.clone(), Poly(Vec::new())));
        assert_eq!(
            Poly::gcd(product.clone(), product.derivative()),
            Poly::one()
        );
        let repeated = f.mul(&f).mul(&g);
        assert_eq!(
            Poly::gcd(repeated.clone(), repeated.derivative()),
            f.monic()
        );
    }

    #[test]
    fn d7_trace_blocks_and_corruption_control() {
        let record = certify(7).unwrap();
        assert!(record.pass);
        assert_eq!(record.degree, 49);
        assert_eq!(record.trace_blocks.len(), 4);

        let roots = roots_of_unity(7).unwrap();
        let factors: Vec<_> = (0..roots.len())
            .flat_map(|i| (i..roots.len()).map(move |j| (i, j)))
            .map(|(i, j)| s3_factor(x_of(roots[i]), x_of(roots[j])))
            .collect();
        let h = factors.iter().fold(Poly::one(), |acc, f| acc.mul(f));
        let mut corrupted = h.clone();
        corrupted.0[0] = addm(corrupted.0[0], 1);
        assert_ne!(corrupted, h);
        assert!(factors.iter().any(|f| !corrupted.div_rem(f).1.is_zero()));
    }
}
