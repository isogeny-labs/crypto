//! Run66: exact original-Rust three-summand public-support GRADER.
//! Do not use its generic pair dictionary as a non-generic root extractor.
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Point, PRIME31};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const Y7_EXTRA: &str =
    include_str!("../../../prime31_f4_admission_20261121/RUN66_Y7_EXTRA168.csv");

fn parse_xy(line: &str) -> Result<Point, String> {
    let (xs, ys) = line.split_once(',').ok_or("missing comma")?;
    let x = xs.parse::<u64>().map_err(|e| e.to_string())?;
    let y = ys.parse::<u64>().map_err(|e| e.to_string())?;
    Ok(Some((x, y)))
}

fn check_base(ar: &mut Arithmetic) -> Result<(Vec<Point>, Vec<Point>), String> {
    check_curve(PRIME31, ar)?;
    if (PRIME31.p, PRIME31.a, PRIME31.b, PRIME31.n)
        != (2_147_483_647, 847_514_846, 2_087_444_724, 2_147_446_831) {
        return Err("prime31 constants have changed".into());
    }
    let canonical = |point: Point| -> Point {
        point.map(|(x, y)| (x, y.min((PRIME31.p - y) % PRIME31.p)))
    };
    let dickson = Base::build(PRIME31, 10, ar)?.points.into_iter()
        .map(|p| canonical(Some(p))).collect::<Vec<_>>();
    if dickson.len() != 509 { return Err("Dickson B509 changed".into()); }
    let extra = Y7_EXTRA.lines().filter(|s| !s.is_empty())
        .map(parse_xy).collect::<Result<Vec<_>, _>>()?;
    if extra.len() != 168 { return Err("expected 168 cubic-fiber leaf points".into()); }
    let mut hybrid = dickson.clone();
    hybrid.extend(extra.into_iter().map(canonical));
    hybrid.sort_unstable();
    hybrid.dedup();
    if hybrid.len() != 677 { return Err(format!("hybrid fixture size {}, expected 677", hybrid.len())); }
    let hd = hybrid.iter().copied().collect::<HashSet<_>>();
    if hd.len() != 677 { return Err("hybrid duplicates".into()); }
    if !dickson.iter().all(|p| hd.contains(p)) {
        return Err("hybrid omitted original Rust Dickson point".into());
    }
    for &point in hybrid.iter().chain(dickson.iter()) {
        if !ar.on_curve(PRIME31, point) {
            return Err("off-curve public factor-base point".into());
        }
    }
    Ok((dickson, hybrid))
}

fn fpow(mut x: u64, mut e: u64) -> u64 {
    let mut r = 1_u64;
    while e > 0 {
        if e & 1 != 0 { r = r * x % PRIME31.p; }
        x = x * x % PRIME31.p;
        e >>= 1;
    }
    r
}
fn walk(start: u64, step: u64, label: char) -> Vec<(char, usize, Point)> {
    let mut x = start;
    let mut points = Vec::with_capacity(256);
    while points.len() < 256 {
        let y2 = (x*x%PRIME31.p*x%PRIME31.p +
            PRIME31.a*x%PRIME31.p + PRIME31.b) % PRIME31.p;
        if y2 == 0 || fpow(y2, (PRIME31.p - 1)/2) == 1 {
            let mut y = fpow(y2, (PRIME31.p + 1)/4);
            if y%2 != (points.len() as u64)%2 {
                y = (PRIME31.p - y)%PRIME31.p;
            }
            points.push((label, points.len(), Some((x,y))));
        }
        x = (x + step) % PRIME31.p;
    }
    points
}
fn query_prefix() -> Result<Vec<(char, usize, Point)>, String> {
    // Two deterministic public x-coordinate walks. No hidden target scalar exists.
    let mut out = walk(17, 1, 'H');
    out.extend(walk(100_000_003, 1_000_003, 'W'));
    if out.len() != 512 ||
       out[0].2 != Some((18,221278090)) ||
       out[1].2 != Some((19,1227894081)) ||
       out[256].2 != Some((101000006,1077058598)) {
        return Err("public deterministic x-walk fixture changed".into());
    }
    Ok(out)
}

fn run_arm(
    ar: &mut Arithmetic,
    base: &[Point],
    prefix: &[(char, usize, Point)],
    expected: &[(usize, usize); 2],
) -> Result<(), String> {
    let t0 = Instant::now();
    let mut signed = Vec::with_capacity(base.len() * 2);
    for &point in base {
        signed.push(point);
        signed.push(ar.neg(PRIME31, point));
    }
    let mut tab: HashMap<Point, (usize, usize)> =
        HashMap::with_capacity(signed.len() * (signed.len() + 1) / 2);
    let mut records = 0usize;
    for i in 0..signed.len() {
        for j in i..signed.len() {
            tab.entry(ar.add(PRIME31, signed[i], signed[j])).or_insert((i, j));
            records += 1;
        }
    }
    let distinct = tab.len();
    let wanted = if base.len() == 509 {
        (518_671, 518_085)
    } else {
        (917_335, 916_469)
    };
    if (records, distinct) != wanted {
        return Err(format!("pair-census mismatch for B{}: {records}/{distinct}", base.len()));
    }
    println!("run66 B={} signed={} pairs={} distinct={} table_build_ms={:.3}",
        base.len(), signed.len(), records, distinct, t0.elapsed().as_secs_f64() * 1e3);
    for (arm, c) in ['H', 'W'].iter().enumerate() {
        let mut hits2 = 0usize;
        let mut hits3 = 0usize;
        let mut witnesses = 0usize;
        let mut ec_checks = 0usize;
        let start = Instant::now();
        for &(cohort, _, target) in prefix.iter().filter(|&&(cohort, _, _)| cohort == *c) {
            if !ar.on_curve(PRIME31, target) {
                return Err("public target off-curve".into());
            }
            if tab.contains_key(&target) { hits2 += 1; }
            for &pt in &signed {
                let need = ar.add(PRIME31, target, ar.neg(PRIME31, pt));
                ec_checks += 1;
                if let Some(&(i, j)) = tab.get(&need) {
                    let partial = ar.add(PRIME31, pt, signed[i]);
                    let sum = ar.add(PRIME31, partial, signed[j]);
                    if sum != target {
                        return Err("generic grader witness failed group-law check".into());
                    }
                    witnesses += 1;
                    hits3 += 1;
                    break;
                }
            }
        }
        if (hits2, hits3) != expected[arm] {
            return Err(format!("B{} cohort {} expected {:?}, got ({hits2}, {hits3})",
                base.len(), c, expected[arm]));
        }
        println!("run66 B={} cohort={} N=256 pair_hits={} triple_hits={} \
                  positive_witnesses={} lookups={} prefix_scan_ms={:.3} arithmetic=PASS \
                  classification=GENERIC_GRADER_NOT_INDEX_CALCULUS",
            base.len(), c, hits2, hits3, witnesses, ec_checks,
            start.elapsed().as_secs_f64() * 1e3);
    }
    Ok(())
}
fn main() -> Result<(), String> {
    let mut ar = Arithmetic::default();
    let (dickson, hybrid) = check_base(&mut ar)?;
    let points = query_prefix()?;
    run_arm(&mut ar, &dickson, &points, &[(0, 15), (1, 16)])?;
    run_arm(&mut ar, &hybrid, &points, &[(0, 43), (1, 41)])?;
    println!("run66 original Prime31 Rust: PASS; no blind root or ECDLP log recovered");
    Ok(())
}
