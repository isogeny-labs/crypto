//! `jacobian_transfers`: discover and price every known route from a
//! registered curve's discrete logarithm into a Jacobian.
//!
//! ```text
//! jacobian_transfers build                   # catalog rows of docs/curves/registry.json → docs/curves/jacobian-transfers.json
//! jacobian_transfers check                   # the committed file is current (CI)
//! jacobian_transfers show "Oakley Group 3"   # one curve, every route
//! jacobian_transfers rank --top 20           # curves by best ratio
//! jacobian_transfers table                   # Markdown summary table
//! jacobian_transfers chart --svg out.svg --tikz out.tex
//! ```
//!
//! See `docs/curves/JACOBIAN_TRANSFERS.md`.

use std::process::ExitCode;

use clap::{Parser, Subcommand};
use crypto_lib::cryptanalysis::jacobian_transfer::{
    class_name, cost::CostModel, read_registry, report, CurveTransfers, Field, RegistryCurve,
    RouteClass, TransferFile, DEFAULT_FACTOR_BUDGET,
};

const REGISTRY: &str = "docs/curves/registry.json";
const OUT: &str = "docs/curves/jacobian-transfers.json";
const CATALOG: &str = crypto_lib::cryptanalysis::cover_catalog::MANIFEST_PATH;

#[derive(Parser)]
#[command(about = "Jacobian transfer routes of registered curves: build, check, show, rank, chart")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Screen every curve of the catalog and write the report.
    Build {
        #[arg(long, default_value = REGISTRY)]
        registry: String,
        /// The manifest naming the registry rows to screen.
        #[arg(long, default_value = CATALOG)]
        catalog: String,
        #[arg(long, default_value = OUT)]
        out: String,
        /// Pollard-rho iterations per order whose subgroup is not recorded.
        #[arg(long, default_value_t = DEFAULT_FACTOR_BUDGET)]
        budget: u64,
    },
    /// Rebuild with the committed file's cost model and fail if it differs.
    Check {
        #[arg(long, default_value = REGISTRY)]
        registry: String,
        #[arg(long, default_value = CATALOG)]
        catalog: String,
        #[arg(long, default_value = OUT)]
        report: String,
    },
    /// Inventory extension fields for security review (not a security verdict).
    ExtensionFields {
        #[arg(long, default_value = REGISTRY)]
        registry: String,
        #[arg(long, default_value = OUT)]
        report: String,
        /// Case-insensitive substring of the slug or standard name.
        #[arg(long)]
        name: Option<String>,
    },
    /// Every route of one curve (slug, standard name or legacy spelling).
    Show {
        name: String,
        #[arg(long, default_value = OUT)]
        report: String,
        #[arg(long)]
        json: bool,
    },
    /// Curves ordered by the best carried route's ratio to rho.
    Rank {
        #[arg(long, default_value_t = 25)]
        top: usize,
        /// Only curves with this verdict (`formula_advantage`, `formula_marginal`, `boundary`, `not_carried`).
        #[arg(long)]
        class: Option<String>,
        #[arg(long, default_value = OUT)]
        report: String,
        #[arg(long)]
        json: bool,
    },
    /// The Markdown summary table; with `--write-doc PATH`, replace everything
    /// after the `<!-- jacobian_transfers table -->` marker in that file.
    Table {
        #[arg(long, default_value = OUT)]
        report: String,
        #[arg(long)]
        write_doc: Option<String>,
    },
    /// The ratio chart as SVG and/or TikZ.
    Chart {
        #[arg(long, default_value = OUT)]
        report: String,
        #[arg(long)]
        svg: Option<String>,
        #[arg(long)]
        tikz: Option<String>,
    },
}

fn load(path: &str) -> Result<TransferFile, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    serde_json::from_str(&text).map_err(|e| format!("{path}: {e}"))
}

fn best_ratio(c: &CurveTransfers) -> f64 {
    c.best_route
        .map(|i| c.routes[i].cost.ratio_log2)
        .unwrap_or(f64::NEG_INFINITY)
}

fn show(c: &CurveTransfers) {
    let names = if c.standard_names.is_empty() {
        String::new()
    } else {
        format!("  [{}]", c.standard_names.join(", "))
    };
    println!("{}{names}", c.slug);
    println!("  {}", c.icv1);
    println!(
        "  family {}  field {} (degree {}, {:.1} bits)  trace {}",
        c.family, c.field.kind, c.field.degree, c.field.log2_size, c.trace
    );
    println!(
        "  subgroup r: {:.1} bits [{}]  rho reference: 2^{:.1}",
        c.subgroup.bits, c.subgroup.status, c.reference_log2
    );
    println!("  verdict {}", class_name(c.verdict));
    println!(
        "  {:<4} {:<24} {:<8} {:>10} {:>9} {:>9} {:<17} {:<17} evidence",
        "#", "route", "genus", "dest log2", "total", "ratio", "transport", "class"
    );
    for (i, r) in c.routes.iter().enumerate() {
        let mark = if Some(i) == c.best_route { "*" } else { " " };
        let genus = if r.destination.genus.len() > 8 {
            format!("2^{:.0}", r.destination.log2_genus)
        } else {
            r.destination.genus.clone()
        };
        println!(
            "  {mark}{:<3} {:<24} {:<8} {:>10.1} {:>9.1} {:>+9.1} {:<17} {:<17} {}",
            i,
            r.kind,
            genus,
            r.cost.destination.log2_cost,
            r.cost.log2_total,
            r.cost.ratio_log2,
            serde_json::to_value(r.transport)
                .unwrap()
                .as_str()
                .unwrap_or(""),
            class_name(r.class),
            serde_json::to_value(r.evidence)
                .unwrap()
                .as_str()
                .unwrap_or("")
        );
        println!("        {}", r.label);
        let inv: Vec<String> = r
            .invariants
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect();
        if !inv.is_empty() {
            println!("        {}", inv.join("  "));
        }
        println!(
            "        solver {:?} (leading term 2^{:.1}, exponent ratio {:+.1}); search {}, transfer {}",
            r.cost.destination.solver,
            r.cost.destination.log2_leading,
            r.cost.exponent_ratio_log2,
            r.cost.log2_search.map_or("none".to_string(), |x| format!("2^{x:.1}")),
            r.cost.log2_transfer.map_or("none".to_string(), |x| format!("2^{x:.1}"))
        );
        for n in &r.notes {
            println!("        - {n}");
        }
    }
}

fn extension_inventory(
    curves: &[RegistryCurve],
    report: &TransferFile,
    name: Option<&str>,
) -> serde_json::Value {
    let included: std::collections::HashSet<&str> =
        report.curves.iter().map(|c| c.slug.as_str()).collect();
    let filter = name.map(str::to_lowercase);
    let mut rows: Vec<_> = curves.iter().filter_map(|c| {
        let (characteristic, degree, kind) = match &c.field {
            Field::Binary { n, .. } if *n > 1 => ("2".to_string(), *n, "binary"),
            Field::Extension { p, k } => (p.to_string(), *k, "extension"),
            _ => return None,
        };
        if let Some(query) = &filter {
            if !c.slug.to_lowercase().contains(query) && !c.standard_names.iter().any(|n| n.to_lowercase().contains(query)) {
                return None;
            }
        }
        Some(serde_json::json!({
            "slug": c.slug, "standard_names": c.standard_names, "family": c.family,
            "field_kind": kind, "characteristic": characteristic, "extension_degree": degree,
            "group_order": c.order.to_string(),
            "recorded_subgroup_order": c.subgroup_order.as_ref().map(ToString::to_string),
            "subgroup_order_status": if c.subgroup_order.is_some() { "registry_recorded_unverified" } else { "missing" },
            "included_in_transfer_report": included.contains(c.slug.as_str()),
            "security_review": "requires_concrete_assessment"
        }))
    }).collect();
    rows.sort_by(|a, b| a["slug"].as_str().cmp(&b["slug"].as_str()));
    serde_json::json!({
        "scope": "all parseable registry rows over extension fields; report membership does not establish report freshness",
        "note": "A field alone is not a security verdict; the Diem theorem is asymptotic and does not assign a fixed-instance work factor.",
        "count": rows.len(), "curves": rows
    })
}

fn run(cli: Cli) -> Result<bool, String> {
    match cli.cmd {
        Cmd::Build {
            registry,
            catalog,
            out,
            budget,
        } => {
            let text =
                std::fs::read_to_string(&registry).map_err(|e| format!("{registry}: {e}"))?;
            let manifest =
                std::fs::read_to_string(&catalog).map_err(|e| format!("{catalog}: {e}"))?;
            let file = TransferFile::build(&text, &manifest, CostModel::default(), budget)?;
            std::fs::write(&out, file.to_json()).map_err(|e| format!("{out}: {e}"))?;
            eprintln!(
                "wrote {} curves, {} routes to {out}; verdicts {:?}",
                file.summary.curves, file.summary.routes, file.summary.by_class
            );
            Ok(true)
        }
        Cmd::Check {
            registry,
            catalog,
            report,
        } => {
            let committed =
                std::fs::read_to_string(&report).map_err(|e| format!("{report}: {e}"))?;
            let parsed = load(&report)?;
            let text =
                std::fs::read_to_string(&registry).map_err(|e| format!("{registry}: {e}"))?;
            let manifest =
                std::fs::read_to_string(&catalog).map_err(|e| format!("{catalog}: {e}"))?;
            let fresh = TransferFile::build(
                &text,
                &manifest,
                parsed.cost_model.clone(),
                parsed.factor_budget,
            )?
            .to_json();
            if fresh == committed {
                eprintln!("{report} is current");
                return Ok(true);
            }
            eprintln!(
                "{report} is stale: run `cargo run --release --bin jacobian_transfers -- build --budget {}`",
                parsed.factor_budget
            );
            Ok(false)
        }
        Cmd::ExtensionFields {
            registry,
            report,
            name,
        } => {
            let text =
                std::fs::read_to_string(&registry).map_err(|e| format!("{registry}: {e}"))?;
            let curves = read_registry(&text)?;
            let file = load(&report)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&extension_inventory(&curves, &file, name.as_deref()))
                    .map_err(|e| e.to_string())?
            );
            Ok(true)
        }
        Cmd::Show { name, report, json } => {
            let file = load(&report)?;
            let c = file
                .find(&name)
                .ok_or_else(|| format!("{name}: not a registered curve name"))?;
            if json {
                println!("{}", serde_json::to_string_pretty(c).expect("serialisable"));
            } else {
                show(c);
            }
            Ok(true)
        }
        Cmd::Rank {
            top,
            class,
            report,
            json,
        } => {
            let file = load(&report)?;
            let mut rows: Vec<&CurveTransfers> = file
                .curves
                .iter()
                .filter(|c| class.as_ref().is_none_or(|k| &class_name(c.verdict) == k))
                .collect();
            rows.sort_by(|a, b| {
                best_ratio(b)
                    .partial_cmp(&best_ratio(a))
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| a.slug.cmp(&b.slug))
            });
            rows.truncate(top);
            if json {
                let v: Vec<_> = rows
                    .iter()
                    .map(|c| {
                        serde_json::json!({
                            "slug": c.slug,
                            "standard_names": c.standard_names,
                            "family": c.family,
                            "subgroup_bits": c.subgroup.bits,
                            "verdict": c.verdict,
                            "best_ratio_log2": best_ratio(c),
                            "best_route": c.best_route.map(|i| c.routes[i].label.clone()),
                        })
                    })
                    .collect();
                println!(
                    "{}",
                    serde_json::to_string_pretty(&v).expect("serialisable")
                );
                return Ok(true);
            }
            println!(
                "{:>4}  {:>7}  {:<18}  {:<10}  {:<50}  route",
                "rank", "ratio", "verdict", "family", "curve"
            );
            for (i, c) in rows.iter().enumerate() {
                let name = c
                    .standard_names
                    .first()
                    .cloned()
                    .unwrap_or_else(|| c.slug.clone());
                println!(
                    "{:>4}  {:>+7.1}  {:<18}  {:<10}  {:<50}  {}",
                    i + 1,
                    best_ratio(c),
                    class_name(c.verdict),
                    c.family,
                    name,
                    c.best_route
                        .map(|j| c.routes[j].label.as_str())
                        .unwrap_or("none carried")
                );
            }
            let adv = file
                .curves
                .iter()
                .filter(|c| c.verdict == RouteClass::FormulaAdvantage)
                .count();
            let marginal = file
                .curves
                .iter()
                .filter(|c| c.verdict == RouteClass::FormulaMarginal)
                .count();
            eprintln!(
                "{} curves, {adv} with a formula advantage, {marginal} marginal",
                file.curves.len()
            );
            Ok(true)
        }
        Cmd::Table { report, write_doc } => {
            let table = report::markdown_table(&load(&report)?);
            match write_doc {
                None => print!("{table}"),
                Some(path) => {
                    const MARKER: &str = "<!-- jacobian_transfers table -->\n";
                    let doc = std::fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
                    let head = doc
                        .split(MARKER)
                        .next()
                        .ok_or_else(|| format!("{path}: no table marker"))?;
                    if !doc.contains(MARKER) {
                        return Err(format!("{path}: no `{}` marker", MARKER.trim()));
                    }
                    std::fs::write(&path, format!("{head}{MARKER}{table}"))
                        .map_err(|e| format!("{path}: {e}"))?;
                    eprintln!("wrote the table into {path}");
                }
            }
            Ok(true)
        }
        Cmd::Chart { report, svg, tikz } => {
            let file = load(&report)?;
            if let Some(p) = svg {
                std::fs::write(&p, report::svg_chart(&file)).map_err(|e| format!("{p}: {e}"))?;
                eprintln!("wrote {p}");
            }
            if let Some(p) = tikz {
                std::fs::write(&p, report::tikz_chart(&file)).map_err(|e| format!("{p}: {e}"))?;
                eprintln!("wrote {p}");
            }
            Ok(true)
        }
    }
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_finds_both_extension_families_and_reports_membership() {
        let curves = read_registry(&std::fs::read_to_string(REGISTRY).unwrap()).unwrap();
        let report = load(OUT).unwrap();
        let result = extension_inventory(&curves, &report, None);
        let rows = result["curves"].as_array().unwrap();
        let expected = curves
            .iter()
            .filter(|c| match &c.field {
                Field::Binary { n, .. } => *n > 1,
                Field::Extension { .. } => true,
                _ => false,
            })
            .count();
        assert_eq!(rows.len(), expected);
        assert!(rows.iter().any(|r| r["field_kind"] == "binary"));
        assert!(rows.iter().any(|r| r["field_kind"] == "extension"));
        assert!(rows
            .iter()
            .any(|r| r["included_in_transfer_report"] == false));
        assert!(rows
            .iter()
            .any(|r| r["subgroup_order_status"] == "missing"
                && r["recorded_subgroup_order"].is_null()));
        let first = rows[0]["slug"].as_str().unwrap();
        let found = extension_inventory(&curves, &report, Some(&first.to_uppercase()));
        assert!(found["curves"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["slug"] == first));
    }
}
