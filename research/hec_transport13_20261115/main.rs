#![allow(dead_code)]
include!("reused.rs");
fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}
fn is_prime_u64(p: u64) -> bool {
    p >= 2 && !(2..).take_while(|d| d * d <= p).any(|d| p % d == 0)
}
fn scalar(j: &QuarticJacobian, x: QClass, mut n: u64) -> QClass {
    let mut a = j.zero();
    let mut b = x;
    while n != 0 {
        if n & 1 != 0 {
            a = j.add_raw(&a, &b)
        }
        n >>= 1;
        if n != 0 {
            b = j.add_raw(&b, &b)
        }
    }
    a
}
fn main() {
    let raw = [
        [
            1, 47, 59, 89, 95, 83, 53, 119, 40, 11, 113, 107, 72, 67, 121,
        ],
        [
            1, 119, 17, 98, 109, 32, 90, 122, 100, 58, 107, 5, 65, 44, 119,
        ],
    ];
    let p = 127;
    assert!(is_prime_u64(1028893));
    for (ki, coeff) in raw.into_iter().enumerate() {
        // Geometry study orders ascending x,y; reused module orders descending x,y.
        let mut f = [0; 15];
        let inv = invm(coeff[14], p);
        for (i, c) in coeff.into_iter().rev().enumerate() {
            f[i] = c * inv % p
        }
        println!("MODEL kernel={ki} coefficients={f:?}");
        let n1 = count_points(&f, p, 1);
        let n2 = count_points(&f, p, 2);
        println!("COUNTS kernel={ki} n1={n1} n2={n2}");
        assert_eq!(n1, if ki == 0 { 127 } else { 129 });
        assert_eq!(n2, 16043);
        let pts = rational_points(&f, p);
        assert_eq!(pts.len() as u64, n1);
        let j = QuarticJacobian::new(p, f).expect("smooth destination");
        let zero = j.zero();
        let order = if ki == 0 { 2027974 } else { 2057786 };
        let mut xs = Vec::new();
        for i in 0..2 {
            let x = j
                .from_points([pts[4 + 3 * i], pts[5 + 3 * i], pts[6 + 3 * i]])
                .unwrap();
            assert_eq!(j.add_raw(&x, &zero), x);
            assert_eq!(j.add_raw(&x, &j.neg_raw(&x)), zero);
            assert_eq!(j.add_raw(&x, &x), scalar(&j, x, 2));
            assert_eq!(scalar(&j, x, order), zero);
            println!(
                "CLASS kernel={ki} index={i} encoding={:?} annihilator={order}",
                x.0
            );
            if ki == 1 {
                let q = scalar(&j, x, 2);
                assert_ne!(q, zero);
                assert_eq!(scalar(&j, q, 1028893), zero);
                println!(
                    "PRIME_ORDER kernel=1 index={i} ell=1028893 encoding={:?}",
                    q.0
                );
            }
            xs.push(x);
        }
        let (a, b) = (xs[0], xs[1]);
        let c = j.add_raw(&a, &b);
        assert_eq!(c, j.add_raw(&b, &a));
        assert_eq!(j.add_raw(&c, &a), j.add_raw(&a, &j.add_raw(&b, &a)));
        println!("GROUP kernel={ki} identity=2 inverse=2 doubling=2 annihilator=2 commutative=1 associative=1 field_muls={}",field_muls());
    }
    println!("PASS destination_arithmetic=true divisor_transport=false");
}
