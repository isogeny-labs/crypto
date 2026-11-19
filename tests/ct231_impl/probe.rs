// CT231: preregistered one-large-prime pairing on the CT226 genus-4 cubic cover.
// Parent source supplies the actual native Cantor arithmetic, source transfer,
// signed-support extraction, F109 eliminator, and elliptic BSGS.
// See CT231_SINGLE_LARGE_PRIME_PROTOCOL.md; [pcc:unverified].
use super::*;
use std::collections::BTreeMap;

#[derive(Clone)]
struct OneLp {
    // Selected-base signed relation (including the negated fixed offset).
    row: Vec<i64>,
    rhs: i64,
    sign: i64, // coefficient of the excluded canonical point, ±1.
}

// For a source-group relation r·log(B) + s·log(L) = rhs (mod 109),
// two relations with the SAME L eliminate L exactly.
fn join_one_lp(first: &OneLp, next: &OneLp) -> (Vec<i64>, i64) {
    assert!(first.sign.abs() == 1 && next.sign.abs() == 1);
    assert_eq!(first.row.len(), next.row.len());
    let row = first.row.iter().zip(&next.row)
        .map(|(a,z)| m(first.sign * z - next.sign * a, R))
        .collect();
    let rhs = m(first.sign * next.rhs - next.sign * first.rhs, R);
    (row, rhs)
}

fn point_log_check(x:i64, log:i64, generator:Ec) {
    let yy=(0..P).find(|&y|
        m(y*y-m(power(x,9,P)+power(x,3,P)+46,P),P)==0
    ).expect("canonical rational point");
    assert_eq!(ec_mul(log,generator),Some((power(x,3,P), yy)),
        "wrong F109 norm logarithm for factor-base or large-prime point");
}

#[test]
fn ct231_single_lp_join_signs_and_rhs() {
    // Synthetic exact F109 relation elimination, not a measured solver result.
    let first=OneLp{row:vec![2,-5,0],rhs:17,sign:-1};
    let next=OneLp{row:vec![7,0,-3],rhs:41,sign:1};
    let (v,t)=join_one_lp(&first,&next);
    assert_eq!(v,vec![m(-9,R),5,3]);
    assert_eq!(t,m(-58,R));
    let logs=[11,19,23];
    let lp=m(first.sign*(first.rhs-first.row.iter().zip(logs)
        .map(|(a,z)|a*z).sum::<i64>()),R);
    let rhs1=m(first.row.iter().zip(logs).map(|(a,z)|a*z).sum::<i64>()
        +first.sign*lp,R);
    let rhs2=m(next.row.iter().zip(logs).map(|(a,z)|a*z).sum::<i64>()
        +next.sign*lp,R);
    // Arbitrary rows need not be consistent; the derived eliminated equation
    // remains the algebraic combination of their augmented equations.
    assert_eq!(m(v.iter().zip(logs).map(|(a,z)|a*z).sum::<i64>()-t,R),
        m(first.sign*(rhs2-next.rhs)-next.sign*(rhs1-first.rhs),R));
    let flipped=OneLp{sign:-next.sign, ..next};
    let (v2,t2)=join_one_lp(&first,&flipped);
    assert_eq!(v2,vec![m(-5,R),m(-5,R),3]);
    assert_eq!(t2,m(-24,R));
}

#[test]
fn ct231_single_lp_complete_solve_panel() {
    const PANEL:[usize;4]=[24,36,48,54];
    const MAX_OFFSETS:usize=60;
    let g:Ec=Some((2,37));
    let target:Ec=Some((64,19));
    let mut saw_control=false;

    for (arm,&basis) in PANEL.iter().enumerate() {
        let mut baseline_ns=0u128;
        if arm%2==0 {
            let t=Instant::now();
            assert_eq!(direct_bsgs(g,target).0,37);
            baseline_ns=t.elapsed().as_nanos();
        }
        let t_total=Instant::now();
        let c=curve();
        let gdiv=image(&c,g);
        let qdiv=image(&c,target);
        let (all_x,first)=x_roster(&c);
        assert_eq!(all_x.len(),54);
        let selected=&all_x[..basis];
        let roster=offsets(&first);
        assert!(roster.len()>=MAX_OFFSETS);
        let setup_ns=t_total.elapsed().as_nanos();

        let t_rel=Instant::now();
        let mut pivots=vec![None;basis];
        let mut first_lp:BTreeMap<usize,OneLp>=BTreeMap::new();
        let mut every_lp:Vec<(usize,OneLp)>=Vec::new();
        let mut accepted:Vec<(Vec<i64>,i64)>=Vec::new();
        let mut rank=0usize;
        let mut candidates=0usize;
        let mut used_offsets=0usize;
        let mut full=0usize;
        let mut single=0usize;
        let mut discarded_multi=0usize;
        let mut joined=0usize;
        let mut cantor_adds=0u64;

        for pts in roster.iter().take(MAX_OFFSETS) {
            used_offsets+=1;
            let mut cur=Div::identity(b(P));
            let mut off=vec![0i64;basis];
            for &(x,y) in pts {
                cur=cur.add_fast(&point(&c,x,y),&c);
                cantor_adds+=1;
                for (a,z) in off.iter_mut().zip(coord(x,y,selected)) { *a+=z; }
            }
            for k in 0..R {
                candidates+=1;
                if let Some(support_all)=support(&cur,&all_x) {
                    let outside:Vec<(usize,i64)>=support_all.iter().enumerate()
                        .skip(basis).filter(|(_,s)| **s!=0)
                        .map(|(i,s)|(i,*s)).collect();
                    if outside.len()<=1 {
                        let row:Vec<i64>=support_all[..basis].iter()
                            .zip(&off).map(|(a,z)|a-z).collect();
                        let rhs=m(3*k,R);
                        if outside.is_empty() {
                            full+=1;
                            if reduce_row(&mut pivots,&row,rhs) {rank+=1;}
                            accepted.push((row,rhs));
                        } else {
                            single+=1;
                            let (key,sign)=outside[0];
                            assert!(sign.abs()==1);
                            let relation=OneLp{row,rhs,sign};
                            if let Some(old)=first_lp.get(&key) {
                                let (newrow,newrhs)=join_one_lp(old,&relation);
                                joined+=1;
                                if reduce_row(&mut pivots,&newrow,newrhs) {rank+=1;}
                                accepted.push((newrow,newrhs));
                            } else {
                                first_lp.insert(key,relation.clone());
                            }
                            every_lp.push((key,relation));
                        }
                    } else {
                        discarded_multi+=1;
                    }
                    if rank==basis { break; }
                }
                if k<R-1 {cur=cur.add_fast(&gdiv,&c);cantor_adds+=1;}
            }
            if rank==basis {break;}
        }
        let relation_ns=t_rel.elapsed().as_nanos();

        let t_la=Instant::now();
        let logs=if rank==basis {
            let logs=solve(&pivots);
            for (v,rhs) in &accepted {
                assert_eq!(m(v.iter().zip(&logs).map(|(a,z)|a*z).sum::<i64>(),R),
                    *rhs,"false direct or joined relation");
            }
            Some(logs)
        } else {None};
        let la_ns=t_la.elapsed().as_nanos();

        let t_verify=Instant::now();
        let mut recovered_lp:BTreeMap<usize,i64>=BTreeMap::new();
        if let Some(ref logs)=logs {
            // No norm logarithm oracle before the original matrix reached rank.
            for (&x,&log) in selected.iter().zip(logs) {
                point_log_check(x,log,g);
            }
            for (&key,first) in &first_lp {
                let dot:i64=first.row.iter().zip(logs)
                    .map(|(a,z)|a*z).sum();
                let lp=m(first.sign*(first.rhs-dot),R);
                point_log_check(all_x[key],lp,g);
                recovered_lp.insert(key,lp);
            }
            // Verify EVERY retained single-large-prime relation with solved logs.
            for (key,rel) in &every_lp {
                let dot:i64=rel.row.iter().zip(logs)
                    .map(|(a,z)|a*z).sum();
                assert_eq!(m(dot+rel.sign*recovered_lp[key],R),rel.rhs);
            }
        }
        let verify_ns=t_verify.elapsed().as_nanos();

        let t_target=Instant::now();
        let mut recovered:Option<i64>=None;
        let mut target_candidates=0usize;
        let mut target_adds=0u64;
        if let Some(ref fb)=logs {
            'target:for pts in roster.iter().take(MAX_OFFSETS) {
                let mut cur=qdiv.clone();
                let mut off=vec![0i64;basis];
                for &(x,y) in pts {
                    cur=cur.add_fast(&point(&c,x,y),&c);
                    target_adds+=1;
                    for (a,z) in off.iter_mut().zip(coord(x,y,selected)) { *a+=z; }
                }
                for a in 0..R {
                    target_candidates+=1;
                    if let Some(sup)=support(&cur,&all_x) {
                        let outside:Vec<(usize,i64)>=sup.iter().enumerate()
                            .skip(basis).filter(|(_,s)| **s!=0)
                            .map(|(i,s)|(i,*s)).collect();
                        if outside.is_empty() ||
                          (outside.len()==1 && recovered_lp.contains_key(&outside[0].0)) {
                            let mut sum:i64=sup[..basis].iter()
                                .zip(&off).zip(fb).map(|((x,y),z)|(x-y)*z).sum();
                            if outside.len()==1 {
                                sum+=outside[0].1*recovered_lp[&outside[0].0];
                            }
                            let scalar=m(sum*inverse(3)-a,R);
                            assert_eq!(ec_mul(scalar,g),target);
                            recovered=Some(scalar);
                            break 'target;
                        }
                    }
                    if a<R-1 {cur=cur.add_fast(&gdiv,&c);target_adds+=1;}
                }
            }
        }
        let target_ns=t_target.elapsed().as_nanos();
        let total_ns=t_total.elapsed().as_nanos();

        if arm%2==1 {
            let t=Instant::now();
            assert_eq!(direct_bsgs(g,target).0,37);
            baseline_ns=t.elapsed().as_nanos();
        }
        assert!(candidates<=MAX_OFFSETS*R as usize);
        assert!(target_candidates<=MAX_OFFSETS*R as usize);
        assert!(first_lp.len()<=54-basis);
        if basis==54 {
            assert_eq!((rank,full,candidates),(54,62,1421),
                "CT226/CT229 unchanged full-base control drift");
            assert_eq!(single,0);
            assert_eq!(recovered,Some(37));
            saw_control=true;
        }
        if let Some(scalar)=recovered {assert_eq!(scalar,37);}
        // No completed-solve ratio for rank/target failures.
        eprintln!(
          "CT231_PANEL_JSON {{\"basis\":{},\"rank\":{},\"candidates\":{},\"offsets\":{},\"full\":{},\"single_lp\":{},\"multi_lp_discard\":{},\"pair_rows\":{},\"distinct_lp\":{},\"target_recovered\":{},\"scalar\":{},\"target_candidates\":{},\"setup_ns\":{},\"relations_ns\":{},\"la_ns\":{},\"verification_ns\":{},\"target_ns\":{},\"total_ns\":{},\"bsgs_ns\":{},\"cantor_adds\":{},\"target_adds\":{},\"model\":\"F101_signed_one_lp\",\"pcc\":\"unverified\"}}",
          basis,rank,candidates,used_offsets,full,single,discarded_multi,
          joined,first_lp.len(),recovered.is_some(),
          recovered.map(|v|v.to_string()).unwrap_or("null".into()),
          target_candidates,setup_ns,relation_ns,la_ns,verify_ns,target_ns,
          total_ns,baseline_ns,cantor_adds,target_adds
        );
    }
    assert!(saw_control);
}
