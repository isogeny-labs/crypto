//! SVG figure for `research/enc_control_20261118/`: reads a TSV of
//! `panel  series  x  y` (panel `k`: D vs k per arm; panel `dose<k>`: D vs r)
//! on stdin and writes a two-panel SVG on stdout. std only.
use std::collections::BTreeMap;
use std::io::Read;

const COLORS: [&str; 8] = ["#2f6fb0", "#c0392b", "#27ae60", "#8e44ad", "#d68910", "#16a085", "#7f8c8d", "#e84393"];

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    // panel -> series -> points
    let mut data: BTreeMap<String, BTreeMap<String, Vec<(f64, f64)>>> = BTreeMap::new();
    for line in s.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 4 {
            continue;
        }
        let (x, y) = (f[2].parse::<f64>().unwrap(), f[3].parse::<f64>().unwrap());
        data.entry(f[0].into()).or_default().entry(f[1].into()).or_default().push((x, y));
    }
    let (w, h) = (460.0, 340.0);
    let panels: Vec<_> = data.keys().cloned().collect();
    let total_w = w * panels.len() as f64;
    let mut out = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{total_w}\" height=\"{h}\" font-family=\"sans-serif\" font-size=\"12\">\n<rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n"
    );
    for (pi, panel) in panels.iter().enumerate() {
        let ox = pi as f64 * w;
        let series = &data[panel];
        // Panel kinds: "a_k" D vs k; "b_dose" D vs r (log2, r = 0 drawn at −1);
        // "c_opsdose" field_ops / tower-H vs r; "d_cu" D vs coset-union j.
        let logx = panel.contains("dose");
        let tx = |x: f64| if logx { (x.max(0.5)).log2() } else { x };
        let (mut x0, mut x1, mut y0, mut y1) = (f64::MAX, f64::MIN, 0.0f64, f64::MIN);
        for pts in series.values() {
            for &(x, y) in pts {
                x0 = x0.min(tx(x));
                x1 = x1.max(tx(x));
                y1 = y1.max(y);
            }
        }
        y0 = y0.min(3.0);
        y1 = (y1 + 1.0).ceil();
        let (l, r, t, b) = (ox + 55.0, ox + w - 120.0, 35.0, h - 45.0);
        let px = |x: f64| l + (tx(x) - x0) / (x1 - x0).max(1e-9) * (r - l);
        let py = |y: f64| b - (y - y0) / (y1 - y0) * (b - t);
        let (title, xlabel, ylabel) = match panel.as_str() {
            p if p.starts_with("a_") => ("F4 solving degree D vs k (random targets)", "k  (|F| = 2^k)", "D"),
            p if p.starts_with("b_") => ("Dose: D vs points of H replaced (log2 r)", "r (0 = H)", "D"),
            p if p.starts_with("c_") => ("Dose: F4 field ops / tower-H ops", "r (0 = H)", "ops ratio"),
            _ => ("Coset union: D vs j (2^j cosets of H_{k-j})", "j = degree bound of closing relation", "D"),
        };
        out += &format!("<text x=\"{}\" y=\"20\" font-weight=\"bold\">{title}</text>\n", l);
        out += &format!("<line x1=\"{l}\" y1=\"{b}\" x2=\"{r}\" y2=\"{b}\" stroke=\"black\"/><line x1=\"{l}\" y1=\"{t}\" x2=\"{l}\" y2=\"{b}\" stroke=\"black\"/>\n");
        for yi in (y0 as i64)..=(y1 as i64) {
            let y = py(yi as f64);
            out += &format!("<line x1=\"{l}\" y1=\"{y}\" x2=\"{r}\" y2=\"{y}\" stroke=\"#eee\"/><text x=\"{}\" y=\"{}\" text-anchor=\"end\">{yi}</text>\n", l - 6.0, y + 4.0);
        }
        let mut xs: Vec<f64> = series.values().flatten().map(|p| p.0).collect();
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        xs.dedup();
        for x in &xs {
            out += &format!("<text x=\"{}\" y=\"{}\" text-anchor=\"middle\">{x}</text>\n", px(*x), b + 16.0);
        }
        out += &format!("<text x=\"{}\" y=\"{}\" text-anchor=\"middle\">{xlabel}</text>\n", (l + r) / 2.0, b + 34.0);
        out += &format!("<text x=\"{}\" y=\"{}\" transform=\"rotate(-90 {} {})\" text-anchor=\"middle\">{ylabel}</text>\n", ox + 18.0, (t + b) / 2.0, ox + 18.0, (t + b) / 2.0);
        for (si, (name, pts)) in series.iter().enumerate() {
            let c = COLORS[si % COLORS.len()];
            let mut pts = pts.clone();
            pts.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let path: Vec<String> = pts.iter().map(|&(x, y)| format!("{:.1},{:.1}", px(x), py(y) + si as f64 * 1.5)).collect();
            out += &format!("<polyline fill=\"none\" stroke=\"{c}\" stroke-width=\"2\" points=\"{}\"/>\n", path.join(" "));
            for &(x, y) in &pts {
                out += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3\" fill=\"{c}\"/>\n", px(x), py(y) + si as f64 * 1.5);
            }
            let ly = t + 14.0 * si as f64;
            out += &format!("<rect x=\"{}\" y=\"{}\" width=\"10\" height=\"10\" fill=\"{c}\"/><text x=\"{}\" y=\"{}\">{name}</text>\n", r + 10.0, ly, r + 24.0, ly + 9.0);
        }
    }
    out += "</svg>\n";
    print!("{out}");
}
