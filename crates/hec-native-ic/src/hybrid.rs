//! Sparse elimination followed by a dense block on the walk columns and k.
use crate::fp::Fp;
use crate::solve::{Row, SolveStats};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct BlockRow {
    sparse: Row,
    dense: Vec<u64>,
    rhs: u64,
}

/// The caller supplies hot columns (the step places); k is always dense.
/// Free variables remain free. Only a k-only row determines the answer.
pub fn solve(
    rows: &[(Row, u64)],
    m: usize,
    ell: u64,
    hot: &[u32],
    st: &mut SolveStats,
) -> Option<u64> {
    let f = Fp::new(ell);
    let mut hot = hot.to_vec();
    hot.push(m as u32);
    hot.sort_unstable();
    hot.dedup();
    assert!(hot.iter().all(|&c| c <= m as u32));
    let mut index = vec![usize::MAX; m + 1];
    for (i, &c) in hot.iter().enumerate() {
        index[c as usize] = i;
    }
    let kc = index[m];
    let n = hot.len();
    let mut live: Vec<Option<BlockRow>> = Vec::new();
    let mut holders = vec![Vec::<usize>::new(); m];
    let mut counts = vec![0u32; m];
    for (ri, (row, rhs)) in rows.iter().enumerate() {
        let mut sparse = Row::new();
        let mut dense = vec![0; n];
        for &(c, v) in row {
            let j = index[c as usize];
            if j != usize::MAX {
                dense[j] = v;
            } else {
                sparse.push((c, v));
                holders[c as usize].push(ri);
                counts[c as usize] += 1;
            }
        }
        live.push(Some(BlockRow {
            sparse,
            dense,
            rhs: *rhs,
        }));
    }
    let mut heap: BinaryHeap<Reverse<(u32, u32)>> = (0..m)
        .filter(|&c| counts[c] > 0)
        .map(|c| Reverse((counts[c], c as u32)))
        .collect();
    let mut done = vec![false; m];
    while let Some(Reverse((cnt, c))) = heap.pop() {
        let cu = c as usize;
        if done[cu] || cnt == 0 || cnt != counts[cu] {
            continue;
        }
        let mut hs = std::mem::take(&mut holders[cu]);
        hs.sort_unstable();
        hs.dedup();
        hs.retain(|&r| {
            live[r]
                .as_ref()
                .is_some_and(|r| r.sparse.binary_search_by_key(&c, |e| e.0).is_ok())
        });
        let Some(&pr) = hs
            .iter()
            .min_by_key(|&&r| live[r].as_ref().unwrap().sparse.len())
        else {
            continue;
        };
        done[cu] = true;
        let p = live[pr].take().unwrap();
        st.pivots += 1;
        st.pivot_hash =
            (st.pivot_hash ^ ((c as u64) << 32 | pr as u64)).wrapping_mul(0x100000001b3);
        let inv = f.inv(p.sparse[p.sparse.binary_search_by_key(&c, |e| e.0).unwrap()].1);
        let mut touched = Vec::new();
        for &(cc, _) in &p.sparse {
            counts[cc as usize] -= 1;
            touched.push(cc);
        }
        for ri in hs {
            if ri == pr {
                continue;
            }
            let mut r = live[ri].take().unwrap();
            let factor = f.mul(
                r.sparse[r.sparse.binary_search_by_key(&c, |e| e.0).unwrap()].1,
                inv,
            );
            let mut out = Row::with_capacity(r.sparse.len() + p.sparse.len());
            let (mut i, mut j) = (0, 0);
            while i < r.sparse.len() || j < p.sparse.len() {
                if j == p.sparse.len() || (i < r.sparse.len() && r.sparse[i].0 < p.sparse[j].0) {
                    out.push(r.sparse[i]);
                    i += 1;
                } else if i == r.sparse.len() || p.sparse[j].0 < r.sparse[i].0 {
                    let (cc, v) = p.sparse[j];
                    out.push((cc, f.neg(f.mul(factor, v))));
                    st.mulmods += 1;
                    counts[cc as usize] += 1;
                    touched.push(cc);
                    holders[cc as usize].push(ri);
                    j += 1;
                } else {
                    let (cc, v) = r.sparse[i];
                    let v = f.sub(v, f.mul(factor, p.sparse[j].1));
                    st.mulmods += 1;
                    if v != 0 {
                        out.push((cc, v));
                    } else {
                        counts[cc as usize] -= 1;
                        touched.push(cc);
                    }
                    i += 1;
                    j += 1;
                }
            }
            r.sparse = out;
            for j in 0..n {
                if p.dense[j] != 0 {
                    r.dense[j] = f.sub(r.dense[j], f.mul(factor, p.dense[j]));
                    st.mulmods += 1;
                }
            }
            r.rhs = f.sub(r.rhs, f.mul(factor, p.rhs));
            st.row_ops += 1;
            st.max_row_len = st
                .max_row_len
                .max(r.sparse.len() + r.dense.iter().filter(|&&v| v != 0).count());
            live[ri] = Some(r);
        }
        touched.sort_unstable();
        touched.dedup();
        for cc in touched {
            if !done[cc as usize] && counts[cc as usize] > 0 {
                heap.push(Reverse((counts[cc as usize], cc)));
            }
        }
    }
    // Echelon rows indexed by their first nonzero dense column. k is last,
    // hence a pivot there is independent of all earlier free variables.
    let mut pivots: Vec<Option<(Vec<u64>, u64)>> = (0..n).map(|_| None).collect();
    let mut answer = None;
    for r in live.into_iter().flatten() {
        if !r.sparse.is_empty() {
            continue;
        }
        let (mut a, mut rhs) = (r.dense, r.rhs);
        let mut stored = false;
        for j in 0..n {
            if a[j] == 0 {
                continue;
            }
            if let Some((p, b)) = &pivots[j] {
                let factor = f.mul(a[j], f.inv(p[j]));
                for k in j..n {
                    a[k] = f.sub(a[k], f.mul(factor, p[k]));
                    st.mulmods += 1;
                }
                rhs = f.sub(rhs, f.mul(factor, *b));
                st.row_ops += 1;
            } else {
                if j == kc {
                    answer = Some(f.mul(rhs, f.inv(a[j])));
                }
                pivots[j] = Some((a.clone(), rhs));
                st.pivots += 1;
                stored = true;
                break;
            }
        }
        if !stored && a.iter().all(|&v| v == 0) && rhs != 0 {
            return None;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn free_and_inconsistent() {
        assert_eq!(
            solve(
                &[(vec![(0, 1), (2, 1)], 7)],
                2,
                101,
                &[0],
                &mut SolveStats::default()
            ),
            None
        );
        assert_eq!(
            solve(
                &[(vec![(2, 2)], 14)],
                2,
                101,
                &[0],
                &mut SolveStats::default()
            ),
            Some(7)
        );
        assert_eq!(
            solve(
                &[(vec![(2, 1)], 7), (vec![(2, 1)], 8)],
                2,
                101,
                &[0],
                &mut SolveStats::default()
            ),
            None
        );
    }
}
