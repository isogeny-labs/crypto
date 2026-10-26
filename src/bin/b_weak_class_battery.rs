//! Run the calibrated battery of "weak class recognisable from `b`" tests on
//! the NIST prime curves and write JSON, raw CSV rows and an SVG.
//!
//! Protocol and report: `research/b_weak_class_battery_20261022/`.

use clap::Parser;
use crypto_lib::cryptanalysis::b_weak_class_battery as bw;
use crypto_lib::cryptanalysis::p224_hypotheses::{nist_seeded_curves, x962_c_from_seed};
use num_bigint::BigUint;
use rayon::prelude::*;
use serde::Serialize;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::time::Instant;

#[derive(Parser, Serialize)]
#[command(
    about = "Calibrated battery of cheap b-testable weak-class statistics for the NIST curves"
)]
struct Cli {
    /// Output directory.
    #[arg(long)]
    out: PathBuf,
    /// Null draws per curve (the calibration panel).
    #[arg(long, default_value_t = 8000)]
    nulls: u32,
    /// Held-out null draws per curve, scored against the panel.
    #[arg(long, default_value_t = 1000)]
    holdout: u32,
    /// Planted instances per curve and statistic.
    #[arg(long, default_value_t = 100)]
    plants: u32,
    /// Attempts per planted instance before giving up.
    #[arg(long, default_value_t = 4000)]
    tries: u32,
    /// Domain label of the null seeds.
    #[arg(long, default_value = "b-weak-class-null/v1")]
    label: String,
    /// Family-wise error rate, Bonferroni over curves × statistics.
    #[arg(long, default_value_t = 0.05)]
    alpha: f64,
    /// Fraction of planted instances that must be detected.
    #[arg(long, default_value_t = 0.8)]
    detect_fraction: f64,
    /// Comma-separated curve names (default: all five).
    #[arg(long, value_delimiter = ',')]
    curves: Vec<String>,
    /// Also score the real NIST curves.  Without it only nulls, held-out
    /// nulls and plants run (the pre-freeze dry run).
    #[arg(long)]
    real: bool,
}

#[derive(Serialize)]
struct NullSummary {
    median: f64,
    q025: f64,
    q975: f64,
    q999: f64,
    max: f64,
}

#[derive(Serialize)]
struct PlantSummary {
    description: &'static str,
    constructed: u32,
    requested: u32,
    detected: u32,
    detect_rate: f64,
    min_p: Option<f64>,
    median_p: Option<f64>,
    passed: bool,
}

#[derive(Serialize)]
struct HoldoutSummary {
    draws: u32,
    below_threshold: u32,
    below_0_05: u32,
    below_0_01: u32,
}

#[derive(Serialize)]
struct RealResult {
    value: f64,
    p: f64,
    flagged: bool,
}

#[derive(Serialize)]
struct Row {
    stat: &'static str,
    family: &'static str,
    null: NullSummary,
    plant: PlantSummary,
    holdout: HoldoutSummary,
    real: Option<RealResult>,
    verdict: String,
}

#[derive(Serialize)]
struct CurveSummary {
    curve: String,
    bits: u64,
    seed_hex: String,
    null_draws: u32,
    mean_seeds_per_draw: f64,
    real_b_hex: Option<String>,
    real_c_hex: Option<String>,
    rows: Vec<Row>,
    seconds: Vec<(String, f64)>,
}

#[derive(Serialize)]
struct Summary {
    args: String,
    tests: usize,
    alpha: f64,
    threshold: f64,
    min_attainable_p: f64,
    flagged: Vec<String>,
    uncorrected_below_0_05: usize,
    curves: Vec<CurveSummary>,
}

#[derive(Serialize)]
struct RunRecord {
    git_revision: String,
    git_dirty: bool,
    rustc: String,
    threads: usize,
    seconds: f64,
}

fn command_output(program: &str, args: &[&str]) -> String {
    Command::new(program)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn write_json<T: Serialize>(dir: &Path, name: &str, value: &T) -> Result<(), String> {
    let path = dir.join(name);
    let mut bytes = serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    fs::write(&path, bytes).map_err(|e| format!("write {}: {e}", path.display()))
}

fn csv_header(first: &[&str]) -> String {
    let mut s = first.join(",");
    for n in bw::STAT_NAMES {
        s.push(',');
        s.push_str(n);
    }
    s.push('\n');
    s
}

fn csv_row(first: &[String], stats: &[f64; bw::NSTAT]) -> String {
    let mut s = first.join(",");
    for v in stats {
        let _ = write!(s, ",{v}");
    }
    s.push('\n');
    s
}

/// Null panel: draws `0..n` of `label`, in parallel, in index order.
fn draw_panel(ctx: &bw::Ctx, label: &str, curve: &str, n: u32) -> Vec<(bw::Instance, u32)> {
    (0..n)
        .into_par_iter()
        .map(|i| bw::null_instance(ctx, label, curve, i))
        .collect()
}

fn column(rows: &[(bw::Instance, u32)], s: usize) -> bw::NullColumn {
    bw::NullColumn::new(&rows.iter().map(|(i, _)| i.stats[s]).collect::<Vec<_>>())
}

fn run_curve(
    cli: &Cli,
    seeded: &crypto_lib::cryptanalysis::p224_hypotheses::SeededCurve,
    threshold: f64,
) -> Result<CurveSummary, String> {
    let name = seeded.params.name;
    let ctx = bw::Ctx::new(&seeded.params.p);
    let mut seconds = Vec::new();

    let started = Instant::now();
    let panel = draw_panel(&ctx, &cli.label, name, cli.nulls);
    seconds.push(("null_panel".to_string(), started.elapsed().as_secs_f64()));
    let mean_seeds = panel.iter().map(|(_, k)| *k as f64).sum::<f64>() / panel.len() as f64;
    let mut csv = csv_header(&["i", "seeds_tried"]);
    for (i, (inst, k)) in panel.iter().enumerate() {
        csv.push_str(&csv_row(&[i.to_string(), k.to_string()], &inst.stats));
    }
    fs::write(cli.out.join(format!("nulls_{name}.csv")), csv).map_err(|e| e.to_string())?;
    let columns: Vec<bw::NullColumn> = (0..bw::NSTAT).map(|s| column(&panel, s)).collect();

    // held-out nulls: a second, disjoint label, scored against the panel
    let started = Instant::now();
    let hold_label = format!("{}/holdout", cli.label);
    let holdout = draw_panel(&ctx, &hold_label, name, cli.holdout);
    seconds.push(("holdout".to_string(), started.elapsed().as_secs_f64()));
    let mut csv = csv_header(&["i", "seeds_tried"]);
    for (i, (inst, k)) in holdout.iter().enumerate() {
        csv.push_str(&csv_row(&[i.to_string(), k.to_string()], &inst.stats));
    }
    fs::write(cli.out.join(format!("holdout_{name}.csv")), csv).map_err(|e| e.to_string())?;

    // planted instances
    let started = Instant::now();
    let d3_max = columns[bw::NSTAT - 1].max();
    let mut plants: Vec<Vec<bw::Instance>> = Vec::new();
    for s in 0..bw::NSTAT {
        let found: Vec<bw::Instance> = if bw::STAT_NAMES[s] == "d3" {
            // rejection sampling from a fresh null stream
            let label = format!("{}/plant-d3", cli.label);
            let mut out = Vec::new();
            let mut start = 0u32;
            while out.len() < cli.plants as usize && start < 40_000 {
                let batch: Vec<bw::Instance> = (start..start + 200)
                    .into_par_iter()
                    .map(|i| bw::null_instance(&ctx, &label, name, i).0)
                    .filter(|inst| inst.stats[s] >= d3_max)
                    .collect();
                out.extend(batch);
                start += 200;
            }
            out.truncate(cli.plants as usize);
            out
        } else {
            (0..cli.plants)
                .into_par_iter()
                .filter_map(|j| {
                    let label = format!("{}/plant/{name}/{}/{j}", cli.label, bw::STAT_NAMES[s]);
                    bw::plant(&ctx, s, &mut bw::Stream::new(&label), cli.tries)
                })
                .collect()
        };
        plants.push(found);
    }
    seconds.push(("plants".to_string(), started.elapsed().as_secs_f64()));
    let mut csv = String::from("stat,j,value,p_empirical\n");
    for (s, found) in plants.iter().enumerate() {
        for (j, inst) in found.iter().enumerate() {
            let v = inst.stats[s];
            let _ = writeln!(
                csv,
                "{},{j},{v},{:.6}",
                bw::STAT_NAMES[s],
                columns[s].p_upper(v)
            );
        }
    }
    fs::write(cli.out.join(format!("plants_{name}.csv")), csv).map_err(|e| e.to_string())?;

    // real curve
    let real = if cli.real {
        let seed = hex::decode(seeded.seed_hex).map_err(|e| e.to_string())?;
        let c = x962_c_from_seed(&seed, ctx.t);
        let published =
            BigUint::parse_bytes(seeded.published_c_hex.as_bytes(), 16).ok_or("published c")?;
        if c != published {
            return Err(format!("{name}: derived c differs from the published c"));
        }
        let inst = bw::evaluate(&ctx, &seeded.params.b, &c)
            .map_err(|t| format!("{name}: real curve fails the conditioning: {t:?}"))?;
        Some(inst)
    } else {
        None
    };

    let mut rows = Vec::new();
    for s in 0..bw::NSTAT {
        let col = &columns[s];
        let ps: Vec<f64> = plants[s].iter().map(|i| col.p_upper(i.stats[s])).collect();
        let detected = ps.iter().filter(|&&p| p <= threshold).count() as u32;
        let mut sorted = ps.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let rate = detected as f64 / cli.plants as f64;
        let passed = rate >= cli.detect_fraction;
        let hold_p: Vec<f64> = holdout
            .iter()
            .map(|(i, _)| col.p_upper(i.stats[s]))
            .collect();
        let real_res = real.as_ref().map(|inst| {
            let p = col.p_upper(inst.stats[s]);
            RealResult {
                value: inst.stats[s],
                p,
                flagged: p <= threshold,
            }
        });
        let verdict = match &real_res {
            None => "dry run: real curve not scored".to_string(),
            Some(_) if !passed => "positive control failed: inconclusive".to_string(),
            Some(r) if r.flagged => {
                format!("flagged after correction (p = {:.2e})", r.p)
            }
            Some(r) => format!("within the null distribution (p = {:.3})", r.p),
        };
        rows.push(Row {
            stat: bw::STAT_NAMES[s],
            family: bw::STAT_FAMILY[s],
            null: NullSummary {
                median: col.quantile(0.5),
                q025: col.quantile(0.025),
                q975: col.quantile(0.975),
                q999: col.quantile(0.999),
                max: col.max(),
            },
            plant: PlantSummary {
                description: bw::plant_description(s),
                constructed: plants[s].len() as u32,
                requested: cli.plants,
                detected,
                detect_rate: rate,
                min_p: sorted.first().copied(),
                median_p: sorted.get(sorted.len() / 2).copied(),
                passed,
            },
            holdout: HoldoutSummary {
                draws: cli.holdout,
                below_threshold: hold_p.iter().filter(|&&p| p <= threshold).count() as u32,
                below_0_05: hold_p.iter().filter(|&&p| p <= 0.05).count() as u32,
                below_0_01: hold_p.iter().filter(|&&p| p <= 0.01).count() as u32,
            },
            real: real_res,
            verdict,
        });
    }
    Ok(CurveSummary {
        curve: name.to_string(),
        bits: ctx.t,
        seed_hex: seeded.seed_hex.to_string(),
        null_draws: cli.nulls,
        mean_seeds_per_draw: mean_seeds,
        real_b_hex: real.as_ref().map(|i| format!("{:x}", i.b)),
        real_c_hex: real.as_ref().map(|i| format!("{:x}", i.c)),
        rows,
        seconds,
    })
}

/// Sequential light-to-dark blue for `x` in `[0, 1]`.
fn shade(x: f64) -> String {
    let x = x.clamp(0.0, 1.0);
    let lerp = |a: f64, b: f64| (a + (b - a) * x).round() as u8;
    format!(
        "#{:02x}{:02x}{:02x}",
        lerp(239.0, 8.0),
        lerp(243.0, 48.0),
        lerp(255.0, 107.0)
    )
}

/// Two heat maps: the real curves' empirical p-values, and the planted
/// instances' detection rates.
fn svg(summary: &Summary, real: bool) -> String {
    let names: Vec<&str> = summary.curves.iter().map(|c| c.curve.as_str()).collect();
    let (cw, rh, left, top) = (86.0, 22.0, 120.0, 92.0);
    let w = left + 2.0 * (names.len() as f64 * cw) + 70.0;
    let h = top + bw::NSTAT as f64 * rh + 78.0;
    let mut s = String::new();
    let _ = writeln!(
        s,
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" font-family="DejaVu Sans, Helvetica, Arial, sans-serif" font-size="11">"##
    );
    let _ = writeln!(s, r##"<rect width="{w}" height="{h}" fill="#ffffff"/>"##);
    let _ = writeln!(
        s,
        r##"<text x="14" y="22" font-size="14" font-weight="bold" fill="#1a1a2e">Weak-class battery: real NIST curves against {} seed-generated nulls per curve</text>"##,
        summary.curves.first().map_or(0, |c| c.null_draws)
    );
    let _ = writeln!(
        s,
        r##"<text x="14" y="40" fill="#444">Left: empirical p-value of the real curve (darker = smaller). Right: fraction of 100 planted instances detected at the corrected threshold {:.2e}.</text>"##,
        summary.threshold
    );
    let _ = writeln!(
        s,
        r##"<text x="14" y="56" fill="#444">Grey cell: positive control failed, so the real-curve result is inconclusive. Flag rule: p at most {:.2e} (Bonferroni, {} tests).</text>"##,
        summary.threshold, summary.tests
    );
    for (panel, x0) in [(0usize, left), (1, left + names.len() as f64 * cw + 40.0)] {
        for (ci, n) in names.iter().enumerate() {
            let _ = writeln!(
                s,
                r##"<text x="{}" y="{}" text-anchor="middle" fill="#1a1a2e">{n}</text>"##,
                x0 + ci as f64 * cw + cw / 2.0,
                top - 8.0
            );
        }
        for si in 0..bw::NSTAT {
            for (ci, c) in summary.curves.iter().enumerate() {
                let row = &c.rows[si];
                let (fill, label, dark) = if panel == 0 {
                    match &row.real {
                        None => ("#eeeeee".to_string(), "-".to_string(), false),
                        Some(r) if !row.plant.passed => {
                            ("#cfcfcf".to_string(), format!("{:.3}", r.p), false)
                        }
                        Some(r) => {
                            let x = (-r.p.log10() / 4.0).min(1.0);
                            (shade(x), format!("{:.3}", r.p), x > 0.5)
                        }
                    }
                } else {
                    let x = row.plant.detect_rate;
                    (shade(x), format!("{:.2}", x), x > 0.5)
                };
                let _ = writeln!(
                    s,
                    r##"<rect x="{}" y="{}" width="{}" height="{}" fill="{fill}" stroke="#ffffff"/><text x="{}" y="{}" text-anchor="middle" fill="{}">{label}</text>"##,
                    x0 + ci as f64 * cw,
                    top + si as f64 * rh,
                    cw,
                    rh,
                    x0 + ci as f64 * cw + cw / 2.0,
                    top + si as f64 * rh + 15.0,
                    if dark { "#ffffff" } else { "#1a1a2e" }
                );
            }
        }
    }
    for si in 0..bw::NSTAT {
        let _ = writeln!(
            s,
            r##"<text x="{}" y="{}" text-anchor="end" fill="#1a1a2e">{} ({})</text>"##,
            left - 8.0,
            top + si as f64 * rh + 15.0,
            bw::STAT_NAMES[si],
            bw::STAT_FAMILY[si]
        );
    }
    let _ = writeln!(
        s,
        r##"<text x="14" y="{}" fill="#444">{} Smallest p the panel can give: {:.2e}. Source: summary.json and plants_*.csv in this directory.</text>"##,
        h - 40.0,
        if real {
            "Real curves scored."
        } else {
            "Dry run: real curves not scored."
        },
        summary.min_attainable_p
    );
    let _ = writeln!(
        s,
        r##"<text x="14" y="{}" fill="#444">Statistics are oriented so large is suspicious; p = (1 + #(null at least as large)) / (N + 1). Family letters: A Jacobi, B power-residue depth, C bits, D smoothness, E j bits, F 3-isogeny.</text>"##,
        h - 22.0
    );
    s.push_str("</svg>\n");
    s
}

fn run(cli: Cli) -> Result<(), String> {
    fs::create_dir_all(&cli.out).map_err(|e| e.to_string())?;
    let all = Instant::now();
    let seeded: Vec<_> = nist_seeded_curves()
        .into_iter()
        .filter(|c| cli.curves.is_empty() || cli.curves.iter().any(|n| n == c.params.name))
        .collect();
    if seeded.is_empty() {
        return Err("no curve selected".into());
    }
    let tests = seeded.len() * bw::NSTAT;
    let threshold = cli.alpha / tests as f64;
    let mut curves = Vec::new();
    for sc in &seeded {
        eprintln!("== {}", sc.params.name);
        curves.push(run_curve(&cli, sc, threshold)?);
    }
    let mut flagged = Vec::new();
    let mut uncorrected = 0usize;
    for c in &curves {
        for r in &c.rows {
            if let Some(real) = &r.real {
                if real.p <= 0.05 {
                    uncorrected += 1;
                }
                if real.flagged && r.plant.passed {
                    flagged.push(format!("{} {} p={:.3e}", c.curve, r.stat, real.p));
                }
            }
        }
    }
    let summary = Summary {
        args: serde_json::to_string(&cli).map_err(|e| e.to_string())?,
        tests,
        alpha: cli.alpha,
        threshold,
        min_attainable_p: 1.0 / (cli.nulls as f64 + 1.0),
        flagged,
        uncorrected_below_0_05: uncorrected,
        curves,
    };
    write_json(&cli.out, "summary.json", &summary)?;
    fs::write(cli.out.join("battery.svg"), svg(&summary, cli.real)).map_err(|e| e.to_string())?;
    write_json(
        &cli.out,
        "run.json",
        &RunRecord {
            git_revision: command_output("git", &["rev-parse", "HEAD"]),
            git_dirty: !command_output("git", &["status", "--porcelain", "--untracked-files=no"])
                .is_empty(),
            rustc: command_output("rustc", &["--version"]),
            threads: rayon::current_num_threads(),
            seconds: all.elapsed().as_secs_f64(),
        },
    )?;
    eprintln!(
        "flagged after correction: {:?}; wall {:.0}s",
        summary.flagged,
        all.elapsed().as_secs_f64()
    );
    Ok(())
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
