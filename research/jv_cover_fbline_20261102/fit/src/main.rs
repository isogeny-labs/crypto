//! Paired analysis of the factor-base line search (PROTOCOL.md).
//! usage: fbline_fit <armA_dir> <armB_dir(results)> <out_dir>
use serde_json::Value;
use std::fmt::Write as _;
use std::path::Path;

const NB: usize = 20000;
const PRIMES: [u64; 3] = [1301, 1451, 1601];

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn idx(&mut self, n: usize) -> usize { (((self.next() as u128) * (n as u128)) >> 64) as usize }
}
fn rec(path: &Path) -> Value {
    let v: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap_or_else(|_| panic!("{path:?}"))).unwrap();
    v[0].clone()
}
fn u(v: &Value, k: &str) -> u64 { v[k].as_u64().unwrap_or_else(|| panic!("missing {k}")) }
fn f(v: &Value, k: &str) -> f64 { v[k].as_f64().unwrap_or_else(|| panic!("missing {k}")) }
fn s_of(v: &Value) -> f64 { u(v, "total_muls") as f64 / (f(v, "c_add_e") * f(v, "l").sqrt()) }
fn gmean(x: &[f64]) -> f64 { (x.iter().map(|v| v.ln()).sum::<f64>() / x.len() as f64).exp() }
fn pct(v: &mut Vec<f64>, q: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[((v.len() - 1) as f64 * q).round() as usize]
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (da, db, out) = (Path::new(&a[1]), Path::new(&a[2]), Path::new(&a[3]));
    let mut t = String::new();
    let mut fails: Vec<String> = vec![];
    macro_rules! say { ($($x:tt)*) => {{ let s = format!($($x)*); println!("{s}"); t += &s; t += "\n"; }} }

    // ---- C1, C2 ----
    let base = rec(&da.join("dlp_p1301_s3.json"));
    let c1 = rec(&db.join("c1_plain_p1301_s3.json"));
    let c2 = rec(&db.join("c2_line1_p1301_s3.json"));
    let same = |x: &Value, y: &Value, k: &str| x[k] == y[k];
    let c1ok = ["residuals", "relation_digest", "total_muls", "decompositions"].iter().all(|k| same(&base, &c1, k));
    say!("C1 baseline unchanged (p=1301 s3, residuals/relation_digest/total_muls/decompositions equal to the crossover record): {}", if c1ok { "PASS" } else { "FAIL" });
    if !c1ok { fails.push("C1".into()); }
    let lm = u(&c2["fb_line"], "muls");
    let c2ok = same(&c1, &c2, "residuals") && same(&c1, &c2, "relation_digest") && u(&c2, "total_muls") == u(&c1, "total_muls") + lm;
    say!("C2 machinery null (JV_FB_LINE=1): residuals {} vs {}, digest equal {}, total_muls diff {} = search muls {}: {}",
        u(&c2, "residuals"), u(&c1, "residuals"), same(&c1, &c2, "relation_digest"),
        u(&c2, "total_muls") as i64 - u(&c1, "total_muls") as i64, lm, if c2ok { "PASS" } else { "FAIL" });
    if !c2ok { fails.push("C2".into()); }

    // ---- per instance ----
    let mut rows = vec![];
    writeln!(t, "\n| p | seed | B plain | B best | S plain (A) | S line (B) | S_B/S_A | predicted (B_plain/B_best)^5 | residuals A | residuals B | search share of B | rate_B / model |").unwrap();
    writeln!(t, "|---|---|---|---|---|---|---|---|---|---|---|---|").unwrap();
    let mut c3 = true;
    let mut c4 = true;
    for p in PRIMES { for s in 1..=3u64 {
        let ra = rec(&da.join(format!("dlp_p{p}_s{s}.json")));
        let path = db.join(format!("line_p{p}_s{s}.json"));
        if !path.exists() { fails.push(format!("missing arm B p={p} s={s} (cut)")); continue; }
        let rb = rec(&path);
        let ok = rb["solved"].as_bool().unwrap() && rb["correct"].as_bool().unwrap() && u(&rb, "mismatches") == 0;
        if !ok { c3 = false; }
        assert_eq!(u(&ra, "l"), u(&rb, "l"));
        let fl = &rb["fb_line"];
        let (bp, bb) = (u(fl, "b_plain") as f64, u(fl, "b_best") as f64);
        assert_eq!(bp as u64, u(&ra, "base"), "b_plain = A's base");
        assert_eq!(bb as u64, u(&rb, "base"), "b_best = B's base");
        let (sa, sb) = (s_of(&ra), s_of(&rb));
        let pred = (bp / bb).powi(5);
        let model_rate = (2.0 * bb / p as f64).powi(6) / 720.0;
        let mr = f(&rb, "decomposition_rate") / model_rate;
        if !(0.8..=1.25).contains(&mr) { c4 = false; }
        let share = u(fl, "muls") as f64 / u(&rb, "total_muls") as f64;
        writeln!(t, "| {p} | {s} | {bp} | {bb} | {sa:.4} | {sb:.4} | **{:.3}** | {pred:.3} | {} | {} | {:.2}% | {mr:.3} |",
            sb / sa, u(&ra, "residuals"), u(&rb, "residuals"), share * 100.0).unwrap();
        rows.push((p, s, sa, sb, pred, mr, bp, bb, ok));
    } }
    say!("\nC3 every arm-B run solved and correct: {}", if c3 { "PASS" } else { "FAIL" });
    say!("C4 rate_B / ((2B/p)^6/720) in [0.8, 1.25] on every run: {}", if c4 { "PASS" } else { "FAIL" });
    if !c3 { fails.push("C3".into()); }
    if !c4 { fails.push("C4".into()); }

    // ---- primary ----
    let r: Vec<f64> = rows.iter().map(|x| x.3 / x.2).collect();
    let pr: Vec<f64> = rows.iter().map(|x| x.4).collect();
    let g = gmean(&r);
    let mut rng = Rng(0xFB11_2026_1102);
    let mut bs = Vec::with_capacity(NB);
    for _ in 0..NB {
        let v: Vec<f64> = (0..r.len()).map(|_| r[rng.idx(r.len())]).collect();
        bs.push(gmean(&v));
    }
    let (lo, hi) = (pct(&mut bs, 0.025), pct(&mut bs, 0.975));
    let all_lower = r.iter().all(|x| *x < 1.0);
    say!("\nPrimary: geometric mean S_B/S_A over {} instances = {g:.4} [{lo:.4}, {hi:.4}] (95% bootstrap over instances, {NB}); speed-up x{:.3} [{:.3}, {:.3}]; predicted geometric mean {:.4}",
        r.len(), 1.0 / g, 1.0 / hi, 1.0 / lo, gmean(&pr));
    say!("every instance S_B < S_A: {all_lower}; H1 {}", if hi < 1.0 && all_lower && r.len() == 9 { "SUPPORTED" } else { "NOT SUPPORTED" });
    for p in PRIMES {
        let v: Vec<f64> = rows.iter().filter(|x| x.0 == p).map(|x| x.3 / x.2).collect();
        let sb: Vec<f64> = rows.iter().filter(|x| x.0 == p).map(|x| x.3).collect();
        let sa: Vec<f64> = rows.iter().filter(|x| x.0 == p).map(|x| x.2).collect();
        say!("p={p}: geo-mean S_A {:.4}, S_B {:.4}, ratio {:.4}", gmean(&sa), gmean(&sb), gmean(&v));
    }
    say!("\nfailures/notes: {:?}", fails);
    std::fs::create_dir_all(out).unwrap();
    std::fs::write(out.join("paired.txt"), &t).unwrap();

    // ---- graph: measured vs predicted ratio per instance ----
    let (w, h, l0, r0, t0, b0) = (720.0, 460.0, 80.0, 30.0, 50.0, 70.0);
    let (xmin, xmax) = (0.3f64, 1.1f64);
    let xs = |v: f64| l0 + (v - xmin) / (xmax - xmin) * (w - l0 - r0);
    let ys = |v: f64| h - b0 - (v - xmin) / (xmax - xmin) * (h - t0 - b0);
    let mut svg = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' font-family='sans-serif' font-size='13'><rect width='100%' height='100%' fill='white'/>");
    write!(svg, "<text x='{}' y='26' text-anchor='middle' font-size='15'>Factor-base line search: S_line/S_plain per instance, measured vs model (B_plain/B_best)^5</text>", w / 2.0).unwrap();
    let mut k = 0.3;
    while k <= 1.1001 {
        write!(svg, "<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='#ddd'/><text x='{}' y='{}' text-anchor='end'>{k:.1}</text><text x='{}' y='{}' text-anchor='middle'>{k:.1}</text>",
            l0, ys(k), w - r0, ys(k), l0 - 6.0, ys(k) + 4.0, xs(k), h - b0 + 18.0).unwrap();
        k += 0.1;
    }
    write!(svg, "<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='#888' stroke-dasharray='5,4'/>", xs(xmin), ys(xmin), xs(xmax), ys(xmax)).unwrap();
    let col = |p: u64| match p { 1301 => "#2563eb", 1451 => "#d97706", _ => "#059669" };
    for x in &rows {
        write!(svg, "<circle cx='{}' cy='{}' r='6' fill='{}'/><text x='{}' y='{}' font-size='11'>s{}</text>", xs(x.4), ys(x.3 / x.2), col(x.0), xs(x.4) + 8.0, ys(x.3 / x.2) + 4.0, x.1).unwrap();
    }
    for (i, p) in PRIMES.iter().enumerate() {
        write!(svg, "<circle cx='{}' cy='{}' r='6' fill='{}'/><text x='{}' y='{}'>p = {p}</text>", l0 + 20.0, t0 + 20.0 + 20.0 * i as f64, col(*p), l0 + 32.0, t0 + 24.0 + 20.0 * i as f64).unwrap();
    }
    write!(svg, "<text x='{}' y='{}' text-anchor='middle'>model (B_plain / B_best)^5 (derived)</text>", (l0 + w - r0) / 2.0, h - 25.0).unwrap();
    write!(svg, "<text transform='translate(22,{}) rotate(-90)' text-anchor='middle'>measured S_line / S_plain</text>", (t0 + h - b0) / 2.0).unwrap();
    write!(svg, "<text x='{}' y='{}' fill='#555' font-size='11'>dashed: measured = model. Below 1: the line search is cheaper (all multiplications counted).</text></svg>", l0, h - 8.0).unwrap();
    std::fs::write(out.join("paired.svg"), svg).unwrap();
}
