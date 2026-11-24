//! Scoped exact-source template/root adapter controls and matched-cost harness.
//! No scientific saved-case mode is enabled until its own protocol is frozen.
pub use crypto_lib::cryptanalysis;
#[path = "../src/cryptanalysis/jv_nagao_template.rs"]
#[allow(dead_code)]
mod jv_nagao_template;
#[path = "../src/cryptanalysis/jv_nagao_template_solver.rs"]
#[allow(dead_code)]
mod jv_nagao_template_solver;
#[path = "../src/cryptanalysis/jv_nagao_unit.rs"]
#[allow(dead_code)]
mod nagao_unit;
#[path = "../src/cryptanalysis/jv_nagao_unit_elimination.rs"]
#[allow(dead_code)]
mod nagao_unit_elimination;

fn main() {
    eprintln!("source qualification only; no saved-case worker mode registered");
}

#[cfg(test)]
mod controls {
    use super::{
        jv_nagao_template::{Limits, Poly, Template},
        jv_nagao_template_solver::{self as solver, Fallback},
    };
    use crypto_lib::cryptanalysis::{
        f4_fp::{F4Options, Ordering},
        jv_cover as cover,
    };
    use rand::{rngs::StdRng, RngCore, SeedableRng};
    use std::time::{Duration, Instant};
    fn family(a: u64) -> Vec<Poly> {
        vec![
            vec![(vec![2, 0], 1), (vec![0, 0], (101 - a) % 101)],
            vec![(vec![0, 1], 1), (vec![1, 0], 100)],
        ]
    }
    fn options() -> F4Options {
        F4Options::new(Ordering::Grevlex, 16)
    }
    fn status(s: &cover::ZeroDimStats) -> (bool, bool, bool, usize) {
        (s.incomplete, s.timed_out, s.inconsistent, s.nonseparating)
    }
    #[test]
    fn accepted_basis_uses_existing_root_stage_and_declares_checker_coverage() {
        let template = Template::learn(&family(1), 2, 101, Limits::default()).unwrap();
        let mut rng = StdRng::seed_from_u64(389);
        let result = solver::solve(
            &template,
            &family(1),
            101,
            &options(),
            &mut rng,
            Fallback::Plain,
        )
        .unwrap();
        assert!(result.audit.basis_accepted && !result.audit.fallback_used);
        assert_eq!(result.roots, vec![vec![1, 1], vec![100, 100]]);
        assert_eq!(status(&result.stats), (false, false, false, 0));
        assert_eq!(
            result.audit.checker.is_some(),
            cfg!(feature = "nagao-template-counters")
        );
        assert!(result.stats.f4_muls >= result.audit.attempted_elimination.modular_products);
        assert!(!result.audit.full_native_cost_available);
        if let Some(count) = result.audit.checker {
            assert!(count.modular_products > 0);
        }
        let mut reference_rng = StdRng::seed_from_u64(389);
        let (roots, stats) =
            cover::solve_zero_dim(&family(1), 2, 101, &options(), &mut reference_rng);
        assert_eq!(result.roots, roots);
        assert_eq!(status(&result.stats), status(&stats));
        assert_eq!(rng.next_u64(), reference_rng.next_u64());
    }
    #[test]
    fn nonradical_root_behavior_and_unresolved_flags_remain_visible() {
        let template = Template::learn(&family(1), 2, 101, Limits::default()).unwrap();
        let result = solver::solve(
            &template,
            &family(0),
            101,
            &options(),
            &mut StdRng::seed_from_u64(389),
            Fallback::Plain,
        )
        .unwrap();
        let (roots, stats) = cover::solve_zero_dim(
            &family(0),
            2,
            101,
            &options(),
            &mut StdRng::seed_from_u64(389),
        );
        assert!(result.audit.basis_accepted);
        assert_eq!(result.roots, roots);
        assert_eq!(status(&result.stats), status(&stats));
        assert_eq!(roots, vec![vec![0, 0]]);
        for root in &result.roots {
            assert!(family(0)
                .iter()
                .all(|f| crypto_lib::cryptanalysis::f4_fp::eval(f, root, 101) == 0));
        }
        println!(
            "nonradical roots={} incomplete={} nonseparating={} (existing solver status)",
            roots.len(),
            stats.incomplete,
            stats.nonseparating
        );
    }
    #[test]
    fn support_refusal_runs_both_explicit_fallback_policies() {
        let template = Template::learn(&family(1), 2, 101, Limits::default()).unwrap();
        let mut input = family(1);
        input[0].push((vec![1, 1], 1));
        for fallback in [Fallback::Plain, Fallback::Traced] {
            cover::clear_traces();
            cover::set_trace_scope(0x7430_0002);
            let result = solver::solve(
                &template,
                &input,
                101,
                &options(),
                &mut StdRng::seed_from_u64(389),
                fallback,
            )
            .unwrap();
            assert!(!result.audit.basis_accepted && result.audit.fallback_used);
            assert!(result.audit.refusal.as_deref().unwrap().contains("support"));
            let (roots, stats) =
                cover::solve_zero_dim(&input, 2, 101, &options(), &mut StdRng::seed_from_u64(389));
            assert_eq!(result.roots, roots);
            assert_eq!(status(&result.stats), status(&stats));
            assert!(result.stats.f4_muls >= stats.f4_muls);
        }
        cover::set_trace_scope(0);
        cover::clear_traces();
    }
    #[test]
    fn expired_budget_does_not_start_a_fresh_fallback() {
        let template = Template::learn(&family(1), 2, 101, Limits::default()).unwrap();
        let mut opts = options();
        opts.deadline = Some(Instant::now() - Duration::from_secs(1));
        let result = solver::solve(
            &template,
            &family(1),
            101,
            &opts,
            &mut StdRng::seed_from_u64(389),
            Fallback::Traced,
        )
        .unwrap();
        assert!(result.stats.timed_out && result.roots.is_empty());
        assert!(!result.audit.fallback_used);
        assert_eq!(result.audit.attempted_elimination.modular_products, 0);
    }
    #[test]
    fn incompatible_domain_is_rejected_before_the_solver() {
        let template = Template::learn(&family(1), 2, 101, Limits::default()).unwrap();
        let mut rng = StdRng::seed_from_u64(389);
        assert!(solver::solve(
            &template,
            &family(1),
            103,
            &options(),
            &mut rng,
            Fallback::Plain
        )
        .is_err());
        assert!(solver::solve(
            &template,
            &[vec![(vec![1], 1)]],
            101,
            &options(),
            &mut rng,
            Fallback::Plain
        )
        .is_err());
        assert!(solver::solve(
            &template,
            &family(1),
            101,
            &F4Options::new(Ordering::Lex, 16),
            &mut rng,
            Fallback::Plain
        )
        .is_err());
    }
    #[test]
    fn certified_unit_ideal_is_an_inconsistent_system() {
        let input = vec![
            vec![(vec![2], 1), (vec![0], 100)],
            vec![(vec![2], 1), (vec![0], 99)],
        ];
        let template = Template::learn(&input, 1, 101, Limits::default()).unwrap();
        let result = solver::solve(
            &template,
            &input,
            101,
            &options(),
            &mut StdRng::seed_from_u64(389),
            Fallback::Plain,
        )
        .unwrap();
        assert!(
            result.audit.basis_accepted && result.stats.inconsistent && result.roots.is_empty()
        );
        assert!(!result.audit.fallback_used && !result.stats.incomplete);
    }

    type Planted = (
        cover::Ctx,
        Vec<cover::BaseEl>,
        std::collections::HashMap<u64, usize>,
        cover::Div<cover::E2>,
    );
    fn planted() -> Planted {
        let spec = cover::generate_spec(101, 409);
        let ctx = cover::Ctx::new(&spec);
        let base = cover::factor_base(&ctx, &mut StdRng::seed_from_u64(13));
        let by_x = base.iter().enumerate().map(|(i, b)| (b.x, i)).collect();
        let mut r = ctx.jac().identity();
        for b in base.iter().take(6) {
            r = ctx.jac().add(&r, &b.d);
        }
        assert_eq!(r.u.len(), 4);
        (ctx, base, by_x, r)
    }
    fn full_template(ctx: &cover::Ctx, r: &cover::Div<cover::E2>, opts: &F4Options) -> Template {
        let mut v = r.v.clone();
        v.resize(3, cover::E2::ZERO);
        let input = cover::weil_forms(&ctx.f.f, &ctx.cov.hx, &r.u, &v).polys(&ctx.reim, 101);
        let report = crypto_lib::cryptanalysis::f4_fp::f4(&input, 6, 101, opts);
        assert!(!report.timed_out && report.pairs_above_bound == 0 && !report.inconsistent);
        Template::principal_syzygy(
            &input,
            &report.basis,
            6,
            101,
            7,
            Limits {
                rows: 3072,
                columns: 2048,
                cells: 16_000_000,
                degree: 16,
            },
        )
        .unwrap()
    }
    #[test]
    fn accepted_template_uses_the_actual_nagao_materializer() {
        let (ctx, base, by_x, r) = planted();
        let opts = F4Options::new(Ordering::Grevlex, 24);
        let (expected, cost) = cover::nagao_decompose(
            &ctx,
            &base,
            &by_x,
            &r,
            &opts,
            &mut StdRng::seed_from_u64(947),
        );
        assert!(!cost.incomplete && !cost.timed_out && !expected.is_empty());
        let template = full_template(&ctx, &r, &opts);
        let mut audits = Vec::new();
        let (rows, cost, unit) = super::nagao_unit::decompose_using(
            &ctx,
            &base,
            &by_x,
            &r,
            &opts,
            &mut StdRng::seed_from_u64(947),
            |input, p, options, rng| {
                let result = solver::solve(&template, input, p, options, rng, Fallback::Traced)?;
                audits.push(result.audit);
                Ok((result.roots, result.stats, None))
            },
        )
        .unwrap();
        assert!(unit.is_none());
        assert_eq!(rows, expected);
        assert_eq!(audits.len(), 1);
        assert!(audits[0].basis_accepted && !audits[0].fallback_used);
        assert!(!cost.incomplete && !cost.timed_out);
        assert!(rows
            .iter()
            .all(|d| cover::verify_dec(&ctx.jac(), &base, &r, d)));
        println!(
            "fixed Nagao accepted template: rows={} roots={} exact original divisor checks passed",
            rows.len(),
            cost.fp_solutions
        );
    }
    #[test]
    fn refused_template_uses_actual_materializer_and_both_fallbacks() {
        let (ctx, base, by_x, r) = planted();
        let opts = F4Options::new(Ordering::Grevlex, 24);
        let (expected, cost) = cover::nagao_decompose(
            &ctx,
            &base,
            &by_x,
            &r,
            &opts,
            &mut StdRng::seed_from_u64(947),
        );
        assert!(!cost.incomplete && !cost.timed_out && !expected.is_empty());
        let trivial: Vec<Poly> = (0..6)
            .map(|i| {
                let mut e = vec![0; 6];
                e[i] = 1;
                vec![(e, 1)]
            })
            .collect();
        let template = Template::learn(&trivial, 6, 101, Limits::default()).unwrap();
        for fallback in [Fallback::Plain, Fallback::Traced] {
            cover::clear_traces();
            cover::set_trace_scope(0x7430_0006);
            let mut audit = None;
            let (rows, cost, _) = super::nagao_unit::decompose_using(
                &ctx,
                &base,
                &by_x,
                &r,
                &opts,
                &mut StdRng::seed_from_u64(947),
                |input, p, options, rng| {
                    let result = solver::solve(&template, input, p, options, rng, fallback)?;
                    audit = Some(result.audit);
                    Ok((result.roots, result.stats, None))
                },
            )
            .unwrap();
            let audit = audit.unwrap();
            assert!(!audit.basis_accepted && audit.fallback_used);
            assert_eq!(rows, expected);
            assert!(!cost.incomplete && !cost.timed_out);
            assert!(rows
                .iter()
                .all(|d| cover::verify_dec(&ctx.jac(), &base, &r, d)));
        }
        cover::set_trace_scope(0);
        cover::clear_traces();
    }
    #[test]
    fn batched_field_hooks_preserve_caller_counters_without_double_pricing_inversions() {
        use crypto_lib::cryptanalysis::jv_bitops as native;
        let input = vec![
            vec![(vec![2], 1), (vec![0], 100)],
            vec![(vec![2], 1), (vec![0], 99)],
        ];
        let template = Template::learn(&input, 1, 101, Limits::default()).unwrap();
        let before = certified_relations::counters();
        native::enable(true);
        native::claim();
        native::set_moduli(101, 103);
        struct Disable;
        impl Drop for Disable {
            fn drop(&mut self) {
                native::enable(false);
            }
        }
        let _disable = Disable;
        let _phase = native::enter(native::Phase::Descent);
        let result = solver::solve(
            &template,
            &input,
            101,
            &options(),
            &mut StdRng::seed_from_u64(389),
            Fallback::Plain,
        )
        .unwrap();
        let after = certified_relations::counters();
        let snapshot = native::snapshot();
        let tally = snapshot.phase(native::Phase::Descent);
        assert!(result.stats.inconsistent && result.roots.is_empty());
        assert_eq!(tally.get(native::Kind::FieldMul), result.stats.f4_muls);
        assert_eq!(
            tally.get(native::Kind::FieldAdd),
            result.audit.attempted_elimination.coefficient_updates
                + result
                    .audit
                    .checker
                    .as_ref()
                    .map_or(0, |w| w.coefficient_updates)
        );
        assert_eq!(tally.get(native::Kind::FieldInv), 0);
        if let Some(checker) = result.audit.checker {
            assert_eq!(
                after.modular_products - before.modular_products,
                checker.modular_products
            );
        }
        assert!(!result.audit.full_native_cost_available);
    }
}
