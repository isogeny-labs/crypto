//! Native plot of recorded medians/ranges; no statistical inference.
use std::{fmt::Write, fs};
fn main() {
    let dir = std::env::args().nth(1).expect("study path");
    let t = fs::read_to_string(format!("{dir}/linear-cold-evidence/plot.tsv")).unwrap();
    let mut svg=String::from("<svg xmlns='http://www.w3.org/2000/svg' width='1100' height='580' viewBox='0 0 1100 580'><rect width='1100' height='580' fill='#f8fafc'/><g font-family='Arial,sans-serif' fill='#183340'><text x='25' y='35' font-size='26' font-weight='bold'>Complete cold cost: exact work, different arithmetic</text><text x='25' y='63' font-size='17'>18 targets · 288 cold cells · 3 target seeds per fixture · source ed0fb2c5f982</text>");
    let x = |v: f64, base: f64| base + (v.log2() + 2.0) * 56.0;
    for (base, title) in [
        (185.0, "Fastest existing / linear"),
        (700.0, "Generic rational / linear"),
    ] {
        write!(
            svg,
            "<text x='{base}' y='108' font-size='21' font-weight='bold'>{title}</text>"
        )
        .unwrap();
        for v in [0.25, 0.5, 1.0, 2.0, 4.0, 8.0, 16.0] {
            let px = x(v, base);
            write!(svg,"<line x1='{px}' y1='128' x2='{px}' y2='437' stroke='{}' stroke-width='{}'/><text x='{px}' y='462' font-size='14' text-anchor='middle'>{v}x</text>",if v==1.0{"#536e7c"}else{"#d8e1e7"},if v==1.0{2}else{1}).unwrap();
        }
    }
    let target = x(10.0, 185.0);
    write!(svg,"<line x1='{target}' y1='125' x2='{target}' y2='439' stroke='#b4473e' stroke-width='2' stroke-dasharray='5 4'/><text x='{target}' y='484' font-size='14' fill='#b4473e' text-anchor='middle'>10x pilot target</text>").unwrap();
    for (i, line) in t.lines().enumerate() {
        let v: Vec<_> = line.split('\t').collect();
        let y = 155 + i * 48;
        write!(
            svg,
            "<text x='25' y='{}' font-size='17'>g{} · p{}</text>",
            y + 5,
            v[0],
            v[1]
        )
        .unwrap();
        for (base, start) in [(185.0, 2), (700.0, 5)] {
            let med = v[start].parse::<f64>().unwrap();
            let lo = v[start + 1].parse::<f64>().unwrap();
            let hi = v[start + 2].parse::<f64>().unwrap();
            let color = if start == 2 && med < 1.0 {
                "#b4473e"
            } else {
                "#187e92"
            };
            let a = x(lo, base);
            let b = x(hi, base);
            let c = x(med, base);
            write!(svg,"<line x1='{a}' y1='{y}' x2='{b}' y2='{y}' stroke='{color}' stroke-width='3'/><circle cx='{c}' cy='{y}' r='5' fill='{color}'/><text x='{}' y='{}' font-size='15' fill='{color}'>{med:.2}x</text>",base+345.0,y+5).unwrap();
        }
    }
    svg.push_str("<text x='25' y='521' font-size='16'>Dots = observed medians; whiskers = three-target min/max, not confidence intervals. Log2 ratio axis.</text><text x='25' y='548' font-size='16'>Below 1x: slower candidate. Best baseline includes compact + generic + 12 native/table controls.</text><text x='25' y='572' font-size='14'>L0 local ARM; A/A spread up to 2.55x. All losses retained; the registered 10x pilot gate fails.</text></g></svg>");
    fs::write(format!("{dir}/LINEAR-COLD.svg"), svg).unwrap();
}
