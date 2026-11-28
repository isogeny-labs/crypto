//! `ic summation-roots`: structure-aware roots of point-origin Semaev
//! summation-polynomial specializations over small prime fields.

use clap::{Args, ValueEnum};
use crypto_lib::cryptanalysis::{
    prime_fast::{FastCurve, MAX_BITS},
    prime_summation_roots::{
        all_roots, coefficients_from_roots, find_target_root, lift_input_xs, one_root,
        CoefficientMethod, SummationRootWorkspace,
    },
    residual_walk::is_prime_u64,
};
use serde_json::{json, Value};

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Mode {
    /// Enumerate every finite root, preserving multiplicity.
    All,
    /// Compute one root selected by a replayable sign mask.
    One,
    /// Search supplied target x-coordinates with a split signed-sum table.
    Target,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Coefficients {
    Auto,
    Quadratic,
    ProductTree,
}

impl From<Coefficients> for CoefficientMethod {
    fn from(value: Coefficients) -> Self {
        match value {
            Coefficients::Auto => Self::Auto,
            Coefficients::Quadratic => Self::Quadratic,
            Coefficients::ProductTree => Self::ProductTree,
        }
    }
}

#[derive(Args, Clone, Debug)]
pub struct SummationRootsArgs {
    /// Root operation to perform.
    #[arg(long, value_enum, default_value_t = Mode::All)]
    pub mode: Mode,
    /// Prime field modulus. The production backend requires 3 < p < 2^62.
    #[arg(long)]
    pub p: u64,
    /// Short-Weierstrass coefficient a in y^2 = x^3 + ax + b.
    #[arg(long)]
    pub a: u64,
    /// Short-Weierstrass coefficient b in y^2 = x^3 + ax + b.
    #[arg(long)]
    pub b: u64,
    /// Input x-coordinates; repeat the option or separate values with commas.
    #[arg(long = "x", value_delimiter = ',', required = true, num_args = 1..)]
    pub xs: Vec<u64>,
    /// Target x-coordinates for --mode target.
    #[arg(long = "target-x", value_delimiter = ',', num_args = 1..)]
    pub target_xs: Vec<u64>,
    /// Sign mask for --mode one. Bit i selects +P_(i+1); a clear bit selects -P_(i+1).
    #[arg(long, default_value_t = 0)]
    pub sign_mask: u64,
    /// Emit coefficients of the monic finite-root polynomial in --mode all.
    #[arg(long)]
    pub coefficients: bool,
    /// Coefficient construction strategy.
    #[arg(long, value_enum, default_value_t = Coefficients::Auto)]
    pub coefficient_method: Coefficients,
    /// Hard bound on signed states allocated by the operation.
    #[arg(long, default_value_t = 1_048_576)]
    pub max_states: usize,
}

fn checked_curve(args: &SummationRootsArgs) -> Result<FastCurve, String> {
    if args.p <= 3 || args.p >= (1u64 << MAX_BITS) || !is_prime_u64(args.p) {
        return Err(format!(
            "--p must be a prime with 3 < p < 2^{MAX_BITS}; got {}",
            args.p
        ));
    }
    let curve = FastCurve::for_arithmetic("summation-roots", args.p, args.a, args.b);
    let f = &curve.f;
    let four = f.to_mont(4);
    let twenty_seven = f.to_mont(27);
    let discriminant = f.add(
        f.mul(four, f.mul(f.sqr(curve.a), curve.a)),
        f.mul(twenty_seven, f.sqr(curve.b)),
    );
    if discriminant == 0 {
        return Err("the curve is singular: 4a^3 + 27b^2 = 0 mod p".into());
    }
    if args.max_states == 0 {
        return Err("--max-states must be positive".into());
    }
    Ok(curve)
}

fn point_json(point: crypto_lib::cryptanalysis::prime_summation_roots::CanonicalPoint) -> Value {
    json!({"x": point.x, "y": point.y})
}

pub fn run(args: SummationRootsArgs) -> Result<Value, String> {
    if args.mode != Mode::Target && !args.target_xs.is_empty() {
        return Err("--target-x is valid only with --mode target".into());
    }
    if args.mode == Mode::Target && args.target_xs.is_empty() {
        return Err("--mode target requires at least one --target-x".into());
    }
    if args.mode != Mode::One && args.sign_mask != 0 {
        return Err("--sign-mask is valid only with --mode one".into());
    }
    if args.mode != Mode::All && args.coefficients {
        return Err("--coefficients is valid only with --mode all".into());
    }
    let curve = checked_curve(&args)?;
    if let Some(&x) = args.xs.iter().find(|&&x| x >= args.p) {
        return Err(format!(
            "input x-coordinate {x} is not canonical for p={}",
            args.p
        ));
    }
    if let Some(&x) = args.target_xs.iter().find(|&&x| x >= args.p) {
        return Err(format!(
            "target x-coordinate {x} is not canonical for p={}",
            args.p
        ));
    }

    let points = lift_input_xs(&curve, &args.xs).map_err(|error| error.to_string())?;
    let common = json!({
        "schema_version": 1,
        "operation": "summation-roots",
        "status": "complete",
        "curve": {
            "model": "short_weierstrass",
            "p": args.p,
            "a": args.a % args.p,
            "b": args.b % args.p,
        },
        "specialization": {
            "representation": "point-origin",
            "input_x": args.xs,
            "lift_convention": "prime_fast::FastCurve::lift_x",
            "first_point_sign": "positive",
        },
        "limits": {"max_states": args.max_states},
    });
    let mut report = common;

    match args.mode {
        Mode::All => {
            let mut roots =
                all_roots(&curve, &points, args.max_states).map_err(|error| error.to_string())?;
            roots.sort_finite();
            let distinct = roots.distinct_finite_count();
            let selected: CoefficientMethod = args.coefficient_method.into();
            let coefficient_value = if args.coefficients {
                let resolved = selected.resolved(roots.finite_x.len());
                json!({
                    "order": "low_degree_first",
                    "requested_method": selected.name(),
                    "resolved_method": resolved.name(),
                    "values": coefficients_from_roots(&curve, &roots.finite_x, selected),
                })
            } else {
                Value::Null
            };
            report["mode"] = json!("all");
            report["counts"] = json!({
                "nominal_signed_states": 1usize << (points.len() - 1),
                "finite_roots_with_multiplicity": roots.finite_x.len(),
                "distinct_finite_roots": distinct,
                "infinity_multiplicity": roots.infinity_multiplicity,
            });
            report["finite_roots"] = json!(roots.finite_x);
            report["coefficients"] = coefficient_value;
        }
        Mode::One => {
            let root =
                one_root(&curve, &points, args.sign_mask).map_err(|error| error.to_string())?;
            report["mode"] = json!("one");
            report["certificate"] = json!({
                "sign_mask": args.sign_mask,
                "sign_bits": points.len() - 1,
            });
            report["root"] = root.map(point_json).unwrap_or(Value::Null);
            report["infinity"] = json!(root.is_none());
        }
        Mode::Target => {
            let mut workspace = SummationRootWorkspace::default();
            let result = find_target_root(
                &curve,
                &points,
                &args.target_xs,
                args.max_states,
                &mut workspace,
            )
            .map_err(|error| error.to_string())?;
            report["mode"] = json!("target");
            report["targets"] = json!({
                "input_x": args.target_xs,
                "liftable": result.liftable_targets,
                "unliftable": result.unliftable_targets,
            });
            report["search"] = json!({
                "found": result.found(),
                "left_states": result.left_count,
                "right_signed_states": result.right_signed_count,
                "right_stored_states": result.right_count,
                "right_states": result.right_count,
                "right_quotient": "global_negation",
            });
            report["match"] = match (result.matched_target_x, result.sign_mask, result.sum) {
                (Some(target_x), Some(sign_mask), Some(sum)) => json!({
                    "target_x": target_x,
                    "sign_mask": sign_mask,
                    "sum": point_json(sum),
                }),
                _ => Value::Null,
            };
        }
    }
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::cryptanalysis::prime_summation_roots::{
        all_roots, coefficients_from_roots, find_target_root, one_root, CoefficientMethod,
        SummationRootWorkspace,
    };

    fn args(mode: Mode) -> SummationRootsArgs {
        let curve = FastCurve::for_arithmetic("fixture", 1009, 2, 3);
        let xs = (0..curve.f.p)
            .filter(|&x| curve.lift_x(x).is_some())
            .take(4)
            .collect();
        SummationRootsArgs {
            mode,
            p: 1009,
            a: 2,
            b: 3,
            xs,
            target_xs: Vec::new(),
            sign_mask: 0,
            coefficients: false,
            coefficient_method: Coefficients::Auto,
            max_states: 64,
        }
    }

    #[test]
    fn all_mode_emits_roots_and_coefficients() {
        let mut args = args(Mode::All);
        args.coefficients = true;
        let report = run(args).unwrap();
        assert_eq!(report["status"], "complete");
        assert_eq!(report["counts"]["nominal_signed_states"], 8);
        assert_eq!(
            report["coefficients"]["values"].as_array().unwrap().len(),
            9
        );
    }

    #[test]
    fn one_certificate_feeds_target_search() {
        let mut one = args(Mode::One);
        one.sign_mask = 3;
        let root = run(one.clone()).unwrap()["root"]["x"].as_u64().unwrap();
        let mut target = args(Mode::Target);
        target.target_xs.push(root);
        let report = run(target).unwrap();
        assert_eq!(report["search"]["found"], true);
        assert_eq!(report["match"]["target_x"], root);
        assert_eq!(
            report["search"]["right_signed_states"].as_u64(),
            report["search"]["right_stored_states"]
                .as_u64()
                .map(|stored| 2 * stored)
        );
    }

    #[test]
    fn rejects_singular_and_mode_mismatches() {
        let mut singular = args(Mode::All);
        singular.a = 0;
        singular.b = 0;
        assert!(run(singular).unwrap_err().contains("singular"));
        let mut target = args(Mode::Target);
        assert!(run(target.clone()).unwrap_err().contains("requires"));
        target.target_xs.push(1);
        target.coefficients = true;
        assert!(run(target).unwrap_err().contains("only"));
    }

    #[test]
    fn production_root_and_coefficient_paths_cross_check() {
        let curve = FastCurve::for_arithmetic("cross-check", 1009, 2, 3);
        let points: Vec<_> = (0..curve.f.p)
            .filter_map(|x| curve.lift_x(x))
            .take(7)
            .collect();
        for count in 1..=points.len() {
            let mut batched = all_roots(&curve, &points[..count], 64).unwrap();
            batched.sort_finite();
            let mut replayed = Vec::new();
            let mut infinity = 0;
            for mask in 0..(1u64 << (count - 1)) {
                match one_root(&curve, &points[..count], mask).unwrap() {
                    Some(point) => replayed.push(point.x),
                    None => infinity += 1,
                }
            }
            replayed.sort_unstable();
            assert_eq!(batched.finite_x, replayed, "point count {count}");
            assert_eq!(
                batched.infinity_multiplicity, infinity,
                "point count {count}"
            );
        }

        let mut distinct = all_roots(&curve, &points, 64).unwrap().finite_x;
        distinct.sort_unstable();
        distinct.dedup();
        let mut workspace = SummationRootWorkspace::default();
        for &target_x in &distinct {
            let result =
                find_target_root(&curve, &points, &[target_x], 12, &mut workspace).unwrap();
            assert!(result.found(), "target x={target_x}");
            assert_eq!(result.left_count + result.right_count, 12);
            assert_eq!(result.right_signed_count, 2 * result.right_count);
            assert_eq!(
                one_root(&curve, &points, result.sign_mask.unwrap())
                    .unwrap()
                    .unwrap()
                    .x,
                target_x
            );
        }
        let miss = (0..curve.f.p)
            .find(|&x| curve.lift_x(x).is_some() && distinct.binary_search(&x).is_err())
            .unwrap();
        let miss_result = find_target_root(&curve, &points, &[miss], 12, &mut workspace).unwrap();
        assert!(!miss_result.found());
        assert_eq!(miss_result.right_signed_count, 2 * miss_result.right_count);

        let point = points[0];
        let target_x = curve.canonical(point).0;
        let degenerate = [point, points[1], points[1]];
        let result = find_target_root(&curve, &degenerate, &[target_x], 3, &mut workspace).unwrap();
        assert!(result.found());
        assert_eq!(result.right_signed_count, 2 * result.right_count);
        assert_eq!(
            one_root(&curve, &degenerate, result.sign_mask.unwrap())
                .unwrap()
                .unwrap()
                .x,
            target_x
        );

        let roots: Vec<u64> = (0..256).map(|i| (17 * i) % 1009).collect();
        let quadratic = coefficients_from_roots(&curve, &roots, CoefficientMethod::Quadratic);
        let product = coefficients_from_roots(&curve, &roots, CoefficientMethod::ProductTree);
        assert_eq!(quadratic, product);
        for root in roots {
            let value = product.iter().rev().fold(0u64, |acc, &coefficient| {
                ((acc as u128 * root as u128 + coefficient as u128) % 1009) as u64
            });
            assert_eq!(value, 0, "root {root}");
        }
    }
}
