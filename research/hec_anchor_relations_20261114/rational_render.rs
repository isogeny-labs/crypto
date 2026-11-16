//! Plot recorded counts only; input is native jq TSV, no timing inference.
use std::{fmt::Write, fs};
fn main() {
    let dir = std::env::args().nth(1).expect("study directory");
    let input = fs::read_to_string(format!("{dir}/RATIONAL-PLOT.tsv")).unwrap();
    let mut svg=String::from("<svg xmlns='http://www.w3.org/2000/svg' width='1000' height='570' viewBox='0 0 1000 570'><rect width='1000' height='570' fill='#f8fafc'/><g font-family='Arial,sans-serif' fill='#172d3a'><text x='28' y='35' font-size='25' font-weight='bold'>Smaller anchor spaces: rank and candidate cost</text><text x='28' y='61' font-size='16'>Exact counts, one deterministic curve per genus; no timing comparison</text>");
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
            let h = v[1].parse::<usize>().unwrap();
            let d = v[2].parse::<usize>().unwrap();
            let row = hs.iter().position(|&a| a == h).unwrap();
            let cx = x + 28 + d * 65;
            let cy = 149 + row * 100;
            let (color, label) = match v[5] {
                "rank_reached" => ("#d4ece1", "reached"),
                "tuple_cap" => ("#fbe2ac", "capped"),
                _ => ("#e2e8ee", "exhausted"),
            };
            write!(svg,"<rect x='{cx}' y='{cy}' width='62' height='88' rx='5' fill='{color}'/><text x='{}' y='{}' font-size='15'>H={h}</text><text x='{}' y='{}' font-size='15'>{}/{}</text><text x='{}' y='{}' font-size='14'>{}</text><text x='{}' y='{}' font-size='11'>{label}</text>",cx+4,cy+18,cx+4,cy+38,v[4],h-1,cx+4,cy+59,v[3],cx+3,cy+77).unwrap();
        }
    }
    svg.push_str("<text x='28' y='477' font-size='16'>Each cell: anchors H; attained/required rank; candidate functions visited; terminal status.</text><text x='28' y='503' font-size='16'>Green = rank reached · gray = full family exhausted · amber = frozen tuple cap</text><text x='28' y='529' font-size='14'>RATIONAL-RANK-VERIFIED.jsonl · 36 arms · 145268 visited outputs · 5360 principal checks</text><text x='28' y='551' font-size='14'>Producer evidence; independent Gaussian rank algorithm shares the field implementation.</text></g></svg>");
    fs::write(format!("{dir}/RATIONAL-RANK.svg"), svg).unwrap();
}
