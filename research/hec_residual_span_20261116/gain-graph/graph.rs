use hec_native_ic::fp::Fp;
// Rows have 0,1,2 nonzero LP entries; other columns/rhs are labels.
// Return exact LP rank and the rank predicted if all cycle gains were ignored.
pub fn ranks(rows: &[Vec<u64>], n: usize, f: &Fp) -> (usize, usize) {
    let mut adj = vec![vec![]; n];
    let mut ground = vec![false; n];
    for row in rows {
        assert_eq!(row.len(), n);
        let support: Vec<_> = row.iter().enumerate().filter(|(_, v)| **v != 0).collect();
        match support.as_slice() {
            [] => {}
            [(u, _)] => ground[*u] = true,
            [(u, a), (v, b)] => {
                let h = f.neg(f.mul(**a, f.inv(**b)));
                assert_ne!(h, 0);
                adj[*u].push((*v, h));
                adj[*v].push((*u, f.inv(h)))
            }
            _ => panic!("more than two LP columns"),
        }
    }
    let mut pot = vec![None; n];
    let (mut balanced, mut ungrounded) = (0, 0);
    for start in 0..n {
        if pot[start].is_some() {
            continue;
        }
        let mut stack = vec![start];
        pot[start] = Some(1);
        let (mut good, mut anchored) = (true, false);
        while let Some(u) = stack.pop() {
            anchored |= ground[u];
            let a = pot[u].unwrap();
            for &(v, h) in &adj[u] {
                let expected = f.mul(h, a);
                if let Some(actual) = pot[v] {
                    good &= actual == expected
                } else {
                    pot[v] = Some(expected);
                    stack.push(v)
                }
            }
        }
        if !anchored {
            ungrounded += 1;
            if good {
                balanced += 1
            }
        }
    }
    (n - balanced, n - ungrounded)
}
