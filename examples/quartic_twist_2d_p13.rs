//! Bounded exact four-factor IC solve on the p=13 two-dimensional twist base.
//! Protocol: research/twist_index_calculus_20261020/QUARTIC_TWIST_LINE_PROTOCOL.md.
use std::collections::HashMap;
use std::time::Instant;

use crypto_lib::cryptanalysis::gaudry_quartic::{mitm_signed, pair_table, Curve4, Pt4, E4};
use crypto_lib::cryptanalysis::residual_walk::{is_prime_u64, pow_mod};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::json;

const P: u64 = 13;
const B0: u64 = 4;
const N: u64 = 28_753;

fn frob(c: &Curve4, pt: Pt4, z: u64, h2: &E4, h3: &E4) -> Pt4 {
    if pt.inf {
        return pt;
    }
    Pt4 {
        x: c.f.mul(h2, &c.f.frob_p(&pt.x, z)),
        y: c.f.mul(h3, &c.f.frob_p(&pt.y, z)),
        inf: false,
    }
}

fn cubic(c: &Curve4, pt: Pt4, omega: u64) -> Pt4 {
    Pt4 {
        x: c.f.scale(&pt.x, omega),
        ..pt
    }
}

fn eigen(c: &Curve4, g: &Pt4, image: &Pt4, order: u64) -> u64 {
    let u = (2..N)
        .map(|a| pow_mod(a, (N - 1) / order, N))
        .find(|&a| a != 1 && (order == 3 || pow_mod(a, 4, N) == N - 1))
        .unwrap();
    (1..order)
        .map(|j| pow_mod(u, j, N))
        .find(|&lambda| c.mul(g, lambda) == *image)
        .unwrap()
}

fn solve(rows: &[(Vec<u64>, u64)], columns: usize) -> (usize, Option<Vec<u64>>, u64) {
    let mut m: Vec<Vec<u64>> = rows
        .iter()
        .map(|(row, rhs)| {
            let mut v = row.clone();
            v.push(*rhs);
            v
        })
        .collect();
    let mut pivots = Vec::new();
    let mut ops = 0u64;
    for col in 0..columns {
        let Some(which) = (pivots.len()..m.len()).find(|&i| m[i][col] != 0) else {
            continue;
        };
        let rank = pivots.len();
        m.swap(rank, which);
        let inverse = pow_mod(m[rank][col], N - 2, N);
        for j in col..=columns {
            m[rank][j] = (m[rank][j] * inverse) % N;
            ops += 1;
        }
        for i in 0..m.len() {
            if i == rank || m[i][col] == 0 {
                continue;
            }
            let factor = m[i][col];
            for j in col..=columns {
                let delta = (factor * m[rank][j]) % N;
                m[i][j] = (m[i][j] + N - delta) % N;
                ops += 1;
            }
        }
        pivots.push(col);
    }
    if pivots.len() != columns {
        return (pivots.len(), None, ops);
    }
    assert!(m.iter().skip(columns).all(|r| r[columns] == 0));
    let mut answer = vec![0u64; columns];
    for (i, &col) in pivots.iter().enumerate() {
        answer[col] = m[i][columns];
    }
    (columns, Some(answer), ops)
}

fn main() {
    let started = Instant::now();
    assert!(is_prime_u64(N) && N > 4 * P * P);
    let f0 = Curve4::with_params(P, E4::ZERO, E4::ZERO);
    let t = E4([0, 1, 0, 0]);
    let t3 = f0.f.mul(&f0.f.sq(&t), &t);
    let c = Curve4::with_params(P, E4::ZERO, f0.f.scale(&t3, B0));
    let f = &c.f;
    let z = f.pow(&t, (P - 1) as u128).0[0];
    let h = f.pow(&t, (P as u128).pow(4) - 1 - (P - 1) as u128 / 2);
    let h2 = f.sq(&h);
    let h3 = f.mul(&h2, &h);
    let omega = (2..P)
        .map(|a| pow_mod(a, (P - 1) / 3, P))
        .find(|&a| a != 1)
        .unwrap();
    let model_fp_muls = f0.f.muls() + f.muls();
    f.reset_muls();
    let mut base_rng = StdRng::seed_from_u64(0x3244_5041_4952_42);
    let base: Vec<Pt4> = (0..P)
        .flat_map(|a| (0..P).map(move |b| (a, b)))
        .filter(|&(a, b)| a != 0 || b != 0)
        .filter_map(|(a, b)| c.lift_x(&E4([a, 0, b, 0]), &mut base_rng))
        .collect();
    assert_eq!(base.len(), 72);
    assert!(base.iter().all(|pt| c.on_curve(pt)));
    let base_fp_muls = f.muls();
    let g = base[0];
    assert!(c.mul(&g, N).inf);
    let q = c.mul(&g, 271);
    let input_fp_muls = f.muls() - base_fp_muls;
    let input_group_ops = c.ops();
    let lambda_frob = eigen(&c, &g, &frob(&c, g, z, &h2, &h3), 8);
    let lambda_cubic = eigen(&c, &g, &cubic(&c, g, omega), 3);
    assert_eq!(pow_mod(lambda_frob, 4, N), N - 1);
    assert_eq!(pow_mod(lambda_cubic, 3, N), 1);
    let eigen_fp_muls = f.muls() - base_fp_muls - input_fp_muls;
    let eigen_group_ops = c.ops() - input_group_ops;
    let by_x: HashMap<_, _> = base.iter().enumerate().map(|(i, pt)| (pt.x, i)).collect();
    let mut cols = vec![usize::MAX; base.len()];
    let mut coef = vec![0u64; base.len()];
    let mut columns = 0usize;
    for &rep in &base {
        let i = by_x[&rep.x];
        if cols[i] != usize::MAX {
            continue;
        }
        let mut fimage = rep;
        for j in 0..4 {
            let mut image = fimage;
            for k in 0..3 {
                let scale = (pow_mod(lambda_frob, j, N) * pow_mod(lambda_cubic, k, N)) % N;
                assert_eq!(c.mul(&rep, scale), image);
                let idx = by_x[&image.x];
                let sign_scale = if base[idx].y == image.y {
                    scale
                } else {
                    assert_eq!(base[idx].y, f.neg(&image.y));
                    (N - scale) % N
                };
                if cols[idx] == usize::MAX {
                    cols[idx] = columns;
                    coef[idx] = sign_scale;
                } else {
                    assert_eq!((cols[idx], coef[idx]), (columns, sign_scale));
                }
                image = cubic(&c, image, omega);
            }
            fimage = frob(&c, fimage, z, &h2, &h3);
        }
        columns += 1;
    }
    assert_eq!(columns, 6);
    assert!(cols.iter().all(|&v| v != usize::MAX));
    let map_fp_muls = f.muls() - base_fp_muls - input_fp_muls - eigen_fp_muls;
    let map_group_ops = c.ops() - input_group_ops - eigen_group_ops;
    let table = pair_table(&c, &base);
    let table_fp_muls = f.muls() - base_fp_muls - input_fp_muls - eigen_fp_muls - map_fp_muls;
    let table_group_ops = c.ops() - input_group_ops - eigen_group_ops - map_group_ops;
    let before_stream_muls = f.muls();
    let before_stream_ops = c.ops();
    let mut query_rng = StdRng::seed_from_u64(P.rotate_left(17) ^ 0x7A12_0A02);
    let mut pick_rng = StdRng::seed_from_u64(0x1A4F_01D5);
    let mut rows: Vec<(Vec<u64>, u64)> = Vec::new();
    let mut trace = Vec::new();
    let mut selected = Vec::new();
    let mut rank = 0;
    let mut matrix_ops = 0u64;
    let mut answer = None;
    let mut computed = 0u64;
    'outer: while trace.len() < 10_000 {
        let batch: Vec<_> = (0..64)
            .map(|_| (query_rng.gen_range(0..N), query_rng.gen_range(1..N)))
            .collect();
        for (a, b) in batch {
            computed += 1;
            let residual = c.add(&c.mul(&g, a), &c.mul(&q, b));
            let decompositions = mitm_signed(&c, &base, &table, &residual);
            trace.push(json!([a, b, decompositions.len()]));
            if decompositions.is_empty() {
                continue;
            }
            let terms = &decompositions[pick_rng.gen_range(0..decompositions.len())];
            let mut replay = Pt4::INF;
            let mut row = vec![0u64; columns + 1];
            for &(i, sign) in terms {
                replay = if sign == 1 {
                    c.add(&replay, &base[i])
                } else {
                    c.sub(&replay, &base[i])
                };
                let value = if sign == 1 {
                    coef[i]
                } else {
                    (N - coef[i]) % N
                };
                row[cols[i]] = (row[cols[i]] + value) % N;
            }
            assert_eq!(replay, residual);
            row[columns] = (N - b) % N;
            rows.push((row.clone(), a));
            let (new_rank, solution, ops) = solve(&rows, columns + 1);
            rank = new_rank;
            matrix_ops += ops;
            selected.push(json!({"a":a,"b":b,"terms":terms,"row":row,"rhs":a,"rank":rank}));
            if let Some(sol) = solution {
                answer = Some(sol);
                break 'outer;
            }
            if trace.len() >= 10_000 {
                break 'outer;
            }
        }
    }
    let stream_fp_muls = f.muls() - before_stream_muls;
    let stream_group_ops = c.ops() - before_stream_ops;
    let sol = answer.expect("10,000-query cap reached without full rank");
    assert_eq!(sol[columns], 271);
    for (i, pt) in base.iter().enumerate() {
        assert_eq!(c.mul(&g, (coef[i] * sol[cols[i]]) % N), *pt);
    }
    assert_eq!(c.mul(&g, sol[columns]), q);
    let verification_fp_muls = f.muls() - before_stream_muls - stream_fp_muls;
    let verification_group_ops = c.ops() - before_stream_ops - stream_group_ops;
    println!(
        "{}",
        json!({
            "p":P,"b0":B0,"seed":0,"n":N,"known_scalar":271,
            "generator_x":g.x.0,"generator_y":g.y.0,"target_x":q.x.0,"target_y":q.y.0,
            "base_coordinates":base.iter().map(|pt| json!([pt.x.0,pt.y.0])).collect::<Vec<_>>(),
            "physical_base":base.len(),"folded_columns":columns,
            "lambda_frob":lambda_frob,"lambda_cubic":lambda_cubic,
            "residuals":trace.len(),"targets_computed":computed,
            "decompositions":trace.iter().map(|v|v[2].as_u64().unwrap()).sum::<u64>(),
            "query_trace":trace,"selected_relations":selected,
            "rank":rank,"recovered":sol[columns],"factor_logs":sol[..columns].to_vec(),
            "matrix_ops":matrix_ops,
            "model_fp_muls":model_fp_muls,"base_fp_muls":base_fp_muls,
            "input_fp_muls":input_fp_muls,"input_group_ops":input_group_ops,
            "eigen_fp_muls":eigen_fp_muls,"eigen_group_ops":eigen_group_ops,
            "map_fp_muls":map_fp_muls,"map_group_ops":map_group_ops,
            "table_fp_muls":table_fp_muls,"table_group_ops":table_group_ops,
            "stream_fp_muls":stream_fp_muls,"stream_group_ops":stream_group_ops,
            "verification_fp_muls":verification_fp_muls,
            "verification_group_ops":verification_group_ops,
            "wall_ms_l0":started.elapsed().as_secs_f64()*1e3,
            "status":"pass"
        })
    );
}
