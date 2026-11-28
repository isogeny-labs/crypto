#[path = "../multi/algebra.rs"]
mod algebra;
#[path = "../order4/cert.rs"]
mod cert;
#[path = "../order4/field.rs"]
mod field;
mod group;
mod order;
use hec_native_ic::{
    fp::Fp,
    instance::{factor_base, order_bsgs, prime_factors},
    jac::Curve,
    poly::Poly,
};
use std::{collections::BTreeMap, fs, io::Write};
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a
}
fn emit(
    p: u64,
    code: usize,
    status: &str,
    n: Option<u64>,
    ell: Option<u64>,
    h: Option<u64>,
    ordinary: bool,
    fq: bool,
    abs: bool,
    qualified: bool,
    m: usize,
    orbits: usize,
    lambda: Option<u64>,
    unresolved: &str,
) {
    let show = |x: Option<u64>| x.map(|x| x.to_string()).unwrap_or("none".into());
    println!("{p}\t{code}\t{}\t{}\t{}\t2\t{status}\t{}\t{}\t{}\t{ordinary}\t{fq}\t{abs}\t{qualified}\t{m}\t{orbits}\t{}\t{unresolved}",code/16,(code/4)%4,code%4,show(n),show(ell),show(h),show(lambda));
}
fn calibration() {
    let root = "research/hec_residual_span_20261116/order4";
    let mut counts: BTreeMap<(u64, usize), [u64; 4]> = BTreeMap::new();
    for l in fs::read_to_string(format!("{root}/POINTS.tsv"))
        .unwrap()
        .lines()
        .skip(1)
    {
        let v: Vec<_> = l.split('\t').collect();
        counts
            .entry((v[0].parse().unwrap(), v[1].parse().unwrap()))
            .or_insert([0; 4])[v[2].parse::<usize>().unwrap() - 1] = v[4].parse().unwrap()
    }
    let mut total = 0;
    for l in fs::read_to_string(format!("{root}/WEIL.tsv"))
        .unwrap()
        .lines()
        .skip(1)
    {
        let v: Vec<_> = l.split('\t').collect();
        let p = v[0].parse().unwrap();
        let id = v[1].parse().unwrap();
        let f: Vec<i128> = v[2]
            .trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .map(|x| x.trim().parse().unwrap())
            .collect();
        let n = f.iter().sum::<i128>() as u64;
        let c = counts[&(p, id)];
        let got = order::weil(p, n, [c[0], c[1], c[2]]);
        assert_eq!(got.as_slice(), f.as_slice());
        let predicted = cert::weil(p, c);
        assert_eq!(got, predicted);
        let (lo, hi) = order::interval(p, [c[0], c[1]]);
        assert!(lo <= n && n <= hi);
        if let Some(found) = order::unique(lo, hi, n) {
            assert_eq!(found, n)
        }
        assert_eq!(order::unique(n, n, n), Some(n));
        assert_eq!(order::unique(n, 2 * n, n), None);
        total += 1
    }
    assert_eq!(total, 6);
    eprintln!("calibration_models={total} coefficient_reconstruction=true interval_coverage=true");
}
fn main() {
    calibration();
    cert::controls();
    let root = "research/hec_residual_span_20261116/order4-strong";
    let mut points = fs::File::create_new(format!("{root}/POINTS.tsv")).unwrap();
    writeln!(points, "p\tmodel\td\tmodulus\tpoints").unwrap();
    let mut orders = fs::File::create_new(format!("{root}/ORDERS.tsv")).unwrap();
    writeln!(orders, "p\tmodel\tx\ty\tpoint_order\tLCM\tlo\thi\tunique_N").unwrap();
    let mut weilout = fs::File::create_new(format!("{root}/WEIL.tsv")).unwrap();
    writeln!(weilout, "p\tmodel\tpolynomial_ascending\tfactor\tordinary").unwrap();
    let mut witness = fs::File::create_new(format!("{root}/WITNESSES.tsv")).unwrap();
    writeln!(witness, "p\tmodel\tm\tprime").unwrap();
    let mut orbit = fs::File::create_new(format!("{root}/ORBITS.tsv")).unwrap();
    writeln!(orbit, "p\tmodel\tx\ty\trep_x\tweight").unwrap();
    let mut sources = fs::File::create_new(format!("{root}/SOURCES.tsv")).unwrap();
    writeln!(sources, "p\tmodel\tN\tell\th\ti\tlambda\tsource").unwrap();
    println!("p\tmodel\ta7\ta5\ta3\ta1\tstatus\tN\tell\th\tordinary\tfq_simple_certified\tabs_simple_certified\tgroup_qualified\tm\torbits\tlambda\tunresolved_m");
    let mut selected = 0;
    for p in [101u64, 149, 197] {
        assert!((2..).take_while(|d| d * d <= p).all(|d| p % d != 0));
        let fp = Fp::new(p);
        assert_eq!(p % 8, 5);
        assert_eq!(fp.legendre(2), -1);
        let f1 = field::Field::new(p, 1);
        f1.validate();
        let f2 = field::Field::new(p, 2);
        f2.validate();
        let mut f3 = None;
        let mut found = false;
        for code in 0..64usize {
            let a = [
                (code / 16) as u64,
                ((code / 4) % 4) as u64,
                (code % 4) as u64,
                2,
            ];
            let f = Poly::from_coeffs(&[0, 2, 0, a[2], 0, a[1], 0, a[0], 0, 1]);
            if Poly::gcd(&f, &f.deriv(&fp), &fp).deg() != 0 {
                emit(
                    p,
                    code,
                    "singular",
                    None,
                    None,
                    None,
                    false,
                    false,
                    false,
                    false,
                    0,
                    0,
                    None,
                    "not_attempted",
                );
                continue;
            }
            let branch = Poly::from_coeffs(&[2, 0, a[2], 0, a[1], 0, a[0], 0, 1]);
            if !algebra::irreducible(&branch, &fp) {
                emit(
                    p,
                    code,
                    "branch_reducible",
                    None,
                    None,
                    None,
                    false,
                    false,
                    false,
                    false,
                    0,
                    0,
                    None,
                    "not_attempted",
                );
                continue;
            }
            let c = Curve::new(fp, f);
            eprintln!("candidate p={p} model={code} a={a:?}");
            let c1 = f1.count(a);
            assert_eq!(c1 % 4, 2);
            let c2 = f2.count(a);
            writeln!(
                points,
                "{p}\t{code}\t1\t{:?}\t{c1}",
                &f1.modulus.c[..f1.modulus.n]
            )
            .unwrap();
            writeln!(
                points,
                "{p}\t{code}\t2\t{:?}\t{c2}",
                &f2.modulus.c[..f2.modulus.n]
            )
            .unwrap();
            let fb = factor_base(&c);
            assert_eq!(
                c1,
                1 + fb
                    .iter()
                    .map(|&(_, y)| if y == 0 { 1 } else { 2 })
                    .sum::<u64>()
            );
            let (lo, hi) = order::interval(p, [c1, c2]);
            let zero = c.place(0, 0);
            assert!(!c.is_zero(&zero));
            assert!(c.is_zero(&c.add(&zero, &zero)));
            let mut l = 2;
            let mut exact = None;
            for &(x, y) in fb.iter().filter(|v| v.1 != 0).take(8) {
                let d = c.place(x, y);
                let Some(o) = order_bsgs(&c, &d, 200000) else {
                    continue;
                };
                assert!(c.is_zero(&c.mul(&d, o)));
                for q in prime_factors(o) {
                    assert!(!c.is_zero(&c.mul(&d, o / q)))
                }
                l = (l / gcd(l, o)).checked_mul(o).unwrap();
                exact = order::unique(lo, hi, l);
                writeln!(
                    orders,
                    "{p}\t{code}\t{x}\t{y}\t{o}\t{l}\t{lo}\t{hi}\t{}",
                    exact.map(|v| v.to_string()).unwrap_or("none".into())
                )
                .unwrap();
                if exact.is_some() {
                    break;
                }
            }
            let Some(n) = exact else {
                emit(
                    p,
                    code,
                    "order_interval_uncertified",
                    None,
                    None,
                    None,
                    false,
                    false,
                    false,
                    false,
                    fb.len(),
                    0,
                    None,
                    "not_attempted",
                );
                continue;
            };
            let ell = *prime_factors(n).last().unwrap();
            let h = n / ell;
            if ell <= 2 || h > 4 || h % ell == 0 {
                emit(
                    p,
                    code,
                    "cofactor_reject",
                    Some(n),
                    Some(ell),
                    Some(h),
                    false,
                    false,
                    false,
                    false,
                    fb.len(),
                    0,
                    None,
                    "not_attempted",
                );
                continue;
            }
            eprintln!("strong_order p={p} model={code} N={n} ell={ell} h={h}");
            if f3.is_none() {
                let v = field::Field::new(p, 3);
                v.validate();
                f3 = Some(v)
            }
            let ff = f3.as_ref().unwrap();
            let c3 = ff.count(a);
            writeln!(
                points,
                "{p}\t{code}\t3\t{:?}\t{c3}",
                &ff.modulus.c[..ff.modulus.n]
            )
            .unwrap();
            let f = order::weil(p, n, [c1, c2, c3]);
            assert_eq!(f.iter().sum::<i128>(), n as i128);
            let ordinary = f[4] % (p as i128) != 0;
            let factor = cert::proper_factor(p, &f);
            let fq = ordinary && factor.is_none();
            writeln!(weilout, "{p}\t{code}\t{f:?}\t{factor:?}\t{ordinary}").unwrap();
            if !fq {
                emit(
                    p,
                    code,
                    "ordinary_irreducibility_uncertified",
                    Some(n),
                    Some(ell),
                    Some(h),
                    ordinary,
                    fq,
                    false,
                    false,
                    fb.len(),
                    0,
                    None,
                    "not_attempted",
                );
                continue;
            }
            if (1..=7).step_by(2).all(|j| f[j] == 0) {
                emit(
                    p,
                    code,
                    "exact_power_collision",
                    Some(n),
                    Some(ell),
                    Some(h),
                    ordinary,
                    fq,
                    false,
                    false,
                    fb.len(),
                    0,
                    None,
                    "even_polynomial",
                );
                continue;
            }
            let mut unresolved = vec![];
            for m in 1..=8192 {
                if cert::phi(m) > 64 {
                    continue;
                }
                let r = cert::primes()
                    .into_iter()
                    .find(|&r| cert::witness(&f, m, r));
                writeln!(
                    witness,
                    "{p}\t{code}\t{m}\t{}",
                    r.map(|v| v.to_string()).unwrap_or("none".into())
                )
                .unwrap();
                if r.is_none() {
                    unresolved.push(m)
                }
            }
            if !unresolved.is_empty() {
                emit(
                    p,
                    code,
                    "absolute_uncertified",
                    Some(n),
                    Some(ell),
                    Some(h),
                    ordinary,
                    fq,
                    false,
                    false,
                    fb.len(),
                    0,
                    None,
                    &format!("{unresolved:?}"),
                );
                continue;
            }
            if let Some((d, i, lambda, orbits)) = group::qualify(&c, n, ell, &mut orbit, p, code) {
                writeln!(
                    sources,
                    "{p}\t{code}\t{n}\t{ell}\t{h}\t{i}\t{lambda}\t{d:?}"
                )
                .unwrap();
                emit(
                    p,
                    code,
                    "selected",
                    Some(n),
                    Some(ell),
                    Some(h),
                    ordinary,
                    fq,
                    true,
                    true,
                    fb.len(),
                    orbits,
                    Some(lambda),
                    "[]",
                );
                selected += 1;
                found = true;
                break;
            } else {
                emit(
                    p,
                    code,
                    "source_projection_unavailable",
                    Some(n),
                    Some(ell),
                    Some(h),
                    ordinary,
                    fq,
                    true,
                    false,
                    fb.len(),
                    0,
                    None,
                    "[]",
                )
            }
        }
        if !found {
            eprintln!("field_no_qualified_model p={p}")
        }
    }
    eprintln!("fields_classified=3 selected={selected} no_targets=true no_performance_claim=true");
}
