use std::{fmt::Write, fs};
fn main() {
    let a: Vec<_> = std::env::args().collect();
    let t = fs::read_to_string(&a[1]).unwrap();
    let mut s=String::from("<svg xmlns='http://www.w3.org/2000/svg' width='1000' height='390' viewBox='0 0 1000 390'><rect width='1000' height='390' fill='#f7fafc'/><g font-family='Arial' fill='#233746'><text x='20' y='30' font-size='21' font-weight='bold'>Fastest existing complete total / zero-core total</text>");
    let x = |v: f64| 210.0 + (v.log2() + 5.0) * 82.0;
    for v in [0.03125, 0.0625, 0.125, 0.25, 0.5, 1.0, 2.0, 4.0, 8.0, 16.0] {
        let p = x(v);
        write!(s,"<line x1='{p}' y1='48' x2='{p}' y2='310' stroke='{}'/><text x='{p}' y='332' text-anchor='middle' font-size='13'>{v}×</text>",if v==1.0{"#233746"}else{"#d5dfe5"}).unwrap();
    }
    for (i, line) in t.lines().enumerate() {
        let v: Vec<_> = line.split('\t').collect();
        let y = 75 + i * 40;
        let (m, l, h) = (
            v[2].parse::<f64>().unwrap(),
            v[3].parse::<f64>().unwrap(),
            v[4].parse::<f64>().unwrap(),
        );
        write!(s,"<text x='20' y='{}' font-size='16'>g{} · p{}</text><line x1='{}' x2='{}' y1='{y}' y2='{y}' stroke='#b34b42' stroke-width='3'/><circle cx='{}' cy='{y}' r='5' fill='#b34b42'/><text x='920' y='{}' font-size='16'>{m:.3}×</text>",y+5,v[0],v[1],x(l),x(h),x(m),y+5).unwrap();
    }
    let p = x(10.0);
    write!(s,"<line x1='{p}' x2='{p}' y1='45' y2='310' stroke='#176f85' stroke-dasharray='4 4'/><text x='20' y='365' font-size='15'>Dots: three-target medians; lines: observed min/max, not confidence intervals. Log2 ratio axis.</text><text x='20' y='386' font-size='15'>Below 1× favors existing controls. Dashed line: registered 10× threshold. All losses retained.</text></g></svg>").unwrap();
    fs::write(&a[2], s).unwrap();
}
