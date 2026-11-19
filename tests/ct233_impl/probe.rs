// CT233: preregistered genuine two-large-prime signed-support graph on the
// CT226 full composite genus-4 Jacobian with *source elliptic norm* logs.
// Protocol: CT233_TWO_LP_GRAPH_PROTOCOL.md. [pcc:unverified].
// The graph topology is diagnostic: in odd characteristic an unbalanced
// signed cycle may determine LP logs instead of giving a B-only equation.
use super::*;

#[derive(Clone,Debug)]
struct EqRow {
    b: Vec<i64>,
    lp: Vec<i64>,
    rhs: i64,
}
impl EqRow {
    fn norm(&mut self) {
        for z in &mut self.b { *z=m(*z,R); }
        for z in &mut self.lp { *z=m(*z,R); }
        self.rhs=m(self.rhs,R);
    }
}

// Gaussian elimination on *only* the out-of-base columns. All base
// coefficients and RHS undergo the same exact row operations. This is an
// exact weighted graph Schur complement, not an assumption that all cycles
// annihilate their signed LP coefficients.
fn lp_insert(pivots: &mut [Option<EqRow>], mut r:EqRow) -> Option<(Vec<i64>,i64)> {
    assert_eq!(r.lp.len(),pivots.len());
    r.norm();
    for i in 0..pivots.len() {
        let coefficient=r.lp[i];
        if coefficient==0 {continue;}
        if let Some(p)=&pivots[i] {
            for (v,z) in r.b.iter_mut().zip(&p.b) {
                *v=m(*v-coefficient*z,R);
            }
            for (v,z) in r.lp.iter_mut().zip(&p.lp) {
                *v=m(*v-coefficient*z,R);
            }
            r.rhs=m(r.rhs-coefficient*p.rhs,R);
        } else {
            let inv=inverse(coefficient);
            for z in &mut r.b {*z=m(*z*inv,R);}
            for z in &mut r.lp {*z=m(*z*inv,R);}
            r.rhs=m(r.rhs*inv,R);
            assert_eq!(r.lp[i],1);
            pivots[i]=Some(r);
            return None;
        }
    }
    assert!(r.lp.iter().all(|v|*v==0));
    Some((r.b,r.rhs))
}

// Only uniquely determined LP coordinates are returned. An unanchored graph
// component can leave free LP variables; do not set those free logs to zero
// and then report the resulting conditional values as verified.
fn recover_lp(pivots:&[Option<EqRow>], fb:&[i64])->Vec<Option<i64>> {
    let n=pivots.len();
    let mut out=vec![None;n];
    for i in (0..n).rev() {
        if let Some(p)=&pivots[i] {
            let mut v=m(p.rhs - p.b.iter().zip(fb).map(|(a,z)|a*z).sum::<i64>(),R);
            let mut determined=true;
            for (j,coeff) in p.lp.iter().enumerate().skip(i+1) {
                if *coeff==0 {continue;}
                if let Some(z)=out[j] {v=m(v-coeff*z,R);}
                else {determined=false;break;}
            }
            if determined {out[i]=Some(v);}
        }
    }
    out
}

struct Graph {
    parent:Vec<usize>,
    components:usize,
    cycles:usize,
    edges:usize,
}
impl Graph {
    fn new(v:usize)->Self {Self{parent:(0..v).collect(),components:v,cycles:0,edges:0}}
    fn root(&mut self,x:usize)->usize {
        let mut a=x;
        while self.parent[a]!=a {a=self.parent[a];}
        let top=a;
        a=x;
        while self.parent[a]!=a {let p=self.parent[a];self.parent[a]=top;a=p;}
        top
    }
    fn edge(&mut self,a:usize,b:usize) {
        let x=self.root(a);let y=self.root(b);
        self.edges+=1;
        if x==y {self.cycles+=1;}
        else {self.parent[x]=y;self.components-=1;}
    }
}

fn checked_norm_log(x:i64,log:i64,g:Ec) {
    let yy=(0..P).find(|&y|
        m(y*y-(power(x,9,P)+power(x,3,P)+46),P)==0
    ).expect("observed LP/FB x has a rational curve point");
    assert_eq!(ec_mul(log,g),Some((power(x,3,P),yy)),
        "invalid norm image logarithm");
}
fn mem_bytes(p:&[Option<EqRow>], b:&[(Vec<i64>,i64)],
             eqs:&[EqRow], graph:&Graph)->usize {
    // Logical owned i64 payload plus graph node indices, excludes allocator
    // metadata and Vec capacities; NOT peak RSS.
    let cells:usize=p.iter().flatten().chain(eqs.iter())
        .map(|z|z.b.len()+z.lp.len()+1).sum();
    cells*8 + b.iter().map(|(r,_)|(r.len()+1)*8).sum::<usize>()
        + graph.parent.len()*std::mem::size_of::<usize>()
}
fn vmhwm_kib()->Option<u64>{
    std::fs::read_to_string("/proc/self/status").ok()?.lines()
       .find(|s|s.starts_with("VmHWM:"))?
       .split_whitespace().nth(1)?.parse().ok()
}

#[test]
fn ct233_weighted_graph_cycle_and_mutation_controls() {
    // A topological cycle is NOT necessarily an eliminated LP cycle:
    // the three all-plus edges have determinant 2 modulo odd 109.
    let mut p=vec![None;3];
    for lp in [[1,1,0],[0,1,1],[1,0,1]] {
        let out=lp_insert(&mut p,EqRow{b:vec![0,0],lp:lp.to_vec(),rhs:0});
        assert!(out.is_none(),"false B-only relation from unbalanced cycle");
    }
    assert_eq!(p.iter().filter(|v|v.is_some()).count(),3);
    assert_eq!(recover_lp(&p,&[0,0]),vec![Some(0),Some(0),Some(0)]);
    let mut p=vec![None;3];
    let rows=[
        EqRow{b:vec![2,0],lp:vec![1,1,0],rhs:1},
        EqRow{b:vec![0,5],lp:vec![0,1,1],rhs:2},
        EqRow{b:vec![7,0],lp:vec![-1,0,1],rhs:3},
    ];
    assert!(lp_insert(&mut p,rows[0].clone()).is_none());
    assert!(lp_insert(&mut p,rows[1].clone()).is_none());
    let (brow,rhs)=lp_insert(&mut p,rows[2].clone()).unwrap();
    assert_eq!((brow,rhs),(vec![9,m(-5,R)],2));
    // A balanced-cycle equation is true at this fixed B witness, but a
    // one-unit RHS mutation is rejected rather than falsely certified.
    assert_eq!(m(9*3-5*5,R),rhs);
    assert_ne!(m(9*3-5*5,R),m(rhs+1,R));
    assert!(recover_lp(&p,&[3,5]).iter().any(Option::is_none),
        "an unanchored signed graph must not certify all its LP logs");
    // Two signed one-LP parallel edges cancel exactly (CT231 compatibility).
    let mut p=vec![None;1];
    assert!(lp_insert(&mut p,EqRow{b:vec![2,-5],lp:vec![-1],rhs:17}).is_none());
    let got=lp_insert(&mut p,EqRow{b:vec![7,0],lp:vec![1],rhs:41}).unwrap();
    assert_eq!(got,(vec![9,m(-5,R)],58));
}

#[test]
fn ct233_double_lp_native_panel() {
    const PANEL:[usize;4]=[24,36,48,54];
    const MAX_OFFSETS:usize=60;
    let g:Ec=Some((2,37));
    let targets:[Ec;2]=[Some((64,19)),Some((19,37))];
    let expected=[37,73];
    for (arm,&basis) in PANEL.iter().enumerate() {
        let mut bsgs_ns=0u128;
        if arm%2==0 {
            let t=Instant::now();
            for (&q,&answer) in targets.iter().zip(&expected) {
                assert_eq!(direct_bsgs(g,q).0,answer);
            }
            bsgs_ns=t.elapsed().as_nanos();
        }
        let t_all=Instant::now();
        let tally0=ops::snapshot().total();
        let c=curve();
        let gdiv=image(&c,g);
        let qs:Vec<Div>=targets.iter().map(|q|image(&c,*q)).collect();
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
        let mut got:[Option<i64>;2]=[None,None];
        let mut target_candidates=[0usize;2];
        let mut target_adds=0u64;
        if let Some(ref logs)=fb {
            for (qi,qdiv) in qs.iter().enumerate() {
                'search:for pts in roster.iter().take(MAX_OFFSETS) {
                    let mut cur=qdiv.clone();
                    let mut off=vec![0i64;basis];
                    for &(x,y) in pts {
                        cur=cur.add_fast(&point(&c,x,y),&c);
                        target_adds+=1;
                        for (a,z) in off.iter_mut().zip(coord(x,y,selected)) {*a+=z;}
                    }
                    for a in 0..R {
                        target_candidates[qi]+=1;
                        if let Some(sup)=support(&cur,&all_x){
                            let outside:Vec<(usize,i64)>=sup.iter().enumerate()
                                .skip(basis).filter(|(_,s)| **s!=0)
                                .map(|(i,s)|(i-basis,*s)).collect();
                            if outside.len()<=2
                                && outside.iter().all(|(i,_)|lp_logs[*i].is_some()){
                                let mut norm:i64=sup[..basis].iter()
                                    .zip(&off).zip(logs)
                                    .map(|((v,o),z)|(v-o)*z).sum();
                                for (i,sign) in outside {
                                    norm+=sign*lp_logs[i].unwrap();
                                }
                                let answer=m(norm*inverse(3)-a,R);
                                assert_eq!(ec_mul(answer,g),targets[qi]);
                                got[qi]=Some(answer);
                                break 'search;
                            }
                        }
                        if a<R-1 {cur=cur.add_fast(&gdiv,&c);target_adds+=1;}
                    }
                }
            }
        }
        let target_ns=t_target.elapsed().as_nanos();
        let total_ns=t_all.elapsed().as_nanos();
        if arm%2==1 {
            let t=Instant::now();
            for (&q,&answer) in targets.iter().zip(&expected) {
                assert_eq!(direct_bsgs(g,q).0,answer);
            }
            bsgs_ns=t.elapsed().as_nanos();
        }
        for (a,known) in got.iter().zip(&expected) {
            if let Some(v)=a {assert_eq!(v,known);}
        }
        if basis==54 {
            assert_eq!((rank,full,candidates),(54,62,1421),
               "CT226/CT231 unchanged full-base control drift");
            assert_eq!((single,double),(0,0));
            assert_eq!(got,[Some(37),Some(73)]);
        }
        assert!(candidates<=MAX_OFFSETS*R as usize);
        assert!(target_candidates.iter().all(|&n|n<=MAX_OFFSETS*R as usize));
        let observed_lp=lp_pivots.iter().filter(|p|p.is_some()).count();
        let known_lp=lp_logs.iter().filter(|v|v.is_some()).count();
        let tally=ops::snapshot().total().since(&tally0);
        let rss=vmhwm_kib().map(|v|v.to_string()).unwrap_or_else(||"null".into());
        eprintln!(
           "CT233_PANEL_JSON {{\"basis\":{},\"rank\":{},\"candidates\":{},\"offsets\":{},\"full\":{},\"single_lp\":{},\"double_lp\":{},\"multi_lp_discard\":{},\"b_only_rows\":{},\"lp_pivot_rank\":{},\"known_lp\":{},\"graph_edges\":{},\"graph_cycles\":{},\"graph_components\":{},\"verified_relations\":{},\"target37\":{},\"target73\":{},\"target_candidates37\":{},\"target_candidates73\":{},\"setup_ns\":{},\"relations_ns\":{},\"la_ns\":{},\"verification_ns\":{},\"target_ns\":{},\"total_ns\":{},\"bsgs_ns\":{},\"cantor_adds\":{},\"target_adds\":{},\"logical_bytes_peak\":{},\"vmhwm_kib\":{},\"field_add\":{},\"field_mul\":{},\"field_sqr\":{},\"field_inv\":{},\"model\":\"F101_signed_two_lp_graph\",\"pcc\":\"unverified\"}}",
           basis,rank,candidates,used_offsets,full,single,double,rejected_multi,
           b_only,observed_lp,known_lp,graph.edges,graph.cycles,graph.components,
           verified_rels,
           got[0].map(|v|v.to_string()).unwrap_or("null".into()),
           got[1].map(|v|v.to_string()).unwrap_or("null".into()),
           target_candidates[0],target_candidates[1],
           setup_ns,relations_ns,la_ns,verify_ns,target_ns,total_ns,bsgs_ns,
           cantor_adds,target_adds,peak_logical,rss,
           tally.get(Kind::FieldAdd),tally.get(Kind::FieldMul),
           tally.get(Kind::FieldSqr),tally.get(Kind::FieldInv)
        );
    }
}
