// CT229: bounded factor-base truncation on the existing CT226 F101 cubic cover.
// Compile against and REUSE the native genus-four Cantor arithmetic and
// elliptic source maps. Only ct229_ tests should be selected for timing;
// the included CT226 test is an independent, deliberately separately filtered test.
include!("ct226_native_genus4_transferred_ic.rs");

#[test]
fn ct229_reduced_base_full_solve_panel() {
    // Pre-registered, fixed lexicographic prefix: no adaptive selection.
    // No large-prime relations, no external input, no large-curve inference.
    const PANEL: [usize; 6] = [12, 18, 24, 36, 48, 54];
    const MAX_OFFSETS: usize = 60; // each offset: at most 109 tested shifts.
    let mut saw_full_control = false;
    for (i, &basis) in PANEL.iter().enumerate() {
        let g: Ec = Some((2, 37));
        let target: Ec = Some((64, 19));
        // Alternate comparator execution order to expose obvious order bias.
        let mut baseline_ns = 0u128;
        if i % 2 == 0 {
            let t = Instant::now();
            assert_eq!(direct_bsgs(g, target).0, 37);
            baseline_ns = t.elapsed().as_nanos();
        }
        let t_total = Instant::now();
        let curve = curve();
        let gdiv = image(&curve, g);
        let qdiv = image(&curve, target);
        let (all_x, first) = x_roster(&curve);
        assert_eq!(all_x.len(), 54);
        let xs = &all_x[..basis];
        let roster = offsets(&first);
        assert!(roster.len() >= MAX_OFFSETS);
        let setup_ns = t_total.elapsed().as_nanos();
        let relation_start = Instant::now();
        let mut pivots = vec![None; basis];
        let mut rank = 0usize;
        let mut candidates = 0usize;
        let mut smooth = 0usize;
        let mut used_offsets = 0usize;
        let mut cantor_adds = 0u64;
        let mut relations = Vec::new();
        for pts in roster.iter().take(MAX_OFFSETS) {
            used_offsets += 1;
            let mut cur = Div::identity(b(P));
            let mut off = vec![0i64; basis];
            for &(x,y) in pts {
                cur = cur.add_fast(&point(&curve, x, y), &curve);
                cantor_adds += 1;
                for (o,z) in off.iter_mut().zip(coord(x,y,xs)) { *o += z; }
            }
            for k in 0..R {
                candidates += 1;
                if let Some(row) = support(&cur, xs) {
                    smooth += 1;
                    let diff: Vec<i64> = row.iter().zip(&off).map(|(a,b)| a-b).collect();
                    let rhs = m(3*k, R);
                    if reduce_row(&mut pivots, &diff, rhs) { rank += 1; }
                    relations.push((diff,rhs));
                    if rank == basis { break; }
                }
                if k < R-1 {
                    cur = cur.add_fast(&gdiv, &curve);
                    cantor_adds += 1;
                }
            }
            if rank == basis { break; }
        }
        let relation_ns = relation_start.elapsed().as_nanos();
        let t_la = Instant::now();
        let logs = if rank == basis {
            let logs = solve(&pivots);
            for (row,rhs) in &relations {
                assert_eq!(m(row.iter().zip(&logs).map(|(a,b)| a*b).sum(), R), *rhs);
            }
            // Known-answer oracle is allowed ONLY as an after-solve check.
            for (j,&x) in xs.iter().enumerate() {
                let y = (0..P).find(|&y| {
                    m(y*y-(m(power(x,9,P)+power(x,3,P)+46,P)),P)==0
                }).unwrap();
                assert_eq!(ec_mul(logs[j],g),Some((power(x,3,P),y)));
            }
            Some(logs)
        } else { None };
        let la_ns = t_la.elapsed().as_nanos();
        let t_target = Instant::now();
        let mut recovered: Option<i64> = None;
        let mut target_candidates = 0usize;
        let mut target_adds = 0u64;
        if let Some(fb) = &logs {
            'outer: for pts in roster.iter().take(MAX_OFFSETS) {
                let mut cur = qdiv.clone();
                let mut off = vec![0i64; basis];
                for &(x,y) in pts {
                    cur=cur.add_fast(&point(&curve,x,y),&curve);
                    target_adds += 1;
                    for (o,z) in off.iter_mut().zip(coord(x,y,xs)) { *o+=z; }
                }
                for a in 0..R {
                    target_candidates += 1;
                    if let Some(row)=support(&cur,xs) {
                        let sum:i64 = row.iter().zip(&off).zip(fb)
                          .map(|((x,y),z)|(x-y)*z).sum();
                        let scalar=m(sum*inverse(3)-a,R);
                        assert_eq!(ec_mul(scalar,g),target,"incorrect recovered scalar");
                        recovered=Some(scalar);
                        break 'outer;
                    }
                    if a<R-1 {
                        cur=cur.add_fast(&gdiv,&curve);
                        target_adds += 1;
                    }
                }
            }
        }
        let recovery_ns = t_target.elapsed().as_nanos();
        let total_ns = t_total.elapsed().as_nanos();
        if i % 2 == 1 {
            let t = Instant::now();
            assert_eq!(direct_bsgs(g,target).0,37);
            baseline_ns=t.elapsed().as_nanos();
        }
        assert!(rank<=basis && candidates<=MAX_OFFSETS*R as usize);
        assert!(smooth<=candidates && baseline_ns>0);
        if basis==54 {
            assert_eq!((rank,smooth,candidates),(54,62,1421));
            assert_eq!(recovered,Some(37));
            saw_full_control=true;
        }
        // Missing target recovery is a recorded failure, not a substituted DLP.
        // Exact integers, no floating-point claims or shared setup amortization.
        eprintln!(
          "CT229_PANEL_JSON {{\"basis\":{},\"rank\":{},\"smooth\":{},\"candidates\":{},\"offsets\":{},\"rank_complete\":{},\"target_recovered\":{},\"target_candidates\":{},\"source_scalar\":{},\"setup_ns\":{},\"relations_ns\":{},\"la_ns\":{},\"target_ns\":{},\"total_ns\":{},\"bsgs_ns\":{},\"cantor_adds\":{},\"target_adds\":{},\"model\":\"F101_known_answer_first_b_x_prefix\",\"pcc\":\"unverified\"}}",
          basis,rank,smooth,candidates,used_offsets,rank==basis,recovered.is_some(),
          target_candidates,recovered.map(|x|x.to_string()).unwrap_or("null".to_string()),
          setup_ns,relation_ns,la_ns,recovery_ns,total_ns,baseline_ns,
          cantor_adds,target_adds
        );
    }
    assert!(saw_full_control);
}
