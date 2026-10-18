//! Classical calculation of finite Fourier outcome laws on small cyclic groups.
use serde_json::json;
use std::io::Write;
#[derive(Clone,Copy,Default,Debug)]
struct C { re:f64, im:f64 }
impl C {
    fn add(self,b:Self)->Self {Self{re:self.re+b.re,im:self.im+b.im}}
    fn sub(self,b:Self)->Self {Self{re:self.re-b.re,im:self.im-b.im}}
    fn mul(self,b:Self)->Self {Self{re:self.re*b.re-self.im*b.im,im:self.re*b.im+self.im*b.re}}
    fn norm(self)->f64 {self.re*self.re+self.im*self.im}
}
fn fft(a:&mut[C])->u64 {
    let n=a.len(); assert!(n.is_power_of_two());
    let bits=n.trailing_zeros();
    for i in 0..n {let j=i.reverse_bits()>>(usize::BITS-bits);if i<j {a.swap(i,j);}}
    let mut work=0;
    let mut width=2;
    while width<=n {
        let angle=2.0*std::f64::consts::PI/width as f64;
        let omega=C{re:angle.cos(),im:angle.sin()};
        for start in (0..n).step_by(width) {
            let mut w=C{re:1.0,im:0.0};
            for j in 0..width/2 {
                let x=a[start+j];let y=w.mul(a[start+j+width/2]);
                a[start+j]=x.add(y);a[start+j+width/2]=x.sub(y);w=w.mul(omega);work+=1;
            }
        }
        width*=2;
    }
    work
}
fn law(r:usize,q:usize,d:usize)->(Vec<f64>,u64) {
    assert!(q.is_power_of_two() && r<q && d<r);
    let mut probabilities=vec![0.0;q*q];let mut work=0;
    for z in 0..r {
        let mut amplitudes=vec![C::default();q*q];
        for a in 0..q {for b in 0..q {if (a+d*b)%r==z {amplitudes[a*q+b].re=1.0;}}}
        for row in amplitudes.chunks_mut(q) {work+=fft(row);}
        let mut column=vec![C::default();q];
        for b in 0..q {
            for a in 0..q {column[a]=amplitudes[a*q+b];}
            work+=fft(&mut column);
            for a in 0..q {amplitudes[a*q+b]=column[a];}
        }
        for (out,amp) in probabilities.iter_mut().zip(amplitudes) {*out+=amp.norm()/(q as f64).powi(4);}
    }
    (probabilities,work)
}
fn candidate(u:usize,v:usize,r:usize,q:usize)->(Option<usize>,usize) {
    let s=((2*u*r+q)/(2*q))%r;let t=((2*v*r+q)/(2*q))%r;
    if s==0 {return(None,0);}
    let(mut a,mut b,mut x,mut y)=(r as i64,s as i64,0i64,1i64);let mut divs=0;
    while b!=0 {let k=a/b;(a,b)=(b,a-k*b);(x,y)=(y,x-k*y);divs+=1;}
    if a!=1 {return(None,divs);}
    (Some(((t as i64*x).rem_euclid(r as i64)) as usize),divs)
}
fn main()->Result<(),Box<dyn std::error::Error>> {
    let output=std::env::args().nth(1).ok_or("usage: fourier-recovery OUTPUT.json")?;
    let mut file=std::fs::OpenOptions::new().write(true).create_new(true).open(output)?;
    let mut rows=Vec::new();
    for(r,q,ds)in [(5,16,vec![1,2,3,4]),(11,32,vec![1,2,3,7]),(19,64,vec![1,2,3,7])] {
        for d in ds {
            let(p,work)=law(r,q,d);let norm:f64=p.iter().sum();
            assert!((norm-1.0).abs()<1e-10);
            let(mut correct,mut wrong,mut reject,mut divisions)=(0.0,0.0,0.0,0.0);
            for u in 0..q {for v in 0..q {let(ans,count)=candidate(u,v,r,q);let mass=p[u*q+v];divisions+=mass*count as f64;
                match ans {Some(x)if x==d=>correct+=mass,Some(_)=>wrong+=mass,None=>reject+=mass}
            }}
            assert!((correct+wrong+reject-1.0).abs()<1e-10);
            rows.push(json!({"r":r,"Q":q,"synthetic_secret":d,"normalization":norm,
                "correct_candidate_probability":correct,"wrong_candidate_probability":wrong,"rejected_probability":reject,
                "expected_euclid_divisions":divisions,"fft_butterflies":work,
                "outcome_probabilities_row_major_u_v":p,"candidate_verified_against_known_synthetic_secret":true,
                "elliptic_curve_verification":false}));
        }
    }
    serde_json::to_writer_pretty(&mut file,&json!({"schema":"finite-fourier-recovery/v1","item":20,
        "protocol":"research/ideas_execution_20261018/fourier_protocol.json",
        "classification":"classical small cyclic-group outcome-law calculation; not quantum execution",
        "rows":rows}))?;
    file.write_all(b"\n")?;
    println!("wrote {} normalized Fourier-law fixtures with explicit successful, wrong, and rejected candidate masses",rows.len());Ok(())
}
#[cfg(test)]mod tests {
    use super::*;
    #[test]fn fft_matches_naive_dft() {
        let a:Vec<_>=(0..8).map(|i|C{re:(i*i%7)as f64,im:(i%3)as f64}).collect();let mut b=a.clone();fft(&mut b);
        for k in 0..8 {let mut expected=C::default();for(j,x)in a.iter().enumerate(){let t=2.0*std::f64::consts::PI*(k*j)as f64/8.0;expected=expected.add(x.mul(C{re:t.cos(),im:t.sin()}));}
            assert!(b[k].sub(expected).norm()<1e-20);
        }
    }
    #[test]fn normalized_law_and_equal_register_symmetry() {
        let(p,_)=law(5,16,1);assert!((p.iter().sum::<f64>()-1.0).abs()<1e-12);
        for u in 0..16 {for v in 0..16 {assert!((p[u*16+v]-p[v*16+u]).abs()<1e-12);}}
    }
    #[test]fn exact_character_centers_decode_and_zero_is_rejected() {
        assert_eq!(candidate(0,0,5,16).0,None);
        assert_eq!(candidate(6,13,5,32).0,Some(2));
        assert_eq!(candidate(6,6,5,32).0,Some(1));
    }
}
