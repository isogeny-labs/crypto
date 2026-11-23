use hec_native_ic::instance::thread_curve;
fn main() {
    let c = thread_curve(37, 0, 4).unwrap();
    let l = 30109;
    let h = 4335696 / l;
    let y = c.fp.sqrt(c.f.eval(1, &c.fp)).unwrap();
    let base = c.mul(&c.place(1, y), h);
    assert!(!c.is_zero(&base) && c.is_zero(&c.mul(&base, l)));
    let image = c.mul(&c.place(10, y), h);
    let roots: Vec<_> = (0..l).filter(|k| (k * k + k + 1) % l == 0).collect();
    let good: Vec<_> = roots
        .iter()
        .copied()
        .filter(|k| c.mul(&base, *k) == image)
        .collect();
    assert_eq!(good.len(), 1);
    let mut count = 0;
    for x in 0..37 {
        if let Some(y) = c.fp.sqrt(c.f.eval(x, &c.fp)) {
            if y == 0 {
                continue;
            }
            assert_eq!(
                c.mul(&c.mul(&c.place(x, y), h), good[0]),
                c.mul(&c.place(10 * x % 37, y), h)
            );
            count += 1;
        }
    }
    println!("roots={roots:?} verified={good:?} places={count}");
}
