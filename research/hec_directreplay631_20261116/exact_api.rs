pub fn checked(input: &str, output: &str) -> (Vec<u64>, Vec<Vec<u64>>) {
    let bytes = std::fs::read(output).unwrap();
    let r = parse(&bytes, &mut 0);
    let r = r.list();
    assert_eq!(r[0].num(), 0);
    let v = r[1].list();
    assert_eq!(v[0].num(), 631);
    assert_eq!(v[1].num(), 8);
    let names = ["w", "u0", "u1", "u2", "v0", "v1", "v2", "t"];
    assert_eq!(v[3].list().iter().map(V::string).collect::<Vec<_>>(), names);
    let q = v[5].list();
    assert_eq!(q[0].num(), 1);
    let q = q[1].list();
    let e = poly(&q[0]);
    assert!(e.len() == 3);
    assert_eq!(gcd(e.clone(), deriv(&e)).len(), 1);
    let den = poly(&q[1]);
    let inv = inverse(&den, &e);
    let ps = equations(input);
    let mut good = Vec::new();
    for sign in [1, 630] {
        let mut x: Vec<_> = q[2]
            .list()
            .iter()
            .map(|c| mm(&scale(&poly(c), sign), &inv, &e))
            .collect();
        x.push(rem(&[0, 1], &e));
        if residual(&ps, &x, &e) {
            good.push(x)
        }
    }
    assert_eq!(good.len(), 1);
    (e, good.pop().unwrap())
}
