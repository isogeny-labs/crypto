//! Exact, deterministic Pluecker-span and restricted-pairing rank gate.
//! No clocks, generic EC half-sum tables, SAT, F4, or ECDLP solve occur here.
//! Protocol: RR9_PLUCKER_SPAN_PROTOCOL_20261119.md.

use prime31_algebraic_ic::{Arithmetic, Counts, Point, PRIME31};
use serde::Serialize;

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

#[derive(Clone, Copy, Default, Debug, Serialize)]
struct FieldCounts {
    additions_subtractions: u64,
    multiplications: u64,
    inversions: u64,
}

impl FieldCounts {
    fn add_assign(&mut self, other: Self) {
        self.additions_subtractions += other.additions_subtractions;
        self.multiplications += other.multiplications;
        self.inversions += other.inversions;
    }
}

#[derive(Default)]
struct Field {
    counts: FieldCounts,
}

impl Field {
    #[inline]
    fn add(&mut self, a: u64, b: u64) -> u64 {
        self.counts.additions_subtractions += 1;
        let z = a + b;
        if z >= P {
            z - P
        } else {
            z
        }
    }

    #[inline]
    fn sub(&mut self, a: u64, b: u64) -> u64 {
        self.counts.additions_subtractions += 1;
        if a >= b {
            a - b
        } else {
            a + P - b
        }
    }

    #[inline]
    fn neg(&mut self, a: u64) -> u64 {
        self.counts.additions_subtractions += 1;
        if a == 0 {
            0
        } else {
            P - a
        }
    }

    #[inline]
    fn mul(&mut self, a: u64, b: u64) -> u64 {
        self.counts.multiplications += 1;
        let z = a * b;
        let z = (z & P) + (z >> 31);
        let z = (z & P) + (z >> 31);
        if z >= P {
            z - P
        } else {
            z
        }
    }

    fn pow(&mut self, mut a: u64, mut e: u64) -> u64 {
        let mut out = 1;
        while e != 0 {
            if e & 1 != 0 {
                out = self.mul(out, a);
            }
            a = self.mul(a, a);
            e >>= 1;
        }
        out
    }

    fn inv(&mut self, a: u64) -> u64 {
        assert_ne!(a, 0);
        self.counts.inversions += 1;
        self.pow(a, P - 2)
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
                    assert_eq!(n, 4);
                    out.push(Columns {
                        left,
                        right,
                        negative: (a + b + c + d) % 2 != 0,
                    });
                }
            }
        }
    }
    assert_eq!(out.len(), C);
    out
}

fn target_row(q: Point, target: Point, field: &mut Field) -> Row {
    let (x, y) = q.expect("signed slot leaf is finite");
    let (r, s) = target.expect("public residual is finite");
    let u = field.sub(x, r);
    let v = field.add(y, s);
    let mut out = [0; 8];
    let mut xj = 1;
    for j in 0..4 {
        out[j] = field.mul(u, xj);
        out[4 + j] = field.mul(v, xj);
        xj = field.mul(xj, x);
    }
    out
}

fn det4(rows: &[Row; 4], selected: [usize; 4], field: &mut Field) -> u64 {
    let mut a = [[0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            a[i][j] = rows[i][selected[j]];
        }
    }
    let mut determinant = 1;
    for k in 0..4 {
        let Some(pivot_row) = (k..4).find(|&i| a[i][k] != 0) else {
            return 0;
        };
        if pivot_row != k {
            a.swap(k, pivot_row);
            determinant = field.neg(determinant);
        }
        let pivot = a[k][k];
        determinant = field.mul(determinant, pivot);
        let inverse = field.inv(pivot);
        for i in k + 1..4 {
            let factor = field.mul(a[i][k], inverse);
            for j in k + 1..4 {
                let product = field.mul(factor, a[k][j]);
                a[i][j] = field.sub(a[i][j], product);
            }
            a[i][k] = 0;
        }
    }
    determinant
}

fn plucker_vector(
    slot_rows: &[Vec<Row>],
    begin: usize,
    word: Word,
    selections: &[Columns],
    field: &mut Field,
) -> Vector {
    let rows = [
        slot_rows[begin][word[0] as usize],
        slot_rows[begin + 1][word[1] as usize],
        slot_rows[begin + 2][word[2] as usize],
        slot_rows[begin + 3][word[3] as usize],
    ];
    let mut out = [0; C];
    for (i, selection) in selections.iter().enumerate() {
        let mut value = det4(
            &rows,
            if begin == 0 {
                selection.left
            } else {
                selection.right
            },
            field,
        );
        if begin != 0 && selection.negative {
            value = field.neg(value);
        }
        out[i] = value;
    }
    out
}

#[derive(Default)]
struct Basis {
    rows: Vec<Vec<u64>>,
    pivots: Vec<usize>,
    words: Vec<Word>,
    raw: Vec<Vec<u64>>,
}

impl Basis {
    fn insert(&mut self, vector: Vector, word: Word, field: &mut Field) -> bool {
        let raw = vector.to_vec();
        let mut row = raw.clone();
        for (basis, &pivot) in self.rows.iter().zip(&self.pivots) {
            let coefficient = row[pivot];
            if coefficient == 0 {
                continue;
            }
            for j in pivot..C {
                let product = field.mul(coefficient, basis[j]);
                row[j] = field.sub(row[j], product);
            }
        }
        let Some(pivot) = row.iter().position(|&x| x != 0) else {
            return false;
        };
        let inverse = field.inv(row[pivot]);
        for value in &mut row[pivot..] {
            *value = field.mul(*value, inverse);
        }
        let position = self.pivots.binary_search(&pivot).unwrap_err();
        self.rows.insert(position, row);
        self.pivots.insert(position, pivot);
        self.words.insert(position, word);
        self.raw.insert(position, raw);
        true
    }
}

#[derive(Serialize)]
struct SpanRecord {
    vectors_total: u64,
    vectors_examined: u64,
    nonzero_vectors_examined: u64,
    complete_enumeration: bool,
    rank: usize,
    pivot_columns: Vec<usize>,
    basis_words: Vec<Word>,
    basis_vectors: Vec<Vec<u64>>,
    basis_determinant: u64,
    field_counts: FieldCounts,
}

fn span(rows: &[Vec<Row>], begin: usize, side: usize, selections: &[Columns]) -> SpanRecord {
    let width = rows[0].len();
    let vectors_total = (width as u64).pow(4);
    let mut basis = Basis::default();
    let mut examined = 0;
    let mut nonzero = 0;
    let mut field = Field::default();
    'all: for a in 0..width {
        for b in 0..width {
            for c in 0..width {
                for d in 0..width {
                    let word = [a as u8, b as u8, c as u8, d as u8];
                    let vector = plucker_vector(rows, begin, word, selections, &mut field);
                    examined += 1;
                    nonzero += u64::from(vector.iter().any(|&x| x != 0));
                    basis.insert(vector, word, &mut field);
                    if basis.rows.len() == C {
                        break 'all;
                    }
                }
            }
        }
    }
    let mut determinant_field = Field::default();
    let determinant = if basis.raw.len() == C {
        determinant(&basis.raw, &mut determinant_field)
    } else {
        0
    };
    field.counts.add_assign(determinant_field.counts);
    assert_eq!(side, usize::from(begin != 0));
    SpanRecord {
        vectors_total,
        vectors_examined: examined,
        nonzero_vectors_examined: nonzero,
        complete_enumeration: examined == vectors_total,
        rank: basis.rows.len(),
        pivot_columns: basis.pivots,
        basis_words: basis.words,
        basis_vectors: basis.raw,
        basis_determinant: determinant,
        field_counts: field.counts,
    }
}

fn rank(matrix: &[Vec<u64>], field: &mut Field) -> usize {
    if matrix.is_empty() {
        return 0;
    }
    let mut a = matrix.to_vec();
    let columns = a[0].len();
    let mut row = 0;
    for column in 0..columns {
        let Some(pivot) = (row..a.len()).find(|&i| a[i][column] != 0) else {
            continue;
        };
        a.swap(row, pivot);
        let inverse = field.inv(a[row][column]);
        for j in column..columns {
            a[row][j] = field.mul(a[row][j], inverse);
        }
        for i in row + 1..a.len() {
            let factor = a[i][column];
            if factor == 0 {
                continue;
            }
            for j in column..columns {
                let product = field.mul(factor, a[row][j]);
                a[i][j] = field.sub(a[i][j], product);
            }
        }
        row += 1;
        if row == a.len() {
            break;
        }
    }
    row
}

fn determinant(matrix: &[Vec<u64>], field: &mut Field) -> u64 {
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
            out = field.neg(out);
        }
        let value = a[column][column];
        out = field.mul(out, value);
        let inverse = field.inv(value);
        for i in column + 1..a.len() {
            let factor = field.mul(a[i][column], inverse);
            for j in column + 1..a.len() {
                let product = field.mul(factor, a[column][j]);
                a[i][j] = field.sub(a[i][j], product);
            }
            a[i][column] = 0;
        }
    }
    out
}

fn pairing(left: &[Vec<u64>], right: &[Vec<u64>], field: &mut Field) -> Vec<Vec<u64>> {
    left.iter()
        .map(|l| {
            right
                .iter()
                .map(|r| {
                    let mut sum = 0;
                    for (&a, &b) in l.iter().zip(r) {
                        let product = field.mul(a, b);
                        sum = field.add(sum, product);
                    }
                    sum
                })
                .collect()
        })
        .collect()
}

fn canonical_base(arithmetic: &mut Arithmetic, size: usize) -> Vec<Point> {
    let mut out = Vec::new();
    for x in 0..10_000 {
        let rhs = arithmetic.rhs(PRIME31, x);
        if let Some(y) = arithmetic.sqrt(rhs, P) {
            if y != 0 {
                let point = Some((x, y.min(P - y)));
                assert!(arithmetic.on_curve(PRIME31, point));
                out.push(point);
                if out.len() == size {
                    break;
                }
            }
        }
    }
    assert_eq!(out.len(), size);
    out
}

fn supports(arithmetic: &mut Arithmetic, base: &[Point]) -> Vec<Vec<Point>> {
    let mut slots = Vec::with_capacity(8);
    let mut positive_x = std::collections::BTreeSet::new();
    for weight in W {
        let mut positive = Vec::with_capacity(base.len());
        for &point in base {
            let image = arithmetic.scalar(PRIME31, weight, point);
            let (x, _) = image.expect("nonzero weighted point");
            assert!(positive_x.insert(x), "weighted positive x overlap");
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
    slots
}

fn planted_target(arithmetic: &mut Arithmetic, slots: &[Vec<Point>]) -> Point {
    slots
        .iter()
        .fold(None, |sum, slot| arithmetic.add(PRIME31, sum, slot[0]))
}

#[derive(Serialize)]
struct CaseRecord {
    id: &'static str,
    factor_base_size: usize,
    signed_slot_width: usize,
    base_x: Vec<u64>,
    target: Point,
    planted_control: bool,
    support_arithmetic: Counts,
    target_row_field_counts: FieldCounts,
    left: SpanRecord,
    right_signed_complement: SpanRecord,
    pairing_rank: usize,
    pairing_determinant: u64,
    pairing_field_counts: FieldCounts,
}

fn one_case(id: &'static str, size: usize, planted: bool) -> CaseRecord {
    let mut arithmetic = Arithmetic::default();
    let base = canonical_base(&mut arithmetic, size);
    let slots = supports(&mut arithmetic, &base);
    let target = if planted {
        planted_target(&mut arithmetic, &slots)
    } else {
        G
    };
    assert!(target.is_some());
    let support_arithmetic = arithmetic.counts;
    let mut row_field = Field::default();
    let rows: Vec<Vec<Row>> = slots
        .iter()
        .map(|slot| {
            slot.iter()
                .map(|&point| target_row(point, target, &mut row_field))
                .collect()
        })
        .collect();
    let selections = columns();
    let left = span(&rows, 0, 0, &selections);
    let right = span(&rows, 4, 1, &selections);
    let mut pairing_field = Field::default();
    let matrix = pairing(
        &left.basis_vectors,
        &right.basis_vectors,
        &mut pairing_field,
    );
    let pairing_rank = rank(&matrix, &mut pairing_field);
    let pairing_determinant = if matrix.len() == C && matrix[0].len() == C {
        determinant(&matrix, &mut pairing_field)
    } else {
        0
    };
    CaseRecord {
        id,
        factor_base_size: size,
        signed_slot_width: slots[0].len(),
        base_x: base.iter().map(|point| point.unwrap().0).collect(),
        target,
        planted_control: planted,
        support_arithmetic,
        target_row_field_counts: row_field.counts,
        left,
        right_signed_complement: right,
        pairing_rank,
        pairing_determinant,
        pairing_field_counts: pairing_field.counts,
    }
}

fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn invertible_random(seed: u64) -> Vec<Vec<u64>> {
    let mut matrix = vec![vec![0; C]; C];
    for (i, row) in matrix.iter_mut().enumerate() {
        row[i] = 1;
    }
    let mut state = seed;
    let mut field = Field::default();
    for _ in 0..700 {
        let a = splitmix(&mut state) as usize % C;
        let mut b = splitmix(&mut state) as usize % C;
        if a == b {
            b = (b + 1) % C;
        }
        let scalar = splitmix(&mut state) % (P - 1) + 1;
        let source = matrix[b].clone();
        for (dst, src) in matrix[a].iter_mut().zip(source) {
            let product = field.mul(scalar, src);
            *dst = field.add(*dst, product);
        }
        if splitmix(&mut state) & 7 == 0 {
            matrix.swap(a, b);
        }
    }
    matrix
}

#[derive(Serialize)]
struct ControlRecord {
    random_left_rank: usize,
    random_right_rank: usize,
    random_pairing_rank: usize,
    synthetic_left_rank: usize,
    synthetic_right_rank: usize,
    synthetic_pairing_rank: usize,
}

fn controls() -> ControlRecord {
    let random_left = invertible_random(0x504c_5543_4b45_5231);
    let random_right = invertible_random(0x504c_5543_4b45_5232);
    let mut field = Field::default();
    let random_left_rank = rank(&random_left, &mut field);
    let random_right_rank = rank(&random_right, &mut field);
    let random_pair = pairing(&random_left, &random_right, &mut field);
    let random_pairing_rank = rank(&random_pair, &mut field);
    let mut synthetic = vec![vec![0; C]; C];
    for i in 0..C {
        synthetic[i][i % 7] = 1;
    }
    let synthetic_left_rank = rank(&synthetic, &mut field);
    let synthetic_right_rank = rank(&synthetic, &mut field);
    let synthetic_pair = pairing(&synthetic, &synthetic, &mut field);
    let synthetic_pairing_rank = rank(&synthetic_pair, &mut field);
    ControlRecord {
        random_left_rank,
        random_right_rank,
        random_pairing_rank,
        synthetic_left_rank,
        synthetic_right_rank,
        synthetic_pairing_rank,
    }
}

#[derive(Serialize)]
struct Output {
    schema: &'static str,
    protocol_path: &'static str,
    protocol_sha256: &'static str,
    parent_source_commit: &'static str,
    original_arithmetic_commit: &'static str,
    p: u64,
    group_order: u64,
    weights: [u64; 8],
    right_message_encoding: &'static str,
    controls: ControlRecord,
    cases: Vec<CaseRecord>,
    verdict: &'static str,
    scope: &'static str,
    pcc: &'static str,
}

fn produce() -> Output {
    assert_eq!(P, PRIME31.p);
    let controls = controls();
    assert_eq!(
        (
            controls.random_left_rank,
            controls.random_right_rank,
            controls.random_pairing_rank,
        ),
        (C, C, C)
    );
    assert_eq!(
        (
            controls.synthetic_left_rank,
            controls.synthetic_right_rank,
            controls.synthetic_pairing_rank,
        ),
        (7, 7, 7)
    );
    let cases = vec![
        one_case("N4", 4, false),
        one_case("P4", 4, true),
        one_case("N8", 8, false),
        one_case("P8", 8, true),
    ];
    let b8: Vec<_> = cases
        .iter()
        .filter(|case| case.factor_base_size == 8)
        .collect();
    let verdict = if b8.iter().all(|case| {
        case.left.rank == C && case.right_signed_complement.rank == C && case.pairing_rank == C
    }) {
        "KILL_FIXED_LINEAR_COMPRESSION"
    } else if b8.iter().all(|case| case.pairing_rank <= 35) {
        "ADVANCE_LINEAR_COMPRESSION"
    } else {
        "WEAK_OR_INCONCLUSIVE"
    };
    Output {
        schema: "prime31.rr9-plucker-span/v1",
        protocol_path: "research/prime31_ic_20261115/rr9/RR9_PLUCKER_SPAN_PROTOCOL_20261119.md",
        protocol_sha256: PROTOCOL_SHA256,
        parent_source_commit: PARENT_SOURCE,
        original_arithmetic_commit: ORIGINAL_ARITHMETIC,
        p: P,
        group_order: PRIME31.n,
        weights: W,
        right_message_encoding:
            "signed complementary minors; dot(left,right) is generalized Laplace",
        controls,
        cases,
        verdict,
        scope: "exact fixed-linear-message rank gate only; no timing or ECDLP speed claim",
        pcc: "unverified",
    }
}

fn main() {
    println!("{}", serde_json::to_string_pretty(&produce()).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn det8(mut matrix: [Row; 8], field: &mut Field) -> u64 {
        let mut out = 1;
        for column in 0..8 {
            let Some(pivot) = (column..8).find(|&i| matrix[i][column] != 0) else {
                return 0;
            };
            if pivot != column {
                matrix.swap(column, pivot);
                out = field.neg(out);
            }
            let value = matrix[column][column];
            out = field.mul(out, value);
            let inverse = field.inv(value);
            for i in column + 1..8 {
                let factor = field.mul(matrix[i][column], inverse);
                for j in column + 1..8 {
                    let product = field.mul(factor, matrix[column][j]);
                    matrix[i][j] = field.sub(matrix[i][j], product);
                }
                matrix[i][column] = 0;
            }
        }
        out
    }

    #[test]
    fn laplace_contraction_matches_direct_determinant() {
        let selections = columns();
        let mut state = 0x173d_ef01;
        for _ in 0..16 {
            let mut rows = [[0; 8]; 8];
            for row in &mut rows {
                for value in row {
                    *value = splitmix(&mut state) % P;
                }
            }
            let slot_rows: Vec<Vec<Row>> = rows.iter().map(|row| vec![*row]).collect();
            let mut field = Field::default();
            let left = plucker_vector(&slot_rows, 0, [0; 4], &selections, &mut field);
            let right = plucker_vector(&slot_rows, 4, [0; 4], &selections, &mut field);
            let mut contracted = 0;
            for (&a, b) in left.iter().zip(right) {
                let product = field.mul(a, b);
                contracted = field.add(contracted, product);
            }
            assert_eq!(contracted, det8(rows, &mut field));
        }
    }

    #[test]
    fn rank_controls_hit_registered_values() {
        let controls = controls();
        assert_eq!(controls.random_left_rank, C);
        assert_eq!(controls.random_right_rank, C);
        assert_eq!(controls.random_pairing_rank, C);
        assert_eq!(controls.synthetic_left_rank, 7);
        assert_eq!(controls.synthetic_right_rank, 7);
        assert_eq!(controls.synthetic_pairing_rank, 7);
    }
}
