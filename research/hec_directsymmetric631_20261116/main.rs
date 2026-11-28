mod field {pub fn p()->u64{631}}
mod setup {pub fn qf()->[u64;15]{std::fs::read_to_string(std::env::var("HEC_SETUP_DIR").unwrap()+"/QF.txt").unwrap().split_whitespace().map(|s|s.parse().unwrap()).collect::<Vec<_>>().try_into().unwrap()}pub fn verify_model(){assert!(crate::target::is_smooth(&qf(),631));}}
mod target {include!("../hec_alignment13_20261115/atlas/target.rs");include!("../hec_calibration631_20261115/target_api631.rs");}
mod config {include!("../hec_calibration631_20261115/field_config.rs");}
#[allow(dead_code)]mod good {include!("../hec_calibration631_20261115/good.rs");include!("../hec_calibration631_20261115/base_control.rs");}
use std::{collections::BTreeMap, fs, path::Path};
const P: u64 = 631;
type M = [u8; 8];
#[derive(Clone)]
struct Pol(BTreeMap<M, u64>);
impl Pol {
    fn zero() -> Self {
        Self(BTreeMap::new())
    }
    fn constant(a: u64) -> Self {
        let mut p = Self::zero();
        p.term([0; 8], a);
        p
    }
    fn variable(i: usize) -> Self {
        let mut m = [0; 8];
        m[i] = 1;
        let mut p = Self::zero();
        p.term(m, 1);
        p
    }
    fn term(&mut self, m: M, a: u64) {
        let c = (self.0.get(&m).copied().unwrap_or(0) + a) % P;
        if c == 0 {
            self.0.remove(&m);
        } else {
            self.0.insert(m, c);
        }
    }
    fn scaled(&self, c: u64) -> Self {
        Self(
            self.0
                .iter()
                .filter_map(|(m, a)| {
                    let a = a * c % P;
                    (a != 0).then_some((*m, a))
                })
                .collect(),
        )
    }
    fn plus(&self, b: &Self) -> Self {
        let mut a = self.clone();
        for (m, c) in &b.0 {
            a.term(*m, *c)
        }
        a
    }
    fn minus(&self, b: &Self) -> Self {
        self.plus(&b.scaled(P - 1))
    }
    fn times(&self, b: &Self) -> Self { assert!(self.0.len().saturating_mul(b.0.len())<=20_000_000,"symbolic_product_cap20M");
        let mut p = Self::zero();
        for (a, c) in &self.0 {
            for (b, d) in &b.0 {
                let m = std::array::from_fn(|i| a[i].checked_add(b[i]).unwrap());
                p.term(m, c * d % P);
            }
        }
        p
    }
    fn eval(&self, x: [u64; 8]) -> u64 {
        self.0.iter().fold(0, |a, (m, c)| {
            let z = (0..8).fold(*c, |z, i| z * pow(x[i], m[i] as u64) % P);
            (a + z) % P
        })
    }
    fn text(&self) -> String {
        let names = ["w", "u0", "u1", "u2", "v0", "v1", "v2", "t"];
        if self.0.is_empty() {
            return "0".into();
        }
        self.0
            .iter()
            .map(|(m, c)| {
                let mut s = c.to_string();
                for i in 0..8 {
                    if m[i] > 0 {
                        s.push('*');
                        s.push_str(names[i]);
                        if m[i] > 1 {
                            s.push('^');
                            s.push_str(&m[i].to_string());
                        }
                    }
                }
                s
            })
            .collect::<Vec<_>>()
            .join("+")
    }
}
fn pow(mut a: u64, mut n: u64) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 != 0 {
            r = r * a % P
        }
        a = a * a % P;
        n >>= 1;
    }
    r
}
type R=[Pol;3];
fn zero()->R{std::array::from_fn(|_|Pol::zero())}
fn ra(a:&R,b:&R)->R{std::array::from_fn(|i|a[i].plus(&b[i]))}
fn rm(a:&R,b:&R,u:&R)->R{let mut t:[Pol;5]=std::array::from_fn(|_|Pol::zero());for i in 0..3{for j in 0..3{t[i+j]=t[i+j].plus(&a[i].times(&b[j]));}}for k in (3..=4).rev(){let c=t[k].clone();for j in 0..3{t[k-3+j]=t[k-3+j].minus(&c.times(&u[j]));}}std::array::from_fn(|i|t[i].clone())}
fn tr(a:&R,u:&R)->Pol{a[0].scaled(3).minus(&a[1].times(&u[2])).plus(&a[2].times(&u[2].times(&u[2]).minus(&u[1].scaled(2))))}
fn polar(a:&R,b:&R,c:&R,u:&R)->Pol{let mut z=tr(a,u).times(&tr(b,u)).times(&tr(c,u));for(x,y,v)in[(a,b,c),(a,c,b),(b,c,a)]{z=z.minus(&tr(&rm(x,y,u),u).times(&tr(v,u)));}z.plus(&tr(&rm(&rm(a,b,u),c,u),u).scaled(2))}
fn rv(n:&[u64;49],u:&R,v:&R,degree:usize)->R{let mut one=zero();one[0]=Pol::constant(1);let mut x=zero();x[1]=Pol::constant(1);let mut xp=vec![one.clone()];let mut yp=xp.clone();for i in 1..=degree{xp.push(rm(&xp[i-1],&x,u));yp.push(rm(&yp[i-1],v,u));}let mut out=zero();for i in 0..=degree{for j in 0..=degree-i{if n[7*i+j]!=0{let p=rm(&xp[i],&yp[j],u);out=ra(&out,&std::array::from_fn(|k|p[k].scaled(n[7*i+j])));}}}out}
fn main(){setup::verify_model();config::install(config::discover(setup::qf()));let args:Vec<_>=std::env::args().collect();assert_eq!(args.len(),3);let out=Path::new(&args[1]);assert!(!out.exists());let lambda:[u64;8]=fs::read_to_string(&args[2]).unwrap().split_whitespace().map(|s|s.parse().unwrap()).collect::<Vec<_>>().try_into().unwrap();assert_ne!(lambda[0],0);let(n,coeff,ts)=good::build();let u:R=std::array::from_fn(|i|Pol::variable(i+1));let v:R=std::array::from_fn(|i|Pol::variable(i+4));let nums:Vec<_>=n.iter().map(|n|rv(n,&u,&v,3)).collect();let tensors:Vec<_>=ts.iter().map(|t|polar(&nums[t[0]],&nums[t[1]],&nums[t[2]],&u)).collect();let h:Vec<_>=coeff.iter().map(|r|r.iter().zip(&tensors).fold(Pol::zero(),|s,(&a,t)|s.plus(&t.scaled(a)))).collect();let mut f=[0;49];for(&(a,b,_),c)in target::monomials(4).iter().zip(setup::qf()){f[7*a as usize+b as usize]=c;}let mut eq=rv(&f,&u,&v,4).to_vec();for i in 1..8{eq.push(h[i].scaled(lambda[0]).minus(&h[0].scaled(lambda[i])));}eq.push(Pol::variable(0).times(&h[0]).minus(&Pol::constant(1)));eq.push(Pol::variable(7).minus(&u[0]));assert_eq!(eq.len(),12);fs::create_dir(out).unwrap();fs::write(out.join("target.ms"),format!("w,u0,u1,u2,v0,v1,v2,t\n631\n{}\n",eq.iter().map(Pol::text).collect::<Vec<_>>().join(",\n"))).unwrap();println!("GENERATED symmetric_variables=6 equations=12 section_terms={:?} equation_terms={:?} solver_or_equivalence_verified=false",h.iter().map(|p|p.0.len()).collect::<Vec<_>>(),eq.iter().map(|p|p.0.len()).collect::<Vec<_>>());}
