#[path = "arithmetic_a.rs"]
mod arithmetic;
#[path = "proof_a.rs"]
mod proof;
#[path = "schemes_a.rs"]
mod schemes;

use arithmetic::{determinant, dot, I, K, Z};
use schemes::{Point, Polynomial};
use std::collections::BTreeSet;

fn field<'a>(line: &'a str, key: &str) -> &'a str {
    line.split('\t')
        .find_map(|part| part.strip_prefix(key).and_then(|s| s.strip_prefix('=')))
        .expect("fixed certificate field")
}
fn elements(text: &str) -> Vec<K> {
    text.split("F(")
        .skip(1)
        .map(|part| {
            K::encoded(
                part.split(')')
                    .next()
                    .expect("field closing")
                    .parse()
                    .expect("field encoding"),
            )
        })
        .collect()
}
fn point(text: &str) -> Point {
    let values = elements(text);
    match values.as_slice() {
        [] => {
            assert_eq!(text, "None");
            None
        }
        [x, y] => Some((*x, *y)),
        _ => panic!("point encoding"),
    }
}
fn indices(text: &str) -> Vec<usize> {
    text.split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect("index"))
        .collect()
}

#[derive(Clone)]
struct Rank {
    pivots: Vec<usize>,
    original: Vec<usize>,
    minor: K,
    nullspace: Vec<Vec<K>>,
}
fn rank(line: &str, columns: usize) -> Rank {
    let values = elements(field(line, "nullspace"));
    assert_eq!(values.len() % columns, 0);
    let minor = elements(field(line, "nonzero_minor"));
    assert_eq!(minor.len(), 1);
    Rank {
        pivots: indices(field(line, "pivots")),
        original: indices(field(line, "original_rows")),
        minor: minor[0],
        nullspace: values.chunks(columns).map(|c| c.to_vec()).collect(),
    }
}
fn verify_rank(rows: &[Vec<K>], witness: &Rank) -> bool {
    let n = rows[0].len();
    let r = witness.pivots.len();
    if r != witness.original.len() || r > n || r > rows.len() || witness.minor == Z {
        return false;
    }
    if rows.iter().any(|row| row.len() != n)
        || witness.pivots.iter().any(|&j| j >= n)
        || witness.original.iter().any(|&i| i >= rows.len())
    {
        return false;
    }
    if witness.pivots.windows(2).any(|v| v[0] >= v[1])
        || witness
            .original
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .len()
            != r
    {
        return false;
    }
    let sub = witness
        .original
        .iter()
        .map(|&i| witness.pivots.iter().map(|&j| rows[i][j]).collect())
        .collect();
    if determinant(sub) != witness.minor {
        return false;
    }
    let free: Vec<_> = (0..n).filter(|j| !witness.pivots.contains(j)).collect();
    if witness.nullspace.len() != free.len() {
        return false;
    }
    for (i, v) in witness.nullspace.iter().enumerate() {
        if v.len() != n || rows.iter().any(|row| dot(row, v) != Z) {
            return false;
        }
        if free
            .iter()
            .enumerate()
            .any(|(j, &column)| v[column] != if i == j { I } else { Z })
        {
            return false;
        }
    }
    true
}
fn conic_conditions(rows: &[Vec<K>], c: &[K]) -> bool {
    rows.len() == 5
        && c.len() == 6
        && c[0] == I
        && rows.iter().all(|row| dot(row, c) == Z)
        && determinant(rows.iter().map(|row| row[1..].to_vec()).collect()) != Z
}

struct Cell {
    point: (K, K),
    sign: i32,
    target: (K, K),
    plus: Vec<K>,
    minus: Vec<K>,
    coprime: Rank,
    macaulay: Rank,
}
fn supplied_rank(raw: &str, p: Point, sign: i32, label: &str, columns: usize) -> Rank {
    let lines: Vec<_> = raw
        .lines()
        .filter(|line| {
            line.starts_with(label)
                && point(field(line, "P")) == p
                && field(line, "sign").parse::<i32>().expect("sign") == sign
        })
        .collect();
    assert_eq!(lines.len(), 1);
    rank(lines[0], columns)
}
fn cells(raw: &str) -> Vec<Cell> {
    raw.lines()
        .filter(|line| line.starts_with("CONIC_CELL\t"))
        .map(|line| {
            let p = point(field(line, "P"));
            let sign = field(line, "sign").parse().expect("sign");
            assert!(sign == 1 || sign == -1);
            assert_eq!(field(line, "positive_rank"), "5");
            assert_eq!(field(line, "negative_rank"), "5");
            Cell {
                point: p.expect("nonzero torsion"),
                sign,
                target: point(field(line, "f3_target")).expect("finite target"),
                plus: elements(field(line, "qplus")),
                minus: elements(field(line, "qminus")),
                coprime: supplied_rank(raw, p, sign, "COPRIME_RANK_CERT\t", 10),
                macaulay: supplied_rank(raw, p, sign, "MACAULAY_RANK_CERT\t", 28),
            }
        })
        .collect()
}
fn common_matrix(a: &Polynomial, b: &Polynomial) -> Vec<Vec<K>> {
    let mut rows = schemes::multiples(a, 4);
    rows.extend(schemes::multiples(b, 4));
    rows.extend(schemes::multiples(&schemes::quartic(), 2));
    assert_eq!(rows.len(), 36);
    assert!(rows.iter().all(|r| r.len() == 28));
    rows
}

fn main() {
    println!("PROFILE\tseparate_actual_C7_certificate_verifier_A\tunsigned_not_benchmark");
    arithmetic::field_controls();
    let raw = include_str!("inputs/CANDIDATE_A.stdout");
    assert_eq!(raw.len(), 26469);
    let points: Vec<_> = raw
        .lines()
        .filter(|line| line.starts_with("E3_POINT\t"))
        .map(|line| point(line.split('\t').nth(1).expect("point record")))
        .collect();
    let population: BTreeSet<_> = points.iter().copied().collect();
    assert_eq!(points.len(), 9);
    assert_eq!(population.len(), 9);
    assert!(population.contains(&None));
    let discriminant = K::prime(-16).product(
        K::prime(4)
            .product(K::prime(2).power(3))
            .sum(K::prime(27).product(K::prime(6).power(2))),
    );
    assert_ne!(discriminant, Z);
    for &p in &points {
        assert!(schemes::valid(p));
        assert_eq!(schemes::times(p, 3), None);
        assert!(population.contains(&schemes::frobenius(p)));
        assert_eq!(
            schemes::frobenius(schemes::frobenius(p)),
            schemes::negative(p)
        );
    }
    for line in raw.lines().filter(|line| line.starts_with("E3_POINT\t")) {
        let p = point(line.split('\t').nth(1).expect("point record"));
        assert_eq!(point(field(line, "Frobenius")), schemes::frobenius(p));
    }
    let signs: Vec<_> = raw
        .lines()
        .filter(|line| line.starts_with("SIGN_RESULT\t"))
        .collect();
    assert_eq!(signs.len(), 2);
    let plus = signs
        .iter()
        .find(|line| field(line, "sign") == "1")
        .expect("positive sign record");
    assert_eq!(field(plus, "certified_cells"), "0");
    assert_eq!(field(plus, "full_kernel_basis"), "None");
    let minus = signs
        .iter()
        .find(|line| field(line, "sign") == "-1")
        .expect("negative sign record");
    assert_eq!(field(minus, "certified_cells"), "8");
    let basis = elements(field(minus, "full_kernel_basis"));
    assert_eq!(basis.len(), 4);
    let p = Some((basis[0], basis[1]));
    let q = Some((basis[2], basis[3]));
    let span: BTreeSet<_> = (0..3)
        .flat_map(|i| (0..3).map(move |j| schemes::add(schemes::times(p, i), schemes::times(q, j))))
        .collect();
    assert_eq!(span, population);
    println!("AUDIT_TORSION\tall9_supplied_actual_points\tnonsingular_discriminant={:?}\torder3_and_actual_Frobenius\tfull9_point_basis={:?},{:?}\tno_point_scan\tPASS",discriminant,p,q);
    let cells = cells(raw);
    assert_eq!(cells.len(), 16);
    let keys: BTreeSet<_> = cells.iter().map(|c| (c.point, c.sign)).collect();
    assert_eq!(keys.len(), 16);
    assert_eq!(
        raw.lines()
            .filter(|l| l.starts_with("COPRIME_RANK_CERT\t"))
            .count(),
        16
    );
    assert_eq!(
        raw.lines()
            .filter(|l| l.starts_with("MACAULAY_RANK_CERT\t"))
            .count(),
        16
    );
    let mut positive = 0;
    let mut negative = 0;
    let mut controls = 0;
    for c in &cells {
        assert!(population.contains(&Some(c.point)));
        let expected = if c.sign == 1 {
            schemes::frobenius(Some(c.point))
        } else {
            schemes::negative(schemes::frobenius(Some(c.point)))
        };
        assert_eq!(Some(c.target), expected);
        let f = schemes::fibres(c.point, c.target);
        let mut plus_rows = schemes::constraints(&f.f0);
        plus_rows.extend(schemes::constraints(&f.f3));
        let mut minus_rows = schemes::constraints(&f.f1);
        minus_rows.extend(schemes::constraints(&f.jet));
        assert!(conic_conditions(&plus_rows, &c.plus));
        assert!(conic_conditions(&minus_rows, &c.minus));
        let qp = schemes::conic(&c.plus);
        let qm = schemes::conic(&c.minus);
        let norms = [
            schemes::evaluate(&qm, &f.f0).norm(),
            schemes::evaluate(&qm, &f.f3).norm(),
            schemes::evaluate(&qp, &f.f1).norm(),
            schemes::evaluate(&qp, &f.jet).norm(),
        ];
        assert!(norms.iter().all(|&n| n != Z));
        let mut coprime = schemes::multiples(&qp, 1);
        coprime.extend(schemes::multiples(&qm, 1));
        let common = common_matrix(&qp, &qm);
        assert!(verify_rank(&coprime, &c.coprime));
        assert!(verify_rank(&common, &c.macaulay));
        assert_eq!(c.coprime.pivots.len(), 6);
        let degree = 28 - c.macaulay.pivots.len();
        if c.sign == 1 {
            assert_eq!(degree, 0);
            positive += 1;
        } else {
            assert_eq!(degree, 3);
            negative += 1;
        }
        let conjugate = cells
            .iter()
            .find(|d| Some(d.point) == schemes::frobenius(Some(c.point)) && d.sign == c.sign)
            .expect("paired cell");
        assert_eq!(
            c.plus.iter().map(|&x| x.power(7)).collect::<Vec<_>>(),
            conjugate.plus
        );
        assert_eq!(
            c.minus.iter().map(|&x| x.power(7)).collect::<Vec<_>>(),
            conjugate.minus
        );
        let mut bad_conic = c.minus.clone();
        bad_conic[0] = bad_conic[0].sum(I);
        assert!(!conic_conditions(&minus_rows, &bad_conic));
        assert!(conic_conditions(&minus_rows, &c.minus));
        let mut bad_minor = c.coprime.clone();
        bad_minor.minor = bad_minor.minor.sum(I);
        assert!(!verify_rank(&coprime, &bad_minor));
        assert!(verify_rank(&coprime, &c.coprime));
        let mut bad_null = c.coprime.clone();
        bad_null.nullspace[0][0] = bad_null.nullspace[0][0].sum(I);
        assert!(!verify_rank(&coprime, &bad_null));
        assert!(verify_rank(&coprime, &c.coprime));
        if c.sign == 1 {
            assert_ne!(degree, 3);
        } else {
            assert_eq!(degree, 3);
        }
        assert_eq!(f.f0[0].ring.scalar(Z).norm(), Z);
        controls += 1;
        println!("AUDIT_CELL\tP={:?}\tsign={}\tconic_ranks=5,5\topposite_multiplication_norms={:?}\tcoprime_rank={}\tMacaulay_rank={}\tcommon_degree={}\tprinted_minor_and_all_nullvectors_verified\tconic_minor_null_mutations_rejected_restored\tPASS",c.point,c.sign,norms,c.coprime.pivots.len(),c.macaulay.pivots.len(),degree);
    }
    assert_eq!((positive, negative, controls), (8, 8, 16));
    proof::audit();
    println!("SUMMARY\tsupplied_points=9\tcells=16\tprinted_rank_certificates=32\tpositive_common0=8\tnegative_common3=8\tmutation_restore_cells=16\tseparate_arithmetic_and_Bareiss_and_norm_implementation\tno_candidate_search_replay\tPASS");
    println!("SCOPE\tsame_session_separate_implementation\tstandard_geometry_remains_written\tno_generic_Lean_or_separately_signed_expert_verdict_or_new_target_J4_engine");
}
