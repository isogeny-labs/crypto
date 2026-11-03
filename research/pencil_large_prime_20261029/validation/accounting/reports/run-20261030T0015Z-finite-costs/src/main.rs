use serde_json::Value;
use std::{fmt::Write, fs};
fn frac(v: &Value) -> f64 {
    v["numerator"].as_str().unwrap().parse::<f64>().unwrap()
        / v["denominator"].as_str().unwrap().parse::<f64>().unwrap()
}
fn main() {
    let v: Value = serde_json::from_slice(&fs::read("observations.json").unwrap()).unwrap();
    let rows = v["aggregate_cells"].as_array().unwrap();
    assert_eq!(rows.len(), 36);
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1100\" height=\"520\" viewBox=\"0 0 1100 520\"><rect width=\"1100\" height=\"520\" fill=\"white\"/><g font-family=\"sans-serif\" fill=\"#142535\"><text x=\"60\" y=\"35\" font-size=\"23\">P739 gate: finite counted candidate/rho cost</text><text x=\"60\" y=\"62\" font-size=\"15\">60 targets × 5 measured streams = 300 paired observations per cell; all 36 prices retained</text>");
    for y in (0..=70).step_by(10) {
        let sy = 420.0 - f64::from(y) * 4.5;
        write!(svg,"<line x1=\"70\" y1=\"{sy}\" x2=\"1050\" y2=\"{sy}\" stroke=\"#d8e0e7\"/><text x=\"35\" y=\"{}\" font-size=\"13\">{y}</text>",sy+4.0).unwrap();
    }
    let mut picture = String::new();
    let mut prior: Option<(f64, f64, f64)> = None;
    let mut a = String::new();
    let mut b = String::new();
    let mut ca = String::new();
    let mut cb = String::new();
    let mut table = String::new();
    for (i, r) in rows.iter().enumerate() {
        assert_eq!(r["admitted_pairs"], 300);
        assert_eq!(r["complete"], true);
        let av = frac(&r["algorithm_ratio_of_sums"]);
        let bv = frac(&r["ratio_of_sums_including_calibration"]);
        let x = 70.0 + i as f64 * 28.0;
        let px = i as f64 * 10.0;
        if let Some((ox, oa, ob)) = prior {
            write!(picture,"\\color{{blue}}\\qbezier({ox},{})({},{})({px},{})\\color{{red}}\\qbezier({ox},{})({},{})({px},{})",oa*1.4,(ox+px)/2.,(oa+av)*0.7,av*1.4,ob*1.4,(ox+px)/2.,(ob+bv)*0.7,bv*1.4).unwrap();
        }
        prior = Some((px, av, bv));
        write!(a, "{x},{} ", 420.0 - av * 4.5).unwrap();
        write!(b, "{x},{} ", 420.0 - bv * 4.5).unwrap();
        write!(ca, "({},{av:.10}) ", i + 1).unwrap();
        write!(cb, "({},{bv:.10}) ", i + 1).unwrap();
        let s = r["square_twice"].as_u64().unwrap() as f64 / 2.;
        let n = r["native_tenths"].as_u64().unwrap() as f64 / 10.;
        writeln!(
            table,
            "{} & {s:.1} & {} & {n:.1} & {av:.6} & {bv:.6} \\\\",
            i + 1,
            r["inversion"]
        )
        .unwrap();
        if i % 3 == 0 {
            write!(
                svg,
                "<text x=\"{}\" y=\"440\" font-size=\"12\">{}</text>",
                x - 5.,
                i + 1
            )
            .unwrap();
        }
    }
    write!(svg,"<polyline points=\"{a}\" fill=\"none\" stroke=\"#17668c\" stroke-width=\"3\"/><polyline points=\"{b}\" fill=\"none\" stroke=\"#ba4d22\" stroke-width=\"2\" stroke-dasharray=\"5 4\"/><text x=\"70\" y=\"472\" font-size=\"14\" fill=\"#17668c\">Solid: method only</text><text x=\"290\" y=\"472\" font-size=\"14\" fill=\"#ba4d22\">Dashed: method + calibration</text><text x=\"70\" y=\"500\" font-size=\"13\">Cell index follows exact CSV; decimal display only. Run pencil-p739-20261030-a1. No qualified wall ratio.</text></g></svg>").unwrap();
    fs::write("cost-grid.svg", svg).unwrap();
    let intro = fs::read_to_string("narrative.tex").unwrap();
    let tex = format!(
        r"\documentclass[10pt]{{article}}
\usepackage[margin=18mm]{{geometry}}\usepackage{{color,longtable,booktabs,hyperref}}
\hypersetup{{colorlinks=true,urlcolor=blue}}
\begin{{document}}\title{{P739 mathematical workload gate: finite accounting observations}}\date{{30 October 2026}}\maketitle
{intro}
\begin{{center}}\setlength{{\unitlength}}{{1pt}}\begin{{picture}}(380,125)(-20,-15)\put(0,0){{\line(1,0){{350}}}}\put(0,0){{\line(0,1){{105}}}}\put(-18,98){{70}}\put(-18,0){{0}}\put(0,-12){{1}}\put(335,-12){{36}}{picture}\end{{picture}}\end{{center}}\noindent Blue: method only. Red: method plus calibration. Horizontal: price-cell index; vertical: candidate/rho ratio.
\noindent Graph shows all 36 recorded price cells, each using all 300 measured pairs. Points are derived from exact rational output; display rounding only. There are no statistical error bars: the grid varies declared prices, not a confidence interval.
\newpage\section*{{All recorded price cells}}
\noindent Every row admits 300 of 300 pairs with zero missing/ineligible pairs. S and I are prices relative to M; N is GAE per declared native event. Exact numerator/denominator pairs are in cost-grid-exact.csv and observations.json. Ratios below are rounded to six decimals for display.
\begin{{longtable}}{{rrrrrr}}\toprule Cell&S/M&I/M&N&Method/rho&With calibration/rho\\\midrule\endhead
{table}\bottomrule\end{{longtable}}
\noindent The JSON retains exact summed candidate and reference costs, not just ratios. The CSV preserves all 36 exact ratio fractions. Earlier null-coverage observations remain separate and unchanged.
\end{{document}}"
    );
    fs::write("report.tex", tex).unwrap();
}
