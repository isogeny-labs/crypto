use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Clone, Copy)]
struct DepthRow {
    depth: u32,
    leaves: f64,
    rank: f64,
    semantic: f64,
    support: f64,
}

fn jq(path: &Path, filter: &str) -> String {
    let output = Command::new("jq")
        .args(["-r", filter])
        .arg(path)
        .output()
        .expect("failed to execute jq");
    assert!(output.status.success(), "jq query failed");
    String::from_utf8(output.stdout).expect("jq output was not UTF-8")
}

fn numbers(line: &str) -> Vec<f64> {
    line.split_whitespace()
        .map(|value| value.parse().expect("invalid numeric field"))
        .collect()
}

fn x_for_depth(depth: u32) -> f64 {
    105.0 + f64::from(depth - 1) * 104.0
}

fn y_for_normalized(value: f64) -> f64 {
    let min = 0.1_f64.log10();
    let max = 4.0_f64.log10();
    590.0 - (value.log10() - min) / (max - min) * 430.0
}

fn xml_text(svg: &mut String, x: f64, y: f64, size: u32, fill: &str, text: &str) {
    writeln!(
        svg,
        r#"<text x="{x:.1}" y="{y:.1}" font-family="DejaVu Sans, sans-serif" font-size="{size}" fill="{fill}">{text}</text>"#
    )
    .unwrap();
}

fn main() {
    let mut args = env::args().skip(1);
    let analysis = args
        .next()
        .expect("usage: generate_visual ANALYSIS SVG TSV");
    let svg_path = args
        .next()
        .expect("usage: generate_visual ANALYSIS SVG TSV");
    let tsv_path = args
        .next()
        .expect("usage: generate_visual ANALYSIS SVG TSV");
    assert!(args.next().is_none(), "unexpected extra argument");
    let analysis = Path::new(&analysis);

    let depth_text = jq(
        analysis,
        ".depths[] | [.depth,.signed_leaves,.leaf_rank_gf2,.semantic_entries,.root_support] | @tsv",
    );
    let depths: Vec<_> = depth_text
        .lines()
        .map(|line| {
            let row = numbers(line);
            assert_eq!(row.len(), 5);
            DepthRow {
                depth: row[0] as u32,
                leaves: row[1],
                rank: row[2],
                semantic: row[3],
                support: row[4],
            }
        })
        .collect();
    assert_eq!(depths.len(), 6);

    let projection = numbers(&jq(
        analysis,
        ".projection | [.optimistic_probes,.panel_fe_tenths[0],.panel_fe_tenths[1],.panel_fe_tenths[2],.probe_limit,.field_equivalent_limit_tenths] | @tsv",
    ));
    assert_eq!(projection.len(), 6);
    let probe_ratio = projection[0] / projection[4];
    let fe_limit = projection[5] / 10.0;
    let budget_ratios = [
        probe_ratio,
        projection[1] / 10.0 / fe_limit,
        projection[2] / 10.0 / fe_limit,
        projection[3] / 10.0 / fe_limit,
    ];
    let slopes = numbers(&jq(
        analysis,
        "[.growth.leaf_rank_gf2.slope_3_to_6,.growth.semantic_entries.slope_3_to_6,.growth.root_support.slope_3_to_6] | @tsv",
    ));
    assert_eq!(slopes.len(), 3);
    let verdict = jq(analysis, ".promotion.verdict").trim().to_owned();

    let mut tsv = String::from(
        "depth\tsigned_leaves\tleaf_rank\tsemantic_entries\troot_support\trank_over_l2\tsemantic_over_l2\tsupport_over_l2\n",
    );
    for row in &depths {
        let square = row.leaves * row.leaves;
        writeln!(
            tsv,
            "{}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\t{:.9}\t{:.9}\t{:.9}",
            row.depth,
            row.leaves,
            row.rank,
            row.semantic,
            row.support,
            row.rank / square,
            row.semantic / square,
            row.support / square
        )
        .unwrap();
    }
    writeln!(tsv, "metric\tratio_to_limit").unwrap();
    for (label, ratio) in [
        "support_probe_floor",
        "field_panel_0.8_50",
        "field_panel_1.0_100",
        "field_panel_1.0_200",
    ]
    .into_iter()
    .zip(budget_ratios)
    {
        writeln!(tsv, "{label}\t{ratio:.9}").unwrap();
    }
    writeln!(
        tsv,
        "slopes_3_to_6\t{:.9}\t{:.9}\t{:.9}",
        slopes[0], slopes[1], slopes[2]
    )
    .unwrap();
    fs::write(tsv_path, tsv).expect("failed to write TSV");

    let mut svg = String::new();
    svg.push_str(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="760" viewBox="0 0 1200 760">
<rect width="1200" height="760" fill="#f8fafc"/>
<rect x="42" y="76" width="650" height="570" rx="14" fill="#ffffff" stroke="#cbd5e1"/>
<rect x="718" y="76" width="440" height="570" rx="14" fill="#ffffff" stroke="#cbd5e1"/>
"##,
    );
    xml_text(
        &mut svg,
        42.0,
        42.0,
        27,
        "#0f172a",
        "Exact oriented pair transducer: verified null result",
    );
    xml_text(
        &mut svg,
        42.0,
        67.0,
        14,
        "#475569",
        "Depths 1–6 · no cross-target amortization · independent direct-state replay",
    );
    xml_text(
        &mut svg,
        66.0,
        112.0,
        18,
        "#0f172a",
        "A. State growth normalized by signed leaves²",
    );
    xml_text(
        &mut svg,
        742.0,
        112.0,
        18,
        "#0f172a",
        "B. Cold m=7 cost / frozen limit",
    );

    for tick in [0.1_f64, 0.2, 0.5, 1.0, 2.0, 4.0] {
        let y = y_for_normalized(tick);
        writeln!(
            svg,
            r##"<line x1="92" y1="{y:.1}" x2="635" y2="{y:.1}" stroke="#e2e8f0" stroke-width="1"{} />"##,
            if (tick - 1.0).abs() < f64::EPSILON {
                r#" stroke-dasharray="7 5""#
            } else {
                ""
            }
        )
        .unwrap();
        xml_text(&mut svg, 54.0, y + 5.0, 12, "#64748b", &format!("{tick}"));
    }
    xml_text(
        &mut svg,
        500.0,
        y_for_normalized(1.0) - 8.0,
        11,
        "#64748b",
        "constant ⇒ quadratic",
    );
    for row in &depths {
        let x = x_for_depth(row.depth);
        writeln!(
            svg,
            r##"<line x1="{x:.1}" y1="154" x2="{x:.1}" y2="590" stroke="#f1f5f9"/>"##
        )
        .unwrap();
        xml_text(
            &mut svg,
            x - 4.0,
            614.0,
            12,
            "#475569",
            &row.depth.to_string(),
        );
    }
    xml_text(&mut svg, 325.0, 635.0, 12, "#64748b", "Dickson depth");

    let series = [
        ("leaf rank", "#2563eb", 0_usize),
        ("semantic entries", "#dc2626", 1_usize),
        ("root support", "#059669", 2_usize),
    ];
    for (label, color, selector) in series {
        let mut points = String::new();
        for row in &depths {
            let square = row.leaves * row.leaves;
            let value = match selector {
                0 => row.rank / square,
                1 => row.semantic / square,
                _ => row.support / square,
            };
            write!(
                points,
                "{:.1},{:.1} ",
                x_for_depth(row.depth),
                y_for_normalized(value)
            )
            .unwrap();
        }
        writeln!(
            svg,
            r#"<polyline points="{points}" fill="none" stroke="{color}" stroke-width="3" stroke-linejoin="round" stroke-linecap="round"/>"#
        )
        .unwrap();
        for row in &depths {
            let square = row.leaves * row.leaves;
            let value = match selector {
                0 => row.rank / square,
                1 => row.semantic / square,
                _ => row.support / square,
            };
            writeln!(
                svg,
                r#"<circle cx="{:.1}" cy="{:.1}" r="4.5" fill="{color}"/>"#,
                x_for_depth(row.depth),
                y_for_normalized(value)
            )
            .unwrap();
        }
        let legend_y = 139.0;
        let legend_x = [92.0, 272.0, 492.0][selector];
        writeln!(
            svg,
            r#"<line x1="{legend_x:.1}" y1="{legend_y:.1}" x2="{:.1}" y2="{legend_y:.1}" stroke="{color}" stroke-width="3"/>"#,
            legend_x + 25.0
        )
        .unwrap();
        xml_text(
            &mut svg,
            legend_x + 32.0,
            legend_y + 4.0,
            12,
            color,
            &format!("{label}, slope {:.3}", slopes[selector]),
        );
    }

    let labels = [
        "support-probe floor",
        "FE panel 0.8 / 50",
        "FE panel 1.0 / 100",
        "FE panel 1.0 / 200",
    ];
    let max_log = 20_000_f64.log10();
    for (index, (label, ratio)) in labels.into_iter().zip(budget_ratios).enumerate() {
        let y = 172.0 + index as f64 * 94.0;
        let width = ratio.log10().max(0.0) / max_log * 350.0;
        xml_text(&mut svg, 748.0, y, 13, "#334155", label);
        writeln!(
            svg,
            r##"<rect x="748" y="{:.1}" width="350" height="22" rx="5" fill="#fee2e2"/>"##,
            y + 13.0
        )
        .unwrap();
        writeln!(
            svg,
            r##"<rect x="748" y="{:.1}" width="{width:.1}" height="22" rx="5" fill="#dc2626"/>"##,
            y + 13.0
        )
        .unwrap();
        xml_text(
            &mut svg,
            748.0,
            y + 54.0,
            12,
            "#991b1b",
            &format!("{ratio:.1}× over limit"),
        );
    }
    xml_text(
        &mut svg,
        748.0,
        565.0,
        12,
        "#64748b",
        "bar length is logarithmic; frozen limit = 1×",
    );
    xml_text(
        &mut svg,
        748.0,
        595.0,
        15,
        "#991b1b",
        "63,306,810 probes before field arithmetic",
    );
    xml_text(
        &mut svg,
        54.0,
        695.0,
        19,
        "#991b1b",
        &format!("Verdict: {verdict}"),
    );
    xml_text(
        &mut svg,
        54.0,
        723.0,
        13,
        "#475569",
        "Correctness passed (60/60 queries); semantic-state and cold-budget gates failed.",
    );
    svg.push_str("</svg>\n");
    fs::write(svg_path, svg).expect("failed to write SVG");
}
