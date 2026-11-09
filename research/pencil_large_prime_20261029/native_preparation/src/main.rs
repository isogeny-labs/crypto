use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

fn usage() -> ! {
    eprintln!(
        "usage:\n  native-preparation prepare-archive ARCHIVE OUT\n  native-preparation prepare-specs INPUTS OUT\n  native-preparation check-archive BINARY INPUTS OUT [--expected-plans DIR]\n  native-preparation check-prepared BINARY INPUTS SPECS OUT [--expected-plans DIR]\n  native-preparation prepare-disjoint-enrollment REQUEST OUT\n  native-preparation check-disjoint-enrollment REQUEST OUT"
    );
    std::process::exit(2);
}

fn optional_expected(args: &[String], start: usize) -> Result<Option<PathBuf>> {
    match &args[start..] {
        [] => Ok(None),
        [flag, path] if flag == "--expected-plans" => Ok(Some(path.into())),
        _ => bail!("expected no trailing arguments or --expected-plans DIR"),
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(String::as_str).unwrap_or_else(|| usage());
    match command {
        "prepare-archive" if args.len() == 4 => {
            pencil_native_preparation::prepare_archive(Path::new(&args[2]), Path::new(&args[3]))
        }
        "prepare-specs" if args.len() == 4 => {
            pencil_native_preparation::prepare_specs(Path::new(&args[2]), Path::new(&args[3]))
        }
        "check-archive" if args.len() >= 5 => {
            let expected = optional_expected(&args, 5)?;
            pencil_native_preparation::check_archive(
                Path::new(&args[2]),
                Path::new(&args[3]),
                Path::new(&args[4]),
                expected.as_deref(),
            )
        }
        "check-prepared" if args.len() >= 6 => {
            let expected = optional_expected(&args, 6)?;
            pencil_native_preparation::check_prepared(
                Path::new(&args[2]),
                Path::new(&args[3]),
                Path::new(&args[4]),
                Path::new(&args[5]),
                expected.as_deref(),
            )
        }
        "prepare-disjoint-enrollment" if args.len() == 4 => {
            pencil_native_preparation::disjoint_enrollment::prepare_disjoint_enrollment(
                Path::new(&args[2]),
                Path::new(&args[3]),
            )
        }
        "check-disjoint-enrollment" if args.len() == 4 => {
            pencil_native_preparation::disjoint_enrollment::verify_disjoint_enrollment(
                Path::new(&args[2]),
                Path::new(&args[3]),
            )
        }
        _ => usage(),
    }
    .with_context(|| format!("{command} failed"))
}
