//! Reproducible subspaces; full bases are recorded, not inferred from names.
use super::model::Result;
use crypto_lib::{
    binary_ecc::F2mElement,
    cryptanalysis::{
        descent_lowgamma::{BasisFamily, FactorSubspace},
        ic_boundary::BinaryInstance,
    },
};
use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn rank(xs: &[u64]) -> usize {
    let mut pivots = [0u64; 64];
    let mut rank = 0;
    for &x in xs {
        let mut x = x;
        while x != 0 {
            let i = 63 - x.leading_zeros() as usize;
            if pivots[i] == 0 {
                pivots[i] = x;
                rank += 1;
                break;
            }
            x ^= pivots[i];
        }
    }
    rank
}
fn trace(i: &BinaryInstance, mut x: u64) -> u64 {
    let mut t = 0;
    for _ in 0..i.n {
        t ^= x;
        x = i.gf.sqr(x);
    }
    t
}
fn pow(i: &BinaryInstance, mut x: u64, mut n: u64) -> u64 {
    let mut y = 1;
    while n != 0 {
        if n & 1 != 0 {
            y = i.gf.mul(y, x);
        }
        x = i.gf.sqr(x);
        n >>= 1;
    }
    y
}

pub fn basis(i: &BinaryInstance, shape: &str, dim: u32, seed: u64) -> Result<Vec<u64>> {
    if dim == 0 || dim > i.n || dim > 8 || i.n > 31 {
        return Err("shape engine requires dimension1..8 <= field degree <=31".into());
    }
    if shape == "subfield" {
        return FactorSubspace::build(BasisFamily::Subfield, i.n, dim, &i.irreducible, seed)
            .map(|v| v.cols.iter().map(|x| x.raw_bits()[0]).collect())
            .ok_or_else(|| "inapplicable: requested subfield does not exist".into());
    }
    let mut rng = StdRng::seed_from_u64(seed);
    let mask = (1u64 << i.n) - 1;
    for _ in 0..256 {
        let g = rng.gen_range(2..=mask);
        let alpha = rng.gen_range(1..=mask);
        let cols: Vec<u64> = match shape {
            "coordinate" => (0..dim).map(|k| 1 << k).collect(),
            "gp" => (0..dim).map(|k| pow(i, g, k.into())).collect(),
            "scaled" => (0..dim).map(|k| i.gf.mul(alpha, 1 << k)).collect(),
            "symmetric" => (0..dim)
                .map(|k| pow(i, g, (mask - u64::from(dim / 2) + u64::from(k)) % mask))
                .collect(),
            "small-order" => {
                let Some(order) = (2..=mask.min(4096))
                    .find(|d| mask.is_multiple_of(*d) && *d >= u64::from(2 * dim - 1))
                else {
                    return Err("no suitable bounded small multiplicative order".into());
                };
                let z = pow(i, g, mask / order);
                (0..dim).map(|k| pow(i, z, k.into())).collect()
            }
            "frobenius" | "frobenius2" | "tracezero-frobenius" => {
                let a = if shape == "tracezero-frobenius" {
                    alpha ^ i.gf.sqr(alpha)
                } else {
                    alpha
                };
                let stride = if shape == "frobenius2" { 2 } else { 1 };
                (0..dim)
                    .map(|k| {
                        let mut x = a;
                        for _ in 0..stride * k {
                            x = i.gf.sqr(x);
                        }
                        x
                    })
                    .collect()
            }
            "mixed" => {
                let Some(a) = (2..=dim).find(|a| dim.is_multiple_of(*a) && a * a <= dim) else {
                    return Err("mixed shape requires a composite dimension".into());
                };
                let mut cols = Vec::new();
                for k in 0..a {
                    let mut x = alpha;
                    for _ in 0..dim / a {
                        cols.push(i.gf.mul(pow(i, g, k.into()), x));
                        x = i.gf.sqr(x);
                    }
                }
                cols
            }
            "random" => (0..dim).map(|_| rng.gen_range(1..=mask)).collect(),
            "tracezero" => (0..dim)
                .map(|_| {
                    let x = rng.gen_range(1..=mask);
                    x ^ i.gf.sqr(x)
                })
                .collect(),
            "tracezero-gp" => {
                let powers: Vec<_> = (0..dim).map(|k| pow(i, g, k.into())).collect();
                let Some(a) = (1..=mask.min(65535))
                    .find(|a| powers.iter().all(|p| trace(i, i.gf.mul(*a, *p)) == 0))
                else {
                    continue;
                };
                powers.iter().map(|p| i.gf.mul(a, *p)).collect()
            }
            _ => return Err(format!("unknown binary shape {shape}")),
        };
        if rank(&cols) == dim as usize {
            return Ok(cols);
        }
    }
    Err("bounded shape construction did not find a full-rank basis".into())
}
pub fn subspace(i: &BinaryInstance, cols: &[u64]) -> FactorSubspace {
    FactorSubspace {
        n: i.n,
        n_sub: cols.len() as u32,
        cols: cols
            .iter()
            .map(|&x| F2mElement::from_biguint(&x.into(), i.n))
            .collect(),
        family: BasisFamily::Random,
    }
}

pub fn recipe_basis(
    params: &std::collections::BTreeMap<String, String>,
    dimension: u32,
) -> Result<Vec<u64>> {
    if let Some(text) = params.get("basis") {
        text.split(';')
            .map(|s| {
                if let Some(s) = s.strip_prefix("0x") {
                    u64::from_str_radix(s, 16)
                } else {
                    s.parse()
                }
            })
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())
    } else {
        Ok((0..dimension).map(|k| 1u64 << k).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::cryptanalysis::ic_boundary::koblitz_instance;
    #[test]
    fn structured_bases_are_reproducible_full_rank_and_trace_zero_is_checked() {
        let i = koblitz_instance(1, 7).unwrap();
        for shape in [
            "coordinate",
            "gp",
            "scaled",
            "symmetric",
            "small-order",
            "frobenius",
            "frobenius2",
            "random",
            "tracezero",
            "tracezero-gp",
            "tracezero-frobenius",
        ] {
            let a = basis(&i, shape, 3, 9).unwrap();
            assert_eq!(a, basis(&i, shape, 3, 9).unwrap());
            assert_eq!(rank(&a), 3);
            if shape.starts_with("tracezero") {
                assert!(a.iter().all(|x| trace(&i, *x) == 0));
            }
        }
        assert!(basis(&i, "subfield", 3, 9).is_err());
        assert!(basis(&i, "mixed", 3, 9).is_err());
        assert_eq!(rank(&basis(&i, "mixed", 4, 9).unwrap()), 4);
    }
}
