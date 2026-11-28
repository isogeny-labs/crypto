//! Fixed recorded mathematics and synthetic contract control, never a benchmark.
use serde_json::{json, Value};
#[cfg(not(test))]
use std::{fs, io::Write};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn ensure(ok: bool, why: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn product(xs: &[u128]) -> Result<u128> {
    xs.iter().try_fold(1u128, |a, &b| {
        a.checked_mul(b)
            .ok_or_else(|| "bound multiplication overflow".into())
    })
}
#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct Bound {
    row_muls: u128,
    adds: u128,
    inverses: u128,
    inverse_muls: u128,
    contributor_ors: u128,
}
impl Bound {
    fn matrix(rows: u128, cols: u128, p: u64) -> Result<Self> {
        ensure(p >= 5, "inverse bound domain")?;
        if rows == 0 || cols == 0 {
            return Ok(Self::default());
        }
        let rank = rows.min(cols);
        let words = rows.checked_add(63).ok_or("word bound overflow")? / 64;
        let exp = p - 2;
        let inverse_loop_muls = (u64::BITS - exp.leading_zeros() + exp.count_ones()) as u128;
        let bound = Self {
            row_muls: product(&[rows, cols, rank])?,
            // Each eliminated row also computes neg = p - f once.
            adds: product(&[
                rows - 1,
                cols.checked_add(1).ok_or("column bound overflow")?,
                rank,
            ])?,
            inverses: rank,
            inverse_muls: product(&[rank, inverse_loop_muls])?,
            contributor_ors: product(&[rows - 1, rank, words])?,
        };
        ensure(
            bound.row_muls.checked_add(bound.inverse_muls).is_some(),
            "combined bound overflow",
        )?;
        Ok(bound)
    }
    fn add(&mut self, b: &Self) -> Result<()> {
        self.row_muls = self
            .row_muls
            .checked_add(b.row_muls)
            .ok_or("row sum overflow")?;
        self.adds = self.adds.checked_add(b.adds).ok_or("add sum overflow")?;
        self.inverses = self
            .inverses
            .checked_add(b.inverses)
            .ok_or("inverse sum overflow")?;
        self.inverse_muls = self
            .inverse_muls
            .checked_add(b.inverse_muls)
            .ok_or("inverse mul sum overflow")?;
        self.contributor_ors = self
            .contributor_ors
            .checked_add(b.contributor_ors)
            .ok_or("OR sum overflow")?;
        ensure(
            self.row_muls.checked_add(self.inverse_muls).is_some(),
            "combined sum overflow",
        )
    }
    fn json(&self) -> Value {
        json!({"row_multiplications_upper":self.row_muls.to_string(),"field_additions_upper":self.adds.to_string(),
        "inverse_calls_upper":self.inverses.to_string(),"fermat_inverse_multiplications_upper":self.inverse_muls.to_string(),
        "contributor_word_ors_upper":self.contributor_ors.to_string(),"multiplications_including_fermat_upper":self.row_muls.checked_add(self.inverse_muls).map(|a|a.to_string()),
        "bound_not_actual_event_count":true,"inverse_calls_are_alternative_units_not_added_again":true})
    }
}

#[cfg(not(test))]
pub use crypto_lib::cryptanalysis;
#[cfg(not(test))]
#[allow(dead_code)]
#[path = "../../src/cryptanalysis/jv_nagao_unit_fast_elimination.rs"]
mod fast;
#[cfg(not(test))]
#[allow(dead_code)]
#[path = "../../src/cryptanalysis/jv_nagao_unit_elimination.rs"]
mod nagao_unit_elimination;

#[cfg(not(test))]
mod runtime {
    use super::*;
    use crypto_lib::cryptanalysis::{
        f4_fp::{self, F4Report, Poly},
        jv_cover as cover,
    };
    use rand::{rngs::StdRng, SeedableRng};
    use sha2::{Digest, Sha256};
    use std::{
        io::{BufRead, BufReader, Read},
        path::Path,
        time::Instant,
    };
    const RAW_SHA: &str = "6e1db64631838ebe3bdc918a88b160ced5c8362b70aea8814d16aca83368286d";
    const FIVE_SHA: &str = "43f4d8bbc9c524063b37c44e2df85bd521b8223704e0f53b645e49cb0caf2328";
    const SIX_SHA: &str = "f9bd2377cfb7dccf073d6bd9932bb7149b095efa0bf7f7027ab80d6e50d5f54d";
    fn file_sha(path: &str) -> Result<String> {
        let mut f = fs::File::open(path)?;
        let mut h = Sha256::new();
        let mut b = [0u8; 65536];
        loop {
            let n = f.read(&mut b)?;
            if n == 0 {
                break;
            }
            h.update(&b[..n]);
        }
        Ok(format!("{:x}", h.finalize()))
    }
    fn hash(value: &impl serde::Serialize) -> Result<String> {
        Ok(format!("{:x}", Sha256::digest(serde_json::to_vec(value)?)))
    }
    fn accepted(r: &F4Report) -> bool {
        !r.timed_out
            && r.pairs_above_bound == 0
            && (r.inconsistent || r.staircase_at_stop.is_some_and(|n| n <= 64))
    }
    fn report(r: &F4Report) -> Value {
        json!({"field_ops":r.field_ops,"steps":r.steps,"max_rows":r.max_rows,"max_cols":r.max_cols,
        "timed_out":r.timed_out,"pairs_above_bound":r.pairs_above_bound,"inconsistent":r.inconsistent,"staircase_at_stop":r.staircase_at_stop,
        "trace_mismatch":r.trace_mismatch,"conservative_staircase_acceptance":accepted(r),"basis_polynomials":r.basis.len()})
    }
    fn saved(path: &str) -> Result<(Value, Value)> {
        let mut first = None;
        let (mut count, mut mismatches, mut uncertified) = (0u64, 0u64, 0u64);
        for (i, line) in BufReader::new(fs::File::open(path)?).lines().enumerate() {
            let v: Value = serde_json::from_str(&line?)?;
            ensure(
                v["passed"] == true && v["residual_index"] == json!(i),
                "saved ordered replay",
            )?;
            if first.is_none() {
                first = Some(v.clone());
            }
            if v["stats"]["trace_mismatch"] == true {
                mismatches += 1;
                let s = &v["stats"];
                let has =
                    s["inconsistent"] == true || s["stopped_at"].as_u64().is_some_and(|n| n <= 64);
                if !has {
                    uncertified += 1;
                }
            }
            count += 1;
        }
        ensure(count == 3415, "saved denominator")?;
        Ok((
            first.ok_or("saved first record")?,
            json!({"systems":count,"mismatch_flags":mismatches,"mismatches_without_inconsistent_or_staircase_certificate":uncertified}),
        ))
    }
    fn roots(
        r: &F4Report,
        input: &[Poly],
        arity: usize,
        reduction: Option<&fast::UnitReduction>,
    ) -> Result<Vec<Vec<u64>>> {
        ensure(accepted(r), "accepted report before linear stage")?;
        if r.inconsistent {
            return Ok(vec![]);
        }
        let mut stats = cover::ZeroDimStats::default();
        let mut rng = StdRng::seed_from_u64(389);
        let rs = cover::linear_stage_from_basis(
            &r.basis,
            input,
            arity,
            1009,
            &mut rng,
            cover::LinMode::Fast,
            &mut stats,
            Instant::now(),
        );
        ensure(
            !stats.incomplete && !stats.timed_out && stats.nonseparating == 0,
            "complete linear stage",
        )?;
        let mut lifted = rs
            .iter()
            .map(|x| match reduction {
                Some(a) => a.lift(x).map_err(Into::into),
                None => Ok(x.clone()),
            })
            .collect::<Result<Vec<_>>>()?;
        lifted.sort();
        Ok(lifted)
    }
    fn arity_run(
        input: &[Poly],
        expected: &[Vec<u64>],
        first: &Value,
        arity: usize,
    ) -> Result<Value> {
        let reduction = if arity == 5 {
            Some(fast::UnitReduction::new(input, 1009)?)
        } else {
            None
        };
        let polys = reduction.as_ref().map_or(input, |a| a.polynomials());
        let mut opts = cover::opts_for_mode(24, 600.0, cover::StopMode::Staircase(64));
        opts.capture_run_log = true;
        let (tx, rx) = std::sync::mpsc::channel();
        let (record, trace) =
            f4_fp::observe_progress(tx, || f4_fp::f4_record(polys, arity, 1009, &opts));
        let log = f4_fp::take_run_log();
        let matrices: Vec<_> = rx.into_iter().filter(|a| a.event == "matrix").collect();
        let shape_ok = matrices.len() == record.steps
            && log.steps.len() == record.steps
            && trace.steps.len() == record.steps;
        let mut tight = Bound::default();
        let mut steps = Vec::new();
        ensure(shape_ok, "one trace/log/matrix per recorded step")?;
        for (i, (m, s)) in matrices.iter().zip(&log.steps).enumerate() {
            ensure(
                m.step == i
                    && m.variables == arity
                    && m.rows == s.all_rows
                    && s.rows <= s.all_rows
                    && m.degree == s.degree,
                "step shape identity",
            )?;
            let b = Bound::matrix(s.rows as u128, m.columns as u128, 1009)?;
            tight.add(&b)?;
            steps.push(json!({"step":i,"pair_rows_upper":s.rows,"all_matrix_rows":s.all_rows,"all_matrix_columns":m.columns,"learning_upper":b.json()}));
        }
        let mut coarse = Bound::default();
        let one = Bound::matrix(record.max_rows as u128, record.max_cols as u128, 1009)?;
        for _ in 0..record.steps {
            coarse.add(&one)?;
        }
        // Synthetic negative: invalid first nonempty recorded row index.
        let mut corrupted = trace.clone();
        let corrupt_step = corrupted
            .steps
            .iter()
            .position(|s| !s.rows.is_empty())
            .ok_or("nonempty stored row")?;
        corrupted.steps[corrupt_step].rows[0].0 = usize::MAX;
        let replay = f4_fp::f4_replay(polys, arity, 1009, &opts, &trace);
        let rejected = f4_fp::f4_replay(polys, arity, 1009, &opts, &corrupted);
        let baseline = f4_fp::f4(polys, arity, 1009, &opts);
        let mut expected = expected.to_vec();
        expected.sort();
        let record_roots = roots(&record, polys, arity, reduction.as_ref())?;
        let replay_roots = roots(&replay, polys, arity, reduction.as_ref())?;
        let fallback_roots = roots(&baseline, polys, arity, reduction.as_ref())?;
        let checks = json!({"matrix_diagnostics_bound":shape_ok,"record_matches_saved_f4_counter":json!(record.field_ops)==first["stats"]["f4_muls"],
            "record_matches_saved_matrix_maxima":json!(record.max_rows)==first["stats"]["max_rows"]&&json!(record.max_cols)==first["stats"]["max_cols"],
            "record_roots_match":record_roots==expected,"self_replay_roots_match":replay_roots==expected,"ordinary_fallback_roots_match":fallback_roots==expected,
            "corruption_has_mismatch":rejected.trace_mismatch,"corruption_not_accepted":!accepted(&rejected),"learning_bound_within_coarse":tight.row_muls<=coarse.row_muls&&tight.inverse_muls<=coarse.inverse_muls&&tight.contributor_ors<=coarse.contributor_ors});
        let passed = checks
            .as_object()
            .ok_or("checks")?
            .values()
            .all(|v| v == true);
        Ok(
            json!({"arity":arity,"input_sha256":hash(&polys)?,"f4_calls":4,"linear_stage_controls":3,"new_natural_yield_calls":0,
            "record":report(&record),"self_replay":report(&replay),"synthetic_corrupt_trace":report(&rejected),"ordinary_fallback":report(&baseline),
            "corrupt_step":corrupt_step,"corrupt_index":"usize::MAX; synthetic software-contract control, not a natural observed trace",
            "per_step_learning_upper":tight.json(),"coarse_learning_upper":coarse.json(),"steps":steps,"checks":checks,"passed":passed,
            "roots_in_each_good_control":expected.len(),"no_elapsed_values_exported":true}),
        )
    }
    pub fn run() -> Result<()> {
        let args: Vec<_> = std::env::args().skip(1).collect();
        ensure(
            args.len() == 4,
            "usage: AUDIT ORIGINAL_UNIT FIVE_JSONL SIX_JSONL NEW_RESULT",
        )?;
        ensure(!Path::new(&args[3]).exists(), "exclusive output")?;
        for (path, sha) in args[..3].iter().zip([RAW_SHA, FIVE_SHA, SIX_SHA]) {
            ensure(file_sha(path)? == sha, "recorded input hash")?;
        }
        let raw: Value = serde_json::from_reader(fs::File::open(&args[0])?)?;
        let input: Vec<Poly> =
            serde_json::from_value(raw["work"]["nagao_attempts"][0]["audit"]["original"].clone())?;
        let expected: Vec<Vec<u64>> = serde_json::from_value(
            raw["work"]["nagao_attempts"][0]["audit"]["lifted_roots"].clone(),
        )?;
        let (five, s5) = saved(&args[1])?;
        let (six, s6) = saved(&args[2])?;
        let arms = vec![
            arity_run(&input, &expected, &five, 5)?,
            arity_run(&input, &expected, &six, 6)?,
        ];
        let passed = arms.iter().all(|a| a["passed"] == true);
        let result = json!({"schema":"cover.unit-trace.learning-contract-audit/v1","original_input_sha256":RAW_SHA,"residual_index":0,"p":1009,
            "arms":arms,"saved_cohort_statuses":[s5,s6],"total_f4_calls":8,"new_natural_yield_calls":0,"timing_claim":false,"elapsed_values_exported":false,
            "internal_budget_clocks_present":true,"passed":passed,"complete_cost":false,"ratios":null,"rho_parity":false,"independent_signed_verdict":false,
            "learning_bounds_exclude":["allocation and row copies","pivot and contributor scans","other native bookkeeping"],
            "bounds_are_upper_limits_not_actual_events":true});
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&args[3])?;
        serde_json::to_writer_pretty(&mut file, &result)?;
        file.write_all(b"\n")?;
        ensure(passed, "audit check failed; original output preserved")?;
        println!("Eight fixed F4controls and six linear-stage controls completed; no elapsed result exported.");
        Ok(())
    }
}
#[cfg(not(test))]
fn main() -> Result<()> {
    runtime::run()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_dense_bound_includes_normalization_and_inverse_arithmetic() {
        let b = Bound::matrix(2, 2, 7).unwrap();
        assert_eq!(b.row_muls, 8);
        assert_eq!(b.adds, 6);
        assert_eq!(b.inverses, 2);
        assert_eq!(b.inverse_muls, 10);
        assert_eq!(b.contributor_ors, 2);
    }
    #[test]
    fn empty_rectangular_and_overflow_controls() {
        assert_eq!(Bound::matrix(0, 9, 7).unwrap(), Bound::default());
        let b = Bound::matrix(2, 1, 7).unwrap();
        assert_eq!(b.row_muls, 2);
        assert_eq!(b.inverses, 1);
        assert!(Bound::matrix(u128::MAX, 2, 7).is_err());
        let mut a = Bound {
            row_muls: u128::MAX,
            ..Default::default()
        };
        assert!(a.add(&b).is_err());
    }
    #[test]
    fn report_bound_keeps_integer_values_exact() {
        let b = Bound::matrix(65, 3, 1009).unwrap();
        assert_eq!(b.contributor_ors, 384);
        assert_eq!(b.inverse_muls, 57);
        assert_eq!(b.json()["row_multiplications_upper"], "585");
    }
}
