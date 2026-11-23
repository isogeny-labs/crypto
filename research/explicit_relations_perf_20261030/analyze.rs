// Frozen descriptive analysis: neutral arm labels; no timing significance
// or speedup verdict is permitted for this shared-host diagnostic panel.
use std::{collections::BTreeMap, env, fs};
fn main() {
    let args: Vec<String> = env::args().collect();
    let raw = fs::read_to_string(&args[1]).unwrap();
    let mut cells = BTreeMap::<(String, usize), Vec<Vec<String>>>::new();
    let mut witnesses = BTreeMap::new();
    for line in raw.lines().skip(1) {
        let v: Vec<String> = line.split(',').map(str::to_owned).collect();
        assert_eq!(v.len(), 15);
        assert_eq!(v[5], "passed");
        let instance: usize = v[1].parse().unwrap();
        let round: usize = v[3].parse().unwrap();
        let pair = (v[11].clone(), v[12].clone());
        if let Some(old) = witnesses.insert((instance, round), pair.clone()) {
            assert_eq!(old, pair, "witness or normalization output differs");
        }
        if v[4] == "false" {
            cells.entry((v[0].clone(), instance)).or_default().push(v);
        }
    }
    assert_eq!(cells.len(), 12);
    assert_eq!(witnesses.len(), 30);
    let mut csv = String::from("arm,instance,metric,n,min_ms,median_ms,max_ms\n");
    let mut bars = Vec::new();
    for ((arm, instance), rows) in &cells {
        assert_eq!(rows.len(), 9);
        for (column, name) in [
            (7, "normalization"),
            (8, "prepare"),
            (9, "query"),
            (10, "cold"),
        ] {
            let mut values: Vec<f64> = rows.iter().map(|r| r[column].parse().unwrap()).collect();
            assert!(values.iter().all(|v| v.is_finite() && *v >= 0.0));
            values.sort_by(f64::total_cmp);
            csv.push_str(&format!(
                "{arm},{instance},{name},9,{},{},{}\n",
                values[0], values[4], values[8]
            ));
            if column == 9 {
                bars.push((
                    format!("{arm} / case {instance}"),
                    values[0],
                    values[4],
                    values[8],
                ));
            }
        }
    }
    let max = bars.iter().map(|b| b.3).fold(0.0_f64, f64::max).max(0.001);
    let mut svg = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"900\" height=\"550\"><rect width=\"900\" height=\"550\" fill=\"white\"/><g font-family=\"sans-serif\" font-size=\"16\" fill=\"#17212b\"><text x=\"25\" y=\"30\">RTX PRO 6000: four-query latency (ms), median and full range, n=9</text><text x=\"25\" y=\"54\">Shared-host L0 diagnostics; no certified timing comparison</text>");
    for (i, (label, lo, med, hi)) in bars.iter().enumerate() {
        let y = 85 + i * 35;
        let scale = 550.0 / max;
        svg.push_str(&format!("<text x=\"25\" y=\"{}\">{label}</text><rect x=\"150\" y=\"{y}\" width=\"{}\" height=\"20\" fill=\"#3678a8\"/><line x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\" stroke=\"#17212b\"/><text x=\"720\" y=\"{}\">{med:.3}</text>", y+16, med*scale, 150.0+lo*scale,150.0+hi*scale,y+10,y+10,y+16));
    }
    svg.push_str("<text x=\"25\" y=\"535\">EXPLICIT-RELATIONS-NORMALIZE-20261030 / raw rows.csv; labels decoded in report</text></g></svg>");
    fs::write(format!("{}/summary.csv", args[2]), csv).unwrap();
    fs::write(format!("{}/latency.svg", args[2]), svg).unwrap();
    println!("PASS 120 rows, 108 measured, 12 complete cells, exact cross-arm digests");
}
