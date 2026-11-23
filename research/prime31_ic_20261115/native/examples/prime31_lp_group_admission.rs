//! Fail-closed bridge from captured ordinary Prime31 group equations to
//! lp_graph_eval's PartialRelation JSONL. No relation generator or solver.
//! Protocol: research/lp_graph_algorithms_20261123/PRIME31_GROUP_ADMISSION_20261123.md.
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Write};

const TARGET: u32 = u32::MAX;
const CAP: usize = 4096;
const BYTES: usize = 8 * 1024 * 1024;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Capture {
    schema: String,
    target: (u64, u64),
    small_points: Vec<(u32, (u64, u64))>,
    large_points: Vec<(u64, (u64, u64))>,
    rows: Vec<Row>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Row {
    id: u64,
    small: Vec<(u32, u64)>,
    large: Vec<(u64, u64)>,
    a: u64,
    b: u64,
}

/// Same wire schema as the existing PartialRelation (no root-crate dependency).
#[derive(Debug, PartialEq, Eq, Serialize)]
struct Export {
    id: u64,
    small: Vec<(u32, u64)>,
    large: Vec<(u64, u64)>,
    rhs: u64,
}

fn table<K: Copy + Ord>(
    entries: Vec<(K, (u64, u64))>,
    seen_x: &mut BTreeSet<u64>,
    ar: &mut Arithmetic,
) -> Result<BTreeMap<K, (u64, u64)>, String> {
    if entries.len() > CAP {
        return Err("point table cap".into());
    }
    let mut out = BTreeMap::new();
    for (id, point) in entries {
        if point.1 == 0 || point.1 > PRIME31.p / 2
            || !ar.on_curve(PRIME31, Some(point))
            || !seen_x.insert(point.0)
            || out.insert(id, point).is_some()
        {
            return Err("invalid/noncanonical/duplicate point or id".into());
        }
    }
    Ok(out)
}

fn canonical<K: Ord>(terms: &[(K, u64)]) -> bool {
    terms.iter().all(|(_, c)| *c > 0 && *c < PRIME31.n)
        && terms.windows(2).all(|w| w[0].0 < w[1].0)
}

fn add_terms<K: Ord>(
    terms: &[(K, u64)],
    points: &BTreeMap<K, (u64, u64)>,
    mut sum: Point,
    ar: &mut Arithmetic,
) -> Result<Point, String> {
    for (id, coef) in terms {
        let p = *points.get(id).ok_or("missing point mapping")?;
        let term = ar.scalar(PRIME31, *coef, Some(p));
        sum = ar.add(PRIME31, sum, term);
    }
    Ok(sum)
}

fn admit(c: Capture) -> Result<Vec<Export>, String> {
    if c.schema != "prime31-lp-capture/v1" || c.rows.is_empty() || c.rows.len() > CAP {
        return Err("schema or row cap".into());
    }
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar)?;
    // PRIME31's certified cofactor one means every on-curve point is in <G>.
    let q = Some(c.target);
    if !ar.on_curve(PRIME31, q) {
        return Err("invalid public target".into());
    }
    let mut seen_x = BTreeSet::new();
    let small = table(c.small_points, &mut seen_x, &mut ar)?;
    let large = table(c.large_points, &mut seen_x, &mut ar)?;
    if small.contains_key(&TARGET) {
        return Err("reserved target column in factor base".into());
    }

    let mut ids = BTreeSet::new();
    let mut result = Vec::with_capacity(c.rows.len());
    for row in c.rows {
        if !ids.insert(row.id) || row.small.len() > 64 || row.large.len() > 2
            || row.a >= PRIME31.n || row.b >= PRIME31.n
            || !canonical(&row.small) || !canonical(&row.large)
        {
            return Err(format!("row {}: invalid id, bounds or canonical form", row.id));
        }
        let lhs = add_terms(&row.small, &small, None, &mut ar)?;
        let lhs = add_terms(&row.large, &large, lhs, &mut ar)?;
        let ag = ar.scalar(PRIME31, row.a, Some(PRIME31.g));
        let bq = ar.scalar(PRIME31, row.b, q);
        if lhs != ar.add(PRIME31, ag, bq) {
            return Err(format!("row {}: nonzero group residual", row.id));
        }
        let mut terms = row.small;
        if row.b != 0 {
            terms.push((TARGET, PRIME31.n - row.b));
        }
        result.push(Export { id: row.id, small: terms, large: row.large, rhs: row.a });
    }
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let input = args.next().ok_or("usage: prime31_lp_group_admission CAPTURE.json")?;
    if args.next().is_some() {
        return Err("expected one capture path".into());
    }
    let mut bytes = Vec::new();
    std::fs::File::open(input)?.take((BYTES + 1) as u64).read_to_end(&mut bytes)?;
    if bytes.len() > BYTES {
        return Err("input byte cap".into());
    }
    let rows = admit(serde_json::from_slice(&bytes)?)?;
    // Stage all rows before emission: malformed input never emits a prefix.
    let mut output = Vec::new();
    for row in &rows {
        serde_json::to_writer(&mut output, row)?;
        output.push(b'\n');
    }
    std::io::stdout().lock().write_all(&output)?;
    eprintln!("group replay: {} rows, {} two-LP; custody of collector not established",
        rows.len(), rows.iter().filter(|r| r.large.len() == 2).count());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    // Planted known-answer controls only, NOT natural relation yield.
    fn oriented_multiple(k: u64, ar: &mut Arithmetic) -> ((u64,u64), u64) {
        let p = ar.scalar(PRIME31, k, Some(PRIME31.g)).unwrap();
        if p.1 <= PRIME31.p / 2 { (p,k) }
        else { ((p.0, PRIME31.p-p.1), PRIME31.n-k) }
    }
    fn fixture() -> Capture {
        let mut ar = Arithmetic::default();
        let (s, sl) = oriented_multiple(3, &mut ar);
        let (l, ll) = oriented_multiple(5, &mut ar);
        let (m, ml) = oriented_multiple(11, &mut ar);
        let q = ar.scalar(PRIME31, 7, Some(PRIME31.g)).unwrap();
        let n = PRIME31.n;
        // 2S-L+3M=aG+4Q, with the signs of canonical points accounted for.
        let a = (2*sl + n-ll + 3*ml + n-28) % n;
        Capture {
            schema: "prime31-lp-capture/v1".into(), target:q,
            small_points:vec![(1,s)], large_points:vec![(9,l),(10,m)],
            rows:vec![Row{id:42,small:vec![(1,2)],large:vec![(9,n-1),(10,3)],a,b:4}],
        }
    }
    #[test]
    fn signed_two_lp_and_target_export() {
        let c = fixture();
        let a = c.rows[0].a;
        let rows = admit(c).unwrap();
        assert_eq!(rows, vec![Export{id:42,small:vec![(1,2),(TARGET,PRIME31.n-4)],
            large:vec![(9,PRIME31.n-1),(10,3)],rhs:a}]);
    }
    #[test]
    fn corrupt_rows_and_tables_refuse() {
        let mut c=fixture(); c.rows[0].a=(c.rows[0].a+1)%PRIME31.n; assert!(admit(c).is_err());
        let mut c=fixture(); c.rows[0].b=5; assert!(admit(c).is_err());
        let mut c=fixture(); c.rows[0].large[0].1=PRIME31.n; assert!(admit(c).is_err());
        let mut c=fixture(); c.rows[0].large[0].0=8; assert!(admit(c).is_err());
        let mut c=fixture(); c.rows[0].large.push((11,1)); assert!(admit(c).is_err());
        let mut c=fixture(); c.rows[0].large.swap(0,1); assert!(admit(c).is_err());
        let mut c=fixture(); c.small_points[0].0=TARGET; assert!(admit(c).is_err());
        let mut c=fixture(); c.large_points[0].1.1=PRIME31.p-c.large_points[0].1.1; assert!(admit(c).is_err());
        let mut c=fixture(); c.target.1=PRIME31.p; assert!(admit(c).is_err());
        let mut c=fixture(); c.large_points[1].1=c.small_points[0].1; assert!(admit(c).is_err());
        let mut c=fixture(); c.rows.push(fixture().rows.remove(0)); assert!(admit(c).is_err());
    }
}
