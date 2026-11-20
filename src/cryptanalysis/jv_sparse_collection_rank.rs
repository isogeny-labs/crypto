//! Bounded rank admission for a growing sparse homogeneous IC matrix.
use super::sparse_kernel::{self, Matrix, ResultRow};
use crate::cryptanalysis::jv_bitops as work;
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Options {
    pub surplus: usize,
    pub failed_checkpoint_stride: usize,
    pub max_checkpoints: usize,
    pub max_preconditioners: usize,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            surplus: 8,
            failed_checkpoint_stride: 16,
            max_checkpoints: 8,
            max_preconditioners: 3,
        }
    }
}
impl Options {
    pub fn validate(self) -> Result<(), String> {
        if !(1..=256).contains(&self.surplus)
            || !(1..=256).contains(&self.failed_checkpoint_stride)
            || !(1..=32).contains(&self.max_checkpoints)
            || !(1..=8).contains(&self.max_preconditioners)
        {
            return Err("sparse rank scheduling bounds".into());
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Checkpoint {
    pub b_count: u64,
    pub rows: usize,
    pub result: ResultRow,
    pub native: serde_json::Value,
}
pub struct Tracker {
    modulus: u64,
    columns: usize,
    next_rows: usize,
    seed: u64,
    options: Options,
    pub checkpoints: Vec<Checkpoint>,
    pub kernel: Option<Vec<u64>>,
}
impl Tracker {
    pub fn new(modulus: u64, columns: usize, seed: u64, options: Options) -> Result<Self, String> {
        options.validate()?;
        if !(2..=4096).contains(&columns) {
            return Err("sparse rank columns".into());
        }
        Ok(Self {
            modulus,
            columns,
            next_rows: columns - 1 + options.surplus,
            seed,
            options,
            checkpoints: vec![],
            kernel: None,
        })
    }
    pub fn due(&self, rows: usize) -> bool {
        self.kernel.is_none() && !self.exhausted() && rows >= self.next_rows
    }
    pub fn exhausted(&self) -> bool {
        self.kernel.is_none() && self.checkpoints.len() >= self.options.max_checkpoints
    }
    pub fn rank_lower_bound(&self) -> usize {
        if self.kernel.is_some() {
            self.columns - 1
        } else {
            0
        }
    }
    pub fn check(&mut self, b_count: u64, rows: Vec<Vec<(usize, u64)>>) -> Result<(), String> {
        self.check_at(b_count, rows, false)
    }
    pub fn check_at_budget_end(
        &mut self,
        b_count: u64,
        rows: Vec<Vec<(usize, u64)>>,
    ) -> Result<(), String> {
        self.check_at(b_count, rows, true)
    }
    fn check_at(
        &mut self,
        b_count: u64,
        rows: Vec<Vec<(usize, u64)>>,
        budget_end: bool,
    ) -> Result<(), String> {
        if !self.due(rows.len())
            || b_count == 0
            || (!b_count.is_multiple_of(1024) && !budget_end)
            || self
                .checkpoints
                .last()
                .is_some_and(|c| c.b_count >= b_count)
        {
            return Err("sparse checkpoint schedule".into());
        }
        let before = work::snapshot();
        let ub = work::uniform_snapshot();
        let other = work::snapshot_other();
        let start = std::time::Instant::now();
        work::words((rows.len() + 2 * rows.iter().map(Vec::len).sum::<usize>()) as u64);
        let matrix = Matrix {
            modulus: self.modulus,
            columns: self.columns,
            rows,
        };
        let seed = self.seed ^ (self.checkpoints.len() as u64).wrapping_mul(0xD1B5_4A32_D192_ED03);
        let result =
            sparse_kernel::randomized_lanczos(&matrix, seed, self.options.max_preconditioners)?;
        let elapsed = start.elapsed().as_nanos();
        let tally = work::snapshot().since(&before);
        let total = tally.total();
        let uniform = work::uniform_snapshot()
            .since(&ub)
            .ok_or("uniform snapshot regression")?
            .sum(&work::Phase::ALL)
            .ok_or("uniform total overflow")?;
        let native = serde_json::json!({"primitives":work::prims_json(&total),"uniform":work::uniform_json(&uniform),"observed_incomplete_subtotal":work::price(&total,&work::widths(1009,self.modulus)).map_err(|e|format!("checkpoint price: {e:?}"))?.to_string(),"inner_wall_ns":elapsed.to_string(),"foreign_thread_hooks":work::foreign(),"other_modulus_events":work::snapshot_other().iter().zip(other).map(|(&a,b)|a-b).collect::<Vec<_>>(),"phases":work::Phase::ALL.iter().map(|&p|(p.name().to_string(),work::prims_json(tally.phase(p)))).collect::<serde_json::Map<String,_>>()});
        if let Some(h) = &result.kernel {
            if result.rank_certificate != Some(self.columns - 1) {
                return Err("uncertified sparse kernel".into());
            }
            self.kernel = Some(h.clone());
        }
        self.next_rows = matrix
            .rows
            .len()
            .checked_add(self.options.failed_checkpoint_stride)
            .ok_or("row schedule overflow")?;
        self.checkpoints.push(Checkpoint {
            b_count,
            rows: matrix.rows.len(),
            result,
            native,
        });
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actual_counted_checkpoint_has_owned_conserved_tallies() {
        // This control asserts a process-wide foreign-thread flag. Other
        // arithmetic tests intentionally trigger those hooks, so run the
        // complete assertion in a fresh process with exactly this test.
        const ISOLATED: &str = "CRYPTO_JV_CHECKPOINT_ISOLATED";
        if std::env::var_os(ISOLATED).is_none() {
            let result = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "cryptanalysis::jv_sieve::two_large_prime::sparse_collection_rank::tests::actual_counted_checkpoint_has_owned_conserved_tallies", "--nocapture"])
                .env(ISOLATED, "1")
                .output()
                .unwrap();
            assert!(
                result.status.success(),
                "{}\n{}",
                String::from_utf8_lossy(&result.stdout),
                String::from_utf8_lossy(&result.stderr)
            );
            assert!(String::from_utf8_lossy(&result.stdout).contains("1 passed"));
            return;
        }
        let l = 263807419614139553;
        work::claim();
        work::set_moduli(1009, l);
        work::enable(true);
        let _phase = work::enter(work::Phase::LinearAlgebra);
        let before = work::snapshot();
        let mut t = Tracker::new(
            l,
            3,
            46,
            Options {
                surplus: 1,
                ..Default::default()
            },
        )
        .unwrap();
        t.check_at_budget_end(
            1025,
            vec![vec![(0, 1), (2, l - 2)], vec![(1, 1), (2, l - 3)], vec![]],
        )
        .unwrap();
        let total = work::snapshot().since(&before).total();
        work::enable(false);
        let c = &t.checkpoints[0];
        assert_eq!(c.native["primitives"], work::prims_json(&total));
        assert_eq!(c.native["foreign_thread_hooks"], 0);
        assert_eq!(c.native["uniform"]["calls"], c.result.counts.draws);
        assert_eq!(c.native["primitives"]["modr_inv"], 2);
        assert_eq!(t.kernel, Some(vec![2, 3, 1]));
    }
    #[test]
    fn rank_is_not_admitted_from_row_count() {
        let t = Tracker::new(101, 4, 46, Options::default()).unwrap();
        assert!(!t.due(10));
        assert!(t.due(11));
        assert_eq!(t.rank_lower_bound(), 0);
        assert!(t.kernel.is_none());
    }
    #[test]
    fn deficient_attempts_are_retained_and_bounded() {
        let o = Options {
            surplus: 1,
            failed_checkpoint_stride: 2,
            max_checkpoints: 2,
            max_preconditioners: 2,
        };
        let mut t = Tracker::new(101, 4, 46, o).unwrap();
        let rows = vec![vec![(0, 1), (1, 1), (2, 1), (3, 100)]; 4];
        t.check(1024, rows.clone()).unwrap();
        assert_eq!(t.checkpoints[0].result.attempts.len(), 2);
        assert_eq!(t.rank_lower_bound(), 0);
        assert!(!t.due(5));
        let mut next = rows;
        next.extend([vec![], vec![]]);
        t.check(2048, next).unwrap();
        assert!(t.exhausted());
        assert!(!t.due(100));
        assert_eq!(t.checkpoints.len(), 2);
    }
    #[test]
    fn wrong_batches_and_invalid_limits_are_refused() {
        assert!(Tracker::new(101, 1, 46, Options::default()).is_err());
        let o = Options {
            max_preconditioners: 0,
            ..Default::default()
        };
        assert!(Tracker::new(101, 4, 46, o).is_err());
        let mut t = Tracker::new(
            101,
            4,
            46,
            Options {
                surplus: 1,
                ..Default::default()
            },
        )
        .unwrap();
        assert!(t.check(17, vec![vec![]; 4]).is_err());
        assert!(t.check(1024, vec![vec![]; 3]).is_err());
    }
    #[test]
    fn full_rank_kernel_is_admitted_only_after_a_certificate() {
        let mut t = Tracker::new(
            263807419614139553,
            3,
            46,
            Options {
                surplus: 1,
                ..Default::default()
            },
        )
        .unwrap();
        let rows = vec![
            vec![(0, 1), (2, 263807419614139551)],
            vec![(1, 1), (2, 263807419614139550)],
            vec![],
        ];
        t.check(1024, rows).unwrap();
        assert_eq!(t.kernel, Some(vec![2, 3, 1]));
        assert_eq!(t.rank_lower_bound(), 2);
        assert!(!t.due(100));
    }
}
