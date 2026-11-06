//! Exact field-subspace diagnostics, independent of point yield and solver behaviour.
use super::model::Result;
use crypto_lib::binary_ecc::{F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::ecbench::canonical::sha256_hex;
use num_bigint::BigUint;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub schema: String,
    pub field_degree: u32,
    /// Exponents below the implicit monic leading term; include zero exactly once.
    pub modulus_low_terms: Vec<u32>,
    /// Polynomial-basis bit strings in hexadecimal, with optional 0x prefix.
    pub basis_hex: Vec<String>,
    pub max_power: u32,
}

#[derive(Debug, Serialize)]
pub struct Report {
    pub schema: &'static str,
    pub request: Request,
    pub canonical_basis_hex: Vec<String>,
    pub subspace_id: String,
    pub product_dimensions: Vec<usize>,
    pub product_basis_hex: Vec<Vec<String>>,
    pub linearization_excess_m2: Option<i64>,
    pub sum_product_dimensions: usize,
    pub product_field_multiplications: usize,
    pub frobenius_stable: bool,
    pub scope: &'static str,
    pub predicted_solving_degree: Option<u32>,
    pub certified_degree_of_regularity: Option<u32>,
    pub total_cost_winner: Option<String>,
}

// Struct field order is part of this identity's wire format. Do not hash a
// serde_json::Map: its key order depends on the preserve_order feature.
#[derive(Serialize)]
struct SubspaceIdentity<'a> {
    schema: &'static str,
    field_degree: u32,
    modulus_low_terms: &'a [u32],
    basis_hex: &'a [String],
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstructRequest {
    pub schema: String,
    pub field_degree: u32,
    pub modulus_low_terms: Vec<u32>,
    pub dimension: u32,
    pub generator_hex: String,
    pub seed: u64,
    pub max_power: u32,
}

#[derive(Debug, Serialize)]
pub struct Construction {
    pub schema: &'static str,
    pub request: ConstructRequest,
    pub generator_hex: String,
    pub scale_hex: String,
    pub kernel_basis_hex: Vec<String>,
    pub selector_hex: String,
    pub sampling_attempts: u32,
    pub constraint_rank: usize,
    pub profile: Report,
    pub scope: &'static str,
}

/// Standard GF(2) nullspace construction; sample coefficients by XOR, not integer addition.
fn kernel(mut rows: Vec<BigUint>, n: u32) -> (Vec<BigUint>, usize) {
    let mut pivots = Vec::new();
    for col in 0..n {
        let r = pivots.len();
        let Some(p) = (r..rows.len()).find(|i| rows[*i].bit(u64::from(col))) else {
            continue;
        };
        rows.swap(r, p);
        let pivot = rows[r].clone();
        for (i, row) in rows.iter_mut().enumerate() {
            if i != r && row.bit(u64::from(col)) {
                *row ^= &pivot;
            }
        }
        pivots.push(col);
    }
    let mut basis = Vec::new();
    for free in (0..n).filter(|p| !pivots.contains(p)) {
        let mut v = BigUint::from(1u8) << free as usize;
        for (r, p) in pivots.iter().enumerate() {
            if rows[r].bit(u64::from(free)) {
                v ^= BigUint::from(1u8) << *p as usize;
            }
        }
        basis.push(v);
    }
    (basis, pivots.len())
}

pub fn construct(r: &ConstructRequest) -> Result<Construction> {
    if r.schema != "factor_base_construct_request/v1"
        || r.dimension == 0
        || r.dimension > 64
        || r.dimension >= r.field_degree
    {
        return Err("construction needs schema factor_base_construct_request/v1 and 1..=64 dimensions strictly below the field degree".into());
    }
    let field_request = Request {
        schema: "factor_base_profile_request/v1".into(),
        field_degree: r.field_degree,
        modulus_low_terms: r.modulus_low_terms.clone(),
        basis_hex: vec!["1".into()],
        max_power: r.max_power,
    };
    let f = checked_field(&field_request)?;
    let g = parse_hex(&r.generator_hex)?;
    if g == BigUint::default() || g.bits() > u64::from(r.field_degree) {
        return Err("progression generator must be a nonzero in-field element".into());
    }
    let g = F2mElement::from_biguint(&g, r.field_degree);
    let mut powers = vec![F2mElement::one(r.field_degree)];
    for j in 1..r.dimension as usize {
        powers.push(powers[j - 1].mul(&g, &f));
    }
    if canonical_basis(powers.clone(), r.field_degree).len() != r.dimension as usize {
        return Err("progression powers are not independent at the requested dimension".into());
    }
    eprintln!(
        "[construct] building trace constraints: degree={}, dimension={}",
        r.field_degree, r.dimension
    );
    let rows = powers
        .iter()
        .map(|p| {
            let mut row = BigUint::default();
            for bit in 0..r.field_degree {
                if F2mElement::from_bit_positions(&[bit], r.field_degree)
                    .mul(p, &f)
                    .trace(&f)
                    != 0
                {
                    row.set_bit(u64::from(bit), true);
                }
            }
            row
        })
        .collect();
    let (kernel_basis, constraint_rank) = kernel(rows, r.field_degree);
    if kernel_basis.is_empty() {
        return Err("trace constraint kernel contains no nonzero scale".into());
    }
    let mut rng = StdRng::seed_from_u64(r.seed);
    for sampling_attempts in 1..=256 {
        let mut c = BigUint::default();
        let mut selector = BigUint::default();
        for (j, v) in kernel_basis.iter().enumerate() {
            if rng.gen::<bool>() {
                c ^= v;
                selector.set_bit(j as u64, true);
            }
        }
        if c == BigUint::default() {
            continue;
        }
        let scale = F2mElement::from_biguint(&c, r.field_degree);
        let columns: Vec<_> = powers.iter().map(|p| scale.mul(p, &f)).collect();
        if columns.iter().any(|x| x.trace(&f) != 0) {
            return Err("internal error: sampled scale violates a trace constraint".into());
        }
        let request = Request {
            basis_hex: hex_basis(&columns),
            ..field_request
        };
        let profile = compute(&request)?;
        eprintln!("[construct] trace-zero progression ready: kernel_dimension={}, draw={sampling_attempts}", kernel_basis.len());
        return Ok(Construction { schema: "factor_base_construction/v1", request:r.clone(),
            generator_hex:format!("0x{:x}",g.to_biguint()), scale_hex:format!("0x{c:x}"),
            kernel_basis_hex:kernel_basis.iter().map(|x|format!("0x{x:x}")).collect(),
            selector_hex:format!("0x{selector:x}"), sampling_attempts, constraint_rank, profile,
            scope:"standard trace-constrained abscissa-subspace construction only; seeded pseudorandom nonzero kernel coefficients are XORed (not a claim of uniform coverage of the entire kernel from 64-bit seeds); no point inventory, subgroup support, measured solver cost, regularity or speed claim; profile multiplication counter excludes construction and other setup work" });
    }
    Err("bounded kernel sampling produced no nonzero scale in 256 draws".into())
}

fn parse_hex(s: &str) -> Result<BigUint> {
    let digits = s.strip_prefix("0x").unwrap_or(s);
    if digits.is_empty() || digits.len() > 144 || !digits.bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err("basis entries must be hexadecimal bit strings of at most 144 digits".into());
    }
    BigUint::parse_bytes(digits.as_bytes(), 16).ok_or_else(|| "invalid hexadecimal element".into())
}

fn polynomial_remainder(mut a: BigUint, b: &BigUint) -> BigUint {
    while a != BigUint::default() && a.bits() >= b.bits() {
        let shift = (a.bits() - b.bits()) as usize;
        a ^= b << shift;
    }
    a
}

fn polynomial_gcd(mut a: BigUint, mut b: BigUint) -> BigUint {
    while b != BigUint::default() {
        let r = polynomial_remainder(a, &b);
        a = b;
        b = r;
    }
    a
}

/// Rabin's criterion; never interpret a quotient ring as a finite field silently.
fn checked_field(r: &Request) -> Result<IrreduciblePoly> {
    if !(2..=571).contains(&r.field_degree) || !(1..=5).contains(&r.max_power) {
        return Err("profile requires field degree 2..=571 and max_power 1..=5".into());
    }
    let mut terms = r.modulus_low_terms.clone();
    terms.sort_unstable();
    if terms.first() != Some(&0)
        || terms.last().is_some_and(|x| *x >= r.field_degree)
        || terms.windows(2).any(|w| w[0] == w[1])
    {
        return Err("modulus needs distinct low terms in 0..field_degree, including zero".into());
    }
    let f = IrreduciblePoly {
        degree: r.field_degree,
        low_terms: terms.clone(),
    };
    let mut modulus = BigUint::from(1u8) << r.field_degree as usize;
    for t in terms {
        modulus ^= BigUint::from(1u8) << t as usize;
    }
    let mut n = r.field_degree;
    let mut factors = Vec::new();
    for p in 2..=r.field_degree {
        if n.is_multiple_of(p) {
            factors.push(p);
            while n.is_multiple_of(p) {
                n /= p;
            }
        }
    }
    let x = F2mElement::z(r.field_degree);
    let mut power = x.clone();
    for k in 1..=r.field_degree {
        power = power.square(&f);
        if factors.iter().any(|p| k == r.field_degree / p)
            && polynomial_gcd(power.add(&x).to_biguint(), modulus.clone()) != BigUint::from(1u8)
        {
            return Err("modulus is reducible over GF(2)".into());
        }
    }
    if power != x {
        return Err("modulus is reducible over GF(2)".into());
    }
    Ok(f)
}

fn canonical_basis(values: impl IntoIterator<Item = F2mElement>, degree: u32) -> Vec<F2mElement> {
    let mut pivots: Vec<Option<F2mElement>> = vec![None; degree as usize];
    for mut x in values {
        while let Some(p) = x.degree().map(|d| d as usize) {
            if let Some(y) = &pivots[p] {
                x.add_assign(y);
            } else {
                pivots[p] = Some(x);
                break;
            }
        }
    }
    // Reduced echelon form makes the certificate independent of input ordering/basis.
    for p in 0..degree as usize {
        if let Some(x) = pivots[p].clone() {
            for y in pivots.iter_mut().skip(p + 1).flatten() {
                if y.to_biguint().bit(p as u64) {
                    y.add_assign(&x);
                }
            }
        }
    }
    pivots.into_iter().flatten().collect()
}

fn hex_basis(v: &[F2mElement]) -> Vec<String> {
    v.iter()
        .map(|x| format!("0x{:x}", x.to_biguint()))
        .collect()
}

pub fn compute(r: &Request) -> Result<Report> {
    if r.schema != "factor_base_profile_request/v1"
        || r.basis_hex.is_empty()
        || r.basis_hex.len() > 64
    {
        return Err(
            "profile needs schema factor_base_profile_request/v1 and 1..=64 independent columns"
                .into(),
        );
    }
    let f = checked_field(r)?;
    let original = r
        .basis_hex
        .iter()
        .map(|s| {
            let x = parse_hex(s)?;
            if x == BigUint::default() || x.bits() > r.field_degree as u64 {
                return Err(
                    "basis entries must be nonzero elements with degree below field_degree".into(),
                );
            }
            Ok(F2mElement::from_biguint(&x, r.field_degree))
        })
        .collect::<Result<Vec<_>>>()?;
    let basis = canonical_basis(original, r.field_degree);
    if basis.len() != r.basis_hex.len() {
        return Err("basis entries must be linearly independent over GF(2)".into());
    }
    let canonical_hex = hex_basis(&basis);
    let subspace_id = {
        let identity = SubspaceIdentity {
            schema: "field_subspace/v1",
            field_degree: r.field_degree,
            modulus_low_terms: &f.low_terms,
            basis_hex: &canonical_hex,
        };
        format!(
            "FSP1h{}",
            sha256_hex(&serde_json::to_vec(&identity).map_err(|e| e.to_string())?)
        )
    };
    let frobenius_stable = canonical_basis(
        basis
            .iter()
            .cloned()
            .chain(basis.iter().map(|x| x.square(&f))),
        r.field_degree,
    )
    .len()
        == basis.len();
    eprintln!(
        "[profile] field checked: degree={}, subspace dimension={}",
        r.field_degree,
        basis.len()
    );
    let mut cur = basis.clone();
    let mut dimensions = Vec::new();
    let mut certificates = Vec::new();
    let mut multiplications = 0;
    for k in 1..=r.max_power {
        if k > 1 {
            let mut products = Vec::with_capacity(cur.len() * basis.len());
            for a in &cur {
                for b in &basis {
                    products.push(a.mul(b, &f));
                    multiplications += 1;
                }
            }
            cur = canonical_basis(products, r.field_degree);
        }
        eprintln!(
            "[profile] power={k}, dimension={}, product multiplications={multiplications}",
            cur.len()
        );
        dimensions.push(cur.len());
        certificates.push(hex_basis(&cur));
    }
    Ok(Report {
        schema: "factor_base_profile/v1",
        request: r.clone(),
        canonical_basis_hex: canonical_hex,
        subspace_id,
        linearization_excess_m2: dimensions.get(1).map(|d| i64::from(r.field_degree) - basis.len() as i64 - *d as i64),
        sum_product_dimensions: dimensions.iter().sum(),
        product_dimensions: dimensions,
        product_basis_hex: certificates,
        product_field_multiplications: multiplications,
        frobenius_stable,
        scope: "exact field-subspace structure only; no point inventory, yield, solver, regularity or speed measurement; multiplication counter excludes field validation, rank and allocation work",
        predicted_solving_degree: None,
        certified_degree_of_regularity: None,
        total_cost_winner: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn request() -> Request {
        Request {
            schema: "factor_base_profile_request/v1".into(),
            field_degree: 3,
            modulus_low_terms: vec![0, 1],
            basis_hex: vec!["1".into(), "2".into()],
            max_power: 3,
        }
    }
    #[test]
    fn small_field_profile_has_hand_checked_products() {
        let r = compute(&request()).unwrap();
        assert_eq!(r.product_dimensions, [2, 3, 3]);
        assert_eq!(r.linearization_excess_m2, Some(-2));
        assert_eq!(
            r.product_basis_hex,
            [
                vec!["0x1", "0x2"],
                vec!["0x1", "0x2", "0x4"],
                vec!["0x1", "0x2", "0x4"]
            ]
        );
        assert_eq!(r.product_field_multiplications, 10);
        let wire = r#"{"schema":"field_subspace/v1","field_degree":3,"modulus_low_terms":[0,1],"basis_hex":["0x1","0x2"]}"#;
        assert_eq!(
            r.subspace_id,
            format!("FSP1h{}", sha256_hex(wire.as_bytes()))
        );
        assert!(r.predicted_solving_degree.is_none());
    }
    #[test]
    fn changing_coordinates_keeps_canonical_certificates() {
        let a = compute(&request()).unwrap();
        let mut b = request();
        b.basis_hex = vec!["3".into(), "2".into()];
        let b = compute(&b).unwrap();
        assert_eq!(a.canonical_basis_hex, b.canonical_basis_hex);
        assert_eq!(a.product_basis_hex, b.product_basis_hex);
        assert_eq!(a.subspace_id, b.subspace_id);
    }
    #[test]
    fn all_two_dimensional_gf8_bases_match_an_independent_exhaustive_oracle() {
        fn mul(mut a: u8, mut b: u8) -> u8 {
            let mut c = 0;
            while b != 0 {
                if b & 1 != 0 {
                    c ^= a;
                }
                b >>= 1;
                a <<= 1;
                if a & 8 != 0 {
                    a ^= 0b1011;
                }
            }
            c
        }
        for a in 1u8..8 {
            for b in 1u8..8 {
                if a == b {
                    continue;
                }
                let mut span = std::collections::BTreeSet::from([0u8]);
                for x in [0, a, b, a ^ b] {
                    for y in [0, a, b, a ^ b] {
                        let product = mul(x, y);
                        let extra: Vec<_> = span.iter().map(|z| z ^ product).collect();
                        span.extend(extra);
                    }
                }
                let mut r = request();
                r.basis_hex = vec![format!("{a:x}"), format!("{b:x}")];
                assert_eq!(
                    compute(&r).unwrap().product_dimensions[1],
                    span.len().ilog2() as usize
                );
            }
        }
    }
    #[test]
    fn reducible_modulus_and_dependent_or_truncated_inputs_are_rejected() {
        let mut r = request();
        r.field_degree = 4;
        r.modulus_low_terms = vec![0, 2];
        assert!(compute(&r).unwrap_err().contains("reducible"));
        let mut r = request();
        r.basis_hex.push("3".into());
        assert!(compute(&r).unwrap_err().contains("independent"));
        let mut r = request();
        r.basis_hex = vec!["8".into()];
        assert!(compute(&r).unwrap_err().contains("below"));
    }
    #[test]
    fn repeated_modulus_terms_and_zero_basis_are_rejected() {
        let mut r = request();
        r.modulus_low_terms = vec![0, 1, 1];
        assert!(compute(&r).is_err());
        let mut r = request();
        r.basis_hex = vec!["0".into()];
        assert!(compute(&r).is_err());
        let mut r = request();
        r.max_power = 0;
        assert!(compute(&r).is_err());
    }
    #[test]
    fn wide_field_profile_does_not_truncate_or_invent_a_degree_prediction() {
        let r = Request {
            field_degree: 131,
            modulus_low_terms: vec![0, 2, 3, 8],
            basis_hex: vec!["1".into(), "2".into(), "4".into()],
            max_power: 5,
            ..request()
        };
        let a = compute(&r).unwrap();
        assert_eq!(a.product_dimensions, [3, 5, 7, 9, 11]);
        assert_eq!(a.linearization_excess_m2, Some(123));
        assert!(a.certified_degree_of_regularity.is_none());
        assert!(a.total_cost_winner.is_none());
        let mut high = r;
        high.basis_hex = vec!["400000000000000000000000000000000".into()];
        assert_eq!(
            compute(&high).unwrap().canonical_basis_hex,
            high.basis_hex
                .iter()
                .map(|x| format!("0x{x}"))
                .collect::<Vec<_>>()
        );
    }
    #[test]
    fn first_power_only_leaves_excess_unavailable() {
        let mut r = request();
        r.max_power = 1;
        assert_eq!(compute(&r).unwrap().linearization_excess_m2, None);
    }
    fn construction_request() -> ConstructRequest {
        ConstructRequest {
            schema: "factor_base_construct_request/v1".into(),
            field_degree: 3,
            modulus_low_terms: vec![0, 1],
            dimension: 2,
            generator_hex: "0x2".into(),
            seed: 42,
            max_power: 3,
        }
    }
    #[test]
    fn kernel_construction_reproduces_and_its_selector_rebuilds_the_scale() {
        let r = construction_request();
        let a = construct(&r).unwrap();
        let b = construct(&r).unwrap();
        assert_eq!(
            serde_json::to_value(&a).unwrap(),
            serde_json::to_value(&b).unwrap()
        );
        assert_eq!(a.constraint_rank, 2);
        assert_eq!(a.kernel_basis_hex.len(), 1);
        let selector = parse_hex(&a.selector_hex).unwrap();
        let mut scale = BigUint::default();
        for (j, v) in a.kernel_basis_hex.iter().enumerate() {
            if selector.bit(j as u64) {
                scale ^= parse_hex(v).unwrap();
            }
        }
        assert_eq!(scale, parse_hex(&a.scale_hex).unwrap());
        assert_ne!(scale, BigUint::default());
        let f = checked_field(&a.profile.request).unwrap();
        for value in &a.profile.request.basis_hex {
            // Direct Frobenius sum, separate from the cached trace implementation used by construction.
            let mut x = F2mElement::from_biguint(&parse_hex(value).unwrap(), r.field_degree);
            let mut t = F2mElement::zero(r.field_degree);
            for _ in 0..r.field_degree {
                t.add_assign(&x);
                x = x.square(&f);
            }
            assert!(t.is_zero());
        }
    }
    #[test]
    fn wide_kernel_construction_keeps_trace_and_progression_dimensions() {
        let r = ConstructRequest {
            field_degree: 131,
            modulus_low_terms: vec![0, 2, 3, 8],
            dimension: 12,
            max_power: 5,
            ..construction_request()
        };
        let a = construct(&r).unwrap();
        assert_eq!(a.constraint_rank, 12);
        assert_eq!(a.kernel_basis_hex.len(), 119);
        assert_eq!(a.profile.product_dimensions, [12, 23, 34, 45, 56]);
        assert_eq!(a.profile.linearization_excess_m2, Some(96));
        let f = checked_field(&a.profile.request).unwrap();
        assert!(a
            .profile
            .request
            .basis_hex
            .iter()
            .all(|x| F2mElement::from_biguint(&parse_hex(x).unwrap(), 131).trace(&f) == 0));
        assert!(a.profile.predicted_solving_degree.is_none());
    }
    #[test]
    fn construction_rejects_full_dimension_dependent_powers_and_out_of_field_generator() {
        let mut r = construction_request();
        r.dimension = 3;
        assert!(construct(&r).is_err());
        let mut r = construction_request();
        r.generator_hex = "1".into();
        assert!(construct(&r).unwrap_err().contains("independent"));
        let mut r = construction_request();
        r.generator_hex = "8".into();
        assert!(construct(&r).unwrap_err().contains("in-field"));
    }
}
