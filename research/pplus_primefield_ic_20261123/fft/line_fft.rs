//! Exact, bounded line-FFT qualification. No performance measurements.
//! The full unfiltered selector is the domain. This is not an ECDLP solver.
mod production;
use std::collections::BTreeSet;

const P: u64 = 2_147_483_647;
type Poly = Vec<u64>;

fn add(a: u64, b: u64) -> u64 { production::addmod(a, b, P) }
fn sub(a: u64, b: u64) -> u64 { if a >= b { a-b } else { P-(b-a) } }
fn mul(a: u64, b: u64) -> u64 { production::mulmod(a, b, P) }
fn pow(mut a: u64, mut n: u64) -> u64 {
    let mut r = 1;
    while n != 0 { if n & 1 != 0 { r=mul(r,a); } a=mul(a,a); n >>= 1; }
    r
}

#[derive(Clone)]
struct Domain { roots: Vec<Poly>, inverse_twiddles: Vec<Poly>, inv2: u64 }
impl Domain {
    fn new(k: usize) -> Result<Self,String> {
        let mut roots = vec![vec![0]];
        let mut inverse_twiddles = vec![vec![]];
        for level in 1..=k {
            let current=production::dickson_abscissae(P,level as u32,0,0)?;
            if current.len()!=1<<level || current.iter().copied().collect::<BTreeSet<_>>().len()!=current.len() {
                return Err(format!("selector cardinality at k={level}"));
            }
            let mut inv=Vec::with_capacity(current.len()/2);
            for (j,pair) in current.chunks_exact(2).enumerate() {
                let x=pair[0];
                if x==0 || pair[1]!=P-x || sub(mul(x,x),2)!=roots[level-1][j] {
                    return Err(format!("selector pairing at k={level}, j={j}"));
                }
                let z=pow(mul(2,x),P-2);
                if ref_mul(ref_mul(2,x),z)!=1 {
                    return Err(format!("inverse twiddle at k={level}, j={j}"));
                }
                inv.push(z);
            }
            roots.push(current); inverse_twiddles.push(inv);
        }
        Ok(Self { roots, inverse_twiddles, inv2:(P+1)/2 })
    }
    fn forward(&self, c: &[u64], k: usize) -> Poly {
        assert_eq!(c.len(),1<<k);
        if k==0 { return c.to_vec(); }
        let even:Poly=c.iter().step_by(2).copied().collect();
        let odd:Poly=c.iter().skip(1).step_by(2).copied().collect();
        let e=self.forward(&even,k-1);
        let o=self.forward(&odd,k-1);
        let mut out=vec![0;c.len()];
        for j in 0..e.len() {
            let xo=mul(self.roots[k][2*j],o[j]);
            out[2*j]=add(e[j],xo); out[2*j+1]=sub(e[j],xo);
        }
        out
    }
    fn inverse(&self, v: &[u64], k: usize) -> Poly {
        assert_eq!(v.len(),1<<k);
        if k==0 { return v.to_vec(); }
        let mut even=vec![0;v.len()/2];
        let mut odd=vec![0;v.len()/2];
        for j in 0..even.len() {
            even[j]=mul(add(v[2*j],v[2*j+1]),self.inv2);
            odd[j]=mul(sub(v[2*j],v[2*j+1]),self.inverse_twiddles[k][j]);
        }
        let e=self.inverse(&even,k-1);
        let o=self.inverse(&odd,k-1);
        let mut out=vec![0;v.len()];
        for j in 0..e.len() { out[2*j]=e[j];out[2*j+1]=o[j]; }
        out
    }
}

// Independent monomial reference: u128 arithmetic only; no transform helpers.
fn ref_add(a:u64,b:u64)->u64 { ((a as u128+b as u128)%P as u128) as u64 }
fn ref_sub(a:u64,b:u64)->u64 { ((a as u128+P as u128-b as u128)%P as u128) as u64 }
fn ref_mul(a:u64,b:u64)->u64 { ((a as u128*b as u128)%P as u128) as u64 }
fn trim(mut a:Poly)->Poly { while a.len()>1 && a.last()==Some(&0) {a.pop();} a }
fn ref_product(a:&[u64],b:&[u64])->Poly {
    let mut c=vec![0;a.len()+b.len()-1];
    for (i,&x) in a.iter().enumerate() {
        for (j,&y) in b.iter().enumerate() { c[i+j]=ref_add(c[i+j],ref_mul(x,y)); }
    }
    trim(c)
}
fn ref_dickson_powers(k:usize)->Vec<Poly> {
    let mut out=vec![vec![0,1]];
    for j in 0..k {let mut next=ref_product(&out[j],&out[j]);next[0]=ref_sub(next[0],2);out.push(next);}
    out
}
fn ref_basis(k:usize,powers:&[Poly])->Vec<Poly> {
    let mut basis=Vec::with_capacity(1<<k);
    for i in 0..1<<k {
        let mut b=vec![1];
        for j in 0..k {if i&(1<<j)!=0 {b=ref_product(&b,&powers[j]);}}
        basis.push(b);
    }
    basis
}
fn ref_expand(c:&[u64],basis:&[Poly])->Poly {
    let mut out=vec![0;c.len()];
    for (b,&coefficient) in basis.iter().zip(c) {
        for (j,&v) in b.iter().enumerate() {out[j]=ref_add(out[j],ref_mul(coefficient,v));}
    }
    trim(out)
}
fn ref_horner(a:&[u64],x:u64)->u64 {
    a.iter().rev().fold(0,|r,&c|ref_add(ref_mul(r,x),c))
}
fn ref_remainder(a:&[u64],monic:&[u64])->Poly {
    assert_eq!(monic.last(),Some(&1));
    let degree=monic.len()-1;
    let mut r=a.to_vec();
    while r.len()>degree {
        let shift=r.len()-monic.len();let leading=*r.last().unwrap();
        for (j,&c) in monic.iter().enumerate() {r[shift+j]=ref_sub(r[shift+j],ref_mul(leading,c));}
        r=trim(r);
    }
    r
}
fn sample(n:usize,mut state:u64)->Poly {
    (0..n).map(|_|{state ^= state<<13;state ^= state>>7;state ^= state<<17;state%P}).collect()
}

fn qualify()->Result<(),String> {
    let mut evaluation_cases=0;
    let mut roundtrip_cases=0;
    let mut product_cases=0;
    let mut output_values=0;
    for k in 1..=8 {
        let n=1usize<<k;
        let domain=Domain::new(k)?;
        let powers=ref_dickson_powers(k);
        let basis=ref_basis(k,&powers);
        for &x in &domain.roots[k] {
            if ref_horner(&powers[k],x)!=0 {return Err(format!("monomial selector check k={k}, x={x}"));}
        }
        let coeffs:Vec<Poly>=(0..4).map(|i|sample(n,0x9e37_79b9_7f4a_7c15u64 ^ ((k as u64)<<32) ^ (i+1))).collect();
        let monomials:Vec<Poly>=coeffs.iter().map(|c|ref_expand(c,&basis)).collect();
        let values:Vec<Poly>=coeffs.iter().map(|c|domain.forward(c,k)).collect();
        for i in 0..4 {
            for (j,&x) in domain.roots[k].iter().enumerate() {
                let want=ref_horner(&monomials[i],x);
                if values[i][j]!=want {return Err(format!("forward mismatch k={k}, sample={i}, index={j}, got={}, want={want}",values[i][j]));}
            }
            evaluation_cases+=1;output_values+=n;
            if domain.inverse(&values[i],k)!=coeffs[i] {return Err(format!("roundtrip mismatch k={k}, sample={i}"));}
            roundtrip_cases+=1;
            let j=(i+1)%4;
            let pointwise:Poly=values[i].iter().zip(&values[j]).map(|(&a,&b)|mul(a,b)).collect();
            let got=ref_expand(&domain.inverse(&pointwise,k),&basis);
            let want=ref_remainder(&ref_product(&monomials[i],&monomials[j]),&powers[k]);
            if got!=want {return Err(format!("quotient product mismatch k={k}, pair=({i},{j})"));}
            product_cases+=1;
        }
        println!("{{\"kind\":\"depth\",\"k\":{k},\"domain_size\":{n},\"forward_cases\":4,\"inverse_cases\":4,\"quotient_products\":4,\"status\":\"pass\"}}");
        if k==8 {
            let mut altered=domain.clone();altered.roots[k][0]=add(altered.roots[k][0],1);
            if altered.forward(&coeffs[0],k)==values[0] {return Err("altered-twiddle negative control not detected".to_string());}
            let mut wrong_norm=domain.clone();wrong_norm.inv2=1;
            if wrong_norm.inverse(&values[0],k)==coeffs[0] {return Err("inverse-normalization negative control not detected".to_string());}
            println!("{{\"kind\":\"negative_controls\",\"altered_twiddle_detected\":true,\"incorrect_inverse_normalization_detected\":true}}");
        }
    }
    println!("{{\"kind\":\"summary\",\"status\":\"pass\",\"field_prime\":{P},\"depths\":8,\"forward_cases\":{evaluation_cases},\"inverse_cases\":{roundtrip_cases},\"quotient_products\":{product_cases},\"forward_output_values_checked\":{output_values},\"negative_controls\":2,\"timing_performed\":false,\"ecd_lp_recovery_performed\":false,\"independent_signed_validation\":false}}");
    Ok(())
}
fn main() {
    if let Err(error)=qualify() {eprintln!("{{\"kind\":\"failure\",\"message\":{error:?}}}");std::process::exit(1);}
}
