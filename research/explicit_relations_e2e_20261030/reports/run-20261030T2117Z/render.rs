// Presentation-only rendering of recorded rows; no fitting, pooling or reruns.
use std::{collections::BTreeSet, env, fs};
fn main() {
    let a: Vec<_> = env::args().collect();
    let raw = fs::read_to_string(&a[1]).unwrap();
    let rows: Vec<Vec<_>> = raw
        .lines()
        .skip(1)
        .map(|s| s.split(',').collect())
        .collect();
    let bits: BTreeSet<u32> = rows.iter().map(|r| r[1].parse().unwrap()).collect();
    let min = *bits.first().unwrap();
    let max = *bits.last().unwrap();
    let mut svg = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1000\" height=\"560\"><rect width=\"1000\" height=\"560\" fill=\"white\"/><g font-family=\"sans-serif\" font-size=\"16\" fill=\"#182a3a\"><text x=\"30\" y=\"28\">Every recorded execution: cold solve seconds</text><text x=\"30\" y=\"54\">Shared-host L0 observations; red points are retained unsuccessful executions</text>");
    for e in -2..=4 {
        let y = 440.0 - (e + 2) as f64 * 350.0 / 6.0;
        svg.push_str(&format!("<line x1=\"85\" x2=\"945\" y1=\"{y}\" y2=\"{y}\" stroke=\"#d8dfe5\"/><text x=\"10\" y=\"{}\">{} s</text>",y+5.0,10f64.powi(e)));
    }
    let xpos = |b: u32| 150.0 + (b - min) as f64 * 750.0 / (max - min).max(1) as f64;
    for b in bits {
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"470\">{b}</text>",
            xpos(b) - 9.0
        ));
    }
    for r in rows {
        assert_eq!(r.len(), 19);
        let b: u32 = r[1].parse().unwrap();
        let seconds: f64 = r[9].parse().unwrap();
        assert!(seconds.is_finite() && (0.01..=10000.0).contains(&seconds));
        let y = 440.0 - (seconds.log10() + 2.0) * 350.0 / 6.0;
        let offset = match r[0] {
            "b" => -27.0,
            "x" => 27.0,
            "h" => -36.0,
            "u" => -9.0,
            "v" => 9.0,
            _ => panic!("unknown arm"),
        };
        let color = if r[5] != "succeeded" {
            "#c43d3d"
        } else {
            match r[0] {
                "b" => "#628b3f",
                "u" => "#009998",
                "v" => "#2879b8",
                "x" => "#8056a8",
                "h" => "#686868",
                _ => unreachable!(),
            }
        };
        svg.push_str(&format!("<circle cx=\"{}\" cy=\"{y}\" r=\"4\" fill=\"{color}\"><title>{b} bits; arm {}; seed {}; {}; {seconds:.6} seconds</title></circle>",xpos(b)+offset+if r[2]=="101" {-4.5} else {4.5},r[0],r[2],r[5]));
    }
    svg.push_str("<text x=\"430\" y=\"505\">Prime-field bits</text><text x=\"30\" y=\"539\">b: m8; u: m10; v/x: m12; red: timeout/limit. All points retained.</text></g></svg>");
    fs::write(&a[2], svg).unwrap();
}
