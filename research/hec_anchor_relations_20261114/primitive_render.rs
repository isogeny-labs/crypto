//! Plot recorded count pairs only, no runtime inference.
use std::{fmt::Write, fs};
fn main() {
    let dir = std::env::args().nth(1).expect("study directory");
    let input = fs::read_to_string(format!("{dir}/PRIMITIVE-PLOT.tsv")).unwrap();
    let mut svg=String::from("<svg xmlns='http://www.w3.org/2000/svg' width='1000' height='620' viewBox='0 0 1000 620'><rect width='1000' height='620' fill='#f8fafc'/><g font-family='Arial,sans-serif' fill='#172d3a'><text x='28' y='35' font-size='25' font-weight='bold'>Primitive normalization: functions needed to reach rank</text><text x='28' y='61' font-size='16'>Exact paired counts · B = reject common factors · P = primitive-normalize</text>");
    for (g, p) in [(2, 1009), (3, 503), (4, 127)] {
        let x = 28 + (g - 2) * 326;
        write!(
            svg,
            "<text x='{x}' y='104' font-size='21' font-weight='bold'>Genus {g} · p={p}</text>"
        )
        .unwrap();
        for d in 0..=3 {
            write!(
                svg,
                "<text x='{}' y='134' font-size='16'>d{d}</text>",
                x + 40 + d * 65
            )
            .unwrap();
        }
        let hs = [g + 6, 2 * g + 8, 4 * g + 12];
        for line in input.lines() {
            let v: Vec<_> = line.split('\t').collect();
            if v[0].parse::<usize>().unwrap() != g {
                continue;
            }
            let h = v[2].parse::<usize>().unwrap();
            let d = v[3].parse::<usize>().unwrap();
            let row = hs.iter().position(|&a| a == h).unwrap();
            let cx = x + 28 + d * 65;
            let cy = 149 + row * 112;
            let color = if v[5].parse::<usize>().unwrap() < v[4].parse::<usize>().unwrap() {
                "#cce7ec"
            } else {
                "#e2e8ee"
            };
            let status = match v[7] {
                "rank_reached" => "rank",
                "tuple_cap" => "cap",
                _ => "exhaust",
            };
            write!(svg,"<rect x='{cx}' y='{cy}' width='62' height='101' rx='5' fill='{color}'/><text x='{}' y='{}' font-size='14'>H={h}</text><text x='{}' y='{}' font-size='12'>B {}</text><text x='{}' y='{}' font-size='12'>P {}</text><text x='{}' y='{}' font-size='12'>N {}</text><text x='{}' y='{}' font-size='12'>{status}</text>",cx+4,cy+17,cx+4,cy+36,v[4],cx+4,cy+54,v[5],cx+4,cy+72,v[6],cx+4,cy+91).unwrap();
        }
    }
    svg.push_str("<text x='28' y='515' font-size='16'>Each cell: anchor count H; baseline B and primitive P visits; normalized admissions N; status.</text><text x='28' y='542' font-size='16'>Blue = fewer visits · gray = unchanged · rank = H−1 reached · cap/exhaust retained</text><text x='28' y='569' font-size='14'>36 paired arms · 11 improved, 25 unchanged · maximum visit-count ratio 1.40549 (not a time ratio)</text><text x='28' y='595' font-size='14'>Source: PRIMITIVE-SUMMARY.json / PRIMITIVE-RANK.jsonl · producer counts, no population confidence claim</text></g></svg>");
    fs::write(format!("{dir}/PRIMITIVE-RANK.svg"), svg).unwrap();
}
