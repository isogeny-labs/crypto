//! Native report generation from native jq reductions; no experiment execution.
use std::{fmt::Write as _, fs, path::Path};
struct Row {
    g: usize,
    p: u64,
    f: usize,
    lo: f64,
    med: f64,
    hi: f64,
    aa: f64,
}
fn main() {
    let root = Path::new("research/hec_anchor_relations_20261114/heldout-evidence");
    let rows: Vec<Row> = fs::read_to_string(root.join("fixtures.tsv"))
        .unwrap()
        .lines()
        .map(|l| {
            let v: Vec<_> = l.split('\t').collect();
            assert_eq!(v.len(), 7);
            Row {
                g: v[0].parse().unwrap(),
                p: v[1].parse().unwrap(),
                f: v[2].parse().unwrap(),
                lo: v[3].parse().unwrap(),
                med: v[4].parse().unwrap(),
                hi: v[5].parse().unwrap(),
                aa: v[6].parse().unwrap(),
            }
        })
        .collect();
    assert_eq!(rows.len(), 20);
    assert!(rows
        .iter()
        .all(|r| r.lo > 0.0 && r.lo <= r.med && r.med <= r.hi));
    let out = Path::new("benchmarks/20261114-hec-anchor-forest");
    fs::create_dir_all(out).unwrap();
    let lower = rows.iter().map(|r| r.lo).fold(0.25, f64::min) * 0.8;
    let upper = rows.iter().map(|r| r.hi * 1.2).fold(12.0, f64::max);
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1000\" height=\"300\" viewBox=\"0 0 1000 300\" font-family=\"sans-serif\" font-size=\"11\"><rect width=\"1000\" height=\"300\" fill=\"white\"/><text x=\"40\" y=\"18\" font-size=\"15\">Held-out cold total: fastest of twelve controls / compact, per target</text>");
    let y = |v: f64| 55.0 + 185.0 * (upper.ln() - v.ln()) / (upper.ln() - lower.ln());
    for (panel, g) in [2, 3, 4].into_iter().enumerate() {
        let data: Vec<_> = rows.iter().filter(|r| r.g == g).collect();
        let p0 = data.iter().map(|r| r.p).min().unwrap() as f64;
        let p1 = data.iter().map(|r| r.p).max().unwrap() as f64;
        let left = 40.0 + panel as f64 * 330.0;
        let x = |p: u64| left + 38.0 + 240.0 * ((p as f64).ln() - p0.ln()) / (p1.ln() - p0.ln());
        write!(
            svg,
            "<text x=\"{:.0}\" y=\"39\" font-weight=\"bold\">Genus {g}</text>",
            left + 110.0
        )
        .unwrap();
        for value in [0.25, 0.5, 1.0, 2.0, 4.0, 10.0] {
            if value < lower || value > upper {
                continue;
            }
            let yy = y(value);
            let col = if value == 10.0 {
                "#a44332"
            } else if value == 1.0 {
                "#555555"
            } else {
                "#e1e7e9"
            };
            write!(svg,"<line x1=\"{:.1}\" y1=\"{yy:.1}\" x2=\"{:.1}\" y2=\"{yy:.1}\" stroke=\"{col}\"/><text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">{value}</text>",left+30.0,left+288.0,left+26.0,yy+4.0).unwrap();
        }
        for f in 0..2 {
            let color = if f == 0 { "#167f73" } else { "#b66b30" };
            let data: Vec<_> = data.iter().filter(|r| r.f == f).collect();
            let points = data
                .iter()
                .map(|r| {
                    format!(
                        "{:.1},{:.1}",
                        x(r.p) + if f == 0 { -2.0 } else { 2.0 },
                        y(r.med)
                    )
                })
                .collect::<Vec<_>>()
                .join(" ");
            write!(svg,"<polyline points=\"{points}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"1.7\"/>").unwrap();
            for r in data {
                let xx = x(r.p) + if f == 0 { -2.0 } else { 2.0 };
                write!(svg,"<line x1=\"{xx:.1}\" y1=\"{:.1}\" x2=\"{xx:.1}\" y2=\"{:.1}\" stroke=\"{color}\"/><circle cx=\"{xx:.1}\" cy=\"{:.1}\" r=\"3.5\" fill=\"{color}\"/>",y(r.lo),y(r.hi),y(r.med)).unwrap();
            }
        }
        for r in data.iter().filter(|r| r.f == 0) {
            write!(
                svg,
                "<text x=\"{:.1}\" y=\"258\" text-anchor=\"middle\">{}</text>",
                x(r.p),
                r.p
            )
            .unwrap();
        }
    }
    svg.push_str("<text x=\"40\" y=\"286\" fill=\"#167f73\">Family 0</text><text x=\"130\" y=\"286\" fill=\"#b66b30\">Family 1</text><text x=\"245\" y=\"286\">Dots: five-target medians; bars: min/max. Both axes logarithmic. L0 observations.</text></svg>\n");
    fs::write(out.join("heldout.svg"), svg).unwrap();
    let min = rows.iter().map(|r| r.med).fold(f64::INFINITY, f64::min);
    let max = rows.iter().map(|r| r.med).fold(0.0, f64::max);
    let target_max = rows.iter().map(|r| r.hi).fold(0.0, f64::max);
    let aa = rows.iter().map(|r| r.aa).fold(0.0, f64::max);
    let mut typ = format!(
        r##"#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: false, leading: 0.5em)
#set heading(numbering: none)
= Held-out hyperelliptic anchor relations
2026-11-15 UTC | source `2404e34dca` | record `20261115T0700Z-hec-heldout-scaling`

#text("[pcc:unverified]") Producer assertions pending independent scientific replay.

All 20 attempted dense-curve fixtures were selected. All 100 single-target group gates and 1,500 interleaved timed cells completed and passed their local answer checks. Curve-level medians of paired fastest-control/compact ratios range *{min:.2}x to {max:.2}x*; the largest individual target ratio is {target_max:.2}x. This screen does not establish the mathematical 10x objective or novelty.

#image("heldout.svg", width: 100%)

#table(columns: (0.35fr, 0.8fr, 0.45fr, 0.65fr, 0.65fr, 0.65fr, 0.75fr), align: (left,right,right,right,right,right,right), inset: 2.4pt, stroke: 0.3pt + rgb("#cad5db"),
table.header([*g*],[*p*],[*family*],[*min*],[*median*],[*max*],[*A/A max*]),
"##
    );
    for r in &rows {
        writeln!(
            typ,
            "[{}],[{}],[{}],[{:.2}x],[{:.2}x],[{:.2}x],[{:.2}x],",
            r.g, r.p, r.f, r.lo, r.med, r.hi, r.aa
        )
        .unwrap();
    }
    typ.push_str(")\n\nEach ratio is a separate single-target cold comparison; no cross-target setup is amortized. The minimum of twelve control times is taken separately for each target. This conservative exploratory baseline selection can bias the ratio downward. Table medians are summaries of five paired ratios, not ratios of pooled throughput.\n\n#pagebreak()\n= Method, limits and structural decision\n\n");
    write!(typ,r##"*Cohort.* Two seeded dense-coefficient families at each field size, monic squarefree odd-degree models of genus 2, 3 and 4. Selection tried at most 128 curves and eight initial degree-one places, accepting the first certified prime-order generator meeting the frozen bounds. No solver timing entered selection. Exact f, generator, subgroup order, attempt and target identity are retained in raw receipts. A generator order is not a full Jacobian-order assertion.

*Targets and controls.* Five independently seeded nonzero scalars per curve; the same public target is given to each arm. Controls comprise native/table, branch count 1/64, Fast/Hybrid, and reference/direct oracle for the native arm: twelve controls. Original orbit is secondary, compact primary. An additional independently cold table/Fast/one-branch A/A arm checks host variability and identical workload counts.

*Correctness.* All 100 group-gated compact solves compare every generated signed orbit against the reference, check 265,559 raw principal identities and 4,449 projected anchor identities, and verify the answer. All 1,500 timed cells have the expected scalar and target; every A/A pair agrees on catalogue size, trials, relation count, solve retries, scalar and target. Native receipt collection validates the frozen build ID, arguments and fixture SHA256 before native jq reduction. These remain producer self-checks, not signed independent replay.

*Timing.* One sequential fresh process per cell, L0 macOS ARM. Method clocks charge all cold catalogue/tables, anchors, inverses, descents, attempts, forest, solver and final group verification. Process launch, fixture parsing/validation and construction of the known-answer target are excluded equally. A further harness check is timed separately. A/A spread reaches *{aa:.2}x*, so no statistical confidence or isolation claim follows. Per-target raw values, all controls and diagnostic repeats remain available.

*Caps and custody.* Selection child cap 30s and cumulative 600s; gate child cap 60s and cumulative 1,800s; timed child cap 30s and cumulative 1,800s. Existing candidate limits and two-million control trials remain fixed. No selection failure, timeout or unstarted cell occurred. Terminal receipts are immutable and resumption rejects a changed build, argument or input. No cloud worker was used. Scientific execution, collection and reduction use Rust/native jq; the historical shared record wrapper only logged initial selection bookkeeping.

*Observed cost structure.* The phase fractions below are medians of per-solve fractions across the two families and five targets at the largest field of each genus. They are descriptive summaries; sums of independently computed medians need not equal one.

#table(columns: (0.5fr,0.8fr,1fr,1fr,1fr,1fr), align: (left,right,right,right,right,right), inset: 4pt,
table.header([*g*],[*p*],[*relation*],[*forest*],[*solve*],[*setup*]),
"##).unwrap();
    for line in fs::read_to_string(root.join("phases.tsv")).unwrap().lines() {
        let v: Vec<f64> = line.split('\t').map(|s| s.parse().unwrap()).collect();
        let g = v[0] as usize;
        let p = v[1] as u64;
        if !matches!((g, p), (2, 65537) | (3, 1009) | (4, 127)) {
            continue;
        }
        writeln!(
            typ,
            "[{g}],[{p}],[{:.1}%],[{:.1}%],[{:.1}%],[{:.1}%],",
            100.0 * v[2],
            100.0 * v[3],
            100.0 * v[4],
            100.0 * v[5]
        )
        .unwrap();
    }
    typ.push_str(")\n\n*Decision.* Stop equivalent arithmetic rewrites. Investigate the structural cost of H-wide affine potentials and dense anchor elimination: genus-2 forest share grows, and genus-3 solve share grows as H increases. A sparse short-cycle representation and an appropriate sparse anchor solve need a fresh cost argument, literature screen and correctness protocol before implementation. No larger-size exponent or mathematical breakthrough is inferred from this ladder.\n\n*Sources and reproduction.* `research/hec_anchor_relations_20261114/HELDOUT-PROTOCOL.md`, `heldout.rs`, `heldout_collect.rs`, `heldout_summary.jq`, `heldout_render.rs`, and the frozen raw archive. Prior art and assumptions are in the principal/anchor protocols (GTTD signed cycles and principal-divisor geometric IC). External fastest-tool comparison and independent controlled replay remain outstanding. No benchmark-index or leaderboard promotion.\n");
    fs::write(out.join("HELDOUT-REPORT.typ"), typ).unwrap();
}
