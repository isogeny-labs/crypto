// CT235: Q73-only, fresh weighted-2LP model and four ABBA paired rounds.
// Preregistered in CT235_SINGLE_TARGET_COLD_PROTOCOL.md; [pcc:unverified].
use super::*;

#[test]
fn ct235_q73_cold_single_panel() {
    const PANEL:[usize;4]=[24,36,48,54];
    const MAX_OFFSETS:usize=60;
    let g:Ec=Some((2,37));
    let target:Ec=Some((19,37)); // only public Q73
    for (_arm,&basis) in PANEL.iter().enumerate() {
      for round in 0..4 {
        let mut bsgs_ns=0u128;
        let candidate_first=round==0 || round==3; // ABBA schedule
        if !candidate_first {
            let t=Instant::now();
            assert_eq!(direct_bsgs(g,target).0,73);
            bsgs_ns=t.elapsed().as_nanos();
        }
        let t_all=Instant::now();
        let tally0=ops::snapshot().total();
        let c=curve();
        let gdiv=image(&c,g);
        let qdiv=image(&c,target);
        let (all_x,first)=x_roster(&c);
        assert_eq!(all_x.len(),54);
        let selected=&all_x[..basis];
        let roster=offsets(&first);
        assert!(roster.len()>=MAX_OFFSETS);
        let outside_count=54-basis;
        let setup_ns=t_all.elapsed().as_nanos();
        let t_rel=Instant::now();
        let mut pivots=vec![None;basis];
        let mut lp_pivots:Vec<Option<EqRow>>=vec![None;outside_count];
        let mut graph=Graph::new(outside_count+1); // last vertex = sentinel
        let mut all_relations:Vec<EqRow>=Vec::new();
        let mut accepted:Vec<(Vec<i64>,i64)>=Vec::new();
        let mut rank=0usize;
        let mut candidates=0usize;
        let mut used_offsets=0usize;
        let mut full=0usize;
        let mut single=0usize;
        let mut double=0usize;
        let mut rejected_multi=0usize;
        let mut b_only=0usize;
        let mut cantor_adds=0u64;
        let mut peak_logical=0usize;
        for pts in roster.iter().take(MAX_OFFSETS) {
            used_offsets+=1;
            let mut cur=Div::identity(b(P));
            let mut off=vec![0i64;basis];
            for &(x,y) in pts {
                cur=cur.add_fast(&point(&c,x,y),&c);
                cantor_adds+=1;
                for (a,z) in off.iter_mut().zip(coord(x,y,selected)) {*a+=z;}
            }
            for k in 0..R {
                candidates+=1;
                if let Some(support_all)=support(&cur,&all_x) {
                    let outside:Vec<(usize,i64)>=support_all.iter().enumerate()
                        .skip(basis).filter(|(_,s)| **s!=0)
                        .map(|(i,s)|(i-basis,*s)).collect();
                    if outside.len()<=2 {
                        let row:Vec<i64>=support_all[..basis].iter()
                            .zip(&off).map(|(a,z)|a-z).collect();
                        let mut lp=vec![0i64;outside_count];
                        for &(i,sign) in &outside {
                            assert_eq!(sign.abs(),1);
                            lp[i]=sign;
                        }
                        match outside.len() {
                            0=>{full+=1;},
                            1=>{single+=1;graph.edge(outside[0].0,outside_count);},
                            2=>{double+=1;graph.edge(outside[0].0,outside[1].0);},
                            _=>unreachable!()
                        }
                        let eq=EqRow{b:row,lp,rhs:m(3*k,R)};
                        if let Some((brow,rhs))=lp_insert(&mut lp_pivots,eq.clone()){
                            b_only+=1;
                            if reduce_row(&mut pivots,&brow,rhs){rank+=1;}
                            accepted.push((brow,rhs));
                        }
                        all_relations.push(eq);
                        peak_logical=peak_logical.max(mem_bytes(
                            &lp_pivots,&accepted,&all_relations,&graph));
                    } else {rejected_multi+=1;}
                    if rank==basis {break;}
                }
                if k<R-1 {cur=cur.add_fast(&gdiv,&c);cantor_adds+=1;}
            }
            if rank==basis {break;}
        }
        let relations_ns=t_rel.elapsed().as_nanos();
        let t_la=Instant::now();
        let fb=if rank==basis {
            let fb=solve(&pivots);
            for (r,rhs) in &accepted {
                assert_eq!(m(r.iter().zip(&fb).map(|(a,z)|a*z).sum(),R),*rhs);
            }
            Some(fb)
        } else {None};
        let la_ns=t_la.elapsed().as_nanos();
        let t_verify=Instant::now();
        let mut lp_logs=vec![None;outside_count];
        let mut verified_rels=0usize;
        if let Some(ref logs)=fb {
            for (&x,&log) in selected.iter().zip(logs) {
                checked_norm_log(x,log,g);
            }
            lp_logs=recover_lp(&lp_pivots,logs);
            for (i,z) in lp_logs.iter().enumerate(){
                if let Some(log)=z {checked_norm_log(all_x[basis+i],*log,g);}
            }
            for eq in &all_relations {
                if eq.lp.iter().enumerate().any(|(i,x)|*x!=0 && lp_logs[i].is_none()){
                    continue;
                }
                let lhs:i64=eq.b.iter().zip(logs).map(|(a,z)|a*z).sum::<i64>()
                   +eq.lp.iter().enumerate()
                       .map(|(i,a)|a*lp_logs[i].unwrap_or(0)).sum::<i64>();
                assert_eq!(m(lhs,R),eq.rhs,"false weighted 2LP relation");
                verified_rels+=1;
            }
        }
        let verify_ns=t_verify.elapsed().as_nanos();
        let t_target=Instant::now();
        let mut got:Option<i64>=None;
        let mut target_candidates=0usize;
        let mut target_adds=0u64;
        if let Some(ref logs)=fb {
            'search:for pts in roster.iter().take(MAX_OFFSETS) {
                let mut cur=qdiv.clone();
                let mut off=vec![0i64;basis];
                for &(x,y) in pts {
                    cur=cur.add_fast(&point(&c,x,y),&c);
                    target_adds+=1;
                    for (a,z) in off.iter_mut().zip(coord(x,y,selected)) {*a+=z;}
                }
                for a in 0..R {
                    target_candidates+=1;
                    if let Some(sup)=support(&cur,&all_x) {
                        let outside:Vec<(usize,i64)>=sup.iter().enumerate()
                            .skip(basis).filter(|(_,s)| **s!=0)
                            .map(|(i,s)|(i-basis,*s)).collect();
                        if outside.len()<=2
                            && outside.iter().all(|(i,_)|lp_logs[*i].is_some()) {
                            let mut norm:i64=sup[..basis].iter()
                                .zip(&off).zip(logs)
                                .map(|((v,o),z)|(v-o)*z).sum();
                            for (i,sign) in outside {norm+=sign*lp_logs[i].unwrap();}
                            let answer=m(norm*inverse(3)-a,R);
                            assert_eq!(ec_mul(answer,g),target,
                                "incorrect computed Q73 scalar");
                            got=Some(answer);
                            break 'search;
                        }
                    }
                    if a<R-1 {cur=cur.add_fast(&gdiv,&c);target_adds+=1;}
                }
            }
        }
        let target_ns=t_target.elapsed().as_nanos();
        let total_ns=t_all.elapsed().as_nanos();
        if candidate_first {
            let t=Instant::now();
            assert_eq!(direct_bsgs(g,target).0,73);
            bsgs_ns=t.elapsed().as_nanos();
        }
        assert_eq!(got,Some(73),"cold Q73 target not recovered");
        if basis==54 {
            assert_eq!((rank,full,candidates),(54,62,1421),
               "CT226/CT231 unchanged full-base control drift");
            assert_eq!((single,double),(0,0));
            assert_eq!(got,Some(73));
        }
        assert!(candidates<=MAX_OFFSETS*R as usize);
        assert!(target_candidates<=MAX_OFFSETS*R as usize);
        let observed_lp=lp_pivots.iter().filter(|p|p.is_some()).count();
        let known_lp=lp_logs.iter().filter(|v|v.is_some()).count();
        let tally=ops::snapshot().total().since(&tally0);
        let rss=vmhwm_kib().map(|v|v.to_string()).unwrap_or_else(||"null".into());
        eprintln!(
            "CT235_SINGLE_JSON {{\"basis\":{},\"round\":{},\"arm_order\":\"{}\",\"rank\":{},\"candidates\":{},\"offsets\":{},\"full\":{},\"single_lp\":{},\"double_lp\":{},\"multi_lp_discard\":{},\"b_only_rows\":{},\"lp_pivot_rank\":{},\"known_lp\":{},\"graph_edges\":{},\"graph_cycles\":{},\"graph_components\":{},\"verified_relations\":{},\"target73\":{},\"target_candidates73\":{},\"setup_ns\":{},\"relations_ns\":{},\"la_ns\":{},\"verification_ns\":{},\"target_ns\":{},\"total_ns\":{},\"bsgs_ns\":{},\"ratio_bsgs_over_candidate\":{:.12},\"cantor_adds\":{},\"target_adds\":{},\"logical_bytes_peak\":{},\"vmhwm_kib\":{},\"field_add\":{},\"field_mul\":{},\"field_sqr\":{},\"field_inv\":{},\"model\":\"F101_signed_two_lp_Q73_only\",\"pcc\":\"unverified\"}}",
            basis,round,if candidate_first {"IC_BSGS"} else {"BSGS_IC"},
            rank,candidates,used_offsets,full,single,double,rejected_multi,
            b_only,observed_lp,known_lp,graph.edges,graph.cycles,graph.components,
            verified_rels,got.unwrap(),target_candidates,
            setup_ns,relations_ns,la_ns,verify_ns,target_ns,total_ns,bsgs_ns,
            bsgs_ns as f64 / total_ns as f64,cantor_adds,target_adds,peak_logical,rss,
            tally.get(Kind::FieldAdd),tally.get(Kind::FieldMul),
            tally.get(Kind::FieldSqr),tally.get(Kind::FieldInv)
        );
    }
    }
}
