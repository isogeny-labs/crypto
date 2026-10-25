//! Native wide-core correctness and retained-profile audits; no complete IC claim.
use crypto_lib::binary_ecc::{
    curve::{point_add, point_neg, scalar_mul},
    BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly,
};
use crypto_lib::cryptanalysis::{
    curve_id,
    gaudry_quartic_bin_wide::{self as wide, BinCurve, PtB, TowerBasis},
    semaev_decomp::Gf2_128,
};
use num_bigint::BigUint;
use num_traits::One;
use rand::{rngs::StdRng, SeedableRng};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::path::Path;

fn digest(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}
fn read(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}
fn prime_big(n: u128) -> bool {
    assert!(n < (1u128 << 73));
    if n < 2 {
        return false;
    }
    let s = (n - 1).trailing_zeros();
    let d = BigUint::from((n - 1) >> s);
    let bn = BigUint::from(n);
    let nm1 = &bn - BigUint::one();
    'bases: for p in [2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n == u128::from(p) {
            return true;
        }
        if n % u128::from(p) == 0 {
            return false;
        }
        let mut x = BigUint::from(p).modpow(&d, &bn);
        if x.is_one() || x == nm1 {
            continue;
        }
        for _ in 1..s {
            x = (&x * &x) % &bn;
            if x == nm1 {
                continue 'bases;
            }
        }
        return false;
    }
    true
}
fn independent_point(inst: &BinCurve, basis: &TowerBasis, p: PtB) -> BinaryPoint {
    if p.inf {
        BinaryPoint::Infinity
    } else {
        let n = 4 * inst.curve.f.f.m;
        BinaryPoint::Affine {
            x: F2mElement::from_biguint(&BigUint::from(basis.coords(&inst.curve.f, &p.x)), n),
            y: F2mElement::from_biguint(&BigUint::from(basis.coords(&inst.curve.f, &p.y)), n),
        }
    }
}
fn point_json(inst: &BinCurve, basis: &TowerBasis, p: PtB) -> Value {
    if p.inf {
        json!({"infinity":true})
    } else {
        json!({
        "tower_x_hex":format!("0x{:x}",inst.curve.f.pack(&p.x)),
        "tower_y_hex":format!("0x{:x}",inst.curve.f.pack(&p.y)),
        "polynomial_x_hex":format!("0x{:x}",basis.coords(&inst.curve.f,&p.x)),
        "polynomial_y_hex":format!("0x{:x}",basis.coords(&inst.curve.f,&p.y))})
    }
}
fn core(m: u32) -> Value {
    assert!([16, 17, 18].contains(&m));
    let inst = wide::find_bin_curve(m, 1, 16).expect("bounded seed search exhausted");
    assert!(prime_big(inst.r));
    assert_eq!(inst.r.checked_mul(inst.cofactor), Some(inst.group_order));
    assert!(!inst.generator.inf && inst.curve.mul(&inst.generator, inst.r).inf);
    let q = 1u128 << m;
    let lo = q.pow(4) + 1 - 2 * q * q;
    let hi = q.pow(4) + 1 + 2 * q * q;
    let first = lo.div_ceil(inst.r) * inst.r;
    assert_eq!(first, inst.group_order);
    assert!(first + inst.r > hi);
    let basis = TowerBasis::new(&inst.curve.f);
    let n = 4 * m;
    let irr = IrreduciblePoly {
        degree: n,
        low_terms: (0..n).filter(|&i| basis.modulus >> i & 1 == 1).collect(),
    };
    let a = basis.coords(&inst.curve.f, &inst.curve.a);
    let b = basis.coords(&inst.curve.f, &inst.curve.b);
    let binary = BinaryCurve {
        m: n,
        irreducible: irr.clone(),
        a: F2mElement::from_biguint(&BigUint::from(a), n),
        b: F2mElement::from_biguint(&BigUint::from(b), n),
        generator: independent_point(&inst, &basis, inst.generator),
        order: BigUint::from(inst.r),
        cofactor: BigUint::from(inst.cofactor),
    };
    assert!(binary.is_on_curve(&binary.generator));
    assert_eq!(
        scalar_mul(&binary, &binary.generator, &BigUint::from(inst.r)),
        BinaryPoint::Infinity
    );
    let id = curve_id::binary(
        n,
        &BigUint::from(basis.modulus),
        &BigUint::from(a),
        &BigUint::from(b),
        &BigUint::from(inst.group_order),
        None,
    )
    .unwrap();
    let independent = Gf2_128::new(&irr);
    let mut rng = StdRng::seed_from_u64(0xc210_0000 | u64::from(m));
    for _ in 0..64 {
        let x = inst.curve.f.random(&mut rng);
        let y = inst.curve.f.random(&mut rng);
        assert_eq!(
            basis.coords(&inst.curve.f, &inst.curve.f.mul(&x, &y)),
            independent.mul(
                basis.coords(&inst.curve.f, &x),
                basis.coords(&inst.curve.f, &y)
            )
        );
        assert_eq!(inst.curve.f.unpack(inst.curve.f.pack(&x)), x);
    }
    let mut ps = vec![];
    while ps.len() < 4 {
        let x = inst.curve.f.random(&mut rng);
        if ps.is_empty() && inst.curve.f.pack(&x) < (1u128 << (n - 1)) {
            continue;
        }
        if let Some(p) = inst.curve.lift_x(&x, &mut rng) {
            assert!(binary.is_on_curve(&independent_point(&inst, &basis, p)));
            ps.push(p);
        }
    }
    let p = ps[0];
    let qp = ps[1];
    let bp = independent_point(&inst, &basis, p);
    assert_eq!(
        independent_point(&inst, &basis, inst.curve.add(&p, &qp)),
        point_add(&binary, &bp, &independent_point(&inst, &basis, qp))
    );
    assert_eq!(
        independent_point(&inst, &basis, inst.curve.neg(&p)),
        point_neg(&bp)
    );
    let ks = [
        0,
        1,
        17,
        (1u128 << 65) + 3,
        (1u128 << 70) + (1u128 << 64) + 123,
        inst.group_order,
    ];
    for k in ks {
        assert_eq!(
            independent_point(&inst, &basis, inst.curve.mul(&p, k)),
            scalar_mul(&binary, &bp, &BigUint::from(k))
        );
    }
    assert!(inst.curve.mul(&p, inst.group_order).inf);
    let r = ps.iter().fold(PtB::INF, |s, p| inst.curve.add(&s, p));
    assert!(!r.inf);
    assert_eq!(
        inst.curve.s5(&[ps[0].x, ps[1].x, ps[2].x, ps[3].x, r.x]),
        wide::Eb::ZERO
    );
    let encoded = point_json(&inst, &basis, p);
    let xstr = encoded["tower_x_hex"].as_str().unwrap();
    let parsed = u128::from_str_radix(xstr.strip_prefix("0x").unwrap(), 16).unwrap();
    assert_eq!(parsed, inst.curve.f.pack(&p.x));
    let number = inst.group_order.to_string();
    assert_eq!(number.parse::<u128>().unwrap(), inst.group_order);
    json!({"schema":"ic-quartic-wide-core-audit/v1","m":m,"n":n,"starting_instance_seed":1,"instance_seed":inst.instance_seed,
        "cofactor_cap":16,"group_order_decimal":number,"r_decimal":inst.r.to_string(),"cofactor_decimal":inst.cofactor.to_string(),
        "hasse_lo_decimal":lo.to_string(),"hasse_hi_decimal":hi.to_string(),"unique_hasse_multiple_of_r":true,
        "prime_check":"BigUint first-12-prime-base Miller-Rabin, deterministic below 318665857834031151167461; input below 2^73",
        "base_modulus":inst.curve.f.f.modulus,"extension_coefficients":inst.curve.f.c,
        "polynomial_modulus_hex":format!("0x{:x}",basis.modulus),"a_hex":format!("0x{a:x}"),"b_hex":format!("0x{b:x}"),
        "icv1":id.icv1,"curve_name":id.slug,"registry_status":"unregistered-research-model",
        "generator":point_json(&inst,&basis,inst.generator),"high_coordinate_control":encoded,
        "scalar_controls_decimal":ks.map(|k|k.to_string()),"independent_field_products":64,
        "independent_scalar_controls":ks.len(),"planted_s5_relation_verified":true,"serialization_roundtrips_verified":true,
        "status":"verified","scope":"core arithmetic, order and encoding controls; full IC pipeline, DLP and price unmeasured"})
}
fn profile(study: &Path) -> Value {
    let mut arms = vec![];
    let mut outputs = vec![];
    for arm in ["before", "after"] {
        let dir = study.join(format!("profiles/wide-constructor-{arm}-20261025"));
        let raw = std::fs::read(dir.join("callgrind.out")).unwrap();
        let text = std::str::from_utf8(&raw).unwrap();
        let extract = |prefix: &str| {
            text.lines()
                .filter_map(|l| l.strip_prefix(prefix))
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        };
        assert_eq!(
            text.lines().find(|l| l.starts_with("events:")),
            Some("events: Ir")
        );
        let summary = extract("summary:");
        let totals = extract("totals:");
        assert_eq!(summary.len(), 1);
        assert_eq!(summary, totals);
        let hotspots = read(&dir.join("hotspots.json"));
        assert_eq!(hotspots["total_ir"], json!(summary[0]));
        let manifest = read(&dir.join("manifest.json"));
        assert_eq!(manifest["status"], "ok");
        assert_eq!(manifest["hotspots_method"], "callgrind");
        let binary = manifest["binary"]["path"].as_str().unwrap();
        let local_binary = std::env::var_os("IC_PROFILE_BINARY_DIR").map_or_else(
            || Path::new(binary).to_path_buf(),
            |d| Path::new(&d).join(Path::new(binary).file_name().unwrap()),
        );
        assert_eq!(
            digest(&std::fs::read(local_binary).unwrap()),
            manifest["binary"]["sha256"].as_str().unwrap()
        );
        assert_eq!(manifest["command"], json!([binary, "wide", "6"]));
        let output = std::fs::read(dir.join("command.stdout")).unwrap();
        outputs.push(output);
        arms.push(
            json!({"arm":arm,"whole_process_ir":summary[0],"raw_sha256":digest(&raw),
            "source_commit":manifest["git"]["commit"],"binary_sha256":manifest["binary"]["sha256"],
            "host":manifest["host"],"isolation":manifest["isolation"]}),
        );
    }
    assert_eq!(outputs[0], outputs[1]);
    let mut constructors = vec![];
    for m in [16, 17, 18] {
        let d = study.join(format!(
            "pipeline-receipts/wide-field-constructors-20261025/m{m}"
        ));
        let status = read(&d.join("status.json"));
        assert_eq!(status["exit_code"], 0);
        let output = read(&d.join("output.json"));
        assert_eq!(output["m"], m);
        assert_eq!(output["mode"], "wide");
        constructors.push(output);
    }
    json!({"schema":"ic-quartic-constructor-profile-audit/v1","arms":arms,
        "constructor_output_byte_identical":true,"output":serde_json::from_slice::<Value>(&outputs[0]).unwrap(),
        "field_constructor_preflights":constructors,"status":"raw_totals_manifests_binaries_and_outputs_agree",
        "scope":"simulated whole-process instructions incl loader and Valgrind allocator; no native runtime, complete IC or fastest-tool claim",
        "missing_controls":["A/A instruction repeat","five interleaved A/B rounds","fastest external field-constructor comparison","full pipeline pricing"]})
}
fn main() {
    let a: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(a.len(), 2, "core M | profile STUDY");
    let result = match a[0].as_str() {
        "core" => core(a[1].parse().unwrap()),
        "profile" => profile(Path::new(&a[1])),
        _ => panic!("unknown mode"),
    };
    println!("{result}");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_big_prime_control_rejects_wide_composites() {
        for n in 0..512 {
            assert_eq!(
                prime_big(n),
                crypto_lib::cryptanalysis::residual_walk::is_prime_u64(n as u64)
            );
        }
        assert!(prime_big(18446744073709551557));
        assert!(!prime_big(3825123056546413051));
        assert!(!prime_big((1u128 << 65) + 2));
    }
}
