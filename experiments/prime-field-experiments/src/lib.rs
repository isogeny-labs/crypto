mod poly;
mod roots;

pub mod benchmark;
pub mod maps;

pub use poly::PrimeField;
pub use roots::{find_roots, RootRun};

use anyhow::{ensure, Result};
use rayon::prelude::*;

fn run_jobs<'a, J, R>(
    jobs: &'a [J],
    workers: usize,
    run: impl Fn(&'a J) -> Result<R> + Send + Sync,
) -> Result<Vec<R>>
where
    J: Sync,
    R: Send,
{
    ensure!(workers > 0, "workers must be positive");
    let workers = workers.min(jobs.len());
    if workers <= 1 {
        return jobs.iter().map(run).collect();
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(workers)
        .build()?;
    // Collect into an ordered vector before propagating errors.
    let results: Vec<_> = pool.install(|| jobs.par_iter().map(run).collect());
    results.into_iter().collect()
}
