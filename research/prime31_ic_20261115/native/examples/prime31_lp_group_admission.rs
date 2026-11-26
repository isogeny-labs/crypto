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

    fn residue(value: i128) -> u64 {
        value.rem_euclid(PRIME31.n as i128) as u64
    }

    /// Predetermined equations, with RHS values derived from the planted logs.
    /// The expected export is constructed independently of `admit`.
    fn mixed_fixture(negative_target: bool) -> (Capture, Vec<Export>) {
        let mut ar = Arithmetic::default();
        let (s, sl) = oriented_multiple(3, &mut ar);
        let (l, ll) = oriented_multiple(5, &mut ar);
        let (m, ml) = oriented_multiple(11, &mut ar);
        let mut q = ar.scalar(PRIME31, 7, Some(PRIME31.g)).unwrap();
        if negative_target {
            q.1 = PRIME31.p - q.1;
        }
        let (sl, ll, ml) = (sl as i128, ll as i128, ml as i128);
        let a = [
            residue(2 * sl),
            residue(-sl + ll - 7),
            residue(2 * sl - ll + 3 * ml - 28),
            residue(sl + ml - 14),
            residue(sl - ll + ml - 21),
        ];
        let n = PRIME31.n;
        // Replacing Q by -Q requires b -> -b. The expected target column
        // is +b for that negative-target control, and -b otherwise.
        let input_b = |b: u64| if negative_target { n - b } else { b };
        let target_coefficient = |b: u64| if negative_target { b } else { n - b };
        let capture = Capture {
            schema: "prime31-lp-capture/v1".into(),
            target: q,
            small_points: vec![(1, s)],
            large_points: vec![(9, l), (10, m)],
            rows: vec![
                Row { id: 40, small: vec![(1, 2)], large: vec![], a: a[0], b: 0 },
                Row { id: 41, small: vec![(1, n - 1)], large: vec![(9, 1)],
                    a: a[1], b: input_b(1) },
                Row { id: 42, small: vec![(1, 2)], large: vec![(9, n - 1), (10, 3)],
                    a: a[2], b: input_b(4) },
                Row { id: 43, small: vec![(1, 1)], large: vec![(10, 1)],
                    a: a[3], b: input_b(2) },
                Row { id: 44, small: vec![(1, 1)], large: vec![(9, n - 1), (10, 1)],
                    a: a[4], b: input_b(3) },
            ],
        };
        let expected = vec![
            Export { id: 40, small: vec![(1, 2)], large: vec![], rhs: a[0] },
            Export { id: 41, small: vec![(1, n - 1), (TARGET, target_coefficient(1))],
                large: vec![(9, 1)], rhs: a[1] },
            Export { id: 42, small: vec![(1, 2), (TARGET, target_coefficient(4))],
                large: vec![(9, n - 1), (10, 3)], rhs: a[2] },
            Export { id: 43, small: vec![(1, 1), (TARGET, target_coefficient(2))],
                large: vec![(10, 1)], rhs: a[3] },
            Export { id: 44, small: vec![(1, 1), (TARGET, target_coefficient(3))],
                large: vec![(9, n - 1), (10, 1)], rhs: a[4] },
        ];
        (capture, expected)
    }

    fn capture_value(c: &Capture) -> serde_json::Value {
        let rows: Vec<_> = c.rows.iter().map(|r| serde_json::json!({
            "id": r.id, "small": &r.small, "large": &r.large, "a": r.a, "b": r.b,
        })).collect();
        serde_json::json!({
            "schema": &c.schema, "target": c.target,
            "small_points": &c.small_points, "large_points": &c.large_points,
            "rows": rows,
        })
    }

    fn expected_jsonl(expected: &[Export]) -> Vec<u8> {
        let mut bytes = Vec::new();
        for row in expected {
            serde_json::to_writer(&mut bytes, row).unwrap();
            bytes.push(b'\n');
        }
        bytes
    }

    #[test]
    fn mixed_lp_arities_and_negative_target_have_exact_exports() {
        for negative_target in [false, true] {
            let (capture, expected) = mixed_fixture(negative_target);
            assert_eq!(capture.rows.iter().map(|r| r.large.len()).collect::<Vec<_>>(),
                vec![0, 1, 2, 1, 2]);
            assert_eq!(capture.rows[0].b, 0);
            let actual = admit(capture).unwrap();
            assert_eq!(actual, expected);
            assert_eq!(actual[0].small, vec![(1, 2)]);

            // R41 + R42 - 3R43 eliminates both LP columns. Together with
            // R40 its small/target matrix is [[2, 0], [-2, +/-1]], so
            // this fixed fixture has useful projected rank exactly two.
            let combination = [(1usize, 1i128), (2, 1), (3, -3)];
            for lp in [9u64, 10] {
                let value: i128 = combination.iter().map(|&(i, weight)| {
                    weight * actual[i].large.iter()
                        .find(|(id, _)| *id == lp).map_or(0, |(_, c)| *c) as i128
                }).sum();
                assert_eq!(residue(value), 0);
            }
            for (column, coefficient) in [(1u32, PRIME31.n - 2),
                (TARGET, if negative_target { PRIME31.n - 1 } else { 1 })]
            {
                let value: i128 = combination.iter().map(|&(i, weight)| {
                    weight * actual[i].small.iter()
                        .find(|(id, _)| *id == column).map_or(0, |(_, c)| *c) as i128
                }).sum();
                assert_eq!(residue(value), coefficient);
            }
            let rhs = residue(combination.iter()
                .map(|&(i, weight)| weight * actual[i].rhs as i128).sum());
            let mut ar = Arithmetic::default();
            let (_, sl) = oriented_multiple(3, &mut ar);
            assert_eq!(rhs, residue(-2 * sl as i128 + 7));
        }
    }

    #[test]
    fn identity_equations_and_inclusive_row_cap_are_accepted() {
        let (mut capture, _) = mixed_fixture(false);
        capture.small_points.clear();
        capture.large_points.clear();
        capture.rows = (0..CAP).map(|id| Row {
            id: id as u64, small: vec![], large: vec![], a: 0, b: 0,
        }).collect();
        // The final row is 0 = -7G + Q, not just a syntactic zero row.
        capture.rows[CAP - 1].a = PRIME31.n - 7;
        capture.rows[CAP - 1].b = 1;
        let actual = admit(capture).unwrap();
        assert_eq!(actual.len(), CAP);
        assert_eq!(actual[0], Export { id: 0, small: vec![], large: vec![], rhs: 0 });
        assert_eq!(actual[CAP - 1], Export { id: (CAP - 1) as u64,
            small: vec![(TARGET, PRIME31.n - 1)], large: vec![], rhs: PRIME31.n - 7 });
    }

    #[test]
    fn bounds_schema_and_point_aliases_refuse() {
        let mut c = fixture(); c.schema.push_str("-unknown"); assert!(admit(c).is_err());
        let mut c = fixture(); c.rows.clear(); assert!(admit(c).is_err());
        let mut c = fixture();
        c.rows = (0..=CAP).map(|id| Row {
            id: id as u64, small: vec![], large: vec![], a: 0, b: 0,
        }).collect();
        assert_eq!(admit(c).unwrap_err(), "schema or row cap");
        let mut c = fixture(); c.rows[0].a = PRIME31.n; assert!(admit(c).is_err());
        let mut c = fixture(); c.rows[0].b = PRIME31.n; assert!(admit(c).is_err());
        let mut c = fixture(); c.rows[0].small[0].1 = 0; assert!(admit(c).is_err());
        let mut c = fixture(); c.rows[0].large[0].1 = 0; assert!(admit(c).is_err());
        let mut c = fixture(); c.rows[0].small.push((1, 1)); assert!(admit(c).is_err());
        let mut c = fixture(); c.rows[0].small = (0..65).map(|id| (id, 1)).collect();
        assert!(admit(c).unwrap_err().contains("bounds or canonical form"));
        let mut c = fixture(); c.small_points.clear();
        assert_eq!(admit(c).unwrap_err(), "missing point mapping");
        let mut c = fixture(); c.large_points.clear();
        assert_eq!(admit(c).unwrap_err(), "missing point mapping");
        let mut c = fixture(); c.small_points[0].1.1 = 0; assert!(admit(c).is_err());
        let mut c = fixture(); c.small_points[0].1.0 = PRIME31.p; assert!(admit(c).is_err());
        let mut c = fixture(); c.target.0 = PRIME31.p; assert!(admit(c).is_err());
        let mut c = fixture(); c.large_points[1].0 = c.large_points[0].0;
        assert!(admit(c).is_err());
        let mut c = fixture();
        c.small_points = vec![c.small_points[0]; CAP + 1];
        assert_eq!(admit(c).unwrap_err(), "point table cap");
        let mut c = fixture();
        c.large_points = vec![c.large_points[0]; CAP + 1];
        assert_eq!(admit(c).unwrap_err(), "point table cap");
    }

    #[test]
    fn unknown_fields_and_wrong_target_sign_refuse() {
        let mut value = capture_value(&fixture());
        value["unexpected"] = serde_json::json!(true);
        assert!(serde_json::from_value::<Capture>(value).is_err());
        let mut value = capture_value(&fixture());
        value["rows"][0]["unexpected"] = serde_json::json!(true);
        assert!(serde_json::from_value::<Capture>(value).is_err());
        let (mut c, _) = mixed_fixture(false);
        c.target.1 = PRIME31.p - c.target.1;
        assert!(admit(c).unwrap_err().contains("row 41: nonzero group residual"));
    }

    #[test]
    fn checked_planted_cli_control_bundle() {
        let (valid, expected) = mixed_fixture(false);
        let valid_bytes = serde_json::to_vec_pretty(&capture_value(&valid)).unwrap();
        assert_eq!(admit(serde_json::from_slice(&valid_bytes).unwrap()).unwrap(), expected);
        let expected_bytes = expected_jsonl(&expected);

        let (mut late_invalid, _) = mixed_fixture(false);
        let last = late_invalid.rows.last_mut().unwrap();
        last.a = (last.a + 1) % PRIME31.n;
        let late_invalid_bytes = serde_json::to_vec_pretty(&capture_value(&late_invalid)).unwrap();
        let error = admit(serde_json::from_slice(&late_invalid_bytes).unwrap()).unwrap_err();
        assert!(error.contains("row 44: nonzero group residual"));

        let (negative, negative_expected) = mixed_fixture(true);
        let negative_bytes = serde_json::to_vec_pretty(&capture_value(&negative)).unwrap();
        assert_eq!(admit(serde_json::from_slice(&negative_bytes).unwrap()).unwrap(), negative_expected);
        let negative_expected_bytes = expected_jsonl(&negative_expected);

        let (directory, remove_after) = match std::env::var_os("PRIME31_ADMISSION_CONTROL_DIR") {
            Some(path) => (std::path::PathBuf::from(path), false),
            None => {
                let nonce = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
                (std::env::temp_dir().join(format!("prime31-admission-controls-{}-{nonce}",
                    std::process::id())), true)
            }
        };
        if remove_after {
            std::fs::create_dir(&directory).unwrap();
        } else {
            std::fs::create_dir_all(&directory).unwrap();
        }
        for (name, bytes) in [
            ("valid.json", valid_bytes),
            ("expected.jsonl", expected_bytes),
            ("late-invalid.json", late_invalid_bytes),
            ("negative-target.json", negative_bytes),
            ("negative-target-expected.jsonl", negative_expected_bytes),
        ] {
            let path = directory.join(name);
            // Allow deterministic replay, but do not overwrite a different
            // existing control under an immutable evidence pathname.
            match std::fs::read(&path) {
                Ok(prior) => assert_eq!(prior, bytes, "changed control: {name}"),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    std::fs::write(&path, &bytes).unwrap();
                }
                Err(error) => panic!("cannot read control {name}: {error}"),
            }
            assert_eq!(std::fs::read(&path).unwrap(), bytes);
        }
        if remove_after {
            std::fs::remove_dir_all(&directory).unwrap();
        } else {
            eprintln!("planted CLI controls (not natural yield): {}", directory.display());
        }
    }
}

