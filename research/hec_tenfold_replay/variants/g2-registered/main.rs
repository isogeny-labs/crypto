#[path="../../sources/harvest.rs"] mod harvest;
#[path="../../sources/centered.rs"] mod centered;
#[path="../../sources/compact.rs"] mod compact;
use hec_native_ic::{
    counted,
    instance::pick_instance,
    jac::{Curve, Div},
    rng::Rng,
};
use std::{hint::black_box, time::Instant};
const L: u64 = 239753;
#[path="../../sources/buckets.rs"] mod buckets;
#[path="../../sources/carried.rs"] mod carried;

#[path="../../sources/dag.rs"] mod dag;
#[path="../../sources/solver.rs"] mod delayed;

#[path="../../sources/powers.rs"] mod powers;
#[path="../../sources/recipe.rs"] mod recipe;
mod candidate {use hec_native_ic::jac::{Curve,Div};
extern "C" { fn stream(p:i32, divisors:*const i32, paired:i32, stop:*mut i32)->i32; }
pub fn solve(c:&Curve,d:&Div,t:&Div,l:u64,paired:bool,_audit:bool)->u64{
    let mut terms=[0;12];
    let sum=(!paired).then(||c.add(d,t)); for (i,input) in [d,t].into_iter().chain(sum.as_ref()).enumerate(){
        assert_eq!(input.u.n,3);
        for j in 0..2{terms[4*i+j]=input.u.c[j] as i32;terms[4*i+j+2]=input.v.c[j] as i32;}
    }
    let mut stop=[0;2];
    let k=unsafe{stream(c.fp.p as i32,terms.as_ptr(),if paired && _audit {2} else {paired as i32},stop.as_mut_ptr())};
    assert!(k>=0 && ((paired && !_audit) || crate::shortcheck::check(c,d,t,k as u64,l)));
    if _audit {eprintln!("{} {} {} {}",c.fp.p,paired,stop[1],stop[0]);}
    k as u64
}
} #[path="../../sources/verify.rs"] mod verify; mod shortcheck {use hec_native_ic::jac::{Curve,Div};
fn gcd(mut a:i64,mut b:i64)->i64{
    while b!=0 {(a,b)=(b,a%b);}
    a
}
pub fn check(c:&Curve,d:&Div,target:&Div,k:u64,l:u64)->bool {
    // The base has order l. Requiring b invertible on the entire Jacobian
    // also prevents torsion in an incorrect target from passing the check.
    let h=match c.fp.p {127=>2,257|4093=>4,1021=>1,_=>return c.mul(d,k)==*target};
    if k==0 {return c.is_zero(target);}
    let (mut prev,mut a)=(l as i64,k as i64);
    let (mut old_b,mut b)=(0i64,1i64);
    let mut best=(a,b);
let score=|a:i64,b:i64|{let u=a as u64;let v=b.unsigned_abs();let z=u|v;(64-z.leading_zeros())+z.count_ones()+u32::from((u&v)!=0)};
let mut best_cost=score(a,b);

    while a>0{
        for (aa,bb) in [(a,b),(prev-a,old_b-b),(prev+a,old_b+b)] {
 if aa<=0 || bb==0 {continue;}
 let cost=score(aa,bb);
 if cost<best_cost && gcd(bb.abs(),h*l as i64)==1 {best=(aa,bb);best_cost=cost;}
}

        let q=prev/a;
        (prev,a)=(a,prev%a);
        (old_b,b)=(b,old_b-q*b);
    }
    let (a,b)=best;
    let t=if b>0 {c.neg(target)}else{*target};
    let width=64-(a as u64|b.unsigned_abs()).leading_zeros();
    let both=if (a as u64 & b.unsigned_abs())!=0 {crate::lift::add(c,d,&t,4,false)}else{c.zero()};
    let mut acc=c.zero();
    for i in (0..width).rev(){
        acc=crate::lift::double(c,&acc,4,false);
        let bit=((a as u64>>i)&1)+2*((b.unsigned_abs()>>i)&1);
        if i==0 && bit!=0 {return acc==c.neg(match bit{1=>d,2=>&t,_=>&both});} if bit!=0 {acc=crate::lift::add(c,&acc,match bit{1=>d,2=>&t,_=>&both},4,false);}
    }
    c.is_zero(&acc)
}
}
mod phases {
 use std::{cell::Cell,time::Instant};
 thread_local!{pub static TIMES:Cell<[u128;8]>=const{Cell::new([0;8])};}
 #[inline(always)] pub fn mark(_: &str) {}
 pub struct Timer(usize,Option<Instant>);
 impl Drop for Timer {fn drop(&mut self){if let Some(start)=self.1{let t=start.elapsed().as_nanos();TIMES.with(|v|{let mut a=v.get();a[self.0]+=t;v.set(a)});}}}
 #[inline(always)] pub fn timer(i:usize)->Timer {Timer(i,if cfg!(measure) && (i<3 || i==7){Some(Instant::now())}else{None})}
}

#[path="../../sources/bounded.rs"] mod bounded;
#[path="../../sources/quotient.rs"] mod quotient;
#[path="../../sources/lift.rs"] mod lift;
#[path="../../sources/finish.rs"] mod finish;
#[path="../../sources/images.rs"] mod images;
#[path="../../sources/linear.rs"] mod linear;
#[path="../../sources/pencil.rs"] mod pencil;
#[path="../../sources/place.rs"] mod place;
#[path="../../sources/prepared.rs"] mod prepared;
#[path="../../sources/public.rs"] mod public;
#[path="../../sources/slopes.rs"] mod slopes;
#[path="../../sources/small.rs"] mod small;
#[path="../../sources/aa4a9720cd-solver.rs"] mod solver;
#[path="../../sources/table_oracle.rs"] mod table_oracle;
#[path="../../sources/tail.rs"] mod tail;
#[path="../../sources/validate.rs"] mod validate;
#[path="../../sources/wide.rs"] mod wide;
#[path="../../sources/catalogue.rs"] mod catalogue; #[path="../../sources/coefficients.rs"] mod coefficients; fn main() {
    for (g,p) in [(2,127),(2,257),(2,1021),(2,4093),(3,73),(3,127),(3,257),(3,383),(4,37),(4,43),(4,59),(4,67)] {
        if g!=2 {continue;} counted::disable();
        let Some(inst)=pick_instance(p,g) else {println!("PANEL g={g} p={p} status=unselected");continue;};
        let ell=inst.ell;
        // Independently enumerate every quadratic on each genus-three field.
        if g == 3 {
            let fp = &inst.curve.fp;
            let roots = prepared::RootTable::build(fp);
            for b in 0..p {
                for c in 0..p {
                    let u = hec_native_ic::poly::Poly::from_coeffs(&[c,b,1]);
                    let stats = &mut table_oracle::OracleStats::default();
                    assert_eq!(centered::recover(&u,fp,stats,&roots),table_oracle::split(&u,fp,stats,&roots));
                }
            }
        }

        if !validate::eligible(&inst.curve,&inst.d1,&inst.d1,ell) {
            println!("PANEL g={g} p={p} c={} ell={ell} status=ineligible",inst.c);
            continue;
        }
        println!("PANEL g={g} p={p} c={} ell={ell} status=selected",inst.c);
        let c = &inst.curve;
        let mut rng = Rng::new(2026121307 ^ p ^ ((g as u64) << 32));
        for case in 0..4 {
            let d = c.mul(&inst.d1, 1 + rng.below(ell - 1));
            let truth = 1 + rng.below(ell - 1);
            let target = c.mul(&d, truth);
            let seed = 2026121311 ^ p ^ ((g as u64) << 32) ^ case as u64;
            let run = |variant:usize, quarters:usize, arm: usize, audit: bool, shared:Option<&[u64]>| {
                let entered=phases::timer(0);let powers = if arm >= 1 {
                    let checked = if arm==10 && g<=3 { powers::Powers::checked::<true> } else { powers::Powers::checked::<false> }; let table=checked(c,&d,&target,ell,if arm==15 {4} else if g==2 {3} else {2},audit).unwrap();
                    if audit {
                        assert_eq!(lift::add(c,&d,&c.neg(&d),2,true),c.zero());
                        assert_eq!(lift::double(c,&c.zero(),2,true),c.zero());
                        assert_eq!(lift::add(c,&d,&d,2,true),c.add(&d,&d));
                        assert!(validate::eligible(c,&d,&target,ell));
                        assert!(powers::Powers::checked::<false>(c,&c.zero(),&target,ell,3,audit).is_none());
                        assert!(powers::Powers::checked::<false>(c,&d,&target,ell*2,3,audit).is_none());
                        assert!(powers::Powers::checked::<false>(c,&d,&target,3,3,audit).is_none());
                        for n in [0,1,ell/2,ell-1,truth,(truth+1)%ell] {
                            assert_eq!(table.product(c,n,true),c.mul(&d,n));
                        }
                        assert_ne!(table.product(c,(truth+1)%ell,true),target);
                    }
                    Some(table)
                } else {
                    assert!(validate::eligible(c,&d,&target,ell));
                    None
                };

                drop(entered);let catalogue=phases::timer(1);
                let prep = if arm==0 {prepared::prepare(c,audit)} else {catalogue::build(c,false,audit)};drop(catalogue);
                if arm == 0 {
                    let r = pencil::solve(
                        c,
                        &d,
                        &target,
                        ell,
                        512,
                        audit,
                        &prep,
                        pencil::Options {
                            normalize: true,
                            finish: false,
                            seed,
                        },
                    );
                    (r.k, r.anchors, 0, r.accepted, 0, 0, 0,0,0)
                } else {
                    phases::mark("initial_rows");
                    let solve = match (arm==13,true){(true,true)=>delayed::solve::<true,3,true>,(true,false)=>delayed::solve::<true,3,false>,(false,true)=>delayed::solve::<true,0,true>,(false,false)=>delayed::solve::<true,0,false>}; let r = solve(
                        c,
                        &d,
                        &target,
                        ell,
                        512,
                        audit,
                        &prep,
                        powers.as_ref(), true, shared,
                        delayed::Options { balance:variant,reciprocal:true,step_choice:0,priority:1,small_mode:2,coefficient_mode:if g==2{1}else{2},initial_pencils:2,admission:2,quarters,
                            layout: if g==2 {2} else {0},
                            integrated: true,
                            delay: true, defer: 2,
                            normalize: true,
                            finish: false,
                            seed,
                            certificate: false,
                            rolling: true,
                            carry: true,
                            indexed: true,
                            mode: if g == 3 { 1 } else { 3 },
                            eager: false,
                            cutoff: if g != 3 { 2 } else { 0 },
                            substitute: true,
                            fused: true,
                        },
                    );
                    (r.k, r.anchors, r.scanned, r.buckets,r.accepted,r.pivots,r.direct,r.saved,r.replayed)
                }
            };
 let call=|i,audit| {
    if i==0 {run(0,4,0,audit,None)} else {
      let answer=candidate::solve(c,&d,&target,ell,i==2,audit);
      (Some(answer),0,0,0,0,0,0,0,0)
    }
 };
 assert!(!c.is_zero(&d) && c.is_zero(&c.mul(&d,ell)));
 for k in [0,1,truth,ell-1]{
    let t=c.mul(&d,k);
    assert!(verify::check(c,&d,&t,k,ell));assert!(shortcheck::check(c,&d,&t,k,ell));assert!(!shortcheck::check(c,&d,&c.add(&t,&d),k,ell));
    assert!(!verify::check(c,&d,&c.add(&t,&d),k,ell));
 }
 for arm in 0..3 {assert_eq!(call(arm,true).0,Some(truth));}
 for rep in 0..8 {for pass in 0..3 {
    let i=(case+rep+if std::env::args().len()>1 {2-pass}else{pass})%3;phases::TIMES.with(|v|v.set([0;8]));
    let start=Instant::now();let r=black_box(call(i,false));let t=start.elapsed().as_nanos();
    assert_eq!(r.0,Some(truth));let v=phases::TIMES.with(|v|v.get());
    println!("P {g} {p} {case} {rep} {i} {t} {} {} {} {}",v[0],v[1],v[2],v[7]);
 }}
 }}}
