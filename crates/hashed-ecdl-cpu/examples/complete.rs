//! Matched complete CPU recovery instrument. Fixed public software fixtures.
use hashed_ecdl_cpu::{paper, CachedMitm};
use paper::{Curve, DecompositionOracle, Error, HashBase, Point};
use std::{
    env, fs,
    time::{Duration, Instant},
};

struct Timed<O> {
    inner: O,
    bases: Duration,
    queries: Duration,
    calls: u64,
}
impl<O: DecompositionOracle> DecompositionOracle for Timed<O> {
    fn enumerate_bases(&mut self, c: &Curve, h: &[HashBase]) -> Result<Vec<Vec<Point>>, Error> {
        let start = Instant::now();
        let out = self.inner.enumerate_bases(c, h);
        self.bases += start.elapsed();
        out
    }
    fn find(
        &mut self,
        c: &Curve,
        b: &[Vec<Point>],
        t: Point,
        w: u64,
    ) -> Result<Option<Vec<Point>>, Error> {
        self.calls += 1;
        let start = Instant::now();
        let out = self.inner.find(c, b, t, w);
        self.queries += start.elapsed();
        out
    }
}
enum Oracle {
    Reference(paper::Enumerate),
    Cached(Box<CachedMitm>),
}
impl DecompositionOracle for Oracle {
    fn find(
        &mut self,
        c: &Curve,
        b: &[Vec<Point>],
        t: Point,
        w: u64,
    ) -> Result<Option<Vec<Point>>, Error> {
        match self {
            Self::Reference(o) => o.find(c, b, t, w),
            Self::Cached(o) => o.find(c, b, t, w),
        }
    }
}
fn ms(d: Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    let profile = args.len() == 6 && args[5] == "--profile-stages";
    if args.len() != 5 && !profile {
        return Err(
            "complete ARM[u/v/w/x] CASE[0..2] ROUND[0..9] OUTPUT_DIR [--profile-stages]".into(),
        );
    }
    let arm = &args[1];
    let case: usize = args[2].parse()?;
    let round: usize = args[3].parse()?;
    if case > 2 || round > 9 {
        return Err("invalid fixture/round".into());
    }
    let start = Instant::now();
    let (c, p, m, tau, scalar) = match case {
        0 => (Curve::new(211, 0, 7, 199)?, Some((3, 33)), 3, 40, 42),
        1 => (
            Curve::new(65521, 65518, 3, 65563)?,
            Some((0, 64894)),
            3,
            80,
            45678,
        ),
        _ => (
            Curve::new(65521, 65518, 3, 65563)?,
            Some((0, 64894)),
            4,
            64,
            54321,
        ),
    };
    let q = c.mul(p, scalar)?;
    let built_curve = start.elapsed();
    let seed = 111 * (case as u64 + 1);
    let config = paper::Config {
        m,
        tau: Some(tau),
        seed,
        max_trials: 16,
        max_work: 1_000_000,
        ..Default::default()
    };
    let inner = match arm.as_str() {
        "u" | "x" => Oracle::Reference(paper::Enumerate),
        "v" => Oracle::Cached(Box::new(CachedMitm::new(1_000_000, 64)?)),
        "w" => Oracle::Cached(Box::new(CachedMitm::new(1_000_000, 1)?)),
        _ => return Err("invalid arm".into()),
    };
    let mut oracle = Timed {
        inner,
        bases: Duration::ZERO,
        queries: Duration::ZERO,
        calls: 0,
    };
    let mut stages = paper::ReductionStats::default();
    let solve_start = Instant::now();
    let cert = if profile {
        paper::solve_with_oracle_profile(&c, p, q, &config, &mut oracle, &mut stages)?
    } else {
        paper::solve_with_oracle(&c, p, q, &config, &mut oracle)?
    };
    let solve = solve_start.elapsed();
    let replay_start = Instant::now();
    paper::verify(&c, p, q, &cert)?;
    assert_eq!(cert.scalar, scalar);
    // A corrupted scalar is a negative verification control, outside timers.
    let replay = replay_start.elapsed();
    let serialized = format!("{cert:?}\n");
    let total = start.elapsed();
    let mut bad = cert.clone();
    bad.scalar = (bad.scalar + 1) % c.order();
    assert!(paper::verify(&c, p, q, &bad).is_err());
    let mut stats = hashed_ecdl_cpu::Stats::default();
    if let Oracle::Cached(o) = &oracle.inner {
        stats = o.stats().clone();
    }
    // Complete certificate bytes are retained and compared across all arms.
    fs::write(
        format!("{}/cert-{arm}-{case}-{round}.txt", args[4]),
        &serialized,
    )?;
    let digest = serialized.bytes().fold(14695981039346656037u64, |h, b| {
        h.wrapping_mul(1099511628211) ^ u64::from(b)
    });
    let other = solve
        .checked_sub(oracle.bases + oracle.queries)
        .ok_or("timer overlap")?;
    // Extra clocks perturb the run. Keep these rows out of ordinary L0 panels;
    // the frozen analyzer rejects this distinct tier rather than pooling them.
    let tier = if profile { "L0-profile" } else { "L0" };
    if profile {
        fs::write(
            format!("{}/stages-{arm}-{case}-{round}.csv", args[4]),
            format!(
                "arm,instance_id,seed,replicate,solve_ms,bases_ms,targets_ms,oracle_ms,linear_ms,internal_replay_ms,external_replay_ms,complete_ms,tier\n{arm},{case},{seed},{round},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{tier}\n",
                ms(solve), ms(stages.bases), ms(stages.targets), ms(stages.oracle),
                ms(stages.linear), ms(stages.replay), ms(replay), ms(total),
            ),
        )?;
    }
    println!("{arm},{case},{seed},{round},{},verified,{},{},{},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{},{},{},{},{},{},{digest},{:.12},{tier}",
        round==0,cert.scalar,cert.attempts,cert.relations.len(),ms(built_curve),ms(oracle.bases),ms(oracle.queries),ms(other),ms(replay),ms(total),
        oracle.calls,stats.preparations,stats.built_rows,stats.probed_rows,stats.query_inversions,stats.peak_table_bytes,total.as_secs_f64());
    Ok(())
}
