use crate::crit::{crit, Crit};
use crate::fp::Fp;
use crate::poly::Poly;
use std::collections::BTreeSet;

pub struct TypeRow {
    pub cycle_type: Vec<usize>, // 8-point type: degrees (partition of 7) then trailing 1
    pub stable_pair: usize,
    pub stable_other: usize,
    pub pair_rule: bool,
    pub chi2: Vec<u32>,
    pub fire_2: bool,
}
pub struct Lag {
    pub total: usize,
    pub pair: usize,
    pub other: usize,
    pub rows: Vec<TypeRow>,
}

fn partitions(n: usize, maxp: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![vec![]];
    }
    let mut out = vec![];
    for k in (1..=n.min(maxp)).rev() {
        for mut rest in partitions(n - k, k) {
            let mut v = vec![k];
            v.append(&mut rest);
            out.push(v);
        }
    }
    out
}

pub fn pop(x: u8) -> u32 {
    x.count_ones()
}

pub fn is_pair_type(l: &[u8; 8]) -> bool {
    let mut twos: Vec<u8> = vec![];
    for &x in l.iter() {
        if x == 0 {
            continue;
        }
        if pop(x) == 2 {
            twos.push(x);
        } else if pop(x) == 6 {
            twos.push(0xFF ^ x);
        }
    }
    // four pairwise disjoint among the (at most 7) values
    for a in 0..twos.len() {
        for b in a + 1..twos.len() {
            if twos[a] & twos[b] != 0 {
                continue;
            }
            for c in b + 1..twos.len() {
                if twos[a] & twos[c] != 0 || twos[b] & twos[c] != 0 {
                    continue;
                }
                for d in c + 1..twos.len() {
                    if twos[a] & twos[d] == 0 && twos[b] & twos[d] == 0 && twos[c] & twos[d] == 0 {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn chi2_from_cycle_type(ct: &[usize]) -> Poly {
    let f2 = Fp::new(2);
    let mut prod = Poly::one();
    for &len in ct {
        let mut t = Poly::zero();
        t.c[0] = 1;
        t.c[len] = 1;
        t.n = len + 1;
        prod = f2.pmul(&prod, &t);
    }
    let t1 = Poly::from(&[1, 1], 2);
    let den = f2.pmul(&t1, &t1);
    let (q, r) = f2.pdivrem(&prod, &den);
    assert!(r.is_zero());
    q
}

pub fn build() -> Lag {
    let elems: Vec<u8> = (1u8..128).filter(|x| pop(*x) % 2 == 0).collect();
    assert_eq!(elems.len(), 63);
    let pr = |a: u8, b: u8| pop(a & b) % 2;
    let mut set: BTreeSet<[u8; 8]> = BTreeSet::new();
    for (ia, &a) in elems.iter().enumerate() {
        for (ib, &b) in elems.iter().enumerate().skip(ia + 1) {
            if pr(a, b) != 0 {
                continue;
            }
            for &c in elems.iter().skip(ib + 1) {
                if c == a ^ b || pr(a, c) != 0 || pr(b, c) != 0 {
                    continue;
                }
                let mut span = [0, a, b, c, a ^ b, a ^ c, b ^ c, a ^ b ^ c];
                span.sort_unstable();
                set.insert(span);
            }
        }
    }
    let total = set.len();
    let pairs: Vec<bool> = set.iter().map(|l| is_pair_type(l)).collect();
    let pair = pairs.iter().filter(|&&b| b).count();
    let fl2 = Fp::new(2);
    let mut rows = vec![];
    for part in partitions(7, 7) {
        // canonical permutation
        let mut sigma = [0usize; 8];
        sigma[7] = 7;
        let mut b = 0;
        for &d in &part {
            for k in 0..d {
                sigma[b + k] = b + (k + 1) % d;
            }
            b += d;
        }
        let apply = |x: u8| -> u8 {
            let mut y = 0u8;
            for i in 0..8 {
                if x >> i & 1 == 1 {
                    y |= 1 << sigma[i];
                }
            }
            y
        };
        let (mut sp, mut so) = (0, 0);
        for (li, l) in set.iter().enumerate() {
            let mut img = *l;
            for x in img.iter_mut() {
                *x = apply(*x);
            }
            img.sort_unstable();
            if img == *l {
                if pairs[li] {
                    sp += 1;
                } else {
                    so += 1;
                }
            }
        }
        let mut ct = part.clone();
        ct.push(1);
        // pair rule: every odd length occurs an even number of times (7 counted as length-1 cycle)
        let mut pair_rule = true;
        for len in (1..=8).step_by(2) {
            if ct.iter().filter(|&&x| x == len).count() % 2 == 1 {
                pair_rule = false;
            }
        }
        let chi2 = chi2_from_cycle_type(&ct);
        let c: Crit = crit(&fl2, &chi2, 1);
        rows.push(TypeRow {
            cycle_type: ct,
            stable_pair: sp,
            stable_other: so,
            pair_rule,
            chi2: (0..7).map(|i| chi2.get(i) as u32).collect(),
            fire_2: c.fire,
        });
    }
    Lag { total, pair, other: total - pair, rows }
}
