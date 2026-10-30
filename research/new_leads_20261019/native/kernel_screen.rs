//! Exact abstract symplectic-module screen. No elliptic-curve or DLP claim.
use std::io::{self, Write};
type V = [u32; 6];
type Basis = [V; 3];
type Matrix = [V; 6];

fn sub(a: u32, b: u32, p: u32) -> u32 {
    (a + p - b % p) % p
}
fn inv(a: u32, p: u32) -> u32 {
    (1..p).find(|&b| a * b % p == 1).unwrap()
}
fn pairing(a: &V, b: &V, p: u32) -> u32 {
    (0..3).fold(0, |s, i| {
        (s + sub(a[2 * i] * b[2 * i + 1] % p, a[2 * i + 1] * b[2 * i] % p, p)) % p
    })
}
fn isotropic(b: &Basis, p: u32) -> bool {
    (0..3).all(|i| (i + 1..3).all(|j| pairing(&b[i], &b[j], p) == 0))
}
fn image(m: &Matrix, v: &V, p: u32) -> V {
    std::array::from_fn(|i| (0..6).map(|j| m[i][j] * v[j]).sum::<u32>() % p)
}
fn rank(mut a: Vec<V>, p: u32) -> usize {
    let mut r = 0;
    for c in 0..6 {
        if let Some(k) = (r..a.len()).find(|&i| a[i][c] != 0) {
            a.swap(r, k);
            let inverse = inv(a[r][c], p);
            for j in c..6 {
                a[r][j] = a[r][j] * inverse % p;
            }
            for i in r + 1..a.len() {
                let z = a[i][c];
                for j in c..6 {
                    a[i][j] = sub(a[i][j], z * a[r][j] % p, p);
                }
            }
            r += 1;
        }
    }
    r
}
fn member(mut v: V, b: &Basis, p: u32) -> bool {
    for row in b {
        let pivot = row.iter().position(|&x| x != 0).unwrap();
        let z = v[pivot];
        for j in pivot..6 {
            v[j] = sub(v[j], z * row[j] % p, p);
        }
    }
    v == [0; 6]
}
fn lagrangians(p: u32) -> (usize, Vec<Basis>) {
    let mut result = Vec::new();
    let mut total = 0;
    for x in 0..4 {
        for y in x + 1..5 {
            for z in y + 1..6 {
                let pivots = [x, y, z];
                let mut slots = Vec::new();
                for (i, &pivot) in pivots.iter().enumerate() {
                    for c in pivot + 1..6 {
                        if !pivots.contains(&c) {
                            slots.push((i, c));
                        }
                    }
                }
                for mut code in 0..p.pow(slots.len() as u32) {
                    let mut b = [[0; 6]; 3];
                    for i in 0..3 {
                        b[i][pivots[i]] = 1;
                    }
                    for &(i, c) in &slots {
                        b[i][c] = code % p;
                        code /= p;
                    }
                    total += 1;
                    if isotropic(&b, p) {
                        assert_eq!(rank(b.to_vec(), p), 3);
                        result.push(b);
                    }
                }
            }
        }
    }
    (total, result)
}
fn frobenius(p: u32, q: u32, t: u32, scalar: Option<u32>) -> Matrix {
    let q3 = q * q % p * q % p;
    let b = if let Some(s) = scalar {
        [[s, 0], [0, s]]
    } else {
        [[0, sub(0, q3, p)], [1, t]]
    };
    let iq2 = inv(q * q % p, p);
    let mut m = [[0; 6]; 6];
    for i in 0..2 {
        m[i][4] = b[i][0];
        m[i][5] = b[i][1] * iq2 % p;
    }
    m[2][0] = 1;
    m[3][1] = q;
    m[4][2] = 1;
    m[5][3] = q;
    m
}
fn check_multiplier(m: &Matrix, q: u32, p: u32) {
    for i in 0..6 {
        for j in 0..6 {
            let mut e = [0; 6];
            e[i] = 1;
            let mut f = [0; 6];
            f[j] = 1;
            assert_eq!(
                pairing(&image(m, &e, p), &image(m, &f, p), p),
                q * pairing(&e, &f, p) % p
            );
        }
    }
    assert_eq!(rank(m.to_vec(), p), 6);
}
fn stable_count(m: &Matrix, all: &[Basis], p: u32) -> (usize, Option<Basis>) {
    let mut count = 0;
    let mut first = None;
    for b in all {
        let images = b.map(|v| image(m, &v, p));
        let by_membership = images.iter().all(|&v| member(v, b, p));
        let mut stacked = b.to_vec();
        stacked.extend(images);
        let by_rank = rank(stacked, p) == 3;
        assert_eq!(by_membership, by_rank, "membership/rank disagree");
        if by_rank {
            count += 1;
            if first.is_none() {
                first = Some(*b);
            }
        }
    }
    (count, first)
}
fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 2, "usage: kernel_screen OUTPUT_DIRECTORY");
    std::fs::create_dir_all(&args[1])?;
    let mut csv = io::BufWriter::new(std::fs::File::create(format!("{}/counts.csv", args[1]))?);
    let mut json = io::BufWriter::new(std::fs::File::create(format!("{}/modules.jsonl", args[1]))?);
    writeln!(
        csv,
        "ell,q_residue,t_residue,module,all_lagrangians,stable_lagrangians"
    )?;
    let mut models = 0;
    for p in [3, 5] {
        let (total, all) = lagrangians(p);
        assert_eq!(all.len(), if p == 3 { 1120 } else { 19656 });
        let mut identity = [[0; 6]; 6];
        for (i, row) in identity.iter_mut().enumerate() {
            row[i] = 1;
        }
        let (positive, _) = stable_count(&identity, &all, p);
        assert_eq!(positive, all.len());
        let mut bad = [[0; 6]; 3];
        bad[0][0] = 1;
        bad[1][1] = 1;
        bad[2][2] = 1;
        assert_eq!(rank(bad.to_vec(), p), 3);
        assert!(!isotropic(&bad, p));
        println!("ell={p}: {total} rank-three subspaces, {} Lagrangians; identity and non-isotropic controls PASS",all.len());
        for q in 1..p {
            for t in 0..p {
                let mut variants = vec![None];
                variants.extend(
                    (1..p)
                        .filter(|&s| s * s % p == q * q % p * q % p && 2 * s % p == t)
                        .map(Some),
                );
                for scalar in variants {
                    let kind = if scalar.is_some() {
                        "scalar"
                    } else {
                        "companion"
                    };
                    let m = frobenius(p, q, t, scalar);
                    check_multiplier(&m, q, p);
                    let (count, witness) = stable_count(&m, &all, p);
                    writeln!(csv, "{p},{q},{t},{kind},{},{count}", all.len())?;
                    let witness_json = witness.map_or("null".to_string(), |b| format!("{:?}", b));
                    writeln!(json,"{{\"ell\":{p},\"q_residue\":{q},\"t_residue\":{t},\"module\":\"{kind}\",\"frobenius\":{m:?},\"all_subspaces\":{total},\"all_lagrangians\":{},\"stable_lagrangians\":{count},\"first_witness\":{witness_json},\"membership_rank_agree\":true,\"similitude_check\":true}}",all.len())?;
                    println!("ell={p},q={q},t={t},{kind}: {count}/{}", all.len());
                    models += 1;
                }
            }
        }
    }
    csv.flush()?;
    json.flush()?;
    println!("PASS: {models} model modules; no elliptic realizability or transfer asserted");
    Ok(())
}
