//! Run66: exact original-Rust three-summand public-support GRADER.
//! Do not use its generic pair dictionary as a non-generic root extractor.
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Point, PRIME31};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const HYBRID_POINTS: &str =
    include_str!("../../../prime31_f4_admission_20261121/RUN66_HYBRID_BASE.csv");
const PUBLIC_PREFIX: &str =
    include_str!("../../../prime31_f4_admission_20261121/RUN66_PUBLIC_PREFIX256.csv");

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
    let dickson = Base::build(PRIME31, 10, ar)?.points.into_iter()
        .map(Some).collect::<Vec<_>>();
    if dickson.len() != 509 { return Err("Dickson B509 changed".into()); }
    let hybrid = HYBRID_POINTS.lines().filter(|s| !s.is_empty())
        .map(parse_xy).collect::<Result<Vec<_>, _>>()?;
    if hybrid.len() != 677 { return Err("hybrid B677 fixture changed".into()); }
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

fn query_prefix() -> Result<Vec<(char, usize, Point)>, String> {
    let mut out = Vec::new();
    for line in PUBLIC_PREFIX.lines().skip(1).filter(|s| !s.is_empty()) {
        let fields = line.split(',').collect::<Vec<_>>();
        if fields.len() != 4 { return Err("invalid public fixture column count".into()); }
        let cohort = fields[0].chars().next().ok_or("empty cohort")?;
        if !matches!(cohort, 'H' | 'W') || fields[0].len() != 1 {
            return Err("invalid cohort code".into());
        }
        let index = fields[1].parse::<usize>().map_err(|e| e.to_string())?;
        let pt = parse_xy(&format!("{},{}", fields[2], fields[3]))?;
        out.push((cohort, index, pt));
    }
    if out.len() != 512 { return Err("public prefix must contain 2*256".into()); }
    for (c, chunk) in [('H', &out[..256]), ('W', &out[256..])] {
        for (i, &(got_c, got_i, _)) in chunk.iter().enumerate() {
            if c != got_c || got_i != i {
                return Err("public cohort ordering changed".into());
            }
        }
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
                    let sum = ar.add(PRIME31, ar.add(PRIME31, pt, signed[i]), signed[j]);
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
    run_arm(&mut ar, &dickson, &points, &[(0, 20), (0, 26)])?;
    run_arm(&mut ar, &hybrid, &points, &[(0, 46), (0, 45)])?;
    println!("run66 original Prime31 Rust: PASS; no blind root or ECDLP log recovered");
    Ok(())
}
