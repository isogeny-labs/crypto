//! Block families: nb labelled sets W_k of nonzero x-values.
//!   frob     W_k = sigma^k V, V a random l-dim subspace          (arm under test)
//!   geom     W_k = sigma^k V, V = span(1,g,..,g^{l-1}), g random  (exploratory)
//!   rsub     W_k independent random l-dim subspaces               (negative control: linear, no Frobenius tie)
//!   rset     W_k independent random subsets of size 2^l - 1       (negative control: no structure)
//!   skew     independent random subspaces, dim l+1 for k < n/3   (positive control: planted size skew)
//!   frobperm the SAME blocks as frob (same V, same draw), block labels permuted by a random
//!            permutation of Z/n  (nuisance-matched control: identical union, trace profile,
//!            overlaps and shape structure; only the diagonal-gauge labelling is destroyed)
//!   plain    one block V                                          (compact baseline)
//!   plainbig one block, random subspace of dim l + ceil(log2 n)   (LARGER compact baseline, 1.1-1.9x the union size; NOT size-matched)

use crate::field::Fq;
use crate::Rng;

pub struct Family {
    pub name: String,
    pub nb: usize,
    pub blocks: Vec<Vec<u32>>,
    /// bit k set iff x in W_k
    pub kmask: Vec<u32>,
}

pub fn span_nonzero(basis: &[u32]) -> Vec<u32> {
    let l = basis.len();
    let mut out = Vec::with_capacity((1 << l) - 1);
    let mut cur = 0u32;
    for g in 1u32..(1u32 << l) {
        cur ^= basis[g.trailing_zeros() as usize]; // Gray code
        out.push(cur);
    }
    out
}

/// Random basis of an l-dim subspace of F_2^n.
pub fn random_basis(rng: &mut Rng, n: u32, l: usize) -> Vec<u32> {
    let mut basis: Vec<u32> = vec![];
    let mut red: Vec<u32> = vec![]; // echelon vectors for the independence test
    while basis.len() < l {
        let v = (rng.below((1u64 << n) - 1) + 1) as u32;
        let mut w = v;
        for &r in &red {
            w = w.min(w ^ r);
        }
        if w != 0 {
            red.push(w);
            red.sort_by(|a, b| b.cmp(a));
            basis.push(v);
        }
    }
    basis
}

fn finish(name: &str, n: u32, blocks: Vec<Vec<u32>>) -> Family {
    let mut kmask = vec![0u32; 1usize << n];
    for (k, b) in blocks.iter().enumerate() {
        for &x in b {
            assert!(x != 0);
            kmask[x as usize] |= 1 << k;
        }
    }
    Family {
        name: name.to_string(),
        nb: blocks.len(),
        blocks,
        kmask,
    }
}

pub fn from_blocks(name: &str, n: u32, blocks: Vec<Vec<u32>>) -> Family {
    finish(name, n, blocks)
}

pub fn frob_from_v(f: &Fq, v: &[u32], name: &str) -> Family {
    let n = f.n;
    let blocks = (0..n)
        .map(|k| v.iter().map(|&x| f.frob_pow(x, k)).collect())
        .collect();
    finish(name, n, blocks)
}

pub fn frob(f: &Fq, rng: &mut Rng, l: usize) -> Family {
    let v = span_nonzero(&random_basis(rng, f.n, l));
    frob_from_v(f, &v, "frob")
}

/// Same V as `frob` (the rng stream must be seeded identically), labels permuted.
pub fn frobperm(f: &Fq, rng: &mut Rng, l: usize) -> Family {
    let n = f.n as usize;
    let v = span_nonzero(&random_basis(rng, f.n, l));
    let base = frob_from_v(f, &v, "frobperm");
    let mut perm: Vec<usize> = (0..n).collect();
    for i in (1..n).rev() {
        perm.swap(i, rng.below(i as u64 + 1) as usize);
    }
    let blocks = perm.iter().map(|&k| base.blocks[k].clone()).collect();
    finish("frobperm", f.n, blocks)
}

pub fn geom(f: &Fq, rng: &mut Rng, l: usize) -> Family {
    let n = f.n;
    // g not in F_2, so (since n is prime) g has degree n and 1,g,..,g^{l-1} are independent.
    let g = (rng.below((1u64 << n) - 2) + 2) as u32;
    let mut basis = vec![1u32];
    let mut p = 1u32;
    for _ in 1..l {
        p = f.mul(p, g);
        basis.push(p);
    }
    let v = span_nonzero(&basis);
    frob_from_v(f, &v, "geom")
}

pub fn rsub(f: &Fq, rng: &mut Rng, l: usize) -> Family {
    let n = f.n;
    let blocks = (0..n)
        .map(|_| span_nonzero(&random_basis(rng, n, l)))
        .collect();
    finish("rsub", n, blocks)
}

pub fn rset(f: &Fq, rng: &mut Rng, l: usize) -> Family {
    let n = f.n;
    let size = (1usize << l) - 1;
    let blocks = (0..n)
        .map(|_| {
            let mut s: Vec<u32> = vec![];
            let mut seen = std::collections::HashSet::new();
            while s.len() < size {
                let x = (rng.below((1u64 << n) - 1) + 1) as u32;
                if seen.insert(x) {
                    s.push(x);
                }
            }
            s
        })
        .collect();
    finish("rset", n, blocks)
}

pub fn skew(f: &Fq, rng: &mut Rng, l: usize) -> Family {
    let n = f.n;
    let heavy = (n as usize) / 3;
    let blocks = (0..n as usize)
        .map(|k| span_nonzero(&random_basis(rng, n, if k < heavy { l + 1 } else { l })))
        .collect();
    finish("skew", n, blocks)
}

pub fn plain(f: &Fq, rng: &mut Rng, l: usize) -> Family {
    finish("plain", f.n, vec![span_nonzero(&random_basis(rng, f.n, l))])
}

pub fn plainbig(f: &Fq, rng: &mut Rng, l: usize) -> Family {
    let extra = (f.n as f64).log2().ceil() as usize;
    finish(
        "plainbig",
        f.n,
        vec![span_nonzero(&random_basis(
            rng,
            f.n,
            (l + extra).min(f.n as usize - 1),
        ))],
    )
}

pub fn build(arm: &str, f: &Fq, rng: &mut Rng, l: usize) -> Family {
    match arm {
        "frob" => frob(f, rng, l),
        "frobperm" => frobperm(f, rng, l),
        "geom" => geom(f, rng, l),
        "rsub" => rsub(f, rng, l),
        "rset" => rset(f, rng, l),
        "skew" => skew(f, rng, l),
        "plain" => plain(f, rng, l),
        "plainbig" => plainbig(f, rng, l),
        _ => panic!("unknown arm {arm}"),
    }
}

/// Arm order is part of the seed derivation (arm_code); append only.
pub const ARMS: [&str; 8] = [
    "frob", "geom", "rsub", "rset", "skew", "plain", "plainbig", "frobperm",
];
