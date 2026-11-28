//! A downstream application can own every stage without editing a registry in
//! crypto_lib. Run with `cargo run --release --example ic_custom_pipeline`.
//! This uses a public planted research fixture, never a production secret.

use std::cell::Cell;

use crypto_lib::cryptanalysis::ic_boundary::{
    roster_prime_instance, Calibration, CompletionRule, CountedGroup, GroupOps, IncrementalGauss,
    PrimeCurve, RelationSolver, RowStatus,
};
use crypto_lib::cryptanalysis::ic_framework::plugins::{MitmOracle, PrimeAbscissaBase};
use crypto_lib::cryptanalysis::ic_framework::registry::PluginRegistry;
use crypto_lib::cryptanalysis::ic_framework::stages::{
    DecompositionOracle, FactorBaseBuilder, InstanceCtx, PipelineStages, RelationSolverFactory,
    Targets,
};
use crypto_lib::cryptanalysis::ic_framework::{
    run_pipeline_with_stages, PipelineEvent, PipelineSpec, RunReport,
};

struct ObservedMatrix<'a> {
    inner: IncrementalGauss,
    rows: &'a Cell<usize>,
}

impl RelationSolver for ObservedMatrix<'_> {
    fn add_row(&mut self, row: Vec<u64>, rhs: u64) -> RowStatus {
        self.rows.set(self.rows.get() + 1);
        self.inner.add_row(row, rhs)
    }
    fn pinned(&self, col: usize) -> Option<u64> {
        self.inner.pinned(col)
    }
    fn rank(&self) -> usize {
        self.inner.rank()
    }
    fn dependent(&self) -> u64 {
        self.inner.dependent
    }
    fn work(&self) -> (u64, &'static str) {
        (self.inner.row_ops, "row_ops")
    }
}

struct ApplicationMatrix<'a> {
    rows: &'a Cell<usize>,
}

impl RelationSolverFactory for ApplicationMatrix<'_> {
    fn name(&self) -> &str {
        "application-matrix"
    }
    fn create(&self, columns: usize, modulus: u64) -> Result<Box<dyn RelationSolver + '_>, String> {
        Ok(Box::new(ObservedMatrix {
            inner: IncrementalGauss::new(columns, modulus),
            rows: self.rows,
        }))
    }
    fn completion_rule(&self) -> CompletionRule {
        CompletionRule::FullRank
    }
    fn verify_base_columns(&self) -> bool {
        true
    }
}

fn run_demo(expected: Option<u64>) -> Result<(RunReport, usize, usize), String> {
    let instance = roster_prime_instance(16).ok_or("missing public fixture")?;
    let generator = instance.generator_point();
    let planted = 30_011 % instance.r;
    let target = instance
        .curve
        .mul(&mut GroupOps::default(), generator, planted);
    let ctx = InstanceCtx {
        group: &instance.curve,
        generator,
        target,
        r: instance.r,
        cofactor: instance.cofactor,
        group_order: instance.group_order,
        name: instance.name.clone(),
        field_degree: None,
    };

    // The base factory borrows this application's curve instance. Factories
    // need neither 'static storage nor Send/Sync nor global registration.
    let mut bases: PluginRegistry<'_, Box<dyn FactorBaseBuilder<PrimeCurve> + '_>> =
        PluginRegistry::new();
    bases.register("local-base", |_| {
        Ok(Box::new(PrimeAbscissaBase {
            instance: &instance,
        }))
    })?;
    let mut oracles: PluginRegistry<'_, Box<dyn DecompositionOracle<PrimeCurve>>> =
        PluginRegistry::new();
    oracles.register("local-oracle", |params| {
        let m = u32::try_from(params.u64_or("m", 2)?).map_err(|_| "m exceeds u32")?;
        Ok(Box::new(MitmOracle::new(m)))
    })?;
    let base = bases.create("local-base")?;
    let mut oracle = oracles.create("local-oracle:m=2")?;
    let rows = Cell::new(0);
    let matrix = ApplicationMatrix { rows: &rows };
    let mut spec = PipelineSpec {
        factor_base: "local-base".into(),
        oracle: "local-oracle".into(),
        // Custom injection bypasses the built-in name registry entirely.
        linalg: "a-name-that-is-not-registered".into(),
        targets: Targets::Walk,
        seed: 7,
        max_trials: 2_000_000,
        ..Default::default()
    };
    spec.factor_base_params.set("size", "24");
    spec.oracle_params.set("negation_folded", "1");
    let mut witnessed = 0;
    let mut finished_verification = None;
    let report = run_pipeline_with_stages(
        &ctx,
        &spec,
        PipelineStages {
            factor_base: base.as_ref(),
            oracle: oracle.as_mut(),
            matrix: &matrix,
        },
        expected,
        &Calibration::default(),
        None,
        Some(&mut |event| {
            if let PipelineEvent::Query { relations, .. } = event {
                witnessed += relations.len();
            }
            if let PipelineEvent::Finished { verified, .. } = event {
                finished_verification = Some(verified);
            }
        }),
    )?;
    assert_eq!(finished_verification, Some(report.verified));
    Ok((report, rows.get(), witnessed))
}

fn main() -> Result<(), String> {
    let (report, rows, _) = run_demo(None)?;
    println!(
        "verified={}, matrix={}, rank={}, submitted_rows={rows}",
        report.verified, report.linear_algebra.name, report.linear_algebra.rank
    );
    if !report.verified {
        return Err("public fixture was not recovered".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrowed_custom_stages_recover_and_verify_every_base_column() {
        let (report, rows, witnessed) = run_demo(None).unwrap();
        assert!(report.verified);
        assert_eq!(report.linear_algebra.name, "application-matrix");
        assert_eq!(report.spec.linalg, "application-matrix");
        assert!(report.label.contains("application-matrix"));
        assert_eq!(
            report.linear_algebra.rank,
            report.factor_base.columns as u64 + 1
        );
        assert_eq!(
            report.verify.native["base_log_columns_checked"],
            report.factor_base.columns as u64
        );
        assert!(rows > 0 && witnessed >= rows);
    }

    #[test]
    fn known_answer_mismatch_is_not_reported_as_verified() {
        let (report, _, _) = run_demo(Some(0)).unwrap();
        assert!(report.recovered.is_some());
        assert!(!report.verified);
    }
}
