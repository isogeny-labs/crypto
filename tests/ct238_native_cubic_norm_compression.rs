//! CT238: F103 cubic-cover source-norm orbit compression complete cold Q73 ECDLP pilot.
//! Uses the repository's FpPoly and genus-generic Mumford/Cantor arithmetic.
//! C/F103: y^2=u^9+u^3+24 -> E: y^2=x^3+x+24, E prime order 109.
//! Public target Q=(42,71); recovered only from public point and relation logs.
//! Diagnostic, not PCC or a growing-prime-field attack. [pcc:unverified]
use crypto_lib::prime_hyperelliptic::{FpPoly, HyperellipticCurveP, MumfordDivisorP};
use crypto_lib::prime_hyperelliptic::ops::{self, Kind};
use num_bigint::BigUint;
use std::time::Instant;

const P: i64 = 103;
const R: i64 = 109;
type Ec = Option<(i64,i64)>;
type Div = MumfordDivisorP;
fn b(x: i64) -> BigUint { BigUint::from(x as u64) }
fn m(x: i64, p: i64) -> i64 { x.rem_euclid(p) }
fn power(mut x:i64, mut n:i64, p:i64)->i64{
    let mut out=1;
    while n>0 {if n&1!=0 {out=m(out*x,p);} x=m(x*x,p);n>>=1;}
    out
}
fn ec_add(a:Ec, z:Ec)->Ec {
    match (a,z) {
        (None,p)|(p,None)=>p,
        (Some((x,y)),Some((u,v)))=>{
            if x==u && m(y+v,P)==0 {return None;}
            let (num,den)=if x==u {(3*x*x+1,2*y)} else {(v-y,u-x)};
            let slope=m(num*power(m(den,P),P-2,P),P);
            let xx=m(slope*slope-x-u,P);
            Some((xx,m(slope*(x-xx)-y,P)))
        }
    }
}
fn ec_mul(mut k:i64, point:Ec)->Ec {
    let mut result=None;
    let mut base=point;
    while k>0 {if k&1!=0 {result=ec_add(result,base);}
        base=ec_add(base,base); k>>=1;}
    result
}
fn direct_bsgs(g:Ec, target:Ec)->(i64,u64) {
    let n=11i64; // ceil(sqrt(109))
    let mut table=Vec::with_capacity(n as usize);
    let mut current=None;
    let mut ops=0u64;
    for k in 0..n {table.push((current,k)); current=ec_add(current,g);ops+=1;}
    let neg_g=g.map(|(x,y)|(x,m(-y,P)));
    let step=ec_mul(n,neg_g);
    let mut giant=target;
    for i in 0..=n {
        if let Some((_,j))=table.iter().find(|(p,_)|*p==giant) {
            let scalar=m(i*n+*j,R);
            assert_eq!(ec_mul(scalar,g),target);
            return (scalar,ops);
        }
        giant=ec_add(giant,step);ops+=1;
    }
    panic!("direct BSGS failed")
}
fn poly(xs:&[i64])->FpPoly{
    FpPoly::from_coeffs(xs.iter().copied().map(b).collect(),b(P))
}
fn curve()->HyperellipticCurveP {
    HyperellipticCurveP::try_new(b(P),poly(&[24,0,0,1,0,0,0,0,0,1]),4).unwrap()
}
fn point(curve:&HyperellipticCurveP, x:i64, y:i64)->Div {
    MumfordDivisorP::try_from_point(curve,&b(x),&b(y)).unwrap()
}
fn image(curve:&HyperellipticCurveP, e:Ec)->Div {
    match e {
        None=>Div::identity(b(P)),
        Some((x,y))=>Div::try_from_polynomials(curve,poly(&[m(-x,P),0,0,1]),poly(&[y])).unwrap()
    }
}
fn x_roster(curve:&HyperellipticCurveP)->(Vec<i64>,Vec<(i64,i64)>) {
    let mut xs=Vec::new();
    let mut first=Vec::new();
    let mut rational_u=0usize;
    for x in 0..P {
        let mut ys=Vec::new();
        let xx=b(x);
        let rhs=curve.f.eval(&xx).to_u64_digits().first().copied().unwrap_or(0) as i64;
        for y in 0..P {if m(y*y-rhs,P)==0 {ys.push(y);}}
        if !ys.is_empty() {
            assert_eq!(ys.len(),2,"unexpected ramified point in 54-variable model");
            rational_u+=1;
            xs.push(power(x,3,P)); // norm-fiber-compressed source E x, not C u
            if first.len()<8 {first.push((x,ys[0]));}
        }
    }
    assert_eq!(rational_u,66,"unexpected C signed rational u count");
    xs.sort_unstable(); xs.dedup();
    assert_eq!(xs.len(),22,"expected exact three-to-one norm fiber roster");
    (xs,first)
}
fn coord(x:i64,y:i64,xs:&[i64])->Vec<i64> {
    let mut out=vec![0;xs.len()];
    let i=xs.binary_search(&power(x,3,P)).unwrap();
    assert!(y>0 && y<P && y!=P-y);
    out[i]=if y<P-y {1} else {-1};
    out
}
fn support(div:&Div,xs:&[i64])->Option<Vec<i64>> {
    if div.u.degree()!=Some(4) {return None;}
    let mut roots=Vec::new();
    // Search genuine C u-roots, not source norm-x columns; all 4 must split.
    for x in 0..P {
        if div.u.eval(&b(x))==b(0) {roots.push(x);}
    }
    if roots.len()!=4 {return None;} // squarefree fully split only
    let mut row=vec![0i64;xs.len()];
    for x in roots {
        let yy=div.v.eval(&b(x)).to_u64_digits().first().copied().unwrap_or(0) as i64;
        let entry=coord(x,yy,xs);
        for (a,z) in row.iter_mut().zip(entry) {*a+=z;}
    }
    Some(row)
}
fn offsets(first:&[(i64,i64)])->Vec<Vec<(i64,i64)>> {
    let mut out=Vec::new();
    // Same lexicographic combination schedule as the frozen CT219 source.
    for a in 0..8 {for b in a+1..8 {out.push(vec![first[a],first[b]]);}}
    for a in 0..8 {for b in a+1..8 {for c in b+1..8 {
        out.push(vec![first[a],first[b],first[c]]);
    }}}
    for a in 0..8 {for b in a+1..8 {for c in b+1..8 {for d in c+1..8 {
        out.push(vec![first[a],first[b],first[c],first[d]]);
    }}}}
    out
}
fn inverse(x:i64)->i64{assert_ne!(m(x,R),0);power(m(x,R),R-2,R)}
fn reduce_row(pivots:&mut [Option<Vec<i64>>],row:&[i64],rhs:i64)->bool {
    let n=pivots.len();
    let mut a:Vec<i64>=row.iter().copied().map(|x|m(x,R)).collect();
    a.push(m(rhs,R));
    for i in 0..n {
        if a[i]==0 {continue;}
        if let Some(pivot)=&pivots[i] {
            let mul=a[i];
            for j in i..=n {a[j]=m(a[j]-mul*pivot[j],R);}
        } else {
            let inv=inverse(a[i]);
            for z in &mut a[i..=n] {*z=m(*z*inv,R);}
            pivots[i]=Some(a);
            return true;
        }
    }
    assert_eq!(a[n],0,"incorrect or inconsistent transfer relation");
    false
}
fn solve(pivots:&[Option<Vec<i64>>])->Vec<i64> {
    let n=pivots.len();
    assert!(pivots.iter().all(Option::is_some));
    let mut logs=vec![0;n];
    for i in (0..n).rev() {
        let row=pivots[i].as_ref().unwrap();
        let mut value=row[n];
        for j in i+1..n {value=m(value-row[j]*logs[j],R);}
        logs[i]=m(value,R);
    }
    logs
}
#[derive(Debug)]
struct Cost {
    setup:u128, relations:u128, linear_algebra:u128, recovery:u128, verification:u128,
    candidates:usize, smooth:usize, rank:usize, used_offsets:usize,
    cantor_adds:u64, target_adds:u64, fp_add:u64, fp_mul:u64, fp_sqr:u64, fp_inv:u64,
}
impl Cost {
    fn total(&self)->u128{
        self.setup+self.relations+self.linear_algebra+self.recovery+self.verification
    }
}
fn ic_cold()->Cost {
    let initial=Instant::now();
    let tally0=ops::snapshot().total();
    let c=curve();
    let g=Some((1,51));
    let q=Some((42,71));
    let gdiv=image(&c,g);
    let qdiv=image(&c,q);
    assert_eq!(gdiv.scalar_mul_fast(&b(R),&c),Div::identity(b(P)));
    assert_eq!(gdiv.scalar_mul_fast(&b(73),&c),qdiv);
    let (xs,first)=x_roster(&c);
    let roster=offsets(&first);
    let setup=initial.elapsed().as_nanos();
    let mut pivots=vec![None;xs.len()];
    let mut candidates=0usize;
    let mut smooth=0usize;
    let mut adds=0u64;
    let mut used_offsets=0usize;
    let mut relations=Vec::new();
    let t=Instant::now();
    for pts in &roster {
        used_offsets+=1;
        let mut cur=Div::identity(b(P));
        let mut off=vec![0i64;xs.len()];
        for &(x,y) in pts {
            cur=cur.add_fast(&point(&c,x,y),&c);adds+=1;
            let row=coord(x,y,&xs);
            for (o,z) in off.iter_mut().zip(row) {*o+=z;}
        }
        for k in 0..R {
            candidates+=1;
            if let Some(row)=support(&cur,&xs) {
                smooth+=1;
                let diff:Vec<i64>=row.iter().zip(&off).map(|(x,y)|x-y).collect();
                let rhs=m(3*k,R);
                relations.push((diff.clone(),rhs));
                reduce_row(&mut pivots,&diff,rhs);
                if pivots.iter().all(Option::is_some) {break;}
            }
            if k<R-1 {cur=cur.add_fast(&gdiv,&c);adds+=1;}
        }
        if pivots.iter().all(Option::is_some) {break;}
    }
    let relation_time=t.elapsed().as_nanos();
    let rank=pivots.iter().filter(|p|p.is_some()).count();
    assert_eq!(rank,22,"CT238 compressed source-log rank not established");
    let t=Instant::now();
    let fb=solve(&pivots);
    for (row,rhs) in &relations {
        assert_eq!(m(row.iter().zip(&fb).map(|(x,y)|x*y).sum(),R),*rhs);
    }
    let linear_algebra=t.elapsed().as_nanos();
    let t=Instant::now();
    let pts=&roster[0];
    let mut off=vec![0i64;xs.len()];
    let mut cur=qdiv.clone();
    let mut target_adds=0u64;
    for &(x,y) in pts {
        cur=cur.add_fast(&point(&c,x,y),&c);target_adds+=1;
        let row=coord(x,y,&xs);
        for (o,z) in off.iter_mut().zip(row) {*o+=z;}
    }
    let mut recovered=None;
    for a in 0..R {
        if let Some(row)=support(&cur,&xs) {
            let scalar=m(
                (row.iter().zip(&off).zip(&fb).map(|((x,y),z)|(x-y)*z).sum::<i64>()
                   *inverse(3))-a,R);
            if ec_mul(scalar,g)==q {recovered=Some(scalar);break;}
            panic!("invalid norm IC target scalar");
        }
        cur=cur.add_fast(&gdiv,&c);target_adds+=1;
    }
    assert_eq!(recovered,Some(73));
    let recovery=t.elapsed().as_nanos();
    let t=Instant::now();
    for (i,&x) in xs.iter().enumerate() {
        let yy=(0..P).find(|&y|m(y*y-(m(power(x,3,P)+x+24,P)),P)==0).unwrap();
        assert_eq!(ec_mul(fb[i],g),Some((x,yy)));
    }
    assert_eq!(gdiv.scalar_mul_fast(&b(recovered.unwrap()),&c),qdiv);
    let verification=t.elapsed().as_nanos();
    let tally=ops::snapshot().total().since(&tally0);
    Cost {setup,relations:relation_time,linear_algebra,recovery,verification,
        candidates,smooth,rank,used_offsets,cantor_adds:adds,target_adds,
        fp_add:tally.get(Kind::FieldAdd),fp_mul:tally.get(Kind::FieldMul),
        fp_sqr:tally.get(Kind::FieldSqr),fp_inv:tally.get(Kind::FieldInv)}
}
#[test]
fn ct238_paired_native_norm_compressed_ic_vs_direct_bsgs() {
    // Repeated cold solver construction. Test names indicate the comparator;
    // rho and preexisting prime-field IC are NOT silently counted as checked.
    for round in 0..3 {
        let public=Some((42,71));
        let baseline=|| {
            let start=Instant::now();
            let (scalar,adds)=direct_bsgs(Some((1,51)),public);
            assert_eq!(scalar,73);
            (start.elapsed().as_nanos(),adds)
        };
        let (ic,(bsgs_ns,bsgs_adds))=if round%2==0 {
            let ic=ic_cold();let bs=baseline();(ic,bs)
        } else {
            let bs=baseline();let ic=ic_cold();(ic,bs)
        };
        assert!(ic.total()>0 && bsgs_ns>0);
        assert_eq!((ic.rank,ic.smooth,ic.candidates),(22,27,381));
        eprintln!(
            "CT238_NORM_COMPRESSED_PAIR_JSON {{\"round\":{},\"arm_order\":\"{}\",\"candidate_ns\":{},\"direct_bsgs_ns\":{},\"ratio_direct_over_candidate\":{:.10},\"ic_setup_ns\":{},\"ic_relation_ns\":{},\"ic_la_ns\":{},\"ic_recovery_ns\":{},\"ic_verify_ns\":{},\"ic_candidates\":{},\"ic_smooth\":{},\"ic_rank\":{},\"offsets\":{},\"cantor_adds\":{},\"target_adds\":{},\"direct_bsgs_adds\":{},\"field_add\":{},\"field_mul\":{},\"field_sqr\":{},\"field_inv\":{},\"source\":\"known_answer_F103\",\"memory\":\"unmeasured\",\"pcc\":\"unverified\"}}",
            round,if round%2==0 {"IC_BSGS"} else {"BSGS_IC"},
            ic.total(),bsgs_ns,bsgs_ns as f64/ic.total() as f64,
            ic.setup,ic.relations,ic.linear_algebra,ic.recovery,ic.verification,
            ic.candidates,ic.smooth,ic.rank,ic.used_offsets,ic.cantor_adds,
            ic.target_adds,bsgs_adds,ic.fp_add,ic.fp_mul,ic.fp_sqr,ic.fp_inv
        );
    }
}




// CT241: preregistered reduced norm-image LP native cold cost panel.
// Finite known-answer F103 diagnostic. Reuses CT238's genuine Cantor arithmetic.
// This is not the original 695/701 norm fixture and not a PCC-verified speedup.
mod ct241 {
    use super::*;
    type LpRow = (Vec<i64>, Vec<i64>, i64);

    fn lp_insert(
        pivots: &mut [Option<LpRow>],
        mut base: Vec<i64>,
        mut lp: Vec<i64>,
        mut rhs: i64
    ) -> Option<(Vec<i64>, i64)> {
        for v in &mut base { *v = m(*v, R); }
        for v in &mut lp { *v = m(*v, R); }
        rhs = m(rhs, R);
        for i in 0..lp.len() {
            let mul = lp[i];
            if mul == 0 { continue; }
            if let Some((pb, pl, pr)) = &pivots[i] {
                for j in 0..base.len() { base[j] = m(base[j] - mul*pb[j], R); }
                for j in 0..lp.len() { lp[j] = m(lp[j] - mul*pl[j], R); }
                rhs = m(rhs - mul*pr, R);
            } else {
                let inv = inverse(mul);
                for v in &mut base { *v = m(*v*inv, R); }
                for v in &mut lp { *v = m(*v*inv, R); }
                rhs = m(rhs*inv, R);
                pivots[i] = Some((base, lp, rhs));
                return None;
            }
        }
        assert!(lp.iter().all(|&v| v == 0), "uneliminated large-prime column");
        Some((base, rhs))
    }

    fn recovered_lp(pivots: &[Option<LpRow>], fb: &[i64]) -> Vec<Option<i64>> {
        let mut result: Vec<Option<i64>> = vec![None; pivots.len()];
        for i in (0..pivots.len()).rev() {
            let Some((base, lp, rhs)) = &pivots[i] else { continue; };
            if (i+1..lp.len()).any(|j| lp[j] != 0 && result[j].is_none()) {
                continue; // Never guess an unresolved LP logarithm.
            }
            let mut value = *rhs;
            for (a, z) in base.iter().zip(fb) { value -= a*z; }
            for j in i+1..lp.len() {
                if lp[j] != 0 { value -= lp[j]*result[j].unwrap(); }
            }
            result[i] = Some(m(value, R));
        }
        result
    }

    fn source_point(x: i64) -> Ec {
        let yy = (0..P).find(|&y| m(y*y-(power(x,3,P)+x+24),P)==0)
            .expect("norm x has no source E y");
        Some((x, yy))
    }

    #[derive(Debug)]
    struct Cold {
        ns: u128, setup: u128, relations: u128, linear_algebra: u128,
        recovery: u128, verify: u128,
        candidates: usize, rank: usize, full: usize, lp1: usize,
        lp2: usize, rejected: usize, b_rows: usize, lp_pivots: usize,
        determined_lp: usize, target_candidates: usize,
        cantor_adds: u64, target_adds: u64, field_add: u64,
        field_mul: u64, field_sqr: u64, field_inv: u64,
        matrix_bytes: usize, vmhwm_kb: usize,
    }

    fn vmhwm_kb() -> usize {
        std::fs::read_to_string("/proc/self/status").ok()
            .and_then(|s| s.lines().find(|l| l.starts_with("VmHWM:"))
                .and_then(|l| l.split_whitespace().nth(1))
                .and_then(|n| n.parse().ok()))
            .unwrap_or(0)
    }

    fn cold(base_size: usize) -> Cold {
        let start = Instant::now();
        let before = ops::snapshot().total();
        let c = curve();
        let g = Some((1, 51));
        let q = Some((42, 71));
        let gdiv = image(&c, g);
        let qdiv = image(&c, q);
        assert_eq!(gdiv.scalar_mul_fast(&b(R), &c), Div::identity(b(P)));
        assert_eq!(gdiv.scalar_mul_fast(&b(73), &c), qdiv);
        let (xs, first) = x_roster(&c);
        assert_eq!(xs.len(), 22);
        let roster = offsets(&first);
        let setup = start.elapsed().as_nanos();
        let excluded = xs.len() - base_size;
        let mut bp = vec![None; base_size];
        let mut lp = vec![None; excluded];
        let mut rank=0usize;
        let mut candidates=0usize;
        let mut full=0usize;
        let mut lp1=0usize;
        let mut lp2=0usize;
        let mut rejected=0usize;
        let mut b_rows=0usize;
        let mut cantor_adds=0u64;
        let rel_start = Instant::now();
        'collect: for pts in roster.iter().take(60) {
            let mut cur = Div::identity(b(P));
            let mut off = vec![0i64;xs.len()];
            for &(x,y) in pts {
                cur = cur.add_fast(&point(&c,x,y),&c);
                cantor_adds+=1;
                let row=coord(x,y,&xs);
                for (a,z) in off.iter_mut().zip(row) { *a+=z; }
            }
            for k in 0..R {
                if candidates == 6540 { break 'collect; }
                candidates+=1;
                if let Some(sup) = support(&cur, &xs) {
                    let diff: Vec<i64> = sup.iter().zip(&off).map(|(a,z)|a-z).collect();
                    let nlp = diff[base_size..].iter().filter(|&&v|m(v,R)!=0).count();
                    if nlp>2 {
                        rejected+=1;
                    } else {
                        match nlp {0=>full+=1,1=>lp1+=1,2=>lp2+=1,_=>unreachable!()}
                        let residual=lp_insert(&mut lp,
                            diff[..base_size].to_vec(), diff[base_size..].to_vec(),
                            m(3*k,R));
                        if let Some((row, rhs))=residual {
                            b_rows+=1;
                            if reduce_row(&mut bp,&row,rhs) {
                                rank+=1;
                                if rank==base_size { break 'collect; }
                            }
                        }
                    }
                }
                if k<R-1 && candidates<6540 {
                    cur=cur.add_fast(&gdiv,&c);
                    cantor_adds+=1;
                }
            }
        }
        let relations = rel_start.elapsed().as_nanos();
        assert_eq!(rank,base_size,"CT240 fixed base failed full rank");
        assert_eq!(candidates,if base_size==12 {589} else {381},
            "CT240 exact relation-candidate stream drift");
        let la_start=Instant::now();
        let fb=solve(&bp);
        let determined=recovered_lp(&lp,&fb);
        let determined_count=determined.iter().filter(|x|x.is_some()).count();
        let lp_pivots=lp.iter().filter(|x|x.is_some()).count();
        let linear_algebra=la_start.elapsed().as_nanos();
        let target_start=Instant::now();
        let mut recovered=None;
        let mut target_candidates=0usize;
        let mut target_adds=0u64;
        'target: for pts in roster.iter().take(60) {
            let mut cur=qdiv.clone();
            let mut off=vec![0i64;xs.len()];
            for &(x,y) in pts {
                cur=cur.add_fast(&point(&c,x,y),&c);
                target_adds+=1;
                let row=coord(x,y,&xs);
                for (a,z) in off.iter_mut().zip(row) {*a+=z;}
            }
            for a in 0..R {
                if target_candidates==6540 {break 'target;}
                target_candidates+=1;
                if let Some(sup)=support(&cur,&xs) {
                    let diff: Vec<i64> = sup.iter().zip(&off).map(|(u,z)|u-z).collect();
                    let mut sum=0i64;
                    let mut ready=true;
                    for (i,&v) in diff.iter().enumerate() {
                        if m(v,R)==0 {continue;}
                        let log=if i<base_size {Some(fb[i])}
                            else {determined[i-base_size]};
                        if let Some(t)=log {sum+=v*t;} else {ready=false;break;}
                    }
                    if ready {
                        let scalar=m(sum*inverse(3)-a,R);
                        assert_eq!(ec_mul(scalar,g),q,"incorrect recovered E discrete log");
                        assert_eq!(gdiv.scalar_mul_fast(&b(scalar),&c),qdiv,
                            "incorrect recovered Jacobian scalar");
                        recovered=Some(scalar);
                        break 'target;
                    }
                }
                if a<R-1 && target_candidates<6540 {
                    cur=cur.add_fast(&gdiv,&c);
                    target_adds+=1;
                }
            }
        }
        assert_eq!(recovered,Some(73),"CT240 scalar not recovered");
        assert_eq!(target_candidates,27,"CT240 target candidate stream drift");
        let recovery=target_start.elapsed().as_nanos();
        let verification_start=Instant::now();
        for (i,&x) in xs.iter().enumerate() {
            let log=if i<base_size {Some(fb[i])}
                else {determined[i-base_size]};
            if let Some(v)=log {
                assert_eq!(ec_mul(v,g),source_point(x),
                    "independently incorrect known source factor log");
            }
        }
        assert_eq!(determined_count, excluded,"CT240 LP recovery drift");
        assert_eq!(lp_pivots, excluded,"CT240 LP rank drift");
        if base_size==22 {
            assert_eq!((full,lp1,lp2,rejected),(27,0,0,0));
        }
        let verify=verification_start.elapsed().as_nanos();
        let tally=ops::snapshot().total().since(&before);
        let matrix_bytes=8*(base_size*(base_size+1)+
            excluded*(base_size+excluded+1));
        Cold {
            ns:setup+relations+linear_algebra+recovery+verify,
            setup,relations,linear_algebra,recovery,verify,
            candidates,rank,full,lp1,lp2,rejected,b_rows,
            lp_pivots,determined_lp:determined_count,target_candidates,
            cantor_adds,target_adds,
            field_add:tally.get(Kind::FieldAdd),
            field_mul:tally.get(Kind::FieldMul),
            field_sqr:tally.get(Kind::FieldSqr),
            field_inv:tally.get(Kind::FieldInv),
            matrix_bytes,vmhwm_kb:vmhwm_kb(),
        }
    }

    #[test]
    fn ct241_paired_native_f103_reduced_norm_lp() {
        for &base_size in &[12usize,16,18,20,22] {
            for round in 0..3usize {
                let bsgs=||{
                    let st=Instant::now();
                    let (scalar,adds)=direct_bsgs(Some((1,51)),Some((42,71)));
                    assert_eq!(scalar,73);
                    (st.elapsed().as_nanos(),adds)
                };
                let (ic, (bs_ns,bs_ops))=if round%2==0 {
                    let ic=cold(base_size);let bs=bsgs();(ic,bs)
                } else {
                    let bs=bsgs();let ic=cold(base_size);(ic,bs)
                };
                assert!(ic.ns>0 && bs_ns>0);
                eprintln!(
                    "CT241_REDUCED_NORM_PAIR_JSON {{\"base\":{},\"round\":{},\"order\":\"{}\",\"candidate_ns\":{},\"direct_bsgs_ns\":{},\"ratio_direct_over_candidate\":{:.12},\"setup_ns\":{},\"relation_online_la_ns\":{},\"backsolve_ns\":{},\"recovery_ns\":{},\"verification_ns\":{},\"candidates\":{},\"rank\":{},\"full\":{},\"single_lp\":{},\"double_lp\":{},\"discarded\":{},\"b_only\":{},\"lp_pivots\":{},\"determined_lp\":{},\"target_candidates\":{},\"cantor_adds\":{},\"target_adds\":{},\"direct_bsgs_adds\":{},\"field_add\":{},\"field_mul\":{},\"field_sqr\":{},\"field_inv\":{},\"logical_matrix_bytes\":{},\"vmhwm_kb\":{},\"pcc\":\"unverified\",\"memory\":\"vmhwm_processwide\"}}",
                    base_size,round,if round%2==0 {"IC_BSGS"} else {"BSGS_IC"},
                    ic.ns,bs_ns,bs_ns as f64/ic.ns as f64,
                    ic.setup,ic.relations,ic.linear_algebra,ic.recovery,ic.verify,
                    ic.candidates,ic.rank,ic.full,ic.lp1,ic.lp2,ic.rejected,
                    ic.b_rows,ic.lp_pivots,ic.determined_lp,ic.target_candidates,
                    ic.cantor_adds,ic.target_adds,bs_ops,
                    ic.field_add,ic.field_mul,ic.field_sqr,ic.field_inv,
                    ic.matrix_bytes,ic.vmhwm_kb
                );
            }
        }
    }
}
