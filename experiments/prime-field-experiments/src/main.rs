use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use prime_field_experiments::{benchmark, maps};

#[derive(Parser)]
#[command(about = "Benchmark prime-field roots and profile auxiliary-curve maps")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Benchmark {
        #[arg(long, default_value_t = 5)]
        repeats: usize,
        #[arg(long, default_value_t = 3)]
        instances: usize,
        #[arg(long, default_value_t = 20261117)]
        seed: u64,
        #[arg(long, default_value_t = 1)]
        workers: usize,
    },
    Measure {
        config: PathBuf,
        #[arg(long, default_value_t = 1)]
        workers: usize,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Benchmark {
            repeats,
            instances,
            seed,
            workers,
        } => {
            let mut output = csv::Writer::from_writer(io::stdout().lock());
            for row in benchmark::run(repeats, seed, instances, workers)? {
                output.serialize(row)?;
            }
            output.flush()?;
        }
        Command::Measure { config, workers } => {
            let input =
                File::open(&config).with_context(|| format!("reading {}", config.display()))?;
            let config: maps::Config = serde_json::from_reader(BufReader::new(input))?;
            let mut output = io::stdout().lock();
            serde_json::to_writer_pretty(&mut output, &maps::measure(&config, workers)?)?;
            writeln!(output)?;
        }
    }
    Ok(())
}
