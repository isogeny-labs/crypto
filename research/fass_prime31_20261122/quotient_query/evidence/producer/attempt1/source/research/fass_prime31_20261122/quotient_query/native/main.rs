//! Frozen bounded development qualifier. No timing or performance claim.
//! All numerical work is Rust; field/poly primitives are repository sources.
#![recursion_limit = "512"]

mod checker;
mod independent;
mod kernel;
mod norm;
#[allow(dead_code)]
#[path = "../../target_quotient/native/oracle.rs"]
mod prior_oracle;
#[allow(dead_code)]
#[path = "../../../../src/cryptanalysis/semaev_prime_poly.rs"]
mod semaev_prime_poly;
#[allow(dead_code)]
#[path = "../../../../src/hash/sha256.rs"]
mod source_sha256;

use kernel::{Certificate, Poly, Tensor};
use semaev_prime_poly::{poly_gcd, poly_powmod, poly_rem, trim, Curve, Fp};
use serde::Serialize;
use serde_json::json;
use std::{
    collections::BTreeSet,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

#[derive(Default, Serialize)]
struct Totals {
    cases: usize,
    curve_cases: usize,
    synthetic_cases: usize,
    tensor_norm_checks: usize,
    persisted_certificates: usize,
    algebraic_support: usize,
    empty_queries: usize,
    grid_cells: usize,
    identity_checks: usize,
    rejected_mutations: usize,
    refused_inputs: usize,
    right_root_checks: usize,
}

fn eval(f: Fp, p: &[u64], x: u64) -> u64 {
    p.iter().rev().fold(0, |s, &c| f.add(f.mul(s, x), c))
}

fn product_roots(f: Fp, roots: &[u64]) -> Poly {
    roots
        .iter()
        .fold(vec![1], |a, &r| kernel::multiply(f, &a, &[f.neg(r), 1]))
}

fn square_root(f: Fp, a: u64) -> Option<u64> {
    if f.p == 97 {
        return (0..97).find(|&y| f.mul(y, y) == a);
    }
    assert_eq!(f.p, 2147483647);
    let y = f.pow(a, (f.p + 1) / 4);
    (f.mul(y, y) == a).then_some(y.min(f.neg(y)))
}

fn point_y(c: Curve, x: u64) -> Option<u64> {
    let f = c.f;
    square_root(f, f.add(f.add(f.mul(f.mul(x, x), x), f.mul(c.a, x)), c.b))
}

fn curve_input(c: Curve, target_x: u64) -> Result<u64, &'static str> {
    if !prior_oracle::discriminant_nonzero(c) {
        return Err("singular_curve");
    }
    if target_x >= c.f.p {
        return Err("noncanonical_target");
    }
    point_y(c, target_x).ok_or("nonrational_target")
}

fn quadratic(c: Curve, z: u64, target_x: u64) -> Poly {
    let f = c.f;
    let q0 = c.s3(0, z, target_x);
    let q1 = c.s3(1, z, target_x);
    let qm1 = c.s3(f.p - 1, z, target_x);
    let inv2 = f.inv(2);
    trim(vec![
        q0,
        f.mul(f.sub(q1, qm1), inv2),
        f.mul(f.sub(f.add(q1, qm1), f.mul(2, q0)), inv2),
    ])
}

fn quadratic_roots(f: Fp, q: &[u64]) -> Vec<u64> {
    assert_ne!(q, [0], "all-field factor requires a different input chart");
    match q.len() {
        1 => Vec::new(),
        2 => vec![f.mul(f.neg(q[0]), f.inv(q[1]))],
        3 => {
            let delta = f.sub(f.mul(q[1], q[1]), f.mul(4, f.mul(q[0], q[2])));
            let Some(s) = square_root(f, delta) else {
                return Vec::new();
            };
            let z = f.inv(f.mul(2, q[2]));
            let mut roots = vec![
                f.mul(f.add(f.neg(q[1]), s), z),
                f.mul(f.sub(f.neg(q[1]), s), z),
            ];
            roots.sort_unstable();
            roots.dedup();
            roots
        }
        _ => panic!("quadratic degree"),
    }
}

fn right_message(
    c: Curve,
    selector: &[u64],
    target_x: u64,
    totals: &mut Totals,
) -> (Poly, Vec<u64>) {
    let f = c.f;
    let mut raw = vec![1];
    let mut root_set = BTreeSet::new();
    for &z in selector {
        let q = quadratic(c, z, target_x);
        raw = kernel::multiply(f, &raw, &q);
        root_set.extend(quadratic_roots(f, &q));
    }
    let g = if raw.len() == 1 {
        vec![1]
    } else {
        let frobenius = poly_powmod(f, &[0, 1], f.p, &raw);
        let t = poly_rem(f, &[0, 1], &raw);
        kernel::monic(f, &poly_gcd(f, &raw, &kernel::sub(f, &frobenius, &t)))
    };
    let roots: Vec<_> = root_set.into_iter().collect();
    assert_eq!(g, product_roots(f, &roots));
    totals.right_root_checks += 1;
    if f.p == 97 {
        for t in 0..97 {
            assert_eq!(eval(f, &raw, t) == 0, roots.binary_search(&t).is_ok());
        }
    }
    (g, roots)
}

fn write_artifact(out: &Path, id: &str, kind: &str, bytes: &[u8]) -> String {
    let filename = format!("{id}.{kind}.json");
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.join(&filename))
        .expect("certificate path must be new");
    file.write_all(bytes).unwrap();
    assert!(bytes.len()<=65536,"prespecified certificate exceeds artifact-store threshold; retained locally, publication requires store");
    filename
}

struct Case<'a> {
    id: &'a str,
    roots_x: &'a [u64],
    roots_t: &'a [u64],
    persist: bool,
    norm_stats: Option<norm::NormStats>,
    input_metadata: serde_json::Value,
    cancellation: Option<serde_json::Value>,
}

fn emit_case(out: &Path, c: Certificate, case: Case<'_>, totals: &mut Totals) {
    let v = independent::verify(&c).expect("independent polynomial identity verification");
    let grid = independent::verify_grid(&c, case.roots_x, case.roots_t)
        .expect("independent grid verification");
    let mutation_checks = if case.persist {
        independent::verify_rejects_mutations(&c).expect("certificate tamper controls")
    } else {
        0
    };
    let bytes = serde_json::to_vec(&c).unwrap();
    let digest = source_sha256::sha256(&bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>();
    if case.persist {
        write_artifact(out, case.id, "certificate", &bytes);
        write_artifact(
            out,
            case.id,
            "input",
            &serde_json::to_vec(&case.input_metadata).unwrap(),
        );
    }
    let positive_blocks = c.leaves.iter().filter(|l| l.gcd.len() > 1).count();
    let mut degree_mass = std::collections::BTreeMap::<usize, usize>::new();
    let mut degree1_t = Vec::new();
    for leaf in &c.leaves {
        *degree_mass.entry(leaf.gcd.len() - 1).or_default() += leaf.modulus.len() - 1;
        if leaf.gcd.len() == 2 && leaf.modulus.len() == 2 {
            degree1_t.push(Fp { p: c.p }.neg(leaf.modulus[0]));
        }
    }
    degree1_t.sort_unstable();
    let norm_metadata = case
        .norm_stats
        .as_ref()
        .map(|n| json!({"mul":n.ring_multiplications,"slots":n.largest_tensor_slots}));
    let mut record = json!({"kind":"case","id":case.id,
        "B":c.f.len()-1,"e":c.g.len()-1,"blocks":c.leaves.len(),"pos":positive_blocks,
        "support":c.support_count,"empty":c.verdict_empty,"splits":c.stats.splits,"euclid":c.stats.euclidean_steps,
        "slots":c.stats.output_coefficients,"sha256":digest,"bytes":bytes.len(),
        "persist":case.persist,"norm":norm_metadata,"cancel":case.cancellation,
        "degree_mass":degree_mass.into_iter().collect::<Vec<_>>(),
        "checks":[v.identity_checks,grid.cells,mutation_checks]});
    if !degree1_t.is_empty() {
        record["degree1_t"] = json!(degree1_t);
    }
    println!("{}", record);
    totals.cases += 1;
    totals.persisted_certificates += usize::from(case.persist);
    totals.algebraic_support += c.support_count;
    totals.empty_queries += usize::from(c.verdict_empty);
    totals.grid_cells += grid.cells;
    totals.identity_checks += v.identity_checks;
    totals.rejected_mutations += mutation_checks;
    totals.tensor_norm_checks += usize::from(case.norm_stats.is_some());
}

fn curve_fixture(
    out: &Path,
    name: &str,
    c: Curve,
    degrees: &[usize],
    targets: &[u64],
    totals: &mut Totals,
) {
    println!(
        "{}",
        json!({"kind":"fixture","name":name,"p":c.f.p,"A":c.a,"B":c.b,"degrees":degrees,"targets":targets,"signed_relation_claim":false})
    );
    for &d in degrees {
        let mut selector = prior_oracle::selector_roots(c.f, d);
        selector.sort_unstable();
        let raw_h = prior_oracle::partial_norm(c, d, &selector);
        for &target_x in targets {
            let target_y = curve_input(c, target_x).expect("fixed target on nonsingular curve");
            assert_eq!(
                c.f.mul(target_y, target_y),
                c.f.add(
                    c.f.add(
                        c.f.mul(c.f.mul(target_x, target_x), target_x),
                        c.f.mul(c.a, target_x)
                    ),
                    c.b
                )
            );
            let (g, roots_t) = right_message(c, &selector, target_x, totals);
            for liftable in [false, true] {
                let roots_x: Vec<_> = selector
                    .iter()
                    .copied()
                    .filter(|&x| !liftable || point_y(c, x).is_some())
                    .collect();
                let fpoly = product_roots(c.f, &roots_x);
                let h = kernel::reduce_tensor(c.f, &raw_h, &fpoly, &g);
                let use_tensor_norm =
                    c.f.p == 97 || d <= 21 || (d == 63 && target_x == 0 && liftable);
                let norm_stats = if use_tensor_norm {
                    let (candidate, stats) = norm::tensor_norm(c, d, &fpoly, &g);
                    assert_eq!(
                        candidate, h,
                        "tensor norm differs from original direct product"
                    );
                    Some(stats)
                } else {
                    None
                };
                let cert = kernel::solve(c.f, &fpoly, &g, &h).expect("dynamic split query");
                independent::verify_s3_input(
                    &cert, c.a, c.b, d, target_x, &selector, &roots_x, &roots_t, liftable,
                )
                .expect("independent original S3 binding");
                let left = if liftable { "liftable" } else { "all" };
                let id = format!("{name}-d{d}-x{target_x}-{left}");
                let persist = (c.f.p == 97 && d == 2) || (c.f.p == 2147483647 && d == 7);
                let cancellation = if selector.binary_search(&target_x).is_ok() {
                    let (mut zeros, mut target_row, mut s3_pair, mut union) =
                        (0usize, 0usize, 0usize, 0usize);
                    for &x in &roots_x {
                        for &t in &roots_t {
                            if prior_oracle::evaluate(c.f, &h, x, t) == 0 {
                                zeros += 1;
                                let row = x == target_x;
                                let pair = c.s3(x, target_x, t) == 0;
                                target_row += usize::from(row);
                                s3_pair += usize::from(pair);
                                union += usize::from(row || pair);
                            }
                        }
                    }
                    Some(
                        json!({"zeros":zeros,"row":target_row,"pair":s3_pair,"union":union,"other":zeros-union}),
                    )
                } else {
                    None
                };
                let input_metadata = json!({"kind":"s3","case_id":id,"p":c.f.p,"a":c.a,"b":c.b,"d":d,"target_x":target_x,"target_y":target_y,"restrict_liftable":liftable,"selector_roots":selector,"x_roots":roots_x,"t_roots":roots_t});
                emit_case(
                    out,
                    cert,
                    Case {
                        id: &id,
                        roots_x: &roots_x,
                        roots_t: &roots_t,
                        persist,
                        norm_stats,
                        input_metadata,
                        cancellation,
                    },
                    totals,
                );
                totals.curve_cases += 1;
            }
        }
    }
}

fn synthetics(out: &Path, totals: &mut Totals) {
    let f = Fp::new(97);
    let roots_x = [1, 2, 3];
    let roots_t = [0, 1, 2];
    let fp = product_roots(f, &roots_x);
    let gp = product_roots(f, &roots_t);
    let fixtures: [(&str, Tensor); 5] = [
        ("unit", vec![vec![1]]),
        ("zero", vec![vec![0]]),
        ("nonunit_lead", vec![vec![96], vec![0, 1]]),
        ("degree_drop", vec![vec![2, 94, 1], vec![0, 96, 1]]),
        ("diagonal", vec![vec![0, 96], vec![1]]),
    ];
    for (name, h) in fixtures {
        let id = format!("synthetic-{name}");
        let cert = kernel::solve(f, &fp, &gp, &h).unwrap();
        let input_metadata =
            json!({"kind":"synthetic","case_id":id,"x_roots":roots_x,"t_roots":roots_t});
        emit_case(
            out,
            cert,
            Case {
                id: &id,
                roots_x: &roots_x,
                roots_t: &roots_t,
                persist: true,
                norm_stats: None,
                input_metadata,
                cancellation: None,
            },
            totals,
        );
        totals.synthetic_cases += 1;
    }
    for empty_left in [true, false] {
        let (ff, gg, xs, ts) = if empty_left {
            (vec![1], gp.clone(), Vec::new(), roots_t.to_vec())
        } else {
            (fp.clone(), vec![1], roots_x.to_vec(), Vec::new())
        };
        let id = if empty_left {
            "synthetic-empty-F"
        } else {
            "synthetic-empty-G"
        };
        let cert = kernel::solve(f, &ff, &gg, &vec![vec![0]]).unwrap();
        let input_metadata = json!({"kind":"synthetic","case_id":id,"x_roots":xs,"t_roots":ts});
        emit_case(
            out,
            cert,
            Case {
                id,
                roots_x: &xs,
                roots_t: &ts,
                persist: true,
                norm_stats: None,
                input_metadata,
                cancellation: None,
            },
            totals,
        );
        totals.synthetic_cases += 1;
    }
}

fn refusal_controls(totals: &mut Totals) {
    let f = Fp::new(97);
    let good = product_roots(f, &[1, 2]);
    let repeated = vec![1, 95, 1];
    let nonsplit = vec![92, 0, 1]; // 5 is a nonsquare modulo 97.
    let bad_moduli = [
        ("F_nonmonic", vec![4, 91, 2], good.clone()),
        ("F_noncanonical", vec![97, 1], good.clone()),
        ("F_nonsquarefree", repeated.clone(), good.clone()),
        ("F_nonsplit", nonsplit.clone(), good.clone()),
        ("G_nonmonic", good.clone(), vec![4, 91, 2]),
        ("G_nonsquarefree", good.clone(), repeated),
        ("G_nonsplit", good.clone(), nonsplit),
        ("F_trailing_zero", vec![2, 94, 1, 0], good.clone()),
    ];
    for (name, ff, gg) in bad_moduli {
        let error =
            kernel::solve(f, &ff, &gg, &vec![vec![0]]).expect_err("invalid input must be refused");
        println!("{}", json!({"kind":"refusal","name":name,"error":error}));
        totals.refused_inputs += 1;
    }
    for (name, h) in [
        ("H_noncanonical", vec![vec![97]]),
        ("H_unreduced_x", vec![vec![0], vec![0], vec![1]]),
        ("H_unreduced_t", vec![vec![0, 0, 1]]),
    ] {
        let error = kernel::solve(f, &good, &good, &h).expect_err("invalid tensor must be refused");
        println!("{}", json!({"kind":"refusal","name":name,"error":error}));
        totals.refused_inputs += 1;
    }
    let e = kernel::solve(Fp { p: 99 }, &good, &good, &vec![vec![0]])
        .expect_err("unsupported composite field");
    println!(
        "{}",
        json!({"kind":"refusal","name":"composite_field","error":e})
    );
    totals.refused_inputs += 1;
    let e = curve_input(Curve::new(97, 0, 0), 0).expect_err("singular curve");
    println!(
        "{}",
        json!({"kind":"refusal","name":"singular_curve","error":e})
    );
    totals.refused_inputs += 1;
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--check") {
        assert!(
            args.len() == 3 || args.len() == 4,
            "usage: qualifier --check CERTIFICATE_DIRECTORY [RAW_JSONL]"
        );
        let receipt = checker::check_directory(Path::new(&args[2]), args.get(3).map(Path::new))
            .expect("independent certificate directory check");
        println!("{}", receipt);
        return;
    }
    assert_eq!(
        args.len(),
        3,
        "usage: qualifier NEW_OUTPUT_DIRECTORY PROTOCOL_SHA256"
    );
    let out = Path::new(&args[1]);
    assert!(!out.exists(), "output directory must be new");
    assert_eq!(args[2].len(), 64, "protocol hash must be provided");
    fs::create_dir_all(out).unwrap();
    println!(
        "{}",
        json!({"kind":"scope","protocol_sha256":args[2],"mode":"bounded_native_development_correctness","signed_relation_claim":false,"timing_claim":false,"coefficient_order":"outer x, inner t, low first",
        "case_keys":{"B":"degree F","e":"degree G","pos":"positive-degree gcd blocks","slots":"certificate scalar slots","sha256":"canonical serialized certificate","bytes":"serialized certificate bytes","persist":"full_certificate_persisted","checks":["identity checks","grid cells","tamper rejections"],"norm":{"mul":"tensor ring multiplications, not total cost","slots":"largest output tensor scalar slots"},"cancel":{"zeros":"zero grid cells","row":"x equals target X","pair":"S3(x,X,t) equals zero","union":"row or pair","other":"zeros outside union"},"degree_mass":"[gcd_x degree, sum modulus degrees]","degree1_t":"roots of linear q blocks with gcd_x degree one"},
        "persisted_paths":["{id}.certificate.json","{id}.input.json"],"other_certificates":"independently checked inline; reconstruct from frozen source"})
    );
    let mut totals = Totals::default();
    synthetics(out, &mut totals);
    refusal_controls(&mut totals);
    let c97 = Curve::new(97, 2, 3);
    let mut targets97: Vec<_> = (0..97)
        .filter(|&x| point_y(c97, x).is_some_and(|y| y != 0))
        .take(3)
        .collect();
    targets97.push(30);
    curve_fixture(
        out,
        "f97",
        c97,
        &[1, 2, 3, 4, 6, 8, 12, 16],
        &targets97,
        &mut totals,
    );
    curve_fixture(
        out,
        "f97-rank-one",
        Curve::new(97, 96, 0),
        &[2],
        &[0, 1, 2],
        &mut totals,
    );
    curve_fixture(
        out,
        "prime31",
        Curve::new(2147483647, 847514846, 2087444724),
        &[7, 21, 63],
        &[0, 1, 2],
        &mut totals,
    );
    assert_eq!(totals.curve_cases, 88);
    assert_eq!(totals.synthetic_cases, 7);
    assert_eq!(totals.cases, 95);
    assert_eq!(totals.persisted_certificates, 27);
    assert_eq!(totals.tensor_norm_checks, 83);
    println!(
        "{}",
        json!({"kind":"summary","totals":totals,"passed":true,"scope":"finite_source_bound_development_checks"})
    );
}
