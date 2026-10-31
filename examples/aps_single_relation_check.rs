//! Correctness instrument for public, tiny APS fixtures. Performance panels
//! must use the repository's frozen protocol, checkpoint and measurement gates.
use std::{fs, path::PathBuf, time::Duration};

use clap::{Parser, Subcommand, ValueEnum};
use crypto_lib::cryptanalysis::{
    aps_single_relation::{self as aps, Attempt, Encoding, Labels},
    curve_id, identity_certificate as ic,
    prime_fast::{FastCurve, Pt},
};
use num_bigint::BigUint;
use serde::Serialize;

#[derive(Parser)]
#[command(about = "Bounded APS correctness checks; no performance or scaling claims")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Independently replay a tiny public-fixture transcript with BigUint EC arithmetic.
    Replay {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Issue and independently recheck the established symmetric S3 identity.
    Certificate {
        #[arg(long)]
        output: PathBuf,
    },
    /// Issue/check the standard line-intersection coefficient identity.
    LineCertificate {
        #[arg(long)]
        output: PathBuf,
    },
    /// Solve a public known-answer fixture and retain all attempted bases.
    Solve {
        #[arg(long, value_enum, default_value = "toy101")]
        fixture: Fixture,
        #[arg(long, value_enum, default_value = "partitioned3")]
        arm: Arm,
        #[arg(long, value_enum, default_value = "guaranteed")]
        labels: LabelChoice,
        #[arg(long, default_value_t = 2, value_parser = clap::value_parser!(u8).range(1..=4))]
        subset_size: u8,
        #[arg(long, default_value_t = 20261026)]
        seed: u64,
        #[arg(long, default_value_t = 32, value_parser = clap::value_parser!(u8).range(1..=128))]
        attempts: u8,
        #[arg(long, default_value_t = 10000, value_parser = clap::value_parser!(u64).range(1..=60000))]
        budget_ms: u64,
        /// Stop only after a verified informative relation; enumeration is partial.
        #[arg(long)]
        first_informative: bool,
        #[arg(long)]
        output: PathBuf,
    },
}

#[derive(Clone, Copy, Debug, Serialize, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Fixture {
    Toy101,
    Toy211,
    Toy103J0,
}

#[derive(Clone, Copy, Debug, Serialize, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Arm {
    Partitioned3,
    Unordered3,
    Line3,
    Expanded4,
    Split4,
    BlockSymmetric4,
    Hybrid3,
    Hybrid4,
    Cover3,
    Orbit3,
    OrbitLine3,
    OrbitPrunedLine3,
}

#[derive(Clone, Copy, Debug, Serialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
enum LabelChoice {
    Random,
    Guaranteed,
}
impl From<LabelChoice> for Labels {
    fn from(value: LabelChoice) -> Self {
        match value {
            LabelChoice::Random => Labels::Random,
            LabelChoice::Guaranteed => Labels::Guaranteed,
        }
    }
}

fn fixture(which: Fixture) -> (FastCurve, Pt, Pt, u64) {
    let (p, a, b, n, gx, gy, d) = match which {
        Fixture::Toy101 => (101, 59, 6, 107, 78, 4, 37),
        Fixture::Toy211 => (211, 166, 158, 227, 155, 121, 73),
        Fixture::Toy103J0 => (103, 0, 5, 97, 2, 42, 17),
    };
    let curve = FastCurve::new("aps-public-fixture", p, a, b, n);
    let g = curve.point(gx, gy);
    let q = curve.mul(Some(g), d).unwrap();
    (curve, g, q, d)
}

fn identity_certificate() -> Result<ic::IdentityCertificate, String> {
    let cert = ic::issue(
        aps::symmetric_s3_identity(),
        20261026,
        32,
        "Identity generated from the standard coefficient formula; checked with an independent seed",
        None,
    ).map_err(|e| format!("{e:?}"))?;
    if !matches!(
        ic::check_with_extra(&cert, Some((9001, 32))).map_err(|e| format!("{e:?}"))?,
        ic::Verdict::Accept { .. }
    ) {
        return Err("independent identity check rejected the certificate".into());
    }
    Ok(cert)
}

fn encoding(arm: Arm) -> Encoding {
    match arm {
        Arm::Partitioned3 | Arm::Hybrid3 | Arm::Cover3 | Arm::Orbit3 => Encoding::Partitioned3,
        Arm::Unordered3 => Encoding::Unordered3,
        Arm::Line3 | Arm::OrbitLine3 | Arm::OrbitPrunedLine3 => Encoding::Line3,
        Arm::Expanded4 | Arm::Hybrid4 => Encoding::Expanded4,
        Arm::Split4 => Encoding::Split4,
        Arm::BlockSymmetric4 => Encoding::BlockSymmetric4,
    }
}

#[derive(Serialize)]
struct Transcript {
    schema: &'static str,
    fixture: Fixture,
    arm: Arm,
    labels: Labels,
    subset_size: u8,
    seed: u64,
    attempt_cap: u8,
    solver_budget_ms_per_attempt: u64,
    first_informative: bool,
    curve_icv1: String,
    curve_model: serde_json::Value,
    generator: (u64, u64),
    target: (u64, u64),
    expected_public_logarithm: u64,
    status: &'static str,
    attempts: Vec<Attempt>,
    limits: &'static str,
}

fn execute(args: Args) -> Result<(), String> {
    match args.command {
        Command::Replay { input, output } => {
            if input == output {
                return Err("replay input and output must differ".into());
            }
            let report = aps::replay::verify_file(&input)?;
            fs::write(
                output,
                serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
        }
        Command::Certificate { output } => {
            let cert = identity_certificate()?;
            fs::write(
                output,
                serde_json::to_vec_pretty(&cert).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
        }
        Command::LineCertificate { output } => {
            let cert = ic::issue(
                aps::line_intersection_identity(),
                20261028,
                32,
                "Standard coefficient expansion; checked using an independent seed",
                None,
            )
            .map_err(|e| format!("{e:?}"))?;
            if !matches!(
                ic::check_with_extra(&cert, Some((9101, 32))).map_err(|e| format!("{e:?}"))?,
                ic::Verdict::Accept { .. }
            ) {
                return Err("line identity check rejected".into());
            }
            fs::write(
                output,
                serde_json::to_vec_pretty(&cert).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
        }
        Command::Solve {
            fixture: which,
            arm,
            labels,
            subset_size,
            seed,
            attempts,
            budget_ms,
            first_informative,
            output,
        } => {
            let orbit = matches!(arm, Arm::Orbit3 | Arm::OrbitLine3 | Arm::OrbitPrunedLine3);
            if orbit && (which != Fixture::Toy103J0 || subset_size != 1) {
                return Err("orbit arms require --fixture toy103-j0 --subset-size 1".into());
            }
            let (curve, g, q, expected) = fixture(which);
            aps::validate_instance(&curve, g, q)?;
            let labels = Labels::from(labels);
            let budget = Duration::from_millis(budget_ms);
            let enc = encoding(arm);
            let runs = if orbit || matches!(arm, Arm::Hybrid3 | Arm::Hybrid4 | Arm::Cover3) {
                let arity = if arm == Arm::Hybrid4 { 4 } else { 3 };
                let mut runs = Vec::new();
                for i in 0..attempts {
                    let sampled = aps::sample_base(
                        &curve,
                        g,
                        q,
                        &vec![usize::from(subset_size); arity],
                        labels,
                        seed.wrapping_add(u64::from(i)),
                        10_000,
                    )?;
                    let base = if orbit {
                        aps::orbit_base(&curve, g, q, &sampled, 56, 35)?
                    } else {
                        sampled
                    };
                    let (algebra, recovery) = if first_informative {
                        let search = match arm {
                            Arm::Cover3 => aps::RelationSearch::Cover3,
                            Arm::Hybrid3 | Arm::Hybrid4 => aps::RelationSearch::Hybrid(enc),
                            Arm::OrbitPrunedLine3 => aps::RelationSearch::PrunedLine3 {
                                beta: 56,
                                lambda: 35,
                            },
                            _ => aps::RelationSearch::System(enc),
                        };
                        aps::find_relation(&curve, g, q, &base, search, 24, budget)?
                    } else {
                        let algebra = if arm == Arm::Cover3 {
                            let values: Vec<_> = base.points.keys().copied().collect();
                            aps::solve_cover(&curve, &values, 24, budget)?
                        } else if matches!(arm, Arm::Hybrid3 | Arm::Hybrid4) {
                            aps::solve_hybrid(&curve, &base.parts, enc, 24, budget)?
                        } else if arm == Arm::OrbitPrunedLine3 {
                            let system =
                                aps::build_pruned_line_system(&curve, g, q, &base, 56, 35)?;
                            aps::solve_system(&system, curve.f.p, 24, budget)?
                        } else {
                            let parts = if arm == Arm::OrbitLine3 {
                                vec![base.points.keys().copied().collect()]
                            } else {
                                base.parts.clone()
                            };
                            let system = aps::build_system(&curve, &parts, enc)?;
                            aps::solve_system(&system, curve.f.p, 24, budget)?
                        };
                        let recovery = aps::recover(&curve, g, q, &base, &algebra.coordinate_sets)?;
                        (algebra, recovery)
                    };
                    let stop = recovery.is_some();
                    runs.push(Attempt {
                        base,
                        algebra,
                        recovery,
                    });
                    if stop {
                        break;
                    }
                }
                runs
            } else {
                aps::solve_dlp_with_mode(
                    &curve,
                    g,
                    q,
                    enc,
                    labels,
                    usize::from(subset_size),
                    seed,
                    usize::from(attempts),
                    budget,
                    first_informative,
                )?
            };
            let recovered = runs.iter().find_map(|r| r.recovery.as_ref());
            if recovered.is_some_and(|r| r.logarithm != expected) {
                return Err("answer differs from the public known-answer fixture".into());
            }
            let status = if recovered.is_some() {
                "verified"
            } else if runs.iter().any(|r| !r.algebra.complete) {
                "incomplete"
            } else {
                "no_informative_relation"
            };
            let id = curve_id::prime(
                &BigUint::from(curve.f.p),
                &BigUint::from(curve.f.from_mont(curve.a)),
                &BigUint::from(curve.f.from_mont(curve.b)),
                &BigUint::from(curve.n),
            )
            .ok_or("curve identity unavailable")?;
            let transcript = Transcript {
                schema: "aps.single-relation.correctness/v1", fixture: which,
                arm, labels, subset_size, seed, attempt_cap: attempts,
                solver_budget_ms_per_attempt: budget_ms, first_informative, curve_icv1: id.icv1,
                curve_model: serde_json::from_str(&id.model_json).map_err(|e| e.to_string())?, generator: curve.canonical(g), target: curve.canonical(q),
                expected_public_logarithm: expected, status, attempts: runs,
                limits: "Tiny fixed fixtures only. No target scalar is passed to relation solving. F4 counters exclude setup and are not a complete cost model. Even-arity weights require one point per original partition; orbit expansion and regrouping still check B != 0. No performance, scaling, or general-security conclusion.",
            };
            let bytes = serde_json::to_vec_pretty(&transcript).map_err(|e| e.to_string())?;
            fs::write(output, &bytes).map_err(|e| e.to_string())?;
            // Keep the original producer output even when replay rejects it.
            // Successful CLI completion now requires separate arithmetic checks.
            aps::replay::verify_bytes(&bytes)
                .map_err(|e| format!("independent transcript replay rejected output: {e}"))?;
            if status != "verified" {
                return Err(format!("bounded search ended: {status}"));
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = execute(Args::parse()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shipped_transcripts_replay_to_their_hash_bound_receipts() {
        for (input, receipt) in [
            (
                include_str!("../research/aps_replay_20261029/fixtures/toy101-complete.json"),
                include_str!(
                    "../research/aps_replay_20261029/fixtures/toy101-complete.replay.json"
                ),
            ),
            (
                include_str!("../research/aps_replay_20261029/fixtures/toy211-streaming.json"),
                include_str!(
                    "../research/aps_replay_20261029/fixtures/toy211-streaming.replay.json"
                ),
            ),
            (
                include_str!("../research/aps_replay_20261029/fixtures/toy103-pruned.json"),
                include_str!("../research/aps_replay_20261029/fixtures/toy103-pruned.replay.json"),
            ),
            (
                include_str!("../research/aps_replay_20261029/fixtures/toy101-no-relation.json"),
                include_str!(
                    "../research/aps_replay_20261029/fixtures/toy101-no-relation.replay.json"
                ),
            ),
        ] {
            let actual = aps::replay::verify_bytes(input.as_bytes()).unwrap();
            let expected: serde_json::Value = serde_json::from_str(receipt).unwrap();
            assert_eq!(serde_json::to_value(actual).unwrap(), expected);
        }
    }

    #[test]
    fn every_cli_arm_replays_in_both_enumeration_modes() {
        for arm in [
            Arm::Partitioned3,
            Arm::Unordered3,
            Arm::Line3,
            Arm::Expanded4,
            Arm::Split4,
            Arm::BlockSymmetric4,
            Arm::Hybrid3,
            Arm::Hybrid4,
            Arm::Cover3,
            Arm::Orbit3,
            Arm::OrbitLine3,
            Arm::OrbitPrunedLine3,
        ] {
            for early in [false, true] {
                let output = std::env::temp_dir().join(format!(
                    "aps-replay-all-{}-{arm:?}-{early}.json",
                    std::process::id()
                ));
                let orbit = matches!(arm, Arm::Orbit3 | Arm::OrbitLine3 | Arm::OrbitPrunedLine3);
                execute(Args {
                    command: Command::Solve {
                        fixture: if orbit {
                            Fixture::Toy103J0
                        } else {
                            Fixture::Toy101
                        },
                        arm,
                        labels: if orbit {
                            LabelChoice::Random
                        } else {
                            LabelChoice::Guaranteed
                        },
                        subset_size: if orbit { 1 } else { 2 },
                        seed: 20261026,
                        attempts: 32,
                        budget_ms: 10000,
                        first_informative: early,
                        output: output.clone(),
                    },
                })
                .unwrap_or_else(|e| panic!("{arm:?}/{early}: {e}"));
                let report = aps::replay::verify_file(&output).unwrap();
                assert_eq!(report.outcome, aps::replay::Outcome::Verified);
                assert_eq!(report.verified_logarithm, Some(if orbit { 17 } else { 37 }));
                if early {
                    assert!(report.complete_attempts_checked < report.attempts_checked);
                }
                fs::remove_file(output).unwrap();
            }
        }
    }

    #[test]
    fn standalone_replay_checks_written_bytes_and_rejects_corrupted_inputs() {
        let input =
            std::env::temp_dir().join(format!("aps-replay-input-{}.json", std::process::id()));
        let output =
            std::env::temp_dir().join(format!("aps-replay-receipt-{}.json", std::process::id()));
        execute(Args {
            command: Command::Solve {
                fixture: Fixture::Toy211,
                arm: Arm::Line3,
                labels: LabelChoice::Guaranteed,
                subset_size: 2,
                seed: 20261026,
                attempts: 32,
                budget_ms: 10000,
                first_informative: true,
                output: input.clone(),
            },
        })
        .unwrap();
        let args = Args::try_parse_from([
            "aps",
            "replay",
            "--input",
            input.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
        ])
        .unwrap();
        execute(args).unwrap();
        let receipt: serde_json::Value =
            serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
        assert_eq!(receipt["schema"], "aps.single-relation.replay/v1");
        assert_eq!(receipt["verified_logarithm"], 73);
        assert_eq!(receipt["costs_attested"], false);
        fs::remove_file(&output).unwrap();
        assert!(execute(Args {
            command: Command::Replay {
                input: input.clone(),
                output: input.clone()
            }
        })
        .is_err());
        let mut transcript: serde_json::Value =
            serde_json::from_slice(&fs::read(&input).unwrap()).unwrap();
        transcript["expected_public_logarithm"] = serde_json::json!(74);
        fs::write(&input, serde_json::to_vec(&transcript).unwrap()).unwrap();
        assert!(execute(Args {
            command: Command::Replay {
                input: input.clone(),
                output: output.clone()
            }
        })
        .is_err());
        assert!(!output.exists());
        fs::remove_file(input).unwrap();
    }

    #[test]
    fn every_public_fixture_has_the_declared_point_orders_and_answer() {
        for which in [Fixture::Toy101, Fixture::Toy211, Fixture::Toy103J0] {
            let (curve, g, q, d) = fixture(which);
            aps::validate_instance(&curve, g, q).unwrap();
            assert_eq!(curve.mul(Some(g), d), Some(q));
        }
    }

    #[test]
    fn command_line_requires_an_output_and_rejects_zero_budgets() {
        assert!(Args::try_parse_from(["aps", "solve"]).is_err());
        assert!(
            Args::try_parse_from(["aps", "solve", "--output", "out.json", "--budget-ms", "0"])
                .is_err()
        );
        assert!(Args::try_parse_from(["aps", "certificate", "--output", "identity.json"]).is_ok());
    }

    #[test]
    fn certificate_uses_the_native_checker_and_a_second_seed() {
        assert!(matches!(
            ic::check(&identity_certificate().unwrap()).unwrap(),
            ic::Verdict::Accept { .. }
        ));
    }

    #[test]
    fn paper_baseline_with_random_labels_recovers_without_a_fallback() {
        let (curve, g, q, expected) = fixture(Fixture::Toy101);
        for enc in [Encoding::Partitioned3, Encoding::Split4] {
            let runs = aps::solve_dlp(
                &curve,
                g,
                q,
                enc,
                Labels::Random,
                2,
                20261026,
                32,
                Duration::from_secs(10),
            )
            .unwrap();
            assert_eq!(
                runs.last()
                    .and_then(|r| r.recovery.as_ref())
                    .map(|r| r.logarithm),
                Some(expected),
            );
        }
    }

    #[test]
    fn streaming_cli_retains_partial_enumeration_and_the_verified_witness() {
        let output =
            std::env::temp_dir().join(format!("aps-streaming-{}.json", std::process::id()));
        let args = Args::try_parse_from([
            "aps",
            "solve",
            "--arm",
            "line3",
            "--first-informative",
            "--output",
            output.to_str().unwrap(),
        ])
        .unwrap();
        execute(args).unwrap();
        let transcript: serde_json::Value =
            serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
        fs::remove_file(output).unwrap();
        assert_eq!(transcript["status"], "verified");
        assert_eq!(transcript["first_informative"], true);
        let last = transcript["attempts"].as_array().unwrap().last().unwrap();
        assert_eq!(last["algebra"]["complete"], false);
        assert_eq!(last["algebra"]["stopped_early"], true);
        assert_eq!(last["algebra"]["timed_out"], false);
        assert_eq!(last["recovery"]["logarithm"], 37);
    }

    #[test]
    fn line_identity_has_a_second_seed_check_and_cli_entry() {
        let cert = ic::issue(
            aps::line_intersection_identity(),
            20261028,
            32,
            "Correctness test of the standard identity",
            None,
        )
        .unwrap();
        assert!(matches!(
            ic::check_with_extra(&cert, Some((9101, 32))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
        assert!(
            Args::try_parse_from(["aps", "line-certificate", "--output", "identity.json"]).is_ok()
        );
        let bad = Args::try_parse_from([
            "aps",
            "solve",
            "--arm",
            "orbit-pruned-line3",
            "--output",
            "unused.json",
        ])
        .unwrap();
        assert!(execute(bad).is_err());
    }

    #[test]
    fn shipped_line_certificate_matches_the_statement_and_checks_with_a_second_seed() {
        let cert: ic::IdentityCertificate = serde_json::from_str(include_str!(
            "../research/aps_followup_20261028/line-identity.json"
        ))
        .unwrap();
        let expected = ic::issue(
            aps::line_intersection_identity(),
            20261028,
            32,
            "Statement fingerprint control",
            None,
        )
        .unwrap();
        assert_eq!(cert.id, expected.id);
        assert!(matches!(
            ic::check_with_extra(&cert, Some((9101, 32))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
    }
}
