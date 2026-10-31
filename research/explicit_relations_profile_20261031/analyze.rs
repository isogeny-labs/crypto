// Strict native analysis. All planned cells and all query outcomes must match.
use std::{collections::BTreeMap, env, fs, io::Write, path::Path};
fn fields(line: &str) -> BTreeMap<&str, &str> {
    line.split_whitespace()
        .filter_map(|s| s.split_once('='))
        .collect()
}
fn main() {
    let arg = env::args().nth(1).unwrap();
    let out = Path::new(&arg);
    let mut csv = fs::File::create(out.join("stages.csv")).unwrap();
    writeln!(csv,"seed,round,arm,query,kernel_ms,tiles,rows,load_convert,subtract,normalize,affine_out,lookup,empty").unwrap();
    let mut summary = String::from("Diagnostic prefix: two 63-bit targets, eight queries each, five rounds, three arms. Block cycles include barriers and scheduling; NOT additive wall time.\n");
    let mut svg = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1000\" height=\"370\" viewBox=\"0 0 1000 370\"><rect width=\"1000\" height=\"370\" fill=\"white\"/><g font-family=\"sans-serif\" font-size=\"16\" fill=\"#172b4d\"><text x=\"25\" y=\"28\">Instrumented block-cycle shares (not wall-time shares)</text>");
    let names = [
        "load_convert",
        "subtract",
        "normalize",
        "affine_out",
        "lookup",
        "empty",
    ];
    let colors = [
        "#0072b2", "#56b4e9", "#e69f00", "#009e73", "#cc79a7", "#999999",
    ];
    for (si, seed) in [101, 202].iter().enumerate() {
        let expected = fs::read(out.join(format!("{seed}.commands.answers"))).unwrap();
        assert_eq!(
            fs::read_to_string(out.join(format!("{seed}.commands.complete"))).unwrap(),
            "8\n"
        );
        let mut sums = [0u64; 6];
        let mut tiles = 0u64;
        let mut ms = [0.0; 3];
        let mut aa: Vec<f64> = Vec::new();
        for round in 0..5 {
            let mut round_ms = [0.0; 3];
            for (ai, arm) in ["b", "s", "x"].iter().enumerate() {
                let id = format!("{seed}-{round}-{arm}");
                assert_eq!(
                    fs::read(out.join(format!("{id}.stdout"))).unwrap(),
                    expected,
                    "outcome mismatch: {id}"
                );
                let log = fs::read_to_string(out.join(format!("{id}.stderr"))).unwrap();
                let records: Vec<_> = log.lines().filter(|l| l.starts_with("PROFILE ")).collect();
                assert_eq!(records.len(), 8, "missing profile records: {id}");
                let relations: Vec<_> = log
                    .lines()
                    .filter(|l| l.starts_with("{\"stage\":\"relation\""))
                    .collect();
                assert_eq!(relations.len(), 8);
                for (qi, line) in records.iter().enumerate() {
                    let f = fields(line);
                    assert_eq!(f["mode"], if *arm == "s" { "stages" } else { "stock" });
                    let time: f64 = f["kernel_ms"].parse().unwrap();
                    assert!(time.is_finite() && time > 0.0);
                    round_ms[ai] += time;
                    write!(
                        csv,
                        "{seed},{round},{arm},{qi},{time},{} ,{}",
                        f["tiles"], f["rows"]
                    )
                    .unwrap();
                    for (j, name) in names.iter().enumerate() {
                        let value: u64 = f[name].parse().unwrap();
                        write!(csv, ",{value}").unwrap();
                        if *arm == "s" {
                            sums[j] = sums[j].checked_add(value).unwrap();
                        }
                    }
                    writeln!(csv).unwrap();
                    if *arm == "s" {
                        let n: u64 = f["tiles"].parse().unwrap();
                        let rows: u64 = f["rows"].parse().unwrap();
                        assert!(n > 0 && rows > 0 && rows <= n * 256);
                        assert!(relations[qi].contains(&format!("\"right_rows_visited\":{rows}}}")));
                        tiles += n;
                    }
                }
                ms[ai] += round_ms[ai];
            }
            aa.push((round_ms[0] - round_ms[2]).abs() / ((round_ms[0] + round_ms[2]) / 2.0));
        }
        let total: u64 = sums.iter().sum();
        summary += &format!("seed={seed} queries_per_arm=40 kernel_ms_b={} kernel_ms_stages={} kernel_ms_x={} staged_over_stock={} AA_relative_spreads={aa:?}\n", ms[0],ms[1],ms[2],ms[1]/((ms[0]+ms[2])/2.0));
        let mut x = 130.0;
        let y = 65 + si * 80;
        svg += &format!("<text x=\"25\" y=\"{}\">seed {seed}</text>", y + 28);
        for (j, name) in names.iter().enumerate() {
            let share = sums[j] as f64 / total as f64;
            summary += &format!(
                "  {name}: block_cycles={} cycles_per_tile={:.3} share_including_empty={:.6}\n",
                sums[j],
                sums[j] as f64 / tiles as f64,
                share
            );
            let width = share * 820.0;
            svg += &format!(
                "<rect x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"45\" fill=\"{}\"/>",
                colors[j]
            );
            x += width;
        }
    }
    for (j, name) in names.iter().enumerate() {
        let x = 25 + (j % 3) * 315;
        let y = 250 + (j / 3) * 40;
        svg += &format!("<rect x=\"{x}\" y=\"{y}\" width=\"20\" height=\"20\" fill=\"{}\"/><text x=\"{}\" y=\"{}\">{name}</text>",colors[j],x+30,y+16);
    }
    svg += "<text x=\"25\" y=\"350\">n=40 staged queries per seed; empty stamp control retained, never subtracted.</text></g></svg>";
    fs::write(out.join("summary.txt"), &summary).unwrap();
    fs::write(out.join("stages.svg"), svg).unwrap();
    print!("{summary}");
}
