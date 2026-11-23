//! Mulders–Storjohann weak Popov reduction of a polynomial matrix over F_2[t].
//! Exact; over F_2 every leading coefficient is 1, so a simple transformation is
//! row_big ^= row_small << (deg_big - deg_small).
use crate::gf2x::P;

#[derive(Default, Clone, Debug)]
pub struct Stats { pub transforms: u64, pub word_xors: u64 }

pub fn row_deg_lp(row: &[P]) -> (i64, usize) {
    let mut d = -1i64;
    let mut lp = 0usize;
    for (j, e) in row.iter().enumerate() {
        let de = e.deg();
        if de >= 0 && de >= d { d = de; lp = j; }
    }
    (d, lp)
}

pub fn row_deg(row: &[P]) -> i64 { row_deg_lp(row).0 }

fn xor_rows(rows: &mut [Vec<P>], big: usize, small: usize, sh: usize, st: &mut Stats) {
    let (a, b) = if big < small {
        let (l, r) = rows.split_at_mut(small);
        (&mut l[big], &r[0])
    } else {
        let (l, r) = rows.split_at_mut(big);
        (&mut r[0], &l[small])
    };
    for c in 0..a.len() { st.word_xors += a[c].xor_shl(&b[c], sh); }
    st.transforms += 1;
}

/// Reduce a generating set (any number of rows) to weak Popov form; zero rows are dropped.
pub fn weak_popov(mut rows: Vec<Vec<P>>, st: &mut Stats) -> Vec<Vec<P>> {
    if rows.is_empty() { return rows; }
    let nc = rows[0].len();
    let mut owner: Vec<Option<usize>> = vec![None; nc];
    let mut info: Vec<(i64, usize)> = rows.iter().map(|r| row_deg_lp(r)).collect();
    for start in 0..rows.len() {
        let mut i = start;
        loop {
            let (d, lp) = info[i];
            if d < 0 { break; }
            match owner[lp] {
                None => { owner[lp] = Some(i); break; }
                Some(j) => {
                    let dj = info[j].0;
                    let (big, small) = if d >= dj { (i, j) } else { owner[lp] = Some(i); (j, i) };
                    let sh = (info[big].0 - info[small].0) as usize;
                    xor_rows(&mut rows, big, small, sh, st);
                    info[big] = row_deg_lp(&rows[big]);
                    i = big;
                }
            }
        }
    }
    let mut out: Vec<Vec<P>> = rows.into_iter().filter(|r| row_deg(r) >= 0).collect();
    out.sort_by_key(|r| row_deg(r));
    out
}

/// check weak Popov property: distinct leading positions
pub fn is_weak_popov(rows: &[Vec<P>]) -> bool {
    let mut seen = std::collections::HashSet::new();
    rows.iter().all(|r| seen.insert(row_deg_lp(r).1))
}
