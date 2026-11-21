use std::{collections::BTreeMap, fs, path::PathBuf};
fn fields(s: &str) -> BTreeMap<&str, &str> {
    s.split_whitespace()
        .skip(1)
        .filter_map(|s| s.split_once('='))
        .collect()
}
fn stats(v: &[u128]) -> (u128, u128, u128) {
    let mut a = v.to_vec();
    a.sort();
    (a[0], a[a.len() / 2], a[a.len() - 1])
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let root = PathBuf::from(&args[1]);
    let out = PathBuf::from(&args[2]);
    let mut arm = BTreeMap::new();
    let mut phases: BTreeMap<String, Vec<u128>> = BTreeMap::new();
    let mut no_maps = Vec::new();
    let mut solve_only = Vec::new();
    for name in ["cold", "compact", "fast", "hybrid"] {
        let mut v = Vec::new();
        for b in 0..5 {
            let s = fs::read_to_string(root.join(format!("{b}-{name}.stdout"))).unwrap();
            let n: u128 = fields(s.lines().find(|s| s.starts_with("MEASURE ")).unwrap())
                ["total_ns"]
                .parse()
                .unwrap();
            v.push(n);
            if name == "cold" {
                let p: BTreeMap<_, _> = s
                    .lines()
                    .filter(|s| s.starts_with("PHASE "))
                    .map(|s| {
                        let f = fields(s);
                        (f["name"].to_string(), f["ns"].parse::<u128>().unwrap())
                    })
                    .collect();
                no_maps.push(n - p["map_generator"] - p["map_target"]);
                solve_only.push(p["relations"] + p["descent"] + p["verification"]);
                for (k, n) in p {
                    phases.entry(k).or_default().push(n)
                }
            }
        }
        arm.insert(name.to_string(), stats(&v));
    }
    let best = ["compact", "fast", "hybrid"]
        .into_iter()
        .min_by_key(|s| arm[*s].1)
        .unwrap();
    let native = arm[best].1;
    let cold = arm["cold"].1;
    let (nl, nm, nh) = stats(&no_maps);
    let (sl, sm, sh) = stats(&solve_only);
    let text=format!("POSTRUN_DERIVED best_native={best} native_median_ns={native} cold_median_ns={cold} cold_over_best={:.12} improvement_needed_for_10x={:.12}\nNO_MAPS n=5 min_ns={nl} median_ns={nm} max_ns={nh} best_over_residual={:.17}\nRELATIONS_DESCENT_VERIFICATION n=5 min_ns={sl} median_ns={sm} max_ns={sh} best_over_residual={:.17}\n",cold as f64/native as f64,10.*cold as f64/native as f64,native as f64/nm as f64,native as f64/sm as f64);
    print!("{text}");
    fs::write(out.join("derived.txt"), text).unwrap();
    let head = |title: &str, h: u32| {
        format!("<svg xmlns='http://www.w3.org/2000/svg' width='860' height='{h}' viewBox='0 0 860 {h}'><rect width='860' height='{h}' fill='#f7f8fa'/><g font-family='Helvetica,Arial,sans-serif' fill='#17263b'><text x='28' y='34' font-size='22'>{title}</text>")
    };
    let mut svg = head("Complete cold cost: numerical gate failed", 390);
    svg.push_str("<text x='28' y='61' font-size='13'>Median and observed min–max • five fresh processes per arm • log time axis</text>");
    let x = |n: u128| 220. + (n as f64).log10().mul_add(100., -500.);
    for j in 0..6 {
        let xx = 220. + j as f64 * 100.;
        let label = ["0.1 ms", "1 ms", "10 ms", "100 ms", "1 s", "10 s"][j];
        svg+=&format!("<line x1='{xx}' y1='80' x2='{xx}' y2='300' stroke='#d7dde5'/><text x='{}' y='323' font-size='12'>{label}</text>",xx-18.)
    }
    for (i, name) in ["cold", "compact", "fast", "hybrid"]
        .into_iter()
        .enumerate()
    {
        let (l, m, h) = arm[name];
        let y = 106. + i as f64 * 52.;
        let color = if name == "cold" { "#b16c61" } else { "#147d83" };
        svg+=&format!("<text x='28' y='{}' font-size='15'>{name}</text><line x1='{}' y1='{y}' x2='{}' y2='{y}' stroke='{color}' stroke-width='3'/><circle cx='{}' cy='{y}' r='6' fill='{color}'/><text x='{}' y='{}' font-size='13'>{:.3} ms</text>",y+5.,x(l),x(h),x(m),x(h)+10.,y+5.,m as f64/1e6)
    }
    svg.push_str("<text x='28' y='355' font-size='13'>Same public genus-3/F127 input • L0/unisolated • no statistical confidence claim</text><text x='28' y='379' font-size='12'>hec_coldcost13_20261115/run • algorithm core includes construction and verification</text></g></svg>");
    fs::write(out.join("COST.svg"), svg).unwrap();
    let mut svg = head("Cold candidate phase costs", 650);
    svg.push_str("<text x='28' y='61' font-size='13'>Per-phase median and observed min–max • five processes • log time axis</text>");
    let xp = |n: u128| 220. + ((n as f64).log10() - 4.) * 85.;
    for j in 0..7 {
        let xx = 220. + j as f64 * 85.;
        let label = [
            "0.01 ms", "0.1 ms", "1 ms", "10 ms", "100 ms", "1 s", "10 s",
        ][j];
        svg+=&format!("<line x1='{xx}' y1='80' x2='{xx}' y2='565' stroke='#d7dde5'/><text x='{}' y='587' font-size='11'>{label}</text>",xx-18.)
    }
    for (i, name) in [
        "input",
        "source_weil",
        "field_kernels",
        "weil_normalization",
        "geometry",
        "points_jacobian",
        "relations",
        "map_generator",
        "map_target",
        "descent",
        "verification",
    ]
    .into_iter()
    .enumerate()
    {
        let (l, m, h) = stats(&phases[name]);
        let y = 100. + i as f64 * 42.;
        svg+=&format!("<text x='28' y='{}' font-size='13'>{name}</text><line x1='{}' y1='{y}' x2='{}' y2='{y}' stroke='#6485a9' stroke-width='3'/><circle cx='{}' cy='{y}' r='5' fill='#147d83'/><text x='{}' y='{}' font-size='12'>{:.3} ms</text>",y+4.,xp(l),xp(h),xp(m),xp(h)+9.,y+4.,m as f64/1e6)
    }
    svg.push_str("<text x='28' y='620' font-size='12'>Individual phase medians need not sum to total median. Raw per-cell phases retained.</text><text x='28' y='642' font-size='12'>hec_coldcost13_20261115/run • diagnostic fixed-case costs, not a scaling result</text></g></svg>");
    fs::write(out.join("PHASES.svg"), svg).unwrap();
}
