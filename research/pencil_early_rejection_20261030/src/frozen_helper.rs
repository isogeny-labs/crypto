fn frozen_verify(s: &Scholten, d: Divisor, lambda: u64, h: &[u64], p: u64, buckets: &[Vec<u64>]) -> Option<Vec<[u64; 2]>> {
                let (quotient, w) = complement(d, lambda, h, p);
                let roots = &buckets[lambda as usize];
                meter::collection(1, 0, 0, 1);
                if roots.len() != 3 {
                    return None;
                }
                let mut sum = Divisor::O;
                let mut points = Vec::with_capacity(3);
                for &x in roots {
                    meter::collection(1, 0, 0, 2);
                    assert_eq!(peval(&quotient, x, p), 0);
                    let y = peval(&w, x, p);
                    assert_eq!(mul(y, y, p), peval(h, x, p));
                    sum = s.j.add(sum, s.j.point(x, y));
                    points.push([x, y]);
                    meter::collection(0, 0, 1, 0);
                }
                assert_eq!(s.j.add(d, sum), Divisor::O);
                Some(points)
}
