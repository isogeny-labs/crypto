//! Two-known-signed-prefix RR9 factor-resultant / 6x5 Jacobian experiment.
//! Reuses original prime31-algebraic-ic EC arithmetic, without generic 4+4 tables.
//! Explicitly CONDITIONAL on TWO KNOWN signed support leaves. This is not a
//! public-only unknown-index solver and not a complete non-generic ECDLP solve.
use prime31_algebraic_ic::{check_curve, Arithmetic, PRIME31};
use serde_json::{json, Value};
use std::collections::HashSet;

const P: u64=2_147_483_647;
const N: u64=2_147_446_831;
const OLD:[u64;8]=[1,3,5,7,11,13,17,19];
const TERN:[u64;8]=[1,3,9,27,81,243,729,2187];
fn add(a:u64,b:u64)->u64{(a+b)%P}
fn sub(a:u64,b:u64)->u64{(a+P-b)%P}
fn mul(a:u64,b:u64)->u64{((a as u128*b as u128)%P as u128) as u64}
fn pow(mut a:u64,mut b:u64)->u64{
    let mut r=1;
    while b>0 {if b&1!=0 {r=mul(r,a);} a=mul(a,a);b>>=1;}
    r
}
fn inv(a:u64)->u64{assert_ne!(a,0);pow(a,P-2)}
fn xpow(x:u64)->[u64;4]{let x2=mul(x,x);[1,x,x2,mul(x2,x)]}
fn poly(c:&[u64],x:u64)->u64{c.iter().rev().fold(0,|a,&b|add(mul(a,x),b))}
fn base8(ar:&mut Arithmetic)->Vec<(u64,u64)>{
    let mut out=Vec::new();
    for x in 0..10_000 {
        let rhs=ar.rhs(PRIME31,x);
        if let Some(y)=ar.sqrt(rhs,P){
            if y!=0{
                let q=(x,y.min(P-y));
                assert!(ar.on_curve(PRIME31,Some(q)));
                out.push(q);
                if out.len()==8{break}
            }
        }
    }
    assert_eq!(out.iter().map(|q|q.0).collect::<Vec<_>>(),vec![0,1,2,3,5,6,8,9]);
    out
}
fn slots(ar:&mut Arithmetic,base:&[(u64,u64)],weights:[u64;8])
    ->Vec<Vec<(u64,u64)>>{
    let mut all=HashSet::new();
    let ret:Vec<_>=weights.iter().map(|&w|{
        base.iter().map(|&p|{
            let q=ar.scalar(PRIME31,w,Some(p)).expect("nonzero support leaf");
            assert!(all.insert(q.0),"overlapping weighted x in slots");
            q
        }).collect()
    }).collect();
    assert_eq!(all.len(),64);
    ret
}
// GRADER ONLY: construct a planted relation from signed leaves.
fn plant(ar:&mut Arithmetic,sl:&[Vec<(u64,u64)>],t:usize)
    ->((u64,u64),Vec<(u64,u64)>,Vec<usize>,Vec<i8>) {
    let (mut sum,mut points,mut indices,mut signs)=(None,Vec::new(),Vec::new(),Vec::new());
    for (j,slot) in sl.iter().enumerate(){
        let i=(3*t+7*j+j*j)%8;
        let sign=if (t+3*j)%4<2{1}else{-1};
        let (x,y)=slot[i];
        let q=(x,if sign==1{y}else{sub(0,y)});
        sum=ar.add(PRIME31,sum,Some(q));
        points.push(q); indices.push(i); signs.push(sign);
    }
    (sum.expect("planted nonzero R"),points,indices,signs)
}
// GRADER ONLY: 8 signed points determine normalized RR9 coefficient chart.
fn grader_full_chart(r:(u64,u64),pts:&[(u64,u64)])->[u64;7]{
    assert_eq!(pts.len(),8);
    let mut a=[[0u64;8];8];
    for (i,&(x,y)) in pts.iter().enumerate(){
        let xp=xpow(x);let u=sub(x,r.0);let v=add(y,r.1);
        for j in 0..4 {a[i][j]=mul(u,xp[j]);}
        for j in 0..3 {a[i][j+4]=mul(v,xp[j]);}
        a[i][7]=sub(0,mul(v,xp[3]));
    }
    for k in 0..7{
        let pos=(k..8).find(|&i|a[i][k]!=0).expect("grater chart rank 7");
        a.swap(k,pos);
        let u=inv(a[k][k]);
        for j in k..8{a[k][j]=mul(a[k][j],u);}
        for i in 0..8{
            if i==k{continue;}
            let f=a[i][k];
            for j in k..8{a[i][j]=sub(a[i][j],mul(f,a[k][j]));}
        }
    }
    assert_eq!(a[7],[0u64;8],"full chart inconsistent");
    std::array::from_fn(|i|a[i][7])
}
// CANDIDATE algebraic projection. Inputs: public R, TWO SUPPLIED SIGNED
// prefix leaves, and five unknown free chart coefficients (not a witness).
// dfull[z][k] = partial full coefficient z / partial free coordinate k.
fn project(r:(u64,u64),q0:(u64,u64),q1:(u64,u64),w:[u64;5])
    ->([u64;7],[[u64;5];7]){
    let x0=q0.0;let x1=q1.0;
    assert!(x0!=x1 && x0!=r.0 && x1!=r.0,"exceptional prefix");
    let alpha0=mul(add(q0.1,r.1),inv(sub(x0,r.0)));
    let alpha1=mul(add(q1.1,r.1),inv(sub(x1,r.0)));
    let xp0=xpow(x0);let xp1=xpow(x1);
    let c = |xp:[u64;4],alpha:u64| -> u64 {
        let v=add(xp[3],add(w[2],add(mul(w[3],xp[1]),mul(w[4],xp[2]))));
        sub(0,add(mul(alpha,v),add(mul(w[0],xp[2]),mul(w[1],xp[3]))))
    };
    let c0=c(xp0,alpha0);let c1=c(xp1,alpha1);
    let dinv=inv(sub(x1,x0));
    let a1=mul(sub(c1,c0),dinv);
    let a0=sub(c0,mul(a1,x0));
    let full=[a0,a1,w[0],w[1],w[2],w[3],w[4]];
    let dc = |xp:[u64;4],alpha:u64| -> [u64;5] {
        [sub(0,xp[2]),sub(0,xp[3]),sub(0,alpha),
         sub(0,mul(alpha,xp[1])),sub(0,mul(alpha,xp[2]))]
    };
    let dc0=dc(xp0,alpha0);let dc1=dc(xp1,alpha1);
    let mut d=[[0u64;5];7];
    for k in 0..5{
        d[1][k]=mul(sub(dc1[k],dc0[k]),dinv);
        d[0][k]=sub(dc0[k],mul(x0,d[1][k]));
        d[k+2][k]=1;
    }
    (full,d)
}
fn f_val(r:(u64,u64),c:&[u64;7],q:(u64,u64))->u64{
    let u=sub(q.0,r.0);let v=add(q.1,r.1);
    add(mul(u,poly(&c[..4],q.0)),
        mul(v,add(xpow(q.0)[3],poly(&c[4..],q.0))))
}
// The norm and *full* seven-coordinate Jacobian via a separate closed form.
fn norm_and_grad7(r:(u64,u64),c:&[u64;7],x:u64)->(u64,[u64;7]){
    let xp=xpow(x);let aa=poly(&c[..4],x);
    let vv=add(xp[3],poly(&c[4..],x));
    let u=sub(x,r.0);
    let d=add(add(xp[2],mul(r.0,x)),add(mul(r.0,r.0),PRIME31.a));
    let m=sub(sub(mul(d,mul(vv,vv)),mul(u,mul(aa,aa))),
              mul(mul(2,r.1),mul(aa,vv)));
    let d_a=sub(0,add(mul(mul(2,u),aa),mul(mul(2,r.1),vv)));
    let d_v=sub(mul(mul(2,d),vv),mul(mul(2,r.1),aa));
    let mut grad=[0u64;7];
    for j in 0..4{grad[j]=mul(d_a,xp[j]);}
    for j in 0..3{grad[j+4]=mul(d_v,xp[j]);}
    (m,grad)
}
// CANDIDATE: no generic point addition or sum tables.
fn public_six_h_jac(r:(u64,u64),q0:(u64,u64),q1:(u64,u64),
                   w:[u64;5],sl:&[Vec<(u64,u64)>])
    ->([u64;6],[[u64;5];6]){
    assert_eq!(sl.len(),8);
    let (full,dfull)=project(r,q0,q1,w);
    let mut hs=[0u64;6];let mut jac=[[0u64;5];6];
    for j in 2..8{
        let mut vals=[0u64;8];let mut dv=[[0u64;5];8];
        for i in 0..8{
            let (val,grad7)=norm_and_grad7(r,&full,sl[j][i].0);
            vals[i]=val;
            for k in 0..5{
                let mut s=0;
                for a in 0..7{s=add(s,mul(grad7[a],dfull[a][k]));}
                dv[i][k]=s;
            }
        }
        let mut pref=[1u64;9];
        for i in 0..8{pref[i+1]=mul(pref[i],vals[i]);}
        hs[j-2]=pref[8];
        let mut suffix=1;
        for i in (0..8).rev(){
            let others=mul(pref[i],suffix);
            for k in 0..5{jac[j-2][k]=add(jac[j-2][k],mul(others,dv[i][k]));}
            suffix=mul(suffix,vals[i]);
        }
    }
    (hs,jac)
}
fn rank5(mut mat:[[u64;5];6])->usize{
    let mut rank=0;
    for col in 0..5{
        let Some(k)=(rank..6).find(|&r|mat[r][col]!=0) else{continue};
        mat.swap(rank,k);let v=inv(mat[rank][col]);
        for c in col..5{mat[rank][c]=mul(mat[rank][c],v);}
        for i in rank+1..6{
            let f=mat[i][col];
            for c in col..5{mat[i][c]=sub(mat[i][c],mul(f,mat[rank][c]));}
        }
        rank+=1;
    }
    rank
}
fn arm(ar:&mut Arithmetic,base:&[(u64,u64)],label:&str,
       weights:[u64;8],count:usize)->Value{
    let sl=slots(ar,base,weights);let mut fixtures=Vec::new();
    let mut ranks=0usize;
    for t in 0..count{
        let (r,pts,idx,signs)=plant(ar,&sl,t);
        let full=grader_full_chart(r,&pts);
        assert!(pts.iter().all(|&q|f_val(r,&full,q)==0));
        let w=[full[2],full[3],full[4],full[5],full[6]];
        let (reconstructed,_)=project(r,pts[0],pts[1],w);
        assert_eq!(full,reconstructed);
        let (hs,jac)=public_six_h_jac(r,pts[0],pts[1],w,&sl);
        assert_eq!(hs,[0;6]);
        let rk=rank5(jac);
        assert_eq!(rk,5);
        ranks+=1;
        let mut mutated=w;mutated[0]=add(mutated[0],1);
        let (bad,_)=public_six_h_jac(r,pts[0],pts[1],mutated,&sl);
        assert!(bad.iter().any(|&v|v!=0));
        fixtures.push(json!({
            "case":t,"target":r,"signed_points":pts,"signed_indices":idx,
            "signs":signs,"grader_full":full,"free":w,
            "candidate_full":reconstructed,"six_resultants":hs,
            "six_by_five_jacobian":jac,"jacobian_rank":rk,
            "mutation_a2_plus_one_resultants":bad
        }));
    }
    json!({"arm":label,"weights":weights,"base8":base,
           "full_rank_fixtures":ranks,"fixtures":fixtures})
}
fn study(count:usize)->Value{
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar).expect("same original curve");
    assert_eq!(PRIME31.n,N);
    let b=base8(&mut ar);
    let old=arm(&mut ar,&b,"old",OLD,count);
    let tern=arm(&mut ar,&b,"ternary",TERN,count);
    // Counting only: this is not an observed F4 matrix rank or wall cost.
    // table: t anchors / 7-t vars / 8-t equations / C(16+7-t,7-t) monomials.
    let monomials=[245_157u64,74_613,20_349,4_845,969];
    let mut powers=1u64;
    let cells:Vec<Value>=(0..=4).map(|t|{
        let b=powers;powers*=16;
        json!({"known_prefixes":t,"remaining_variables":7-t,
               "remaining_resultants":8-t,"prefix_branches":b,
               "dense_degree16_monomial_ceiling":monomials[t],
               "branch_times_monomial_ceiling":b*monomials[t]})
    }).collect();
    json!({
        "schema":"prime31.rr9.two-prefix.v1",
        "evidence":"source-specific producer; pcc:unverified",
        "curve":{"p":P,"a":PRIME31.a,"b":PRIME31.b,"n":N,"cofactor":1},
        "candidate_has_two_supplied_signed_leaves":true,
        "public_unknown_prefix_extractor":false,
        "generic_group_sum_tables_used":false,
        "complete_nongeneric_ecdlp_solver":false,
        "algebraic_branch_cost_ceiling_table":cells,
        "arms":[old,tern]
    })
}
fn main(){println!("{}",serde_json::to_string_pretty(&study(16)).unwrap());}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]fn two_prefix_original_curve_projection_and_rank(){
        let doc=study(2);
        for arm in doc["arms"].as_array().unwrap(){
            assert_eq!(arm["full_rank_fixtures"],2);
            assert_eq!(arm["fixtures"].as_array().unwrap().len(),2);
        }
    }
    #[test]fn exact_branch_counts_and_enumeration_waiver(){
        let doc=study(1);
        assert_eq!(doc["generic_group_sum_tables_used"],false);
        assert_eq!(doc["complete_nongeneric_ecdlp_solver"],false);
        let v=&doc["algebraic_branch_cost_ceiling_table"];
        assert_eq!(v[2]["prefix_branches"],256);
        assert_eq!(v[2]["dense_degree16_monomial_ceiling"],20349);
        assert_eq!(v[4]["branch_times_monomial_ceiling"],63_504_384);
    }
}
