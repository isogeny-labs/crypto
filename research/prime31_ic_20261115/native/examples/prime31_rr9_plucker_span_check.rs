//! Independent replay for prime31_rr9_plucker_span.
//! Minors use permutation expansion; the producer uses Gaussian elimination.

use prime31_algebraic_ic::{Arithmetic, Point, PRIME31};
use serde::Serialize;
use serde_json::{json, Value};

const P: u64 = 2_147_483_647;
const G: Point = Some((625_037_715, 2_100_627_251));
const W: [u64; 8] = [1, 3, 5, 7, 11, 13, 17, 19];
const C: usize = 70;
const PROTOCOL_SHA256: &str = "ab67f98a2f1e58f4e5f6c610d5b4ca6ab3fe7eabc8050e94f2103200322ca62a";
const PARENT_SOURCE: &str = "71859709335f1178bf7ffba043bdc989bddf6ac9";
const ORIGINAL_ARITHMETIC: &str = "1df714645aa683cf975bc76fcacb9842cb4780fd";

type Word = [u8; 4];
type Row = [u64; 8];
type Vector = [u64; C];

#[derive(Clone, Copy)]
struct Columns {
    left: [usize; 4],
    right: [usize; 4],
    negative: bool,
}

#[inline]
fn add(a: u64, b: u64) -> u64 {
    ((a as u128 + b as u128) % P as u128) as u64
}

#[inline]
fn sub(a: u64, b: u64) -> u64 {
    ((a as u128 + P as u128 - b as u128) % P as u128) as u64
}

#[inline]
fn neg(a: u64) -> u64 {
    if a == 0 {
        0
    } else {
        P - a
    }
}

#[inline]
fn mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}

fn pow(mut a: u64, mut e: u64) -> u64 {
    let mut out = 1;
    while e != 0 {
        if e & 1 != 0 {
            out = mul(out, a);
        }
        a = mul(a, a);
        e >>= 1;
    }
    out
}

fn inv(a: u64) -> u64 {
    if a == 0 {
        0
    } else {
        pow(a, P - 2)
    }
}

fn columns() -> Vec<Columns> {
    let mut out = Vec::with_capacity(C);
    for a in 0..8 {
        for b in a + 1..8 {
            for c in b + 1..8 {
                for d in c + 1..8 {
                    let left = [a, b, c, d];
                    let mut right = [0; 4];
                    let mut n = 0;
                    for j in 0..8 {
                        if !left.contains(&j) {
                            right[n] = j;
                            n += 1;
                        }
                    }
                    out.push(Columns {
                        left,
                        right,
                        negative: (a + b + c + d) % 2 != 0,
                    });
                }
            }
        }
    }
    out
}

fn permutations() -> Vec<([usize; 4], bool)> {
    let mut out = Vec::with_capacity(24);
    for a in 0..4 {
        for b in 0..4 {
            if b == a {
                continue;
            }
            for c in 0..4 {
                if c == a || c == b {
                    continue;
                }
                for d in 0..4 {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    let permutation = [a, b, c, d];
                    let mut inversions = 0;
                    for i in 0..4 {
                        for j in i + 1..4 {
                            inversions += usize::from(permutation[i] > permutation[j]);
                        }
                    }
                    out.push((permutation, inversions % 2 != 0));
                }
            }
        }
    }
    out
}

fn det4_permutation(rows: &[Row; 4], selected: [usize; 4]) -> u64 {
    let mut determinant = 0;
    for (permutation, negative) in permutations() {
        let mut term = 1;
        for i in 0..4 {
            term = mul(term, rows[i][selected[permutation[i]]]);
        }
        determinant = if negative {
            sub(determinant, term)
        } else {
            add(determinant, term)
        };
    }
    determinant
}

fn target_row(q: Point, target: Point) -> Result<Row, String> {
    let (x, y) = q.ok_or("infinite signed slot leaf")?;
    let (r, s) = target.ok_or("infinite public residual")?;
    let u = sub(x, r);
    let v = add(y, s);
    let mut out = [0; 8];
    let mut xj = 1;
    for j in 0..4 {
        out[j] = mul(u, xj);
        out[4 + j] = mul(v, xj);
        xj = mul(xj, x);
    }
    Ok(out)
}

fn vector(rows: &[Vec<Row>], begin: usize, word: Word, selections: &[Columns]) -> Vector {
    let selected_rows = [
        rows[begin][word[0] as usize],
        rows[begin + 1][word[1] as usize],
        rows[begin + 2][word[2] as usize],
        rows[begin + 3][word[3] as usize],
    ];
    let mut out = [0; C];
    for (i, selection) in selections.iter().enumerate() {
        let mut value = det4_permutation(
            &selected_rows,
            if begin == 0 {
                selection.left
            } else {
                selection.right
            },
        );
        if begin != 0 && selection.negative {
            value = neg(value);
        }
        out[i] = value;
    }
    out
}

fn canonical_base(arithmetic: &mut Arithmetic, size: usize) -> Vec<Point> {
    let mut out = Vec::new();
    for x in 0..10_000 {
        let rhs = arithmetic.rhs(PRIME31, x);
        if let Some(y) = arithmetic.sqrt(rhs, P) {
            if y != 0 {
                let point = Some((x, y.min(P - y)));
                if arithmetic.on_curve(PRIME31, point) {
                    out.push(point);
                }
                if out.len() == size {
                    break;
                }
            }
        }
    }
    out
}

fn supports(arithmetic: &mut Arithmetic, base: &[Point]) -> Result<Vec<Vec<Point>>, String> {
    let mut slots = Vec::with_capacity(8);
    let mut positive_x = std::collections::BTreeSet::new();
    for weight in W {
        let mut positive = Vec::with_capacity(base.len());
        for &point in base {
            let image = arithmetic.scalar(PRIME31, weight, point);
            let (x, _) = image.ok_or("weighted image is infinity")?;
            if !positive_x.insert(x) {
                return Err("weighted positive x overlap".into());
            }
            positive.push(image);
        }
        let mut signed = positive.clone();
        signed.extend(
            positive
                .into_iter()
                .map(|point| arithmetic.neg(PRIME31, point)),
        );
        slots.push(signed);
    }
    Ok(slots)
}

fn planted_target(arithmetic: &mut Arithmetic, slots: &[Vec<Point>]) -> Point {
    slots
        .iter()
        .fold(None, |sum, slot| arithmetic.add(PRIME31, sum, slot[0]))
}

fn parse_u64(value: &Value, name: &str) -> Result<u64, String> {
    value.as_u64().ok_or_else(|| format!("invalid {name}"))
}

fn parse_word(value: &Value, width: usize) -> Result<Word, String> {
    let values = value.as_array().ok_or("word is not an array")?;
    if values.len() != 4 {
        return Err("word length is not four".into());
    }
    let mut out = [0; 4];
    for (i, value) in values.iter().enumerate() {
        let coordinate = parse_u64(value, "word coordinate")?;
        if coordinate >= width as u64 {
            return Err("word coordinate is outside signed slot".into());
        }
        out[i] = coordinate as u8;
    }
    Ok(out)
}

fn parse_vector(value: &Value) -> Result<Vec<u64>, String> {
    let values = value.as_array().ok_or("vector is not an array")?;
    if values.len() != C {
        return Err("vector length is not 70".into());
    }
    values
        .iter()
        .map(|value| {
            let coordinate = parse_u64(value, "vector coordinate")?;
            if coordinate >= P {
                return Err("noncanonical field coordinate".into());
            }
            Ok(coordinate)
        })
        .collect()
}

fn rank_and_pivots(matrix: &[Vec<u64>]) -> (usize, Vec<usize>) {
    if matrix.is_empty() {
        return (0, Vec::new());
    }
    let mut a = matrix.to_vec();
    let mut row = 0;
    let mut pivots = Vec::new();
    for column in 0..a[0].len() {
        let Some(pivot) = (row..a.len()).find(|&i| a[i][column] != 0) else {
            continue;
        };
        a.swap(row, pivot);
        let inverse = inv(a[row][column]);
        for j in column..a[0].len() {
            a[row][j] = mul(a[row][j], inverse);
        }
        for i in 0..a.len() {
            if i == row {
                continue;
            }
            let factor = a[i][column];
            if factor == 0 {
                continue;
            }
            for j in column..a[0].len() {
                a[i][j] = sub(a[i][j], mul(factor, a[row][j]));
            }
        }
        pivots.push(column);
        row += 1;
        if row == a.len() {
            break;
        }
    }
    (row, pivots)
}

fn determinant(matrix: &[Vec<u64>]) -> u64 {
    if matrix.is_empty() || matrix.iter().any(|row| row.len() != matrix.len()) {
        return 0;
    }
    let mut a = matrix.to_vec();
    let mut out = 1;
    for column in 0..a.len() {
        let Some(pivot) = (column..a.len()).find(|&i| a[i][column] != 0) else {
            return 0;
        };
        if pivot != column {
            a.swap(column, pivot);
            out = neg(out);
        }
        let value = a[column][column];
        out = mul(out, value);
        let inverse = inv(value);
        for i in column + 1..a.len() {
            let factor = mul(a[i][column], inverse);
            for j in column + 1..a.len() {
                a[i][j] = sub(a[i][j], mul(factor, a[column][j]));
            }
            a[i][column] = 0;
        }
    }
    out
}

fn pairing(left: &[Vec<u64>], right: &[Vec<u64>]) -> Vec<Vec<u64>> {
    left.iter()
        .map(|l| {
            right
                .iter()
                .map(|r| l.iter().zip(r).fold(0, |sum, (&a, &b)| add(sum, mul(a, b))))
                .collect()
        })
        .collect()
}

#[derive(Default)]
struct ReplayBasis {
    rows: Vec<Vec<u64>>,
    pivots: Vec<usize>,
}

impl ReplayBasis {
    fn insert(&mut self, mut row: Vec<u64>) -> bool {
        for (basis, &pivot) in self.rows.iter().zip(&self.pivots) {
            let coefficient = row[pivot];
            if coefficient == 0 {
                continue;
            }
            for j in pivot..C {
                row[j] = sub(row[j], mul(coefficient, basis[j]));
            }
        }
        let Some(pivot) = row.iter().position(|&coordinate| coordinate != 0) else {
            return false;
        };
        let inverse = inv(row[pivot]);
        for coordinate in &mut row[pivot..] {
            *coordinate = mul(*coordinate, inverse);
        }
        let position = self.pivots.binary_search(&pivot).unwrap_err();
        self.rows.insert(position, row);
        self.pivots.insert(position, pivot);
        true
    }
}

struct SpanReplay {
    rank: usize,
    vectors_examined: u64,
    nonzero_vectors_examined: u64,
    complete_enumeration: bool,
}

fn replay_span(rows: &[Vec<Row>], begin: usize, width: usize) -> SpanReplay {
    let selections = columns();
    let vectors_total = (width as u64).pow(4);
    let mut basis = ReplayBasis::default();
    let mut vectors_examined = 0;
    let mut nonzero_vectors_examined = 0;
    'all: for a in 0..width {
        for b in 0..width {
            for c in 0..width {
                for d in 0..width {
                    let generated = vector(
                        rows,
                        begin,
                        [a as u8, b as u8, c as u8, d as u8],
                        &selections,
                    );
                    vectors_examined += 1;
                    nonzero_vectors_examined +=
                        u64::from(generated.iter().any(|&coordinate| coordinate != 0));
                    basis.insert(generated.to_vec());
                    if basis.rows.len() == C {
                        break 'all;
                    }
                }
            }
        }
    }
    SpanReplay {
        rank: basis.rows.len(),
        vectors_examined,
        nonzero_vectors_examined,
        complete_enumeration: vectors_examined == vectors_total,
    }
}

fn verify_span(
    record: &Value,
    rows: &[Vec<Row>],
    begin: usize,
    width: usize,
) -> Result<Vec<Vec<u64>>, String> {
    let rank = parse_u64(&record["rank"], "span rank")? as usize;
    if rank > C {
        return Err("rank exceeds ambient dimension".into());
    }
    let words = record["basis_words"]
        .as_array()
        .ok_or("missing basis words")?;
    let vectors = record["basis_vectors"]
        .as_array()
        .ok_or("missing basis vectors")?;
    if words.len() != rank || vectors.len() != rank {
        return Err("basis length and rank disagree".into());
    }
    let selections = columns();
    let mut parsed = Vec::with_capacity(rank);
    for (word, declared) in words.iter().zip(vectors) {
        let word = parse_word(word, width)?;
        let declared = parse_vector(declared)?;
        let actual = vector(rows, begin, word, &selections).to_vec();
        if declared != actual {
            return Err("basis vector does not match source word".into());
        }
        parsed.push(declared);
    }
    let (actual_rank, pivots) = rank_and_pivots(&parsed);
    if actual_rank != rank || record["pivot_columns"] != json!(pivots) {
        return Err("basis rank or pivot columns disagree".into());
    }
    let declared_determinant = parse_u64(&record["basis_determinant"], "basis determinant")?;
    let actual_determinant = if rank == C { determinant(&parsed) } else { 0 };
    if declared_determinant != actual_determinant || (rank == C && actual_determinant == 0) {
        return Err("basis determinant certificate failed".into());
    }
    let vectors_total = (width as u64).pow(4);
    if parse_u64(&record["vectors_total"], "vectors total")? != vectors_total {
        return Err("vector population mismatch".into());
    }
    let replay = replay_span(rows, begin, width);
    let declared_nonzero = parse_u64(
        &record["nonzero_vectors_examined"],
        "nonzero vectors examined",
    )?;
    if parse_u64(&record["vectors_examined"], "vectors examined")? != replay.vectors_examined
        || declared_nonzero != replay.nonzero_vectors_examined
        || record["complete_enumeration"].as_bool() != Some(replay.complete_enumeration)
        || rank != replay.rank
    {
        return Err("independent span replay mismatch".into());
    }
    if rank < C && replay.vectors_examined != vectors_total {
        return Err("deficient rank lacks complete enumeration".into());
    }
    Ok(parsed)
}

#[derive(Serialize)]
struct CaseCheck {
    id: String,
    left_rank: usize,
    right_rank: usize,
    pairing_rank: usize,
}

#[derive(Serialize)]
struct Check {
    schema: &'static str,
    cases: Vec<CaseCheck>,
    verdict: String,
    controls_passed: bool,
    certificate_replay: &'static str,
    pcc: &'static str,
}

fn point_from_value(value: &Value) -> Result<Point, String> {
    if value.is_null() {
        return Ok(None);
    }
    let pair = value.as_array().ok_or("point is not a pair")?;
    if pair.len() != 2 {
        return Err("point length is not two".into());
    }
    Ok(Some((
        parse_u64(&pair[0], "point x")?,
        parse_u64(&pair[1], "point y")?,
    )))
}

fn verify(value: &Value) -> Result<Check, String> {
    if value["schema"] != "prime31.rr9-plucker-span/v1"
        || value["protocol_sha256"] != PROTOCOL_SHA256
        || value["parent_source_commit"] != PARENT_SOURCE
        || value["original_arithmetic_commit"] != ORIGINAL_ARITHMETIC
        || value["p"] != P
        || value["group_order"] != PRIME31.n
        || value["weights"] != json!(W)
    {
        return Err("source, protocol, field, or weight binding mismatch".into());
    }
    let controls = &value["controls"];
    for name in [
        "random_left_rank",
        "random_right_rank",
        "random_pairing_rank",
    ] {
        if controls[name] != C {
            return Err(format!("random control {name} failed"));
        }
    }
    for name in [
        "synthetic_left_rank",
        "synthetic_right_rank",
        "synthetic_pairing_rank",
    ] {
        if controls[name] != 7 {
            return Err(format!("synthetic control {name} failed"));
        }
    }
    let cases = value["cases"].as_array().ok_or("missing cases")?;
    let expected = [
        ("N4", 4, false),
        ("P4", 4, true),
        ("N8", 8, false),
        ("P8", 8, true),
    ];
    if cases.len() != expected.len() {
        return Err("wrong case count".into());
    }
    let mut checks = Vec::new();
    for (case, &(id, size, planted)) in cases.iter().zip(&expected) {
        if case["id"] != id
            || case["factor_base_size"] != size
            || case["signed_slot_width"] != 2 * size
            || case["planted_control"] != planted
        {
            return Err("case identity mismatch".into());
        }
        let mut arithmetic = Arithmetic::default();
        let base = canonical_base(&mut arithmetic, size);
        if case["base_x"]
            != json!(base
                .iter()
                .map(|point| point.unwrap().0)
                .collect::<Vec<_>>())
        {
            return Err("canonical base mismatch".into());
        }
        let slots = supports(&mut arithmetic, &base)?;
        let target = if planted {
            planted_target(&mut arithmetic, &slots)
        } else {
            G
        };
        if point_from_value(&case["target"])? != target {
            return Err("public target mismatch".into());
        }
        let rows: Vec<Vec<Row>> = slots
            .iter()
            .map(|slot| {
                slot.iter()
                    .map(|&point| target_row(point, target))
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<_, _>>()?;
        let width = 2 * size;
        let left = verify_span(&case["left"], &rows, 0, width)?;
        let right = verify_span(&case["right_signed_complement"], &rows, 4, width)?;
        let pair = pairing(&left, &right);
        let (pairing_rank, _) = rank_and_pivots(&pair);
        let pairing_determinant = if pair.len() == C && pair[0].len() == C {
            determinant(&pair)
        } else {
            0
        };
        if case["pairing_rank"] != pairing_rank
            || case["pairing_determinant"] != pairing_determinant
            || (pairing_rank == C && pairing_determinant == 0)
        {
            return Err("restricted pairing certificate failed".into());
        }
        checks.push(CaseCheck {
            id: id.into(),
            left_rank: left.len(),
            right_rank: right.len(),
            pairing_rank,
        });
    }
    let b8: Vec<_> = checks
        .iter()
        .filter(|case| case.id.ends_with('8'))
        .collect();
    let verdict = if b8
        .iter()
        .all(|case| case.left_rank == C && case.right_rank == C && case.pairing_rank == C)
    {
        "KILL_FIXED_LINEAR_COMPRESSION"
    } else if b8.iter().all(|case| case.pairing_rank <= 35) {
        "ADVANCE_LINEAR_COMPRESSION"
    } else {
        "WEAK_OR_INCONCLUSIVE"
    };
    if value["verdict"] != verdict {
        return Err("verdict does not follow frozen rule".into());
    }
    Ok(Check {
        schema: "prime31.rr9-plucker-span-check/v1",
        cases: checks,
        verdict: verdict.into(),
        controls_passed: true,
        certificate_replay: "verified_complete",
        pcc: "unverified",
    })
}

fn mutation_rejections(value: &Value) -> Result<Value, String> {
    let mut mutations: Vec<(&str, Box<dyn Fn(&mut Value)>)> = vec![
        (
            "target",
            Box::new(|v| {
                let x = v["cases"][0]["target"][0].as_u64().unwrap();
                v["cases"][0]["target"][0] = json!((x + 1) % P);
            }),
        ),
        (
            "word",
            Box::new(|v| {
                let x = v["cases"][0]["left"]["basis_words"][0][0].as_u64().unwrap();
                v["cases"][0]["left"]["basis_words"][0][0] = json!((x + 1) % 8);
            }),
        ),
        (
            "coordinate",
            Box::new(|v| {
                let x = v["cases"][0]["left"]["basis_vectors"][0][0]
                    .as_u64()
                    .unwrap();
                v["cases"][0]["left"]["basis_vectors"][0][0] = json!((x + 1) % P);
            }),
        ),
        (
            "pivot",
            Box::new(|v| {
                let x = v["cases"][0]["left"]["pivot_columns"][0].as_u64().unwrap();
                v["cases"][0]["left"]["pivot_columns"][0] = json!((x + 1) % C as u64);
            }),
        ),
        (
            "determinant",
            Box::new(|v| {
                let x = v["cases"][0]["left"]["basis_determinant"].as_u64().unwrap();
                v["cases"][0]["left"]["basis_determinant"] = json!((x + 1) % P);
            }),
        ),
        (
            "rank",
            Box::new(|v| {
                let rank = v["cases"][0]["left"]["rank"].as_u64().unwrap();
                v["cases"][0]["left"]["rank"] = json!((rank + 1) % (C as u64 + 1));
            }),
        ),
    ];
    let mut rejected = Vec::new();
    for (name, mutate) in mutations.drain(..) {
        let mut changed = value.clone();
        mutate(&mut changed);
        if verify(&changed).is_ok() {
            return Err(format!("mutation {name} was accepted"));
        }
        rejected.push(name);
    }
    Ok(json!({
        "schema":"prime31.rr9-plucker-span-mutations/v1",
        "rejected":rejected,
        "count":rejected.len(),
        "pcc":"unverified"
    }))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 || !matches!(args[1].as_str(), "check" | "mutations") {
        return Err("usage: prime31_rr9_plucker_span_check check|mutations OUTPUT.json".into());
    }
    let value: Value = serde_json::from_slice(&std::fs::read(&args[2])?)?;
    if args[1] == "check" {
        println!("{}", serde_json::to_string_pretty(&verify(&value)?)?);
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&mutation_rejections(&value)?)?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation_determinant_matches_known_matrix() {
        let rows = [
            [1, 2, 3, 4, 0, 0, 0, 0],
            [0, 1, 4, 2, 0, 0, 0, 0],
            [5, 2, 1, 3, 0, 0, 0, 0],
            [1, 0, 2, 1, 0, 0, 0, 0],
        ];
        let matrix = rows.iter().map(|row| row[..4].to_vec()).collect::<Vec<_>>();
        assert_eq!(det4_permutation(&rows, [0, 1, 2, 3]), determinant(&matrix));
    }

    #[test]
    fn rank_replay_detects_dependent_row() {
        let mut matrix = vec![vec![0; 7]; 8];
        for i in 0..7 {
            matrix[i][i] = 1;
        }
        matrix[7][0] = 2;
        assert_eq!(rank_and_pivots(&matrix).0, 7);
    }
}
