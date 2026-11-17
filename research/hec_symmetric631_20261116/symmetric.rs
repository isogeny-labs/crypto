fn ucoeff(x: &[U], m: &[u64]) -> Option<Vec<U>> {
    let xs = [x[1].clone(), x[3].clone(), x[5].clone()];
    let ys = [x[2].clone(), x[4].clone(), x[6].clone()];
    let mut u = vec![vec![1]];
    for a in &xs {
        let mut z = vec![vec![]; u.len() + 1];
        for i in 0..u.len() {
            z[i] = add(&z[i], &mm(&u[i], a, m), true);
            z[i + 1] = add(&z[i + 1], &u[i], false);
        }
        u = z;
    }
    let mut v = vec![vec![]; 3];
    for i in 0..3 {
        let mut top = vec![vec![1]];
        let mut den = vec![1];
        for j in 0..3 {
            if i == j {
                continue;
            }
            let diff = add(&xs[i], &xs[j], true);
            if diff.is_empty() {
                return None;
            }
            den = mm(&den, &diff, m);
            let mut z = vec![vec![]; top.len() + 1];
            for k in 0..top.len() {
                z[k] = add(&z[k], &mm(&top[k], &xs[j], m), true);
                z[k + 1] = add(&z[k + 1], &top[k], false);
            }
            top = z;
        }
        let c = mm(&ys[i], &inverse(&den, m), m);
        for k in 0..3 {
            v[k] = add(&v[k], &mm(&top[k], &c, m), false);
        }
    }
    u.pop();u.extend(v);Some(u)
}
