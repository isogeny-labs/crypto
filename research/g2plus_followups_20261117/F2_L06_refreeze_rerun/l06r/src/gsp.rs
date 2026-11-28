use crate::crit::crit;
use crate::fp::Fp;
use crate::poly::Poly;
use crate::rng::SplitMix64;

pub type Vecn = [u64; 6];

pub fn om(g: usize, l: u64, x: &Vecn, y: &Vecn) -> u64 {
    let mut s = 0u64;
    for i in 0..g {
        s += x[i] * y[g + i] + (l - x[g + i]) * y[i];
    }
    s % l
}

/// Columns of a uniform Sp_2g(F_l) matrix: e_1..e_g, f_1..f_g.
pub fn sample_sp(rng: &mut SplitMix64, g: usize, l: u64, fl: &Fp) -> Vec<Vecn> {
    let n = 2 * g;
    let mut es: Vec<Vecn> = vec![];
    let mut fs: Vec<Vecn> = vec![];
    let proj = |x: &Vecn, es: &Vec<Vecn>, fs: &Vec<Vecn>| -> Vecn {
        let mut r = *x;
        for j in 0..es.len() {
            let a = om(g, l, x, &fs[j]);
            let bb = om(g, l, x, &es[j]);
            for t in 0..n {
                // r -= a e_j - bb f_j
                r[t] = (r[t] + l * 2 - a * es[j][t] % l + bb * fs[j][t] % l) % l;
            }
        }
        r
    };
    for _k in 0..g {
        let ek;
        loop {
            let mut v = [0u64; 6];
            for t in 0..n {
                v[t] = rng.uniform(l);
            }
            let w = proj(&v, &es, &fs);
            if w.iter().any(|&a| a != 0) {
                ek = w;
                break;
            }
        }
        let fk;
        loop {
            let mut v = [0u64; 6];
            for t in 0..n {
                v[t] = rng.uniform(l);
            }
            let v = proj(&v, &es, &fs);
            let c = om(g, l, &ek, &v);
            if c != 0 {
                let ic = fl.inv(c);
                let mut f = [0u64; 6];
                for t in 0..n {
                    f[t] = v[t] * ic % l;
                }
                fk = f;
                break;
            }
        }
        es.push(ek);
        fs.push(fk);
    }
    let mut cols = es;
    cols.extend(fs);
    cols
}

/// Characteristic polynomial det(T I - G) by subset expansion; G given by columns.
pub fn charpoly(fl: &Fp, g_cols: &[Vecn], n: usize) -> Poly {
    let l = fl.p;
    // A[r][j] = T delta - G[r][j], G[r][j] = cols[j][r]
    let mut a = vec![vec![Poly::zero(); n]; n];
    for r in 0..n {
        for j in 0..n {
            let c0 = (l - g_cols[j][r] % l) % l;
            let c1 = if r == j { 1 } else { 0 };
            a[r][j] = Poly::from(&[c0, c1], l);
        }
    }
    let full = (1usize << n) - 1;
    let mut dp = vec![Poly::zero(); full + 1];
    dp[0] = Poly::one();
    for s in 1..=full {
        let r = (s as u32).count_ones() as usize - 1;
        let mut acc = Poly::zero();
        for j in 0..n {
            if s >> j & 1 == 0 {
                continue;
            }
            let greater = ((s >> (j + 1)) as u32).count_ones();
            let term = fl.pmul(&a[r][j], &dp[s & !(1 << j)]);
            if greater % 2 == 0 {
                acc = fl.padd(&acc, &term);
            } else {
                acc = fl.psub(&acc, &term);
            }
        }
        dp[s] = acc;
    }
    dp[full]
}

pub struct Row3 {
    pub l: u64,
    pub m: u64,
    pub samples: u64,
    pub fire: u64,
    pub nz1: u64,
    pub fire_nz1: u64,
    pub squarefree: u64,
    pub none: u64,
    pub undetermined: u64,
    pub g4_agree: u64,
    pub sim_fail: u64,
}

pub fn run(g: usize, l: u64, m: u64, samples: u64) -> Row3 {
    let fl = Fp::new(l);
    let mut rng = SplitMix64::new(0x4C06_5500_0000_0000u64 ^ ((g as u64) << 32) ^ (l << 16) ^ m);
    let mut row = Row3 {
        l,
        m,
        samples,
        fire: 0,
        nz1: 0,
        fire_nz1: 0,
        squarefree: 0,
        none: 0,
        undetermined: 0,
        g4_agree: 0,
        sim_fail: 0,
    };
    let n = 2 * g;
    for _ in 0..samples {
        let mut cols = sample_sp(&mut rng, g, l, &fl);
        for k in g..n {
            for t in 0..n {
                cols[k][t] = cols[k][t] * m % l;
            }
        }
        // G^T Om G = m Om
        let mut ok = true;
        for i in 0..n {
            for j in 0..n {
                let expect = if j == i + g && i < g {
                    m % l
                } else if i == j + g && j < g {
                    (l - m % l) % l
                } else {
                    0
                };
                if om(g, l, &cols[i], &cols[j]) != expect {
                    ok = false;
                }
            }
        }
        if !ok {
            row.sim_fail += 1;
        }
        let c = charpoly(&fl, &cols, n);
        let cr = crit(&fl, &c, m % l);
        let nz = fl.peval(&c, 1) != 0;
        if cr.fire {
            row.fire += 1;
        }
        if nz {
            row.nz1 += 1;
            if cr.fire {
                row.fire_nz1 += 1;
            }
        }
        if cr.squarefree {
            row.squarefree += 1;
        }
        match cr.class {
            "none" => row.none += 1,
            "undetermined" => row.undetermined += 1,
            _ => {}
        }
        if cr.g4() {
            row.g4_agree += 1;
        }
    }
    row
}
