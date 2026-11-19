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
#[path="../../sources/complete.rs"] mod complete; mod candidate {use hec_native_ic::{jac::{Curve,Div}, oracle::{split,OracleStats},fp::Fp};
extern "C" {fn quartic(p:i32,ys:*mut i32,values:*mut i32,retain:i32,inv:*mut u64,roots:*mut i32,xs:*mut u64,len:*mut i32)->i32;}
fn coordinate(c:&Curve,d:&Div,l:u64,ys:&[i32],values:&[i32],anchor:usize)->u64{
    let mut current=*d;
    for shift in 0..=1024u64{
        if let Some(roots)=split(&current.u,&c.fp,&mut OracleStats::default()){
            let mut sum=0;
            for &(x,m) in roots.as_slice(){
                let y=ys[x as usize] as u64;
                let log=if y==0 {0}else{values[x as usize] as u64};
                let actual=current.v.eval(x,&c.fp);
                assert!(actual==y || actual==c.fp.neg(y));
                sum=(sum+m as u64*if actual==y{log}else{l-log})%l;
            }
            return (sum+l-shift*(values[anchor] as u64)%l)%l;
        }
        current=c.add_place(&current,anchor as u64,ys[anchor] as u64);
    }
    panic!("shift bound")
}
fn solve_field<const P:u64>(c:&Curve,d:&Div,target:&Div,l:u64,probe:bool,audit:bool)->u64{
    let p=c.fp.p as usize;
    let mut ys=vec![-1;p];
    let mut values=vec![0;p];
    let mut inv=[0;383];let mut roots=[-1;383];let mut places=[0;383];let mut len=0;let a=unsafe {quartic(p as i32,ys.as_mut_ptr(),values.as_mut_ptr(),probe as i32,inv.as_mut_ptr(),roots.as_mut_ptr(),places.as_mut_ptr(),&mut len)};
    assert!(a>=0);
    let mut xs=Vec::new();
    if audit {
        let mut ref_inv=[0;383];let mut ref_roots=[-1;383];ref_inv[1]=1;
        for i in 2..p{ref_inv[i]=P-(P/i as u64)*ref_inv[p%i]%P;}
        for y in 0..=P/2{ref_roots[(y*y%P) as usize]=y as i32;}
        assert_eq!(inv,ref_inv);assert_eq!(roots,ref_roots); xs.extend((0..p).filter(|&x|ys[x]>=0).map(|x|x as u64));
    }
    if audit {assert_eq!(xs.as_slice(),&places[..len as usize]);} let xs:&[u64]=&places[..len as usize];let get=|input:&Div|{
        let old=||coordinate(c,input,l,&ys,&values,a as usize);
        {
            let found=crate::complete::coordinate::<P>(c,input,l,&ys,&values,&xs,&inv,&roots,audit);
            if audit{eprintln!("{} {}",P,u8::from(found.is_some()));}
            if let Some(k)=found{if audit{assert_eq!(old(),k);}return k;}
        }
        old()
    };
    let u=get(d);let v=get(target);
    assert_ne!(u,0);
    let f=Fp::new(l);
    let k=f.mul(v,f.inv(u));
    assert!(crate::shortcheck::check(c,d,target,k,l));
    k
}

pub fn solve(c:&Curve,d:&Div,t:&Div,l:u64,probe:bool,audit:bool)->u64{match c.fp.p{73=>solve_field::<73>(c,d,t,l,probe,audit),127=>solve_field::<127>(c,d,t,l,probe,audit),257=>solve_field::<257>(c,d,t,l,probe,audit),383=>solve_field::<383>(c,d,t,l,probe,audit),_=>unreachable!()}}
} #[path="../../sources/9f22b6584c-verify.rs"] mod verify; mod shortcheck {use hec_native_ic::jac::{Curve,Div};
fn gcd(mut a:i64,mut b:i64)->i64{
    while b!=0 {(a,b)=(b,a%b);}
    a
}
pub fn check(c:&Curve,d:&Div,target:&Div,k:u64,l:u64)->bool {
    // The base has order l. Requiring b invertible on the entire Jacobian
    // also prevents torsion in an incorrect target from passing the check.
    let h=match c.fp.p {73=>6,127=>1,257|383=>3,_=>return c.mul(d,k)==*target};
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
        if g!=3 {continue;} counted::disable();
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
 for rep in 0..32 {for pass in 0..3 {
    let i=(case+rep+if std::env::args().len()>1 {2-pass}else{pass})%3;phases::TIMES.with(|v|v.set([0;8]));
    let start=Instant::now();let r=black_box(call(i,false));let t=start.elapsed().as_nanos();
    assert_eq!(r.0,Some(truth));let v=phases::TIMES.with(|v|v.get());
    println!("P {g} {p} {case} {rep} {i} {t} {} {} {} {}",v[0],v[1],v[2],v[7]);
 }}
 }}}
