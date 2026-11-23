use hec_native_ic::instance::{hasse_interval, order_bsgs, prime_factors, thread_curve};
fn main() {
    for (p, original) in [(37, 9), (43, 5), (59, 4), (67, 1)] {
        let mut found = 0;
        let mut seen = vec![u64::pow(original, 3) % p];
        for c in 1..p {
            let orbit = c * c * c % p;
            if seen.contains(&orbit) {
                continue;
            }
            let Some(curve) = thread_curve(p, c, 4) else {
                continue;
            };
            let point = curve.place(0, 1);
            let Some(order) = order_bsgs(&curve, &point, 200_000) else {
                continue;
            };
            let Some(&ell) = prime_factors(order).last() else {
                continue;
            };
            if ell < 1000 || 4 * ell < hasse_interval(p, 4).0 || ell > 20_000_000 {
                continue;
            }
            let d = curve.mul(&point, order / ell);
            assert!(!curve.is_zero(&d) && curve.is_zero(&curve.mul(&d, ell)));
            println!("{p} {c} {ell} {}", order / ell);
            seen.push(orbit);
            found += 1;
            if found == 2 {
                break;
            }
        }
        eprintln!("field {p} selected {found}");
    }
}
