use std::collections::BTreeSet;
type Point = [u8; 3];
type EPoint = Option<(u8, u8)>;
type JPoint = [EPoint; 3];
const INPUT: &str = include_str!("first-A-native/actual-fibres.stdout");
const CENSUS: &str = include_str!("original-C17-census.stdout");
const T: EPoint = Some((1, 3));
const PULLBACK: [u8; 3] = [8, 2, 9];
fn m(x: i32) -> u8 {
    u8::try_from(x.rem_euclid(7)).unwrap()
}
fn add(a: u8, b: u8) -> u8 {
    m(i32::from(a) + i32::from(b))
}
fn sub(a: u8, b: u8) -> u8 {
    m(i32::from(a) - i32::from(b))
}
fn mul(a: u8, b: u8) -> u8 {
    m(i32::from(a) * i32::from(b))
}
fn square(a: u8) -> u8 {
    mul(a, a)
}
fn cube(a: u8) -> u8 {
    mul(square(a), a)
}
fn inv(a: u8) -> u8 {
    (1..7u8).find(|x| mul(a, *x) == 1).unwrap()
}
fn valid(p: EPoint) -> bool {
    p.is_none_or(|(x, y)| square(y) == add(add(cube(x), mul(2, x)), 6))
}
fn eneg(p: EPoint) -> EPoint {
    p.map(|(x, y)| (x, sub(0, y)))
}
fn eadd(a: EPoint, b: EPoint) -> EPoint {
    assert!(valid(a) && valid(b));
    let Some((x, y)) = a else {
        return b;
    };
    let Some((xx, yy)) = b else {
        return a;
    };
    if x == xx && add(y, yy) == 0 {
        return None;
    }
    let slope = if x == xx {
        mul(add(mul(3, square(x)), 2), inv(mul(2, y)))
    } else {
        mul(sub(yy, y), inv(sub(xx, x)))
    };
    let nx = sub(sub(square(slope), x), xx);
    let ny = sub(mul(slope, sub(x, nx)), y);
    let result = Some((nx, ny));
    assert!(valid(result));
    result
}
fn emul(mut n: usize, mut p: EPoint) -> EPoint {
    let mut result = None;
    while n != 0 {
        if n & 1 != 0 {
            result = eadd(result, p);
        }
        p = eadd(p, p);
        n >>= 1;
    }
    result
}
fn jadd(a: JPoint, b: JPoint) -> JPoint {
    std::array::from_fn(|i| eadd(a[i], b[i]))
}
fn jsub(a: JPoint, b: JPoint) -> JPoint {
    jadd(a, b.map(eneg))
}
fn jmul(n: usize, a: JPoint) -> JPoint {
    a.map(|x| emul(n, x))
}
fn elliptic_linear(row: [u8; 3], p: JPoint) -> EPoint {
    row.into_iter()
        .zip(p)
        .fold(None, |a, (n, b)| eadd(a, emul(usize::from(n), b)))
}
fn pullback(p: EPoint, column: [u8; 3]) -> JPoint {
    column.map(|n| emul(usize::from(n), p))
}
fn split_row(p: JPoint) -> EPoint {
    eneg(eadd(p[0], p[1]))
}
fn push(p: JPoint) -> EPoint {
    elliptic_linear([3, 1, 0], p)
}
fn curve(p: Point) -> u8 {
    let [x, y, z] = p;
    let g = [square(z), mul(6, square(x)), mul(3, mul(x, y)), square(y)]
        .into_iter()
        .fold(0, add);
    sub(
        square(g),
        mul(
            y,
            add(add(cube(x), mul(2, mul(x, square(y)))), mul(6, cube(y))),
        ),
    )
}
fn point_digits(text: &str) -> Point {
    let d: Vec<_> = text
        .bytes()
        .filter(u8::is_ascii_digit)
        .map(|x| x - b'0')
        .collect();
    d.try_into().unwrap()
}
fn parse_e(text: &str) -> EPoint {
    if text == "None" {
        return None;
    }
    assert!(text.starts_with("Some((") && text.ends_with("))"));
    let d: Vec<_> = text
        .bytes()
        .filter(u8::is_ascii_digit)
        .map(|x| x - b'0')
        .collect();
    assert_eq!(d.len(), 2);
    Some((d[0], d[1]))
}
fn line_through(a: Point, b: Point) -> Point {
    let mut line = [
        sub(mul(a[1], b[2]), mul(a[2], b[1])),
        sub(mul(a[2], b[0]), mul(a[0], b[2])),
        sub(mul(a[0], b[1]), mul(a[1], b[0])),
    ];
    let scale = inv(*line.iter().find(|x| **x != 0).unwrap());
    line = line.map(|x| mul(x, scale));
    line
}
fn on_line(p: Point, line: Point) -> bool {
    p.into_iter().zip(line).map(|(a, b)| mul(a, b)).fold(0, add) == 0
}
fn parse_j(mut text: &str) -> JPoint {
    text = text.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
    let mut result = Vec::new();
    while !text.is_empty() {
        let n = if text.starts_with("None") {
            4
        } else {
            text.find("))").unwrap() + 2
        };
        result.push(parse_e(&text[..n]));
        text = &text[n..];
        if text.is_empty() {
            break;
        }
        text = text.strip_prefix(", ").unwrap();
    }
    result.try_into().unwrap()
}
fn field<'a>(line: &'a str, prefix: &str) -> &'a str {
    line.split('\t')
        .find_map(|s| s.strip_prefix(prefix))
        .unwrap()
}
#[derive(Clone, Copy, Debug)]
struct Row {
    point: Point,
    class: JPoint,
    image: EPoint,
    scalar: usize,
}
fn rows() -> Vec<Row> {
    let result: Vec<_> = INPUT
        .lines()
        .filter(|s| s.starts_with("ACTUAL_POINT\t"))
        .map(|line| {
            let point = point_digits(field(line, "C="));
            assert_eq!(curve(point), 0);
            let class = parse_j(field(line, "alpha-Abel="));
            let image = parse_e(field(line, "f4="));
            let scalar = field(line, "scalar-T=").parse::<usize>().unwrap();
            assert_eq!(push(class), image);
            assert_eq!(emul(scalar, T), image);
            Row {
                point,
                class,
                image,
                scalar,
            }
        })
        .collect();
    assert_eq!(result.len(), 17);
    assert_eq!(
        result
            .iter()
            .map(|r| r.point)
            .collect::<BTreeSet<_>>()
            .len(),
        17
    );
    let census = CENSUS
        .lines()
        .find(|s| s.starts_with("COUNT\tq=7\tkappa=1\t"))
        .unwrap();
    let values: Vec<_> = census
        .split("pointsF7=")
        .nth(1)
        .unwrap()
        .bytes()
        .filter(u8::is_ascii_digit)
        .map(|x| x - b'0')
        .collect();
    assert_eq!(values.len(), 51);
    let originals: BTreeSet<_> = values
        .as_chunks::<3>()
        .0
        .iter()
        .copied()
        .map(canonical_line)
        .collect();
    assert_eq!(originals, result.iter().map(|r| r.point).collect());
    println!("COMPLETE_ORIGINAL_CENSUS\t17 exact points match original COUNT\tno source population replay");
    result
}
fn gradient(p: Point) -> Point {
    let [x, y, z] = p;
    [
        [
            mul(4, cube(x)),
            mul(4, cube(y)),
            mul(3, mul(x, square(z))),
            mul(6, mul(y, square(z))),
        ]
        .into_iter()
        .fold(0, add),
        [
            mul(5, mul(x, square(y))),
            cube(y),
            mul(6, mul(x, square(z))),
            mul(4, mul(y, square(z))),
        ]
        .into_iter()
        .fold(0, add),
        [
            mul(3, mul(square(x), z)),
            mul(5, mul(mul(x, y), z)),
            mul(4, mul(square(y), z)),
            mul(4, cube(z)),
        ]
        .into_iter()
        .fold(0, add),
    ]
}
fn canonical_line(p: Point) -> Point {
    let factor = inv(*p.iter().find(|x| **x != 0).unwrap());
    p.map(|x| mul(x, factor))
}
fn weighted_class(rows: &[Row], support: &[usize], double: usize) -> JPoint {
    assert!(support.contains(&double));
    jadd(
        support
            .iter()
            .fold([None; 3], |a, i| jadd(a, rows[*i].class)),
        rows[double].class,
    )
}
fn main() {
    println!("PROFILE\tactual-CM19-weighted-fibre-certificate-C\tpreserved-four-distinct-fibre-negative\tunsigned\tno-performance-claim");
    let rows = rows();
    let target: Vec<_> = (0..11).map(|i| emul(i, T)).collect();
    assert_eq!(target.iter().copied().collect::<BTreeSet<_>>().len(), 11);
    let mut fibres = Vec::new();
    for (s, &q) in target.iter().enumerate() {
        let support: Vec<_> = rows
            .iter()
            .enumerate()
            .filter_map(|(i, r)| (r.image == q).then_some(i))
            .collect();
        assert!(support.len() < 4);
        if support.len() == 3 {
            fibres.push((s, q, support));
        }
    }
    assert_eq!(fibres.len(), 2);
    assert_eq!((fibres[0].0, fibres[1].0), (4, 7));
    let (ls, lq, left) = &fibres[0];
    let (rs, rq, right) = &fibres[1];
    let expected = pullback(eadd(*lq, eneg(*rq)), PULLBACK);
    let mut matches = Vec::new();
    for &i in left {
        for &j in right {
            let a = weighted_class(&rows, left, i);
            let b = weighted_class(&rows, right, j);
            let difference = jsub(a, b);
            let equal = difference == expected;
            println!("WEIGHTED_PAIR_CASE\tleft-double={i}\tright-double={j}\tleft-class={a:?}\tright-class={b:?}\tdifference={difference:?}\texpected={expected:?}\tequal={equal}");
            if equal {
                matches.push((i, j, a, b));
            }
        }
    }
    assert!(
        !matches.is_empty(),
        "no true weighted pair: fixed actual input/bridge contradiction"
    );
    println!(
        "COMPLETE_NINE_PAIR_RESULT\tcases=9\tmatching-pairs={}",
        matches.len()
    );
    let mut bad = PULLBACK;
    bad[0] = 7;
    let delta = eadd(*lq, eneg(*rq));
    assert_ne!(split_row(pullback(delta, bad)), delta);
    assert_eq!(split_row(pullback(delta, PULLBACK)), delta);
    let missing_double_degree = left.len();
    assert_ne!(missing_double_degree, 4);
    assert_eq!(left.len() + 1, 4);
    println!("COMMON_WEIGHTED_CONTROLS\twrong-pullback-column-rejected\tomitted-double-rejects-degree4\tpristine-restored");
    if matches.len() != 1 {
        println!("WEIGHTED_AMBIGUITY\tno full divisor selected\tactual local jets needed\tstanding=unsigned");
        return;
    }
    let (i, j, a, b) = matches[0];
    let origin = jsub(a, pullback(*lq, PULLBACK));
    assert_eq!(origin, jsub(b, pullback(*rq, PULLBACK)));
    let minus = rows.iter().position(|r| r.point == [1, 0, 6]).unwrap();
    let plus = rows.iter().position(|r| r.point == [1, 0, 1]).unwrap();
    assert_eq!(rows[minus].class, [None; 3]);
    let k = jmul(2, rows[plus].class);
    assert_eq!(push(k), emul(3, T));
    for (scalar, q, support, double, class) in [(*ls, *lq, left, i, a), (*rs, *rq, right, j, b)] {
        let divisor: Vec<_> = support
            .iter()
            .map(|n| (*n, rows[*n].point, if *n == double { 2 } else { 1 }))
            .collect();
        let degree: usize = divisor.iter().map(|(_, _, m)| m).sum();
        assert_eq!(degree, 4);
        let p = rows[double].point;
        let other = *support.iter().find(|n| **n != double).unwrap();
        let line = line_through(p, rows[other].point);
        let plane_condition = support.iter().all(|n| on_line(rows[*n].point, line))
            && line == canonical_line(gradient(p));
        assert_eq!(class == k, plane_condition);
        println!("ACTUAL_FULL_WEIGHTED_FIBRE\tscalar-T={scalar}\tR={q:?}\tdouble-index={double}\tD={divisor:?}\tdegree={degree}\tclass={class:?}\torigin-class={origin:?}\tcanonical={plane_condition}\tline-contact-test={line:?}\tcompleteness=global-degree4-plus-entire-C17-plus-unique-Pic0-pair");
    }
    let mut canonical = Vec::new();
    for (scalar, &q) in target.iter().enumerate() {
        let class = jadd(origin, pullback(q, PULLBACK));
        let equal = class == k;
        assert_eq!(split_row(pullback(q, PULLBACK)), q);
        if equal {
            canonical.push((scalar, q));
        }
        println!("ALL_TRANSLATED_CANONICAL\tscalar-T={scalar}\tQ={q:?}\tclass={class:?}\tK={k:?}\tequal={equal}");
    }
    assert!(canonical.len() <= 1);
    let candidate = split_row(jsub(k, origin));
    let candidate_scalar = target.iter().position(|p| *p == candidate).unwrap();
    println!("GEOMETRIC_CANONICAL_TRANSLATE\tparameters={canonical:?}\tsplit-candidate={candidate:?}\tcandidate-scalar={candidate_scalar}\tgeometric-completeness=written-global-injective-F7-pullback");
    for (scalar, q, support, _) in [(ls, lq, left, i), (rs, rq, right, j)] {
        let finite: Vec<_> = rows
            .iter()
            .enumerate()
            .filter(|(n, _)| !support.contains(n))
            .map(|(n, r)| (n, r.point, eadd(r.image, eneg(*q))))
            .collect();
        assert_eq!(finite.len(), 14);
        assert!(finite.iter().all(|(_, _, e)| e.is_some()));
        println!("RR_FINITE_VALUE_INPUT\tpole-fibre-scalar={scalar}\t14-other-actual-points={finite:?}\tx-pole-bound=8\ty-pole-bound=12\tfunctions-not-yet-extracted");
    }
    assert_ne!(i, j);
    assert_eq!(rows[i].scalar, 4);
    assert_eq!(rows[j].scalar, 7);
    println!("ACTUAL_WEIGHTED_CONTROLS\twrong-pullback-column-rejects-split\tomitted-double-rejects-degree4\tpristine-restored");
    println!("SUMMARY\tall9-pairs\tunique-pair\tfull-weighted-fibres=2\tcanonical-translates={}\toriginal-four-distinct-fibres=0\tactual-functions=NEXT\tECDLP-cost=UNMEASURED",canonical.len());
}
