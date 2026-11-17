#[cfg(test)]
mod public_source_parity_probe {
    use super::*;
    use std::fs::File;
    use std::io::{BufWriter, Write};

    fn all_reduced(j: &Jacobian) -> Vec<Divisor> {
        let p = j.f.p;
        let mut out = vec![Divisor::O];
        for x in 0..p {
            for y in 0..p {
                if j.f.sq(y) == evaluate(&j.f, &j.polynomial, x) {
                    out.push(j.point(x, y));
                }
            }
        }
        for q in 0..p {
            for r in 0..p {
                for s in 0..p {
                    for t in 0..p {
                        let d = Divisor { degree: 2, u: [r, q, 1], v: [t, s] };
                        if j.valid(d) { out.push(d); }
                    }
                }
            }
        }
        out.sort();
        out.dedup();
        assert!(out.iter().all(|&d| j.valid(d)));
        out
    }

    fn weighted_add(j: &Jacobian, a: Divisor, b: Divisor,
                    za: u64, wa: u64, zb: u64, wb: u64)
                    -> (Option<ResultPoint>, &'static str) {
        let d1 = |d: Divisor, z: u64, w: u64| D1 {
            x: j.f.mul(j.f.neg(d.u[0]), j.f.sq(z)),
            y: j.f.mul(d.v[0], j.f.mul(j.f.pow(z, 5), w)),
            z, w,
        };
        match (a.degree, b.degree) {
            (0, _) => (Some(from_affine(b, &j.f)), "identity_left"),
            (_, 0) => (Some(from_affine(a, &j.f)), "identity_right"),
            (1, 1) => {
                let (v, branch) = add11(d1(a, za, wa), d1(b, zb, wb), &j.polynomial, &j.f);
                (Some(v), branch)
            }
            (1, 2) => add12_weighted_complete(
                d1(a, za, wa), weight_degree_two(b, zb, wb, &j.f), &j.polynomial, &j.f),
            (2, 1) => add12_weighted_complete(
                d1(b, zb, wb), weight_degree_two(a, za, wa, &j.f), &j.polynomial, &j.f),
            (2, 2) => add22_weighted(
                weight_degree_two(a, za, wa, &j.f),
                weight_degree_two(b, zb, wb, &j.f), &j.polynomial, &j.f),
            _ => unreachable!(),
        }
    }

    fn fields(d: Divisor) -> String {
        format!("{},{},{},{},{},{}", d.degree, d.u[0], d.u[1], d.u[2], d.v[0], d.v[1])
    }

    #[test]
    fn emit_valid_pair_corpus() {
        let rows_path = std::env::var("PUBLIC_PARITY_ROWS_PATH").unwrap();
        let inputs_path = std::env::var("PUBLIC_PARITY_INPUTS_PATH").unwrap();
        let per_field_limit: usize = std::env::var("PUBLIC_PARITY_LIMIT")
            .ok().map(|s| s.parse().unwrap()).unwrap_or(usize::MAX);
        let mut rows = BufWriter::new(File::create(rows_path).unwrap());
        let mut inputs = BufWriter::new(File::create(inputs_path).unwrap());
        writeln!(rows, "p,a_index,b_index,weight_index,rust_branch,result_degree,result_u0,result_u1,result_u2,result_v0,result_v1,cantor_degree,cantor_u0,cantor_u1,cantor_u2,cantor_v0,cantor_v1").unwrap();
        writeln!(inputs, "p,index,degree,u0,u1,u2,v0,v1").unwrap();
        for p in [7u64, 11] {
            let j = Jacobian { f: Field::new(p, 1, 0),
                               polynomial: vec![2, 1, 0, 0, 0, 1] };
            let divisors = all_reduced(&j);
            for (i, &d) in divisors.iter().enumerate() {
                writeln!(inputs, "{p},{i},{}", fields(d)).unwrap();
            }
            let mut emitted = 0usize;
            'pairs: for (ia, &a) in divisors.iter().enumerate() {
                for (ib, &b) in divisors.iter().enumerate() {
                    let cantor = j.add(a, b);
                    for (wi, &(za, wa, zb, wb)) in
                        [(1, 1, 1, 1), (2, 3, 3, 5)].iter().enumerate() {
                        if emitted == per_field_limit { break 'pairs; }
                        let (out, branch) = weighted_add(&j, a, b, za, wa, zb, wb);
                        let out = out.expect("weighted public branch unresolved");
                        let got = normalize(out, &j.f);
                        assert_eq!(got, cantor, "p={p} a={a:?} b={b:?} w={wi}");
                        writeln!(rows, "{p},{ia},{ib},{wi},{branch},{},{}", fields(got), fields(cantor)).unwrap();
                        emitted += 1;
                    }
                }
            }
            println!("PUBLIC_PARITY_RUST_JSON {}", serde_json::json!({
                "p":p,"divisors":divisors.len(),"rows":emitted,
                "full": emitted == divisors.len()*divisors.len()*2,
            }));
        }
    }
}
