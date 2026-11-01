//! Results table, CSV and SVG figures, built from the committed
//! certificates.  Nothing here recomputes a cost: every number is read from
//! a `summary.json`, so a figure can never disagree with its evidence.

use std::fs;
use std::path::Path;

use serde_json::Value;

struct Row {
    curve: String,
    kind: String,
    id: String,
    subgroup: String,
    construction: String,
    solver: String,
    verdict: String,
    lead: String,
    evaluation: String,
    log2_total: Option<f64>,
    ratio: Option<f64>,
    lower_bound: bool,
    carried: bool,
    registered: bool,
    crosscheck: Option<bool>,
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn read_rows(certs: &Path) -> Result<Vec<Row>, String> {
    let mut dirs: Vec<_> = fs::read_dir(certs)
        .map_err(|e| format!("{}: {e}", certs.display()))?
        .filter_map(Result::ok)
        .filter(|e| e.path().join("summary.json").exists())
        .map(|e| e.path())
        .collect();
    dirs.sort();
    let mut rows = Vec::new();
    for d in dirs {
        let text = fs::read_to_string(d.join("summary.json")).map_err(|e| e.to_string())?;
        let v: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
        let curve = v["curve"]["name"].as_str().unwrap_or("?").to_string();
        let registered = v["curve"]["registered"].as_bool().unwrap_or(false);
        let role = v["role"].as_str().unwrap_or("audit").to_string();
        for list in ["candidates", "controls"] {
            for c in v[list].as_array().into_iter().flatten() {
                let s = |k: &str| c[k].as_str().unwrap_or("-").to_string();
                rows.push(Row {
                    curve: if list == "controls" {
                        format!("{curve} (control)")
                    } else if role == "control" {
                        format!("{curve} [positive control]")
                    } else if role == "boundary-control" {
                        format!("{curve} [boundary control]")
                    } else {
                        curve.clone()
                    },
                    kind: s("transfer_type"),
                    id: s("id"),
                    subgroup: s("subgroup_status"),
                    construction: s("construction_status"),
                    solver: s("destination_solver"),
                    verdict: s("verdict"),
                    lead: s("lead_class"),
                    evaluation: s("evaluation"),
                    log2_total: c["log2_total"].as_f64(),
                    ratio: c["log2_cost_ratio"].as_f64(),
                    lower_bound: c["total_is_lower_bound"].as_bool().unwrap_or(false),
                    carried: c["carried"].as_bool().unwrap_or(false),
                    registered,
                    crosscheck: c["crosscheck_within_0_002_bits"].as_bool(),
                });
            }
        }
    }
    Ok(rows)
}

fn type_label(kind: &str) -> &'static str {
    match kind {
        "ghs_weil_descent" => "Weil/GHS descent",
        "isogeny_move" => "isogeny move",
        "genus_cover" => "genus-g cover",
        "pairing_transfer" => "pairing (MOV/Frey-Ruck)",
        _ => "other",
    }
}

fn fmt_ratio(r: &Row) -> String {
    match (r.ratio, r.carried) {
        (None, _) => "not priced".into(),
        (Some(x), false) => format!("not carried (priced for the record: {x:.1})"),
        (Some(x), true) => format!("{}{x:.1}", if r.lower_bound { ">= " } else { "" }),
    }
}

fn table(rows: &[Row]) -> String {
    let mut s = String::from(
        "| curve | transfer type | candidate | subgroup | map | log2(route/rho) | lead class | evaluation | verdict |\n|:--|:--|:--|:--|:--|--:|:--|:--|:--|\n",
    );
    for r in rows {
        s.push_str(&format!(
            "| {} | {} | `{}` | {} | {} | {} | {} | {} | {} |\n",
            r.curve,
            type_label(&r.kind),
            r.id,
            r.subgroup,
            r.construction,
            fmt_ratio(r),
            r.lead,
            r.evaluation.replace('_', " "),
            r.verdict
        ));
    }
    s
}

/// One row per curve and transfer type: the subgroup statuses seen, the
/// cheapest carried route (log2 route/rho; positive means it costs more than
/// rho), and the strongest verdict.
fn summary_table(rows: &[Row]) -> String {
    let mut s = String::from(
        "| curve | transfer type | candidates | subgroup statuses | cheapest carried route, log2(route/rho) | evaluated / excluded / not evaluable | verdict |\n|:--|:--|--:|:--|--:|:--|:--|\n",
    );
    let mut keys: Vec<(&str, &str)> = Vec::new();
    for r in rows {
        let k = (r.curve.as_str(), r.kind.as_str());
        if !keys.contains(&k) {
            keys.push(k);
        }
    }
    for (curve, kind) in keys {
        let group: Vec<&Row> = rows
            .iter()
            .filter(|r| r.curve == curve && r.kind == kind)
            .collect();
        let mut statuses: Vec<&str> = group.iter().map(|r| r.subgroup.as_str()).collect();
        statuses.sort_unstable();
        statuses.dedup();
        let best = group
            .iter()
            .filter(|r| r.carried)
            .filter_map(|r| r.ratio.map(|x| (x, r.lower_bound, r.id.as_str())))
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let best_txt = match best {
            Some((x, lb, id)) => format!("{}{x:.1} (`{id}`)", if lb { ">= " } else { "" }),
            None => "no carried route".to_string(),
        };
        let verdict = [
            "class-weak",
            "source-transfer-weak",
            "weak representative exists",
            "implementation-weak",
        ]
        .iter()
        .find(|v| group.iter().any(|r| r.verdict == **v))
        .copied()
        .unwrap_or("no weakness found in scope");
        let n_eval = |st: &str| group.iter().filter(|r| r.evaluation == st).count();
        s.push_str(&format!(
            "| {curve} | {} | {} | {} | {best_txt} | {} / {} / {} | {verdict} |\n",
            type_label(kind),
            group.len(),
            statuses.join(", "),
            n_eval("evaluated"),
            n_eval("excluded"),
            n_eval("not_evaluable")
        ));
    }
    s
}

fn csv(rows: &[Row]) -> String {
    let mut s = String::from(
        "curve,transfer_type,candidate,subgroup_status,map_status,carried,destination_solver,log2_total,log2_cost_ratio_to_rho,total_is_lower_bound,lead_class,evaluation,verdict\n",
    );
    for r in rows {
        let f = |x: Option<f64>| x.map_or(String::new(), |v| format!("{v}"));
        s.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            r.curve,
            r.kind,
            r.id,
            r.subgroup,
            r.construction.replace(',', ";"),
            r.carried,
            r.solver,
            f(r.log2_total),
            f(r.ratio),
            r.lower_bound,
            r.lead,
            r.evaluation,
            r.verdict
        ));
    }
    s
}

fn colour(kind: &str) -> &'static str {
    match kind {
        "ghs_weil_descent" => "#0072B2",
        "isogeny_move" => "#009E73",
        "genus_cover" => "#D55E00",
        "pairing_transfer" => "#CC79A7",
        _ => "#777777",
    }
}

fn marker(x: f64, y: f64, status: &str, fill: &str) -> String {
    match status {
        // Executed and verified: filled circle.
        "verified" => format!("<circle cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"4.5\" fill=\"{fill}\" stroke=\"{fill}\"/>"),
        // Refuted: filled square (the subgroup is lost).
        "refuted" => format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"8\" height=\"8\" fill=\"{fill}\" stroke=\"{fill}\"/>", x - 4.0, y - 4.0),
        // Arithmetic argument, not executed: hollow diamond.
        "arithmetic" => format!(
            "<path d=\"M{:.1} {:.1} L{:.1} {:.1} L{:.1} {:.1} L{:.1} {:.1} Z\" fill=\"#ffffff\" stroke=\"{fill}\" stroke-width=\"1.8\"/>",
            x, y - 5.5, x + 5.5, y, x, y + 5.5, x - 5.5, y
        ),
        // Not evaluated / not applicable: hollow circle.
        _ => format!("<circle cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"4.5\" fill=\"#ffffff\" stroke=\"{fill}\" stroke-width=\"1.8\"/>"),
    }
}

fn chart(rows: &[&Row], title: &str, subtitle: &str) -> String {
    let left = 330.0;
    let right_pad = 130.0;
    let plot_w = 640.0;
    let row_h = 17.0;
    let top = 116.0;
    let ratios: Vec<f64> = rows
        .iter()
        .filter(|r| r.carried)
        .filter_map(|r| r.ratio)
        .collect();
    let xmin = ratios.iter().copied().fold(0.0f64, f64::min).min(-5.0);
    let xmax_raw = ratios.iter().copied().fold(0.0f64, f64::max);
    let step = if xmax_raw > 300.0 {
        100.0
    } else if xmax_raw > 120.0 {
        50.0
    } else {
        20.0
    };
    let xmax = (xmax_raw / step).ceil() * step;
    let xmin = (xmin / step).floor() * step;
    let sx = |v: f64| left + (v - xmin) / (xmax - xmin) * plot_w;
    let h = top + rows.len() as f64 * row_h + 78.0;
    let w = left + plot_w + right_pad;
    let mut s = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w:.0}\" height=\"{h:.0}\" viewBox=\"0 0 {w:.0} {h:.0}\" font-family=\"Helvetica, Arial, sans-serif\" font-size=\"11\">\n<title>{}</title>\n<rect width=\"100%\" height=\"100%\" fill=\"#ffffff\"/>\n",
        esc(title)
    );
    s.push_str(&format!(
        "<text x=\"16\" y=\"24\" font-size=\"15\" font-weight=\"bold\" fill=\"#111\">{}</text>\n<text x=\"16\" y=\"42\" fill=\"#444\">{}</text>\n",
        esc(title),
        esc(subtitle)
    ));
    // Legend.
    let mut lx = 16.0;
    for (k, name) in [
        ("ghs_weil_descent", "Weil/GHS"),
        ("isogeny_move", "isogeny"),
        ("genus_cover", "genus-g cover"),
        ("pairing_transfer", "pairing"),
    ] {
        s.push_str(&format!(
            "<rect x=\"{lx}\" y=\"52\" width=\"10\" height=\"10\" fill=\"{}\"/><text x=\"{}\" y=\"61\" fill=\"#222\">{name}</text>\n",
            colour(k),
            lx + 14.0
        ));
        lx += 100.0;
    }
    let mut mx = 16.0;
    for (st, name) in [
        ("verified", "map executed, subgroup verified"),
        ("arithmetic", "arithmetic argument, not executed"),
        ("not_evaluated", "subgroup not evaluated"),
    ] {
        s.push_str(&marker(mx + 5.0, 78.0, st, "#444444"));
        s.push_str(&format!(
            "<text x=\"{}\" y=\"82\" fill=\"#222\">{name}</text>\n",
            mx + 14.0
        ));
        mx += 200.0;
    }
    s.push_str(&format!(
        "<text x=\"{mx}\" y=\"82\" fill=\"#222\">no marker: route does not carry the subgroup</text>\n"
    ));
    // Grid and axis.
    let axis_y = top + rows.len() as f64 * row_h + 4.0;
    let mut v = xmin;
    while v <= xmax + 1e-9 {
        let x = sx(v);
        s.push_str(&format!(
            "<line x1=\"{x:.1}\" y1=\"{:.1}\" x2=\"{x:.1}\" y2=\"{axis_y:.1}\" stroke=\"#e2e2e2\"/><text x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\" fill=\"#444\">{v:.0}</text>\n",
            top - 6.0,
            axis_y + 14.0
        ));
        v += step;
    }
    let x0 = sx(0.0);
    s.push_str(&format!(
        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"#fff3e0\"/>\n<line x1=\"{x0:.1}\" y1=\"{:.1}\" x2=\"{x0:.1}\" y2=\"{axis_y:.1}\" stroke=\"#b00020\" stroke-width=\"1.5\"/>\n<text x=\"{x0:.1}\" y=\"{:.1}\" text-anchor=\"middle\" fill=\"#b00020\" font-weight=\"bold\">parity with the matched rho</text>\n",
        sx(xmin),
        top - 6.0,
        x0 - sx(xmin),
        axis_y - top + 6.0,
        top - 8.0,
        top - 14.0
    ));
    s.push_str(&format!(
        "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" fill=\"#222\">log2( route cost / matched rho cost )   (right of the red line: the route costs more than rho)</text>\n",
        left + plot_w / 2.0,
        axis_y + 32.0
    ));
    // Rows.
    let mut last_curve = String::new();
    for (i, r) in rows.iter().enumerate() {
        let y = top + (i as f64 + 0.5) * row_h;
        if r.curve != last_curve {
            s.push_str(&format!(
                "<line x1=\"8\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#bbb\"/>\n",
                y - row_h / 2.0,
                left + plot_w,
                y - row_h / 2.0
            ));
            last_curve = r.curve.clone();
        }
        let name = format!("{} · {}", r.curve, r.id);
        let name = if name.chars().count() > 50 {
            format!("{}…", name.chars().take(49).collect::<String>())
        } else {
            name
        };
        s.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" fill=\"#222\">{}</text>\n",
            left - 8.0,
            y + 4.0,
            esc(&name)
        ));
        let c = colour(&r.kind);
        match r.ratio.filter(|_| r.carried) {
            Some(ratio) => {
                let x = sx(ratio.clamp(xmin, xmax));
                s.push_str(&marker(x, y, &r.subgroup, c));
                if r.lower_bound {
                    s.push_str(&format!(
                        "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"{c}\" font-size=\"10\">≥ (lower bound)</text>\n",
                        x + 9.0,
                        y + 4.0
                    ));
                }
            }
            None => s.push_str(&format!(
                "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"#777\" font-size=\"10\">{}</text>\n",
                x0 + 6.0,
                y + 4.0,
                if r.ratio.is_some() && !r.carried {
                    "not carried: the route does not carry the subgroup (no attack cost)"
                } else {
                    "not priced"
                }
            )),
        }
    }
    s.push_str("</svg>\n");
    s
}

fn count(rows: &[Row], kind: &str, status: &str) -> usize {
    rows.iter()
        .filter(|r| r.kind == kind && r.subgroup == status && !r.curve.contains("(control)"))
        .count()
}

fn pipeline(rows: &[Row]) -> String {
    let w = 1180.0;
    let h = 470.0;
    let mut s = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\" font-family=\"Helvetica, Arial, sans-serif\" font-size=\"12\">\n<title>transfer-audit pipeline</title>\n<rect width=\"100%\" height=\"100%\" fill=\"#ffffff\"/>\n<defs><marker id=\"a\" markerWidth=\"8\" markerHeight=\"8\" refX=\"7\" refY=\"4\" orient=\"auto\"><path d=\"M0 0 L8 4 L0 8 Z\" fill=\"#444\"/></marker></defs>\n<text x=\"16\" y=\"24\" font-size=\"15\" font-weight=\"bold\" fill=\"#111\">transfer-audit: from a curve to a certified, priced verdict</text>\n<text x=\"16\" y=\"42\" fill=\"#444\">counts are subgroup-preservation statuses over the audited curves in this run (toys included); every box is a module of src/bin/transfer_audit/</text>\n"
    );
    let boxx = |x: f64,
                y: f64,
                w2: f64,
                h2: f64,
                title: &str,
                lines: &[String],
                fill: &str,
                stroke: &str| {
        let mut b = format!(
            "<rect x=\"{x}\" y=\"{y}\" width=\"{w2}\" height=\"{h2}\" rx=\"6\" fill=\"{fill}\" stroke=\"{stroke}\" stroke-width=\"1.5\"/>\n<text x=\"{}\" y=\"{}\" font-weight=\"bold\" fill=\"#111\">{}</text>\n",
            x + 10.0,
            y + 20.0,
            esc(title)
        );
        for (i, l) in lines.iter().enumerate() {
            b.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" fill=\"#333\" font-size=\"11\">{}</text>\n",
                x + 10.0,
                y + 38.0 + 14.0 * i as f64,
                esc(l)
            ));
        }
        b
    };
    let arrow = |x1: f64, y1: f64, x2: f64, y2: f64| {
        format!("<line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" stroke=\"#444\" stroke-width=\"1.5\" marker-end=\"url(#a)\"/>\n")
    };
    s.push_str(&boxx(
        16.0,
        70.0,
        210.0,
        86.0,
        "1. curve + generator + r",
        &[
            "registry (ICV1 slug) or toy spec".into(),
            "spec.rs: validate; else INVALID_INSTANCE".into(),
            "field, curve, [r]G = O, r prime".into(),
        ],
        "#f4f4f4",
        "#888",
    ));
    s.push_str(&arrow(226.0, 113.0, 262.0, 113.0));
    s.push_str(&boxx(
        262.0,
        70.0,
        330.0,
        86.0,
        "2. repository traits + baseline",
        &[
            "curve_traits::compute: Δ, d_K, conductor,".into(),
            "volcano levels, subfield, embedding degree".into(),
            "matched rho: sqrt(pi r/(2c)), c = class size (negation incl.)".into(),
        ],
        "#f4f4f4",
        "#888",
    ));
    let families = [
        (
            "ghs_weil_descent",
            "A. Weil/GHS descent",
            "ghs.rs",
            "per divisor l of n; trace executed",
            "#0072B2",
        ),
        (
            "isogeny_move",
            "B. isogeny moves",
            "isogeny.rs",
            "invariants, volcano, GHS extension",
            "#009E73",
        ),
        (
            "genus_cover",
            "C. genus-g covers",
            "cover.rs",
            "g=3 binary executed; g=2..8 priced",
            "#D55E00",
        ),
        (
            "pairing_transfer",
            "D. pairing",
            "pairing.rs",
            "embedding degree, L[1/3] price",
            "#CC79A7",
        ),
    ];
    for (i, (kind, title, file, what, col)) in families.iter().enumerate() {
        let y = 190.0 + 62.0 * i as f64;
        let lines = vec![
            format!("{file}: {what}"),
            format!(
                "verified {} | arithmetic {} | refuted {} | other {}",
                count(rows, kind, "verified"),
                count(rows, kind, "arithmetic"),
                count(rows, kind, "refuted"),
                count(rows, kind, "not_evaluated") + count(rows, kind, "not_applicable")
            ),
        ];
        s.push_str(&boxx(262.0, y, 330.0, 54.0, title, &lines, "#ffffff", col));
        s.push_str(&arrow(
            427.0,
            if i == 0 { 156.0 } else { y - 8.0 },
            427.0,
            y,
        ));
    }
    s.push_str(&arrow(592.0, 300.0, 640.0, 300.0));
    s.push_str(&boxx(
        640.0,
        190.0,
        524.0,
        240.0,
        "3. certificate per candidate (certificate.rs)",
        &[
            "map: type, degrees, field, executed | arithmetic, not executed | not evaluated".into(),
            "subgroup preservation (subgroup.rs): verified | arithmetic |".into(),
            "  not_evaluated | refuted; image order r; kernel ∩ <G> = 1;".into(),
            "  additivity and scalar equivariance on seeded samples".into(),
            "cost (cost.rs, frozen CostModel): map construction, transfer".into(),
            "  evaluation, relation collection, linear algebra, descent/".into(),
            "  back-mapping; measured | modeled | not_evaluated per cell".into(),
            "verdict (verdict.rs): class-weak | weak representative exists |".into(),
            "  source-transfer-weak | implementation-weak | no weakness".into(),
            "  found in scope; formula leads recorded, never promoted".into(),
            "evaluation: evaluated | excluded | not evaluable (a field,".into(),
            "  not a verdict label); margin 8 bits before a weakness verdict".into(),
            "open obligations; references".into(),
        ],
        "#ffffff",
        "#111",
    ));
    s.push_str(&boxx(
        640.0,
        70.0,
        524.0,
        86.0,
        "controls (controls.rs)",
        &[
            "F10 synthetic formula advantage is flagged".into(),
            "F6 subfield subgroup survives the trace".into(),
            "F9 measured IC vs rho in odd characteristic".into(),
        ],
        "#fff8e1",
        "#b58900",
    ));
    s.push_str(&arrow(902.0, 156.0, 902.0, 190.0));
    s.push_str("</svg>\n");
    s
}

pub fn build(certs: &Path, out: &Path) -> Result<(), String> {
    let rows = read_rows(certs)?;
    fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let deployed: Vec<&Row> = rows
        .iter()
        .filter(|r| {
            r.registered
                && !r.curve.contains("(control)")
                && !r.curve.contains("[positive control]")
                && !r.curve.contains("[boundary control]")
        })
        .collect();
    let pos_controls: Vec<&Row> = rows
        .iter()
        .filter(|r| r.curve.contains("control]"))
        .collect();
    let toy: Vec<&Row> = rows.iter().filter(|r| !r.registered).collect();
    fs::write(out.join("RESULTS_TABLE.md"), table(&rows)).map_err(|e| e.to_string())?;
    fs::write(out.join("RESULTS_SUMMARY.md"), summary_table(&rows)).map_err(|e| e.to_string())?;
    fs::write(out.join("cost_ratio.csv"), csv(&rows)).map_err(|e| e.to_string())?;
    fs::write(
        out.join("cost_ratio.svg"),
        chart(
            &deployed,
            "Cost of each candidate transfer relative to the matched Pollard rho (registered curves)",
            "unit: log2(route/rho), frozen formula-level model; hollow = not executed; the red line is parity",
        ),
    )
    .map_err(|e| e.to_string())?;
    fs::write(
        out.join("cost_ratio_toy.svg"),
        chart(
            &toy,
            "Same, for the toy Koblitz curves (n = 15, 21, 35) where the trace branch is executed",
            "toy sizes: the ratios say nothing about deployed security; controls are subfield subgroups",
        ),
    )
    .map_err(|e| e.to_string())?;
    fs::write(
        out.join("cost_ratio_controls.svg"),
        chart(
            &pos_controls,
            "Pairing-friendly controls: positive (mnt/mnt1 k = 3, BLS12-381 k = 12) and boundary (BN254, BN318, BLS12-638, BN350, BN446)",
            "the pairing route is the intended attack on mnt/mnt1; the boundary curves sit on both sides of the 8-bit margin; relative to rho on E, not absolute security",
        ),
    )
    .map_err(|e| e.to_string())?;
    fs::write(out.join("pipeline.svg"), pipeline(&rows)).map_err(|e| e.to_string())?;
    let bad: Vec<&str> = rows
        .iter()
        .filter(|r| r.crosscheck == Some(false))
        .map(|r| r.id.as_str())
        .collect();
    println!(
        "report: {} rows -> RESULTS_TABLE.md, RESULTS_SUMMARY.md, cost_ratio.csv, cost_ratio.svg, cost_ratio_toy.svg, cost_ratio_controls.svg, pipeline.svg{}",
        rows.len(),
        if bad.is_empty() { String::new() } else { format!("; cross-check failures: {bad:?}") }
    );
    Ok(())
}
