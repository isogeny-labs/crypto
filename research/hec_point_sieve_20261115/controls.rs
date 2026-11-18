mod sieve;
use hec_native_ic::{
    instance::{factor_base, thread_curve},
    rng::Rng,
};

fn main() {
    let mut controls = 0;
    for g in [2, 3, 4] {
        let c = (1..10).find_map(|cc| thread_curve(101, cc, g)).unwrap();
        let fb = factor_base(&c);
        let mut rng = Rng::new(99117);
        let mut exercised = false;
        for _ in 0..128 {
            let d = (0..g).fold(c.zero(), |d, _| {
                let (x, y) = fb[rng.below(fb.len() as u64) as usize];
                c.add(&d, &c.place(x, y))
            });
            if c.is_zero(&c.mul(&d, 2)) {
                continue;
            }
            for r in sieve::batch(&c, &d, &fb, fb.len(), &mut sieve::Counts::default()) {
                let sum = sieve::sum(&c, &fb, &r.terms);
                assert_eq!(c.add(&d, &sum), c.zero());
                // Negating the public challenge contribution must be rejected.
                assert_ne!(c.add(&c.neg(&d), &sum), c.zero());
                if let Some(index) = r.terms.iter().position(|&(j, m)| {
                    !c.is_zero(&c.mul(&c.place(fb[j].0, fb[j].1), 2 * m.unsigned_abs()))
                }) {
                    let mut wrong = r.terms.clone();
                    wrong[index].1 = -wrong[index].1;
                    assert_ne!(c.add(&d, &sieve::sum(&c, &fb, &wrong)), c.zero());
                    controls += 2;
                    exercised = true;
                    break;
                }
            }
            if exercised {
                break;
            }
        }
        assert!(exercised, "missing sign-negative controls genus {g}");
    }
    println!("CONTROLS negative_sign_checks={controls} genera=3 passed=true");
}
