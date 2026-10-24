//! Explicit public ECDLP input over a checked cubic extension of F_p.

use clap::Args;
use crypto_lib::cryptanalysis::gaudry_cubic::{
    run_gaudry_opts, run_rho3, Curve3, Fp3, GaudryOptions, Instance3, Pt3, SubspaceBase, E3,
};
use rand::{rngs::StdRng, SeedableRng};
use serde::Deserialize;
use serde_json::{json, Value};
use std::path::PathBuf;

#[derive(Args)]
pub struct CubicArgs {
    /// JSON file or store reference with p, exactly one of c or modulus, a, b, n, gx, gy, qx, qy.
    #[arg(long)]
    input: PathBuf,
    /// Checked cubic-field isogeny graph JSON or store:NAME/sha256:DIGEST reference.
    #[arg(long, requires = "route_id")]
    route_file: Option<PathBuf>,
    /// Root-origin route identifier in --route-file.
    #[arg(long, requires = "route_file")]
    route_id: Option<String>,
    #[arg(long, default_value_t = 1)]
    seed: u64,
    #[arg(long, default_value_t = 100_000)]
    max_residuals: u64,
    /// Skip the same-target Pollard-rho reference.
    #[arg(long)]
    skip_rho: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    p: u64,
    c: Option<u64>,
    modulus: Option<[u64; 3]>,
    a: [u64; 3],
    b: [u64; 3],
    n: u64,
    gx: [u64; 3],
    gy: [u64; 3],
    qx: [u64; 3],
    qy: [u64; 3],
}

/// Count every affine point using the two roots of each nonzero square.
/// The CLI's p <= 101 bound keeps this at at most 1,030,301 field elements.
fn count_curve_order3(field: &Fp3, a: &E3, b: &E3) -> u64 {
    let p = field.p;
    let size = (p * p * p) as usize;
    let index = |value: &E3| -> usize { ((value.0[2] * p + value.0[1]) * p + value.0[0]) as usize };
    let mut roots = vec![0u8; size];
    for c2 in 0..p {
        for c1 in 0..p {
            for c0 in 0..p {
                let y = E3([c0, c1, c2]);
                roots[index(&field.sq(&y))] += 1;
            }
        }
    }
    let mut order = 1u64;
    for c2 in 0..p {
        for c1 in 0..p {
            for c0 in 0..p {
                let x = E3([c0, c1, c2]);
                let rhs = field.add(
                    &field.add(&field.mul(&field.sq(&x), &x), &field.mul(a, &x)),
                    b,
                );
                order += u64::from(roots[index(&rhs)]);
            }
        }
    }
    order
}

pub fn solve(args: &CubicArgs) -> Result<Value, String> {
    if args.route_file.is_some() != args.route_id.is_some() {
        return Err("--route-file and --route-id must be supplied together".into());
    }
    let resolved = crypto_lib::store::resolve_path(&args.input)?;
    if std::fs::metadata(&resolved)
        .map_err(|e| format!("{}: {e}", args.input.display()))?
        .len()
        > 64 * 1024
    {
        return Err("cubic target input exceeds 64 KiB".into());
    }
    let bytes = std::fs::read(&resolved)
        .map_err(|e| format!("cannot read {}: {e}", args.input.display()))?;
    if bytes.len() > 64 * 1024 {
        return Err("cubic target input exceeds 64 KiB".into());
    }
    let input_sha256 = hex::encode(crypto_lib::hash::sha256::sha256(&bytes));
    let input: Input =
        serde_json::from_slice(&bytes).map_err(|e| format!("invalid cubic input JSON: {e}"))?;
    if !(7..=101).contains(&input.p) || !super::prime_external::prime(input.p) {
        return Err("p must be prime and in 7..=101".into());
    }
    if !(3..=2_000_000).contains(&input.n) || !super::prime_external::prime(input.n) {
        return Err("n must be prime and in 3..=2000000".into());
    }
    if [
        &input.a, &input.b, &input.gx, &input.gy, &input.qx, &input.qy,
    ]
    .into_iter()
    .flatten()
    .any(|&v| v >= input.p)
    {
        return Err("all cubic-field coefficients must be less than p".into());
    }
    if !(1..=10_000_000).contains(&args.max_residuals) {
        return Err("max-residuals must be in 1..=10000000".into());
    }
    let field = match (input.c, input.modulus) {
        (Some(c), None) => {
            if c >= input.p {
                return Err("c must use its canonical representative below p".into());
            }
            Fp3::with_cube_nonresidue(input.p, c)?
        }
        (None, Some(modulus)) => Fp3::with_irreducible_cubic(input.p, modulus)?,
        _ => return Err("supply exactly one of c or modulus for the cubic field".into()),
    };
    if input.modulus.is_some() && args.route_file.is_some() {
        return Err("cubic isogeny routes currently require the t^3-c field presentation".into());
    }
    let a = E3(input.a);
    let b = E3(input.b);
    let a3 = field.mul(&field.sq(&a), &a);
    let b2 = field.sq(&b);
    let discriminant = field.add(&field.scale(&a3, 4), &field.scale(&b2, 27));
    if discriminant == Fp3::ZERO {
        return Err("singular curve: discriminant is zero".into());
    }
    let g = Pt3::affine(E3(input.gx), E3(input.gy));
    let q = Pt3::affine(E3(input.qx), E3(input.qy));
    let curve = Curve3::new(field, a, b, input.n, g);
    if !curve.is_on_curve(&g) || !curve.is_on_curve(&q) {
        return Err("G and Q must both lie on the specified curve".into());
    }
    if !curve.mul(&g, input.n).inf {
        return Err("G does not have the specified prime order".into());
    }
    if !curve.mul(&q, input.n).inf {
        return Err("Q is outside the subgroup generated by G".into());
    }
    let curve_order = count_curve_order3(&curve.field, &a, &b);
    if !curve_order.is_multiple_of(input.n) {
        return Err(format!(
            "supplied n = {} does not divide counted #E(F_p^3) = {curve_order}",
            input.n
        ));
    }
    let cofactor = curve_order / input.n;
    if cofactor.is_multiple_of(input.n) {
        return Err(format!(
            "Gaudry cubic cofactor must be invertible modulo n; counted cofactor = {cofactor}"
        ));
    }
    // `d` is used only for generated known-answer reports. External input is
    // verified by replay of the recovered scalar, without a planted answer.
    let source = Instance3 { curve, q, d: 0 };
    let (inst, route) = if let (Some(path), Some(id)) = (&args.route_file, &args.route_id) {
        let transport =
            super::cubic_route::transport(&source.curve, q, path, id, Some(&input_sha256))?;
        let destination_order = count_curve_order3(
            &transport.curve.field,
            &transport.curve.a,
            &transport.curve.b,
        );
        if destination_order != curve_order {
            return Err("cubic route destination has a different counted curve order".into());
        }
        (
            Instance3 {
                curve: transport.curve,
                q: transport.target,
                d: 0,
            },
            Some(transport.receipt),
        )
    } else {
        (
            Instance3 {
                curve: source.curve.clone(),
                q,
                d: 0,
            },
            None,
        )
    };
    let mut rng = StdRng::seed_from_u64(args.seed);
    let base = SubspaceBase::build(&inst, &mut rng);
    if base.is_empty() {
        return Err("the subspace factor base is empty".into());
    }
    if cofactor > 1
        && base
            .points
            .iter()
            .all(|point| inst.curve.mul(point, cofactor).inf)
    {
        return Err(
            "the subspace factor base projects entirely to the identity in the target subgroup"
                .into(),
        );
    }
    let report = run_gaudry_opts(
        &inst,
        &base,
        args.seed,
        &GaudryOptions {
            max_residuals: args.max_residuals,
            cofactor,
            ..GaudryOptions::default()
        },
    );
    let verified = report.recovered_scalar.is_some_and(|d| {
        inst.curve.mul(&inst.curve.g, d) == inst.q && source.curve.mul(&g, d) == q
    });
    let mut ic = serde_json::to_value(&report).map_err(|e| e.to_string())?;
    ic.as_object_mut().unwrap().remove("correct");
    ic["verified"] = json!(verified);
    let rho = if args.skip_rho {
        None
    } else {
        let rr = run_rho3(&source, args.seed ^ 0x3141_5926);
        let rho_verified = rr
            .recovered_scalar
            .is_some_and(|d| source.curve.mul(&g, d) == q);
        let mut row = serde_json::to_value(&rr).map_err(|e| e.to_string())?;
        row.as_object_mut().unwrap().remove("correct");
        row["verified"] = json!(rho_verified);
        Some(row)
    };
    let mut input_report = json!({
        "p": input.p, "a": input.a, "b": input.b,
        "n": input.n, "curve_order": curve_order, "cofactor": cofactor,
        "generator": [input.gx, input.gy],
        "target": [input.qx, input.qy],
    });
    if let Some(c) = input.c {
        input_report["c"] = json!(c);
    }
    if let Some(modulus) = input.modulus {
        input_report["modulus"] = json!(modulus);
    }
    let mut result = json!({
        "schema_version": 1,
        "operation": "solve-cubic",
        "status": if verified { "complete" } else { "bounded_unknown" },
        "verified": verified,
        "input_artifact": {
            "reference": args.input.display().to_string(),
            "sha256": input_sha256,
            "bytes": bytes.len(),
        },
        "input": input_report,
        "config": {"seed": args.seed, "max_residuals": args.max_residuals, "cofactor": cofactor},
        "ic": ic,
        "rho": rho,
    });
    if let Some(receipt) = route {
        result["route"] = receipt;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_prime_order_and_cofactor_curves() {
        let field = Fp3::with_cube_nonresidue(7, 2).unwrap();
        assert_eq!(
            count_curve_order3(&field, &E3([2, 1, 5]), &E3([5, 6, 5])),
            347
        );
        assert_eq!(
            count_curve_order3(&field, &E3([1, 0, 0]), &E3([0, 0, 0])),
            344
        );
    }

    #[test]
    fn non_binomial_cubic_target_recovers_with_matched_rho() {
        let report = solve(&CubicArgs {
            input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("docs/ic/fixtures/cubic_f11_public_target.json"),
            route_file: None,
            route_id: None,
            seed: 1,
            max_residuals: 100_000,
            skip_rho: false,
        })
        .unwrap();
        assert_eq!(report["input"]["modulus"], json!([1, 0, 4]));
        assert_eq!(report["input"]["curve_order"], 1289);
        assert_eq!(report["ic"]["recovered_scalar"], 5);
        assert_eq!(report["rho"]["recovered_scalar"], 5);
        assert_eq!(report["verified"], true);
        assert_eq!(report["rho"]["verified"], true);
    }

    #[test]
    fn cofactor_relations_recover_and_replay_external_target() {
        let fixture = |name: &str| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("docs/ic/fixtures")
                .join(name)
        };
        let report = solve(&CubicArgs {
            input: fixture("cubic_f7_cofactor12_public_target.json"),
            route_file: None,
            route_id: None,
            seed: 1,
            max_residuals: 1_000,
            skip_rho: true,
        })
        .unwrap();
        assert_eq!(report["input"]["curve_order"], 348);
        assert_eq!(report["input"]["cofactor"], 12);
        let input_bytes = std::fs::read(fixture("cubic_f7_cofactor12_public_target.json")).unwrap();
        assert_eq!(report["input_artifact"]["bytes"], input_bytes.len());
        assert_eq!(
            report["input_artifact"]["sha256"],
            hex::encode(crypto_lib::hash::sha256::sha256(&input_bytes))
        );
        assert_eq!(report["ic"]["recovered_scalar"], 5);
        assert_eq!(report["ic"]["verified"], true);
        assert!(report["ic"]["torsion_lift_fp_muls"].as_u64().unwrap() > 0);

        let rejected = solve(&CubicArgs {
            input: fixture("cubic_f7_cofactor8_input.json"),
            route_file: None,
            route_id: None,
            seed: 1,
            max_residuals: 100,
            skip_rho: true,
        })
        .unwrap_err();
        assert!(rejected.contains("factor base projects entirely to the identity"));
    }

    #[test]
    fn coprime_cofactor_above_64_recovers_and_replays_external_target() {
        let report = solve(&CubicArgs {
            input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("docs/ic/fixtures/cubic_f7_cofactor116_public_target.json"),
            route_file: None,
            route_id: None,
            seed: 1,
            max_residuals: 1_000,
            skip_rho: true,
        })
        .unwrap();
        assert_eq!(report["input"]["curve_order"], 348);
        assert_eq!(report["input"]["cofactor"], 116);
        assert_eq!(report["ic"]["recovered_scalar"], 2);
        assert_eq!(report["ic"]["relations_verified"], 4);
        assert_eq!(report["verified"], true);
    }

    #[test]
    fn cubic_isogeny_route_recovers_source_scalar() {
        let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/ic/fixtures");
        let report = solve(&CubicArgs {
            input: fixtures.join("cubic_f7_cofactor12_public_target.json"),
            route_file: Some(fixtures.join("cubic_f7_cofactor12_isogeny_route.json")),
            route_id: Some("root-to-velu3".into()),
            seed: 1,
            max_residuals: 1_000,
            skip_rho: true,
        })
        .unwrap();
        assert_eq!(report["route"]["nodes_verified"], 2);
        assert_eq!(report["route"]["edges_verified"], 1);
        assert_eq!(report["ic"]["recovered_scalar"], 5);
        assert_eq!(report["verified"], true);
    }
}
