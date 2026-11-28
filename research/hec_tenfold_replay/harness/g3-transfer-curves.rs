use hec_native_ic::instance::{hasse_interval, order_bsgs, prime_factors, thread_curve};
fn main() {
    for (p, original) in [(73, 22), (127, 12), (257, 15), (383, 35)] {
        let mut found = 0;
        for c in 1..p {
            if c == original {
                continue;
            }
            let Some(curve) = thread_curve(p, c, 3) else {
                continue;
            };
            let point = curve.place(0, 1);
            let Some(order) = order_bsgs(&curve, &point, 200_000) else {
                continue;
            };
            let Some(&ell) = prime_factors(order).last() else {
                continue;
            };
            if ell < 1000 || 4 * ell < hasse_interval(p, 3).0 || ell > 20_000_000 {
                continue;
            }
            let d = curve.mul(&point, order / ell);
            assert!(!curve.is_zero(&d) && curve.is_zero(&curve.mul(&d, ell)));
            println!("{p} {c} {ell} {}", order / ell);
            found += 1;
            if found == 2 {
                break;
            }
        }
        assert_eq!(found, 2);
    }
}
