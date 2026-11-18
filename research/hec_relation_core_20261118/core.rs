use hec_native_ic::solve::{solve_fast, Row, SolveStats};

pub fn inspect(rows: &[(Row, u64)], small: usize, target: usize, ell: u64) {
    let mut original_stats = SolveStats::default();
    let original = solve_fast(rows, target, ell, &mut original_stats);
    let mut live = vec![true; rows.len()];
    let nnz = rows.iter().map(|(r, _)| r.len()).sum::<usize>();
    let mut rounds = 0;
    let mut first = 0;
    let mut large_only_first = 0;
    loop {
        let mut degree = vec![0usize; target + 1];
        for (i, (row, _)) in rows.iter().enumerate() {
            if live[i] {
                for &(j, a) in row {
                    assert!(a > 0 && a < ell);
                    degree[j as usize] += 1;
                }
            }
        }
        let mut removed = 0;
        for (i, (row, _)) in rows.iter().enumerate() {
            if !live[i] {
                continue;
            }
            let singleton = |j: u32| j as usize != target && degree[j as usize] == 1;
            if row.iter().any(|&(j, _)| singleton(j)) {
                if rounds == 0
                    && row
                        .iter()
                        .any(|&(j, _)| singleton(j) && j as usize >= small)
                {
                    large_only_first += 1;
                }
                live[i] = false;
                removed += 1;
            }
        }
        if removed == 0 {
            break;
        }
        if rounds == 0 {
            first = removed;
        }
        rounds += 1;
    }
    let core: Vec<_> = rows
        .iter()
        .zip(live)
        .filter(|(_, keep)| *keep)
        .map(|(r, _)| r.clone())
        .collect();
    let mut stats = SolveStats::default();
    let k = solve_fast(&core, target, ell, &mut stats);
    assert_eq!(original, k);
    println!("CORE rows={} nnz={} core={} core_nnz={} first={} large_first={} rounds={} original_mul={} core_mul={} k={:?}",rows.len(),nnz,core.len(),core.iter().map(|(r,_)|r.len()).sum::<usize>(),first,large_only_first,rounds,original_stats.mulmods,stats.mulmods,k);
}
