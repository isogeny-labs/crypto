//! Complete proof/native/JV-counter parity snapshots; no worker clocks.
use certified_relations::workload;
use crypto_lib::cryptanalysis::{
    f4_fp::{
        certified::{self, NativeCertification, ReplayForm, ReplayKernels},
        Poly,
    },
    jv_bitops as jv,
};
use serde_json::json;
use std::{error::Error, fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn save(path: &Path, bytes: &[u8]) -> Result<()> {
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(path)?
        .write_all(bytes)?;
    Ok(())
}
fn digest(bytes: &[u8]) -> String {
    crypto_lib::hash::sha256::sha256(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage:relation-canonical-scan-qualification NEW_OUT".into());
    }
    let out = Path::new(&args[1]);
    fs::create_dir(out)?;
    let mut rows = Vec::new();
    jv::claim();
    for p in [2147483647, 9007199254740881] {
        for n in [3, 4, 6] {
            let edges: Vec<_> = (0..n - 1).map(|i| (i, n - 1)).collect();
            let system = workload::generate_graph(p, n, 7, &edges)?.system;
            let input: Vec<Poly> = system
                .decode()?
                .into_iter()
                .map(|f| {
                    f.into_iter()
                        .map(|(m, c)| (m.into_iter().map(|e| e as u32).collect(), c))
                        .collect()
                })
                .collect();
            for (form, name) in [
                (ReplayForm::CompactSparse, "compact-sparse"),
                (ReplayForm::CompactDag, "compact-dag"),
                (ReplayForm::PackedSparse, "packed-sparse"),
                (ReplayForm::PackedDag, "packed-dag"),
                (ReplayForm::FlatSparse, "flat-sparse"),
                (ReplayForm::FlatDag, "flat-dag"),
            ] {
                for (kernel, key) in [
                    (ReplayKernels::BothUpdatesFrontier, "frontier103"),
                    (ReplayKernels::FrontierPackedProducts, "products231"),
                    (ReplayKernels::FrontierPackedBatch, "batch487"),
                ] {
                    for (kind, label) in [
                        (NativeCertification::Exhaustive, "exhaustive"),
                        (NativeCertification::PairGraph, "pairgraph"),
                    ] {
                        for counted in [false, true] {
                            jv::set_moduli(p, 0);
                            jv::enable(counted);
                            let before = jv::snapshot();
                            let report = certified::try_packed_replay_basis_with_native_check(
                                &input, n, p, form, kernel, kind,
                            )?;
                            if !certified::verify_packed_replay_basis(&report, &input)
                                || report.observed_solving_degree.is_some()
                            {
                                return Err("complete public verification failed".into());
                            }
                            let tally = jv::snapshot().since(&before);
                            jv::enable(false);
                            if jv::foreign() != 0 {
                                return Err("foreign accounting work".into());
                            }
                            let id = format!(
                                "p{p}-n{n}-{name}-{key}-{label}-{}",
                                if counted { "counted" } else { "off" }
                            );
                            let dir = out.join(&id);
                            fs::create_dir(&dir)?;
                            save(&dir.join("original.json"), &report.original_json)?;
                            save(&dir.join("proof.json"), &report.witness_json)?;
                            let basis = serde_json::to_vec(&report.basis)?;
                            save(&dir.join("basis.json"), &basis)?;
                            let primitives: Vec<_> =
                                tally.phases.iter().map(jv::prims_json).collect();
                            let row = json!({"case":id,"prime":p,"n":n,"seed":7,"graph":"LAST-star","form":name,"kernel":key,"kind":label,"bitop_enabled":counted,"original_sha256":digest(&report.original_json),"proof_sha256":digest(&report.witness_json),"basis_sha256":digest(&basis),"native_checks":report.native_checks,"replay_stats":report.replay_stats,"bitop_phases":primitives,"observed_solving_degree":null,"performance_observations":0});
                            save(&dir.join("outcome.json"), &serde_json::to_vec_pretty(&row)?)?;
                            rows.push(row);
                        }
                    }
                }
            }
        }
    }
    if rows.len() != 432 {
        return Err("incomplete fixed roster".into());
    }
    save(
        &out.join("summary.json"),
        &serde_json::to_vec_pretty(
            &json!({"schema":"canonical-row-scan-parity/v1","cases":rows,"complete_cases":432,"public_verifications":432,"performance_observations":0,"15x_admitted":false,"signed_verdict":false}),
        )?,
    )?;
    println!(
        "{}",
        json!({"complete_cases":432,"public_verifications":432,"accounting_modes":2,"performance_observations":0,"15x_admitted":false})
    );
    Ok(())
}
