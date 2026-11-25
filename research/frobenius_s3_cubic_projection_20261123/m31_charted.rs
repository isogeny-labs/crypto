//! Matched degree accounting on the repository's M31 example curve.
use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;

#[allow(dead_code)]
mod upstream {
    include!(env!("S3_PRIME_FIELD"));
}
use upstream::{resultant_formal, Curve, Fp};
#[cfg(feature = "native-f4")]
mod m31_f4_bench;
mod m31_single_target;
mod m31_larger;

const P: u64 = 2_147_483_647;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
const K: usize = 4;
const WIDTH: usize = 20;
const SIZE: usize = 1 << K;
type Point = (u64, u64);
type Poly = Vec<u64>;

fn sqrt(f: Fp, value: u64) -> Option<u64> {
    let root = f.pow(value, (P + 1) / 4);
    (f.mul(root, root) == value).then_some(root)
}

fn lift(c: Curve, x: u64) -> Option<Point> {
    let f = c.f;
    sqrt(f, f.add(f.mul(x, f.add(f.mul(x, x), c.a)), c.b)).map(|y| (x, y))
}

fn neg(f: Fp, (x, y): Point) -> Point {
    (x, f.neg(y))
}

fn group_add(c: Curve, p: Option<Point>, q: Option<Point>) -> Option<Point> {
    let (Some((x, y)), Some((u, v))) = (p, q) else {
        return p.or(q);
    };
    let f = c.f;
    let slope = if x != u {
        f.mul(f.sub(v, y), f.inv(f.sub(u, x)))
    } else if y == v && y != 0 {
        f.mul(f.add(f.mul(3, f.mul(x, x)), c.a), f.inv(f.mul(2, y)))
    } else {
        return None;
    };
    let out = f.sub(f.sub(f.mul(slope, slope), x), u);
    Some((out, f.sub(f.mul(slope, f.sub(x, out)), y)))
}

fn factor_base(c: Curve) -> Vec<Point> {
    let mut run = Vec::new();
    for x in 1..2_000_000 {
        match lift(c, x) {
            Some(point) if point.1 != 0 => {
                run.push(point);
                if run.len() == SIZE {
                    return run;
                }
            }
            _ => run.clear(),
        }
    }
    panic!("No contiguous factor base within the fixed setup budget")
}

fn add(f: Fp, a: &[u64], b: &[u64]) -> Poly {
    let mut out = vec![0; a.len().max(b.len())];
    out[..a.len()].copy_from_slice(a);
    for (x, &y) in out.iter_mut().zip(b) {
        *x = f.add(*x, y);
    }
    out
}

fn scale(f: Fp, a: &[u64], factor: u64) -> Poly {
    a.iter().map(|&x| f.mul(x, factor)).collect()
}

fn mul(f: Fp, a: &[u64], b: &[u64]) -> Poly {
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate().filter(|&(_, x)| *x != 0) {
        for (j, &y) in b.iter().enumerate().filter(|&(_, y)| *y != 0) {
            out[i + j] = f.add(out[i + j], f.mul(x, y));
        }
    }
    out
}

fn evaluate(f: Fp, a: &[u64], x: u64) -> u64 {
    a.iter().rev().fold(0, |acc, &v| f.add(f.mul(acc, x), v))
}

fn coefficients(c: Curve, x: u64) -> [Poly; 3] {
    // S3(T,x,X) = a(X)T² + b(X)T + c(X).
    let f = c.f;
    let x2 = f.mul(x, x);
    [
        vec![x2, f.neg(f.mul(2, x)), 1],
        vec![
            f.neg(f.mul(2, f.add(f.mul(x, c.a), f.mul(2, c.b)))),
            f.neg(f.mul(2, f.add(x2, c.a))),
            f.neg(f.mul(2, x)),
        ],
        vec![
            f.sub(f.mul(c.a, c.a), f.mul(f.mul(4, c.b), x)),
            f.neg(f.add(f.mul(f.mul(2, x), c.a), f.mul(4, c.b))),
            x2,
        ],
    ]
}

fn resultant(c: Curve, previous: &[u64], x: u64) -> Poly {
    let f = c.f;
    let d = previous.len() - 1;
    let [a, b, constant] = coefficients(c, x);
    let ac = mul(f, &a, &constant);
    let minus_b = scale(f, &b, P - 1);
    let minus_ac = scale(f, &ac, P - 1);
    let mut u = vec![0];
    let mut v = vec![previous[d]];
    let mut a_power = vec![1];
    for &term in previous[..d].iter().rev() {
        a_power = mul(f, &a_power, &a);
        (u, v) = (
            add(f, &v, &mul(f, &minus_b, &u)),
            add(f, &scale(f, &a_power, term), &mul(f, &minus_ac, &u)),
        );
    }
    let mut norm = add(
        f,
        &add(f, &mul(f, &v, &v), &mul(f, &minus_b, &mul(f, &u, &v))),
        &mul(f, &ac, &mul(f, &u, &u)),
    );
    let degree = a_power.len() - 1;
    assert_eq!(a_power[degree], 1);
    let mut out = vec![0; 2 * d + 1];
    for i in (degree..norm.len()).rev() {
        let leading = norm[i];
        if leading == 0 {
            continue;
        }
        out[i - degree] = leading;
        for (j, &value) in a_power.iter().enumerate() {
            norm[i - degree + j] = f.sub(norm[i - degree + j], f.mul(leading, value));
        }
    }
    assert!(norm.iter().all(|&x| x == 0));
    out
}

fn polynomial<'a>(
    c: Curve,
    base: &[Point],
    key: &[u8],
    cache: &'a mut HashMap<Vec<u8>, Poly>,
) -> &'a [u64] {
    if !cache.contains_key(key) {
        let (&last, prefix) = key.split_last().unwrap();
        let x = base[last as usize].0;
        let row = if prefix.is_empty() {
            vec![c.f.neg(x), 1]
        } else {
            resultant(c, polynomial(c, base, prefix, cache), x)
        };
        cache.insert(key.to_vec(), row);
    }
    &cache[key]
}

fn quadratic_roots(f: Fp, row: [u64; 3]) -> Option<[u64; 2]> {
    let [a, b, d] = row;
    if a != 0 {
        sqrt(f, f.sub(f.mul(b, b), f.mul(f.mul(4, a), d))).map(|r| {
            let inv = f.inv(f.mul(2, a));
            [f.mul(f.sub(r, b), inv), f.mul(f.neg(f.add(r, b)), inv)]
        })
    } else if b != 0 {
        let r = f.mul(f.neg(d), f.inv(b));
        Some([r, r])
    } else {
        assert_ne!(d, 0, "singular quadratic on a nonsingular curve");
        None
    }
}

fn pair_roots(c: Curve, base: &[Point], target: Point) -> BTreeSet<(usize, usize)> {
    let f = c.f;
    let mut out = BTreeSet::new();
    let start = base[0].0;
    for (i, &point) in base.iter().enumerate() {
        let row = coefficients(c, point.0).map(|coeff| evaluate(f, &coeff, target.0));
        let roots = quadratic_roots(f, row);
        for root in roots.into_iter().flatten() {
            if !(start..start + SIZE as u64).contains(&root) {
                continue;
            }
            let j = (root - start) as usize;
            let q = base[j];
            if [point, neg(f, point)].into_iter().any(|p| {
                [q, neg(f, q)]
                    .into_iter()
                    .any(|q| group_add(c, Some(p), Some(q)) == Some(target))
            }) {
                out.insert((i.min(j), i.max(j)));
            }
        }
    }
    out
}

fn signed_sum(c: Curve, points: impl IntoIterator<Item = Point>) -> BTreeSet<Option<Point>> {
    let mut values = BTreeSet::from([None]);
    for point in points {
        values = values
            .into_iter()
            .flat_map(|previous| [point, neg(c.f, point)].map(|p| group_add(c, previous, Some(p))))
            .collect();
    }
    values
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Chart {
    origin: usize,
    flips: Vec<usize>,
}

#[derive(Default)]
struct Work {
    prefix_sums: usize,
    residuals: usize,
    identity_prefixes: usize,
}

fn charted(c: Curve, base: &[Point], target: Point) -> (Vec<Chart>, Work) {
    charted_with(c, base, target, |w| Some(pair_roots(c, base, w))).unwrap()
}

fn charted_with(
    c: Curve,
    base: &[Point],
    target: Point,
    mut query: impl FnMut(Point) -> Option<BTreeSet<(usize, usize)>>,
) -> Option<(Vec<Chart>, Work)> {
    let mut cache = HashMap::new();
    let mut charts = BTreeSet::new();
    let mut work = Work::default();
    for i in 0..SIZE {
        for j in i..SIZE {
            for k in j..SIZE {
                let sums = signed_sum(c, [base[i], base[j], base[k]]);
                work.prefix_sums += sums.len();
                let mut diagonals = false;
                let mut pairs = BTreeSet::new();
                for prefix in sums {
                    let residual = group_add(c, Some(target), prefix.map(|p| neg(c.f, p)));
                    if let Some(w) = residual {
                        // The abscissa polynomial is independent of the residual sign.
                        // Negating both summands transports a verified pair between signs.
                        if let std::collections::hash_map::Entry::Vacant(entry) = cache.entry(w.0) {
                            entry.insert(query(w)?);
                        }
                        pairs.extend(cache[&w.0].iter().copied());
                    } else {
                        diagonals = true;
                    }
                }
                work.identity_prefixes += usize::from(diagonals);
                let prefixes: BTreeSet<_> = [
                    [i, j, k],
                    [i, k, j],
                    [j, i, k],
                    [j, k, i],
                    [k, i, j],
                    [k, j, i],
                ]
                .map(|[a, b, c]| a | b << K | c << (2 * K))
                .into_iter()
                .collect();
                for prefix in prefixes {
                    if diagonals {
                        charts.insert(Chart {
                            origin: prefix,
                            flips: (0..K)
                                .map(|b| (1 << (3 * K + b)) | (1 << (4 * K + b)))
                                .collect(),
                        });
                    }
                    for &(a, b) in &pairs {
                        if diagonals && a == b {
                            continue;
                        }
                        let flip = a ^ b;
                        charts.insert(Chart {
                            origin: prefix | a << (3 * K) | b << (4 * K),
                            flips: if flip == 0 {
                                vec![]
                            } else {
                                vec![flip << (3 * K) | flip << (4 * K)]
                            },
                        });
                    }
                }
            }
        }
    }
    work.residuals = cache.len();
    Some((charts.into_iter().collect(), work))
}

fn bits(word: usize, coordinate: usize) -> u64 {
    ((word >> coordinate) & 1) as u64
}

fn assemble(
    c: Curve,
    charts: &[Chart],
    width: usize,
    mut output: Option<BufWriter<File>>,
) -> (BTreeSet<usize>, usize, usize) {
    if charts.is_empty() {
        if let Some(ref mut file) = output {
            writeln!(file, "1").unwrap();
            file.flush().unwrap();
        }
        return (BTreeSet::new(), 0, 1);
    }
    let f = c.f;
    let nvars = charts.len() - 1 + charts.iter().map(|x| x.flips.len()).sum::<usize>();
    let mut owners = Vec::with_capacity(nvars);
    let mut roots = BTreeSet::new();
    let mut selectors = Vec::new();
    let mut axes = Vec::new();
    let mut graph: Vec<_> = (0..width)
        .map(|i| (bits(charts[0].origin, i), vec![]))
        .collect();
    for (j, chart) in charts.iter().enumerate() {
        let selector = (j != 0).then(|| {
            let var = owners.len();
            owners.push(j);
            var
        });
        selectors.push(selector);
        if let Some(selector) = selector {
            for (i, (constant, coefficients)) in graph.iter_mut().enumerate() {
                let coefficient = f.sub(bits(chart.origin, i), *constant);
                if coefficient != 0 {
                    coefficients.push((selector, coefficient));
                }
            }
        }
        let mut occupied = 0;
        axes.push(
            chart
                .flips
                .iter()
                .map(|&flip| {
                    assert_eq!(occupied & flip, 0);
                    occupied |= flip;
                    let var = owners.len();
                    owners.push(j);
                    for (i, (_, coefficients)) in graph.iter_mut().enumerate() {
                        let coefficient =
                            f.sub(bits(chart.origin ^ flip, i), bits(chart.origin, i));
                        if coefficient != 0 {
                            coefficients.push((var, coefficient));
                        }
                    }
                    var
                })
                .collect::<Vec<_>>(),
        );
    }
    assert_eq!(owners.len(), nvars);
    let mut rows = nvars + width;
    for (a, &owner) in owners.iter().enumerate() {
        for (b, &other) in owners.iter().enumerate().skip(a + 1) {
            let gate = selectors[owner] == Some(a) && owner == other;
            if gate || owner != other {
                if let Some(ref mut file) = output {
                    write!(file, "v{a}*v{b}").unwrap();
                    if gate {
                        write!(file, " - v{b}").unwrap();
                    }
                    writeln!(file).unwrap();
                }
                rows += 1;
            }
        }
    }
    if let Some(ref mut file) = output {
        for i in 0..nvars {
            writeln!(file, "v{i}^2 - v{i}").unwrap();
        }
        for (i, (constant, coefficients)) in graph.iter().enumerate() {
            write!(file, "x{i}").unwrap();
            if *constant != 0 {
                write!(file, " - 1").unwrap();
            }
            for &(j, value) in coefficients {
                assert!(value == 1 || value == P - 1);
                write!(file, " {} v{j}", if value == 1 { '-' } else { '+' }).unwrap();
            }
            writeln!(file).unwrap();
        }
        file.flush().unwrap();
    }
    for (j, chart) in charts.iter().enumerate() {
        let mut active = vec![false; nvars];
        if let Some(v) = selectors[j] {
            active[v] = true;
        }
        for selection in 0..(1 << chart.flips.len()) {
            let mut expected = chart.origin;
            for (i, (&var, &flip)) in axes[j].iter().zip(&chart.flips).enumerate() {
                active[var] = selection >> i & 1 != 0;
                if active[var] {
                    expected ^= flip;
                }
            }
            let mut point = 0;
            for (i, (constant, coefficients)) in graph.iter().enumerate() {
                let value = coefficients.iter().fold(*constant, |v, &(j, coefficient)| {
                    if active[j] {
                        f.add(v, coefficient)
                    } else {
                        v
                    }
                });
                assert!(value <= 1);
                point |= (value as usize) << i;
            }
            assert_eq!(point, expected);
            assert!(roots.insert(point));
        }
    }
    let standard_count: usize = charts.iter().map(|x| 1usize << x.flips.len()).sum();
    assert_eq!(standard_count, roots.len());
    (roots, nvars, rows)
}

fn direct(
    c: Curve,
    base: &[Point],
    t: u64,
    cache: &mut HashMap<Vec<u8>, Poly>,
) -> (BTreeSet<usize>, u64) {
    let mut result = BTreeSet::new();
    let mut evaluations = HashMap::new();
    let mut top = 0;
    for word in 0usize..(1 << WIDTH) {
        let mut key = [0; 5];
        for (i, entry) in key.iter_mut().enumerate() {
            *entry = ((word >> (i * K)) & (SIZE - 1)) as u8;
        }
        key.sort_unstable();
        let value = *evaluations
            .entry(key)
            .or_insert_with(|| evaluate(c.f, polynomial(c, base, &key, cache), t));
        if value == 0 {
            result.insert(word);
        }
        if (WIDTH as u32 - word.count_ones()) & 1 == 0 {
            top = c.f.add(top, value);
        } else {
            top = c.f.sub(top, value);
        }
    }
    (result, top)
}

fn point_sum(c: Curve, base: &[Point], indices: &[usize]) -> Point {
    indices
        .iter()
        .fold(None, |p, &i| group_add(c, p, Some(base[i])))
        .unwrap()
}

fn main() {
    if std::env::args_os()
        .nth(1)
        .is_some_and(|arg| arg == "larger-six")
    {
        m31_larger::run();
        return;
    }
    #[cfg(feature = "native-f4")]
    if std::env::args_os()
        .nth(1)
        .is_some_and(|arg| arg == "f4-single")
    {
        m31_f4_bench::run();
        return;
    }
    if std::env::args_os()
        .nth(1)
        .is_some_and(|arg| arg == "single-target")
    {
        m31_single_target::run();
        return;
    }
    let output_dir = std::env::args_os().nth(1).map(PathBuf::from);
    let c = Curve::new(P, A, B);
    let started = Instant::now();
    let base = factor_base(c);
    let setup = started.elapsed().as_secs_f64();
    let mut control = HashMap::new();
    for i in 0..SIZE {
        for j in i..SIZE {
            let p = polynomial(c, &base, &[i as u8, j as u8], &mut control);
            for t in [0, 1, base[i].0, base[j].0, base[0].0] {
                assert_eq!(evaluate(c.f, p, t), c.s3(base[i].0, base[j].0, t));
            }
        }
    }
    for i in 0..SIZE {
        let mut previous = vec![c.f.neg(base[i].0), 1];
        for j in [i, (i + 3) % SIZE, (i + 5) % SIZE, (i + 11) % SIZE] {
            let x = base[j].0;
            let next = resultant(c, &previous, x);
            for t in [0, x, base[(i + j) % SIZE].0, 0x543210 + i as u64] {
                let [a, b, d] = coefficients(c, x).map(|row| evaluate(c.f, &row, t));
                let determinant =
                    resultant_formal(c.f, &[d, b, a], 2, &previous, previous.len() - 1);
                assert_eq!(evaluate(c.f, &next, t), determinant);
            }
            previous = next;
        }
    }
    let targets = [
        ("five_distinct", point_sum(c, &base, &[0, 1, 2, 3, 4])),
        ("five_distinct", point_sum(c, &base, &[3, 5, 7, 11, 15])),
        ("five_distinct", point_sum(c, &base, &[0, 6, 9, 10, 14])),
        ("cancellation", point_sum(c, &base, &[1, 7, 12])),
    ];
    let mut cache = HashMap::new();
    for (kind, target) in targets.into_iter().chain(
        (1..)
            .filter_map(|x| lift(c, x))
            .take(2)
            .map(|t| ("unplanted", t)),
    ) {
        let started = Instant::now();
        let (charts, work) = charted(c, &base, target);
        let extraction = started.elapsed().as_secs_f64();
        let started = Instant::now();
        let output = output_dir.as_ref().map(|dir| {
            BufWriter::new(
                File::create(dir.join(format!("target-{}.basis.txt", target.0))).unwrap(),
            )
        });
        let (roots, nvars, rows) = assemble(c, &charts, WIDTH, output);
        let reconstruction = started.elapsed().as_secs_f64();
        let started = Instant::now();
        let (reference, top) = direct(c, &base, target.0, &mut cache);
        let reference_seconds = started.elapsed().as_secs_f64();
        assert_eq!(roots, reference);
        if kind == "five_distinct" {
            assert!(!roots.is_empty());
        }
        if kind == "cancellation" {
            assert!(work.identity_prefixes > 0);
        }
        assert_ne!(top, 0);
        println!(
            concat!(
            "{{\"p\":{},\"a\":{},\"b\":{},\"base_start\":{},\"base_size\":{},",
            "\"kind\":\"{}\",\"target\":{},\"roots\":{},\"direct_degree_lower_bound\":{},",
            "\"top_boolean_coefficient\":{},\"charted_degree_cap\":2,\"charts\":{},",
            "\"auxiliary_variables\":{},\"basis_rows\":{},\"quadratic_rows\":{},\"signed_prefix_sums\":{},",
            "\"distinct_residuals\":{},\"quadratic_suffix_queries\":{},\"identity_prefixes\":{},",
            "\"setup_seconds\":{:.9},\"extraction_seconds\":{:.9},",
            "\"reconstruction_seconds\":{:.9},\"reference_seconds\":{:.9}}}"
        ),
            P,
            A,
            B,
            base[0].0,
            SIZE,
            kind,
            target.0,
            roots.len(),
            WIDTH,
            top,
            charts.len(),
            nvars,
            rows,
            if roots.is_empty() { 0 } else { rows - nvars - WIDTH },
            work.prefix_sums,
            work.residuals,
            work.residuals * SIZE,
            work.identity_prefixes,
            setup,
            extraction,
            reconstruction,
            reference_seconds
        );
    }
}
