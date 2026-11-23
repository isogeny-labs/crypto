//! Frozen finite correctness qualifier, with separately implemented group
//! enumeration and certificate checking. No timing or speed measurement.
#![recursion_limit = "512"]
mod checker;
mod extractor;
mod independent;
#[path = "../../quotient_query/native/kernel.rs"]
mod kernel;
#[path = "../../quotient_query/native/norm.rs"]
mod norm;
#[path = "../../target_quotient/native/oracle.rs"]
#[allow(dead_code)]
mod prior_oracle;
#[path = "../../quotient_query/native/independent.rs"]
#[allow(dead_code)]
mod query_independent;
#[path = "../../../../src/cryptanalysis/semaev_prime_poly.rs"]
#[allow(dead_code)]
mod semaev_prime_poly;
#[path = "../../../../src/hash/sha256.rs"]
#[allow(dead_code)]
mod source_sha256;

use extractor::{Extraction, Input, Point, SubgroupSpec};
use semaev_prime_poly::{Curve, Fp};
use serde_json::json;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

const PROTOCOL: &[u8] = include_bytes!("../PROTOCOL.md");
const CASE_BYTE_CAP: usize = 32 * 1024 * 1024;
const STORED_BYTE_CAP: usize = 65536;

pub(crate) fn digest(bytes: &[u8]) -> String {
    source_sha256::sha256(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
fn canonical_target(c: Curve, x: u64) -> Point {
    let f = c.f;
    let rhs = f.add(f.add(f.mul(f.mul(x, x), x), f.mul(c.a, x)), c.b);
    let y = if f.p == 97 {
        (0..97)
            .find(|&y| f.mul(y, y) == rhs)
            .expect("declared target rational")
    } else {
        let y = f.pow(rhs, (f.p + 1) / 4);
        assert_eq!(f.mul(y, y), rhs, "declared target rational");
        y.min(f.neg(y))
    };
    [x, y]
}
fn add_fixture(
    out: &mut Vec<(Input, bool)>,
    label: &str,
    c: Curve,
    degrees: &[usize],
    target_xs: &[u64],
) {
    for &d in degrees {
        for target in target_xs
            .iter()
            .map(|&x| Some(canonical_target(c, x)))
            .chain(std::iter::once(None))
        {
            let tag = target.map_or_else(|| "O".to_owned(), |p| format!("x{}", p[0]));
            for restrict_liftable in [false, true] {
                let policy = if restrict_liftable { "liftable" } else { "all" };
                let id = format!("{label}-d{d}-{tag}-{policy}");
                let persist = (label == "f97" && d == 2)
                    || (label == "rank97" && d == 2)
                    || (label == "m31" && d == 7);
                out.push((
                    Input {
                        case_id: id,
                        p: c.f.p,
                        a: c.a,
                        b: c.b,
                        d,
                        target,
                        restrict_liftable,
                        subgroup: None,
                    },
                    persist,
                ));
            }
        }
    }
}
fn fixtures() -> Vec<(Input, bool)> {
    let mut cases = Vec::new();
    let ordinary = Curve::new(97, 2, 3);
    // These are the predecessor's first three rational non-two-torsion x's,
    // together with its named rational two-torsion target.
    add_fixture(
        &mut cases,
        "f97",
        ordinary,
        &[1, 2, 3, 4, 6, 8, 12, 16],
        &[0, 1, 3, 30],
    );
    add_fixture(
        &mut cases,
        "rank97",
        Curve::new(97, 96, 0),
        &[2],
        &[0, 1, 2],
    );
    let m31 = Curve::new(2147483647, 847514846, 2087444724);
    add_fixture(&mut cases, "m31", m31, &[7, 21, 63], &[0, 1, 2]);
    for (label, c, d) in [("f97", ordinary, 2), ("m31", m31, 7)] {
        let mut target = canonical_target(c, 1);
        target[1] = c.f.neg(target[1]);
        for restrict_liftable in [false, true] {
            let policy = if restrict_liftable { "liftable" } else { "all" };
            cases.push((
                Input {
                    case_id: format!("{label}-d{d}-nx1-{policy}"),
                    p: c.f.p,
                    a: c.a,
                    b: c.b,
                    d,
                    target: Some(target),
                    restrict_liftable,
                    subgroup: None,
                },
                true,
            ));
        }
    }
    for (tag, target) in [
        ("plus", Some([80, 10])),
        ("minus", Some([80, 87])),
        ("O", None),
        ("outside", Some([0, 10])),
    ] {
        for restrict_liftable in [false, true] {
            let policy = if restrict_liftable { "liftable" } else { "all" };
            cases.push((
                Input {
                    case_id: format!("sub5-d96-{tag}-{policy}"),
                    p: 97,
                    a: 2,
                    b: 3,
                    d: 96,
                    target,
                    restrict_liftable,
                    subgroup: Some(SubgroupSpec {
                        generator: [3, 6],
                        order: 5,
                    }),
                },
                restrict_liftable,
            ));
        }
    }
    for (tag, target) in [
        ("plus", Some([1, 14])),
        ("minus", Some([1, 83])),
        ("O", None),
        ("torsion", Some([0, 0])),
    ] {
        for restrict_liftable in [false, true] {
            let policy = if restrict_liftable { "liftable" } else { "all" };
            cases.push((
                Input {
                    case_id: format!("order4-d1-{tag}-{policy}"),
                    p: 97,
                    a: 1,
                    b: 0,
                    d: 1,
                    target,
                    restrict_liftable,
                    subgroup: None,
                },
                true,
            ));
        }
    }
    assert_eq!(cases.len(), 132);
    assert_eq!(cases.iter().filter(|(_, p)| *p).count(), 42);
    cases
}
fn save(out: &Path, id: &str, suffix: &str, bytes: &[u8]) {
    assert!(
        bytes.len() <= STORED_BYTE_CAP,
        "predeclared artifact exceeded store threshold; stop and preserve failure"
    );
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.join(format!("{id}.{suffix}.json")))
        .unwrap();
    file.write_all(bytes).unwrap();
}
fn mask_regression(input: &Input, out: &Extraction) {
    if !input.case_id.starts_with("order4-d1-minus-") {
        return;
    }
    let p = out.base.binary_search(&[1, 14]).unwrap();
    let minus_p = out.base.binary_search(&[1, 83]).unwrap();
    for triple in [[p, p, p], [minus_p, minus_p, p]] {
        let i = out
            .triples
            .binary_search(&triple)
            .expect("the finite cancellation-mask cell must preserve both signed rows");
        assert_eq!(out.routes[i].chart, 0);
        assert_eq!(out.routes[i].t, Some(0));
    }
    let i = out
        .triples
        .binary_search(&[minus_p, p, minus_p])
        .expect("identity prefix separately retained");
    assert_eq!(out.routes[i].chart, 1);
    assert!(out.stats.masked_noncancelling_triples > 0);
}
fn cancellation_regression(input: &Input, out: &Extraction) {
    if let Some(target) = input.target.filter(|p| out.base.binary_search(p).is_ok()) {
        let expected = 3 * out.base.len() - if target[1] == 0 { 2 } else { 3 };
        assert_eq!(
            out.stats.inverse_pair_triples, expected,
            "complete inverse-pair union count"
        );
        assert_eq!(
            out.stats.identity_prefix_triples,
            out.base.len(),
            "one identity prefix per signed base point"
        );
    } else {
        assert_eq!(
            out.stats.inverse_pair_triples, 0,
            "an inverse pair would force the target into the base"
        );
        assert_eq!(out.stats.identity_prefix_triples, 0);
    }
}
fn refusal_controls() -> usize {
    let good = Input {
        case_id: "refusal-control".into(),
        p: 97,
        a: 2,
        b: 3,
        d: 2,
        target: Some([0, 10]),
        restrict_liftable: true,
        subgroup: None,
    };
    let mut inputs = Vec::new();
    let mut q = good.clone();
    q.p = 101;
    inputs.push(("unsupported_prime", q));
    let mut q = good.clone();
    q.p = 1;
    inputs.push(("invalid_prime", q));
    let mut q = good.clone();
    q.a = 97;
    inputs.push(("noncanonical_a", q));
    let mut q = good.clone();
    q.b = 97;
    inputs.push(("noncanonical_b", q));
    let mut q = good.clone();
    q.a = 0;
    q.b = 0;
    inputs.push(("singular_curve", q));
    let mut q = good.clone();
    q.d = 0;
    inputs.push(("zero_degree", q));
    let mut q = good.clone();
    q.d = 5;
    inputs.push(("nondivisor_degree", q));
    let mut q = good.clone();
    q.d = 256;
    inputs.push(("overdegree", q));
    let mut q = good.clone();
    q.target = Some([97, 10]);
    inputs.push(("noncanonical_target_x", q));
    let mut q = good.clone();
    q.target = Some([0, 107]);
    inputs.push(("noncanonical_target_y", q));
    let mut q = good.clone();
    q.target = Some([0, 0]);
    inputs.push(("off_curve_target", q));
    let mut q = good.clone();
    q.subgroup = Some(SubgroupSpec {
        generator: [3, 6],
        order: 10,
    });
    inputs.push(("subgroup_nonminimal_order", q));
    let mut q = good.clone();
    q.subgroup = Some(SubgroupSpec {
        generator: [3, 6],
        order: 4,
    });
    inputs.push(("subgroup_wrong_order", q));
    let mut q = good.clone();
    q.subgroup = Some(SubgroupSpec {
        generator: [3, 6],
        order: 1,
    });
    inputs.push(("subgroup_unit_order", q));
    let mut q = good.clone();
    q.subgroup = Some(SubgroupSpec {
        generator: [3, 6],
        order: 513,
    });
    inputs.push(("subgroup_cap", q));
    let mut q = good;
    q.subgroup = Some(SubgroupSpec {
        generator: [0, 0],
        order: 5,
    });
    inputs.push(("subgroup_off_curve_generator", q));
    for (id, input) in &inputs {
        let error =
            extractor::extract(input).expect_err("invalid input must be refused before extraction");
        println!("{}", json!({"kind":"refusal","id":id,"error":error}));
    }
    inputs.len()
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).is_some_and(|s| s == "--check") {
        assert!(
            args.len() == 3 || args.len() == 4,
            "usage: --check DIR [RAW_JSONL]"
        );
        println!(
            "{}",
            checker::check_directory(Path::new(&args[2]), args.get(3).map(Path::new))
                .expect("independent persisted reader")
        );
        return;
    }
    if args.get(1).is_some_and(|s| s == "--check-controls") {
        assert_eq!(args.len(), 4, "usage: --check-controls DIR RAW_JSONL");
        println!(
            "{}",
            checker::controls(Path::new(&args[2]), Path::new(&args[3]))
                .expect("independent mutation controls")
        );
        return;
    }
    assert_eq!(
        args.len(),
        3,
        "usage: qualifier NEW_OUTPUT_DIRECTORY PROTOCOL_SHA256"
    );
    assert_eq!(
        args[2],
        digest(PROTOCOL),
        "supplied protocol differs from compiled protocol"
    );
    let out = Path::new(&args[1]);
    assert!(!out.exists(), "output directory must be new");
    fs::create_dir_all(out).unwrap();
    println!(
        "{}",
        json!({"kind":"scope","k":["pairs","grid","identities","triples","prefixO","masked_noncc"],"protocol_sha256":args[2],"mode":"finite_native_correctness","timing_claim":false,"cases":132,"persisted":42,"point_encoding":"[x,y]; identity null; base sorted lexicographically; ordered triples use base indices","n":["base","degreeF","degreeG","algebraic_support","ordered_triples","affine_triples","identity_prefix_triples","permutation_groups","integer_row_groups","inverse_pair_triples","masked_noncancelling_triples"],"c":["selector_pow_calls","square_root_calls","subgroup_additions","right_polynomial_products","tensor_ring_multiplications","scalar_norm_factors","positive_block_root_tests","positive_fibre_root_tests","left_gcd_queries","right_gcd_queries","left_lift_pair_attempts","prefix_additions","residual_subtractions","base_membership_queries","replay_additions"],"h":["exact certificate SHA256","exact input SHA256"],"b":["certificate bytes","input bytes"],"cost_scope":"call counters and output sizes; field-operation totals, peak memory, external comparisons and complete ECDLP costs unmeasured","paths":["{id}.certificate.json","{id}.input.json"]})
    );
    let refusals = refusal_controls();
    let mut totals = [0usize; 12];
    for (input, persist) in fixtures() {
        let result = extractor::extract(&input).expect("fixed valid extraction case");
        let checks = independent::compare(&input, &result)
            .expect("independent full signed output and certificate comparison");
        mask_regression(&input, &result);
        cancellation_regression(&input, &result);
        let bytes = serde_json::to_vec(&result).unwrap();
        let input_bytes = serde_json::to_vec(&input).unwrap();
        assert!(
            bytes.len() <= CASE_BYTE_CAP,
            "frozen in-memory serialized-case cap exceeded"
        );
        assert!(
            input_bytes.len() <= STORED_BYTE_CAP,
            "input sidecar cap exceeded"
        );
        let h = [digest(&bytes), digest(&input_bytes)];
        let b = [bytes.len(), input_bytes.len()];
        if persist {
            save(out, &input.case_id, "certificate", &bytes);
            save(out, &input.case_id, "input", &input_bytes);
        }
        let s = &result.stats;
        let n = [
            result.base.len(),
            result.certificate.f.len() - 1,
            result.certificate.g.len() - 1,
            result.certificate.support_count,
            result.triples.len(),
            s.affine_triples,
            s.identity_prefix_triples,
            result.permutations.len(),
            result.rows.len(),
            s.inverse_pair_triples,
            s.masked_noncancelling_triples,
        ];
        let counters = [
            s.selector_pow_calls,
            s.square_root_calls,
            s.subgroup_additions,
            s.right_polynomial_products,
            s.tensor_ring_multiplications,
            s.scalar_norm_factors,
            s.positive_block_root_tests,
            s.positive_fibre_root_tests,
            s.left_gcd_queries,
            s.right_gcd_queries,
            s.left_lift_pair_attempts,
            s.prefix_additions,
            s.residual_subtractions,
            s.base_membership_queries,
            s.replay_additions,
        ];
        println!(
            "{}",
            json!({"kind":"case","id":input.case_id,"persist":persist,"h":h,"b":b,"n":n,"c":counters,"k":[checks["pairs"],checks["grid"],checks["identities"],checks["triples"],checks["prefixO"],checks["masked_noncc"]]})
        );
        totals[0] += 1;
        totals[1] += usize::from(persist);
        totals[2] += result.certificate.support_count;
        totals[3] += result.triples.len();
        totals[4] += s.affine_triples;
        totals[5] += s.identity_prefix_triples;
        totals[6] += result.permutations.len();
        totals[7] += result.rows.len();
        totals[8] += usize::from(result.triples.is_empty());
        totals[9] += s.inverse_pair_triples;
        totals[10] += s.masked_noncancelling_triples;
        totals[11] += s.support_cells_without_signed_output;
    }
    assert_eq!(totals[0], 132);
    assert_eq!(totals[1], 42);
    assert_eq!(refusals, 16);
    println!(
        "{}",
        json!({"kind":"summary","passed":true,"refused_inputs":refusals,"totals":{"cases":totals[0],"persisted":totals[1],"algebraic_support":totals[2],"ordered_triples":totals[3],"affine_triples":totals[4],"identity_prefix_triples":totals[5],"permutation_groups":totals[6],"integer_row_groups":totals[7],"empty_signed_outputs":totals[8],"inverse_pair_triples":totals[9],"masked_noncancelling_triples":totals[10],"support_cells_without_signed_output":totals[11]},"standing":"unsigned finite development observations"})
    );
}
