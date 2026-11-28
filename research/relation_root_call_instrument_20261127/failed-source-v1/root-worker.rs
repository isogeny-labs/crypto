//! Complete root API call, additional wire readback and proof transport.
//! Qualification uses --qualify and creates no worker performance clocks.
use certified_relations::System;
use crypto_lib::cryptanalysis::f4_fp::{
    certified::{self, NativeCertification, ReplayForm, ReplayKernels},
    Poly,
};
use serde_json::json;
use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    io::{Read, Write},
    path::Path,
    time::Instant,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
enum Clock {
    Disabled,
    Wall(Instant),
}
impl Clock {
    fn start(qualify: bool) -> Self {
        if qualify {
            Self::Disabled
        } else {
            Self::Wall(Instant::now())
        }
    }
    fn elapsed(&self) -> Option<String> {
        match self {
            Self::Disabled => None,
            Self::Wall(t) => Some(t.elapsed().as_nanos().to_string()),
        }
    }
}
fn mode(kind: &str) -> Result<(ReplayForm, NativeCertification)> {
    let (form, kind) = if let Some(f) = kind.strip_prefix("root-exhaustive-") {
        (f, NativeCertification::Exhaustive)
    } else if let Some(f) = kind.strip_prefix("root-pairgraph-") {
        (f, NativeCertification::PairGraph)
    } else {
        return Err("explicit root native-check kind required".into());
    };
    let form = match form {
        "compact-sparse" => ReplayForm::CompactSparse,
        "compact-dag" => ReplayForm::CompactDag,
        "packed-sparse" => ReplayForm::PackedSparse,
        "packed-dag" => ReplayForm::PackedDag,
        "flat-sparse" => ReplayForm::FlatSparse,
        "flat-dag" => ReplayForm::FlatDag,
        _ => return Err("root wire form".into()),
    };
    Ok((form, kind))
}
fn save(path: &Path, bytes: &[u8]) -> Result<()> {
    if bytes.len() > 16 << 20 {
        return Err("artifact byte cap".into());
    }
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(path)?
        .write_all(bytes)?;
    Ok(())
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 5 && !(args.len() == 6 && args[5] == "--qualify") {
        return Err("usage:root-worker ROOT_KIND packed frontier|packed-products|packed-batch NEW_PROOF_DIR [--qualify]; System on stdin".into());
    }
    let qualify = args.len() == 6;
    let whole = Clock::start(qualify);
    let mut original = Vec::new();
    std::io::stdin()
        .lock()
        .take((4 << 20) + 1)
        .read_to_end(&mut original)?;
    if original.len() > 4 << 20 {
        return Err("original byte cap".into());
    }
    let system: System = serde_json::from_slice(&original)?;
    let (form, kind) = mode(&args[1])?;
    if args[2] != "packed" {
        return Err("root worker requires complete packed checker".into());
    }
    let kernel = match args[3].as_str() {
        "frontier" => ReplayKernels::BothUpdatesFrontier,
        "packed-products" => ReplayKernels::FrontierPackedProducts,
        "packed-batch" => ReplayKernels::FrontierPackedBatch,
        _ => return Err("root kernel enum".into()),
    };
    let input: Vec<Poly> = system
        .decode()?
        .into_iter()
        .map(|f| {
            f.into_iter()
                .map(|(m, c)| (m.into_iter().map(u32::from).collect(), c))
                .collect()
        })
        .collect();
    let construct = Clock::start(qualify);
    // This span includes the API's own wire replay, conversion and native
    // graph/exhaustive certification. No root setup or verification is amortized.
    let report = certified::try_packed_replay_basis_with_native_check(
        &input,
        system.variables.len(),
        system.prime,
        form,
        kernel,
        kind,
    )?;
    let construct_ns = construct.elapsed();
    if report.original_json != original {
        return Err("root canonical original differs from fixed wire".into());
    }
    if report.observed_solving_degree.is_some() {
        return Err("unexpected solving-degree claim".into());
    }
    let replay = Clock::start(qualify);
    // Additional readback remains visible and charged. Parent also replays the
    // transported bytes; none of these checks replaces the API's native checks.
    let checked = if args[1].ends_with("-dag") {
        relation_replay_checker::packed::verify_dag_json(&original, &report.witness_json)?
    } else {
        relation_replay_checker::packed::verify_json(&original, &report.witness_json)?
    };
    let native: Vec<BTreeMap<Vec<usize>, u64>> = report
        .basis
        .iter()
        .map(|f| {
            f.iter()
                .map(|(m, c)| (m.iter().map(|&e| e as usize).collect(), *c))
                .collect()
        })
        .collect();
    let checked_maps: Vec<BTreeMap<Vec<usize>, u64>> = checked
        .basis
        .iter()
        .map(|f| f.iter().cloned().collect())
        .collect();
    if native != checked_maps
        || serde_json::to_value(&report.replay_stats)? != serde_json::to_value(&checked.stats)?
    {
        return Err("root native basis or receipt differs from complete wire readback".into());
    }
    let replay_ns = replay.elapsed();
    let out = Path::new(&args[4]);
    fs::create_dir(out)?;
    save(&out.join("original.json"), &original)?;
    save(&out.join("witness.json"), &report.witness_json)?;
    let whole_ns = whole.elapsed();
    println!(
        "{}",
        json!({"status":"complete","prime":system.prime,"variables":system.variables.len(),
        "constructor":args[1],"checker":"packed","kernel":args[3],"basis":checked.basis,"consumer_stats":checked.stats,
        "native_basis":report.basis,"native_checks":report.native_checks,
        "native_check_kind":if kind==NativeCertification::PairGraph {"pairgraph"} else {"exhaustive"},
        "root_api_contains_wire_replay_and_native_checks":true,
        "phase_scope":"construct_and_wire includes complete root API, its wire replay, conversion and native certification; independent_replay is additional worker readback; parent additionally reads and replays transported proof",
        "phases_diagnostic_ns":{"construct_and_wire":construct_ns,"independent_replay":replay_ns,"full_worker_including_proof_write":whole_ns},
        "qualification_without_worker_clocks":qualify,"performance_observations_collected":if qualify {0} else {1},
        "S_pair_and_input_containment_checks":true})
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_unknown_kind_or_form_falls_through_to_a_cheaper_checker() {
        for kind in [
            "flat-dag",
            "root-pairgraph-flat-dog",
            "root-exhaustive-",
            "root-compact-dag",
        ] {
            assert!(mode(kind).is_err());
        }
        for prefix in ["root-exhaustive-", "root-pairgraph-"] {
            for form in [
                "compact-sparse",
                "compact-dag",
                "packed-sparse",
                "packed-dag",
                "flat-sparse",
                "flat-dag",
            ] {
                let (_, kind) = mode(&format!("{prefix}{form}")).unwrap();
                assert_eq!(
                    kind == NativeCertification::PairGraph,
                    prefix == "root-pairgraph-"
                );
            }
        }
    }
    #[test]
    fn qualification_never_constructs_or_reports_a_worker_clock() {
        let q = Clock::start(true);
        assert!(matches!(q, Clock::Disabled));
        assert_eq!(q.elapsed(), None);
    }
}
