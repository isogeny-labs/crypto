// Exact CM cyclic recovery. No floating-point arithmetic or performance metric.
use std::{env,fs,path::Path};
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]struct C(i64,i64);
impl C {
 fn add(self,b:C)->C {C(self.0+b.0,self.1+b.1)}
 fn neg(self)->C {C(-self.0,-self.1)}
 fn mul(self,b:C)->C {C(self.0*b.0-2*self.1*b.1,self.0*b.1+self.1*b.0-self.1*b.1)}
 fn scale(self,n:i64)->C {C(self.0*n,self.1*n)}
 fn red(self)->C {let f=|x:i64|(x+3).rem_euclid(7)-3;C(f(self.0),f(self.1))}
}
fn plus(a:&[C],b:&[C])->Vec<C>{a.iter().zip(b).map(|(x,y)|x.add(*y)).collect()}
fn product(a:&[C],b:&[C])->Vec<C>{let n=a.len();assert_eq!(n,b.len());let mut z=vec![C::default();n];for(i,x)in a.iter().enumerate(){if *x==C::default(){continue;}for(j,y)in b.iter().enumerate(){if *y==C::default(){continue;}let k=(i+j)%n;z[k]=z[k].add(x.mul(*y));}}z}
// Separate sparse action on cyclic vectors, rather than polynomial-product output.
fn action(a:&[C],v:&[C])->Vec<C>{let n=a.len();let support:Vec<_>=v.iter().enumerate().filter(|(_,c)|**c!=C::default()).collect();let mut z=vec![C::default();n];for(j,c)in support{for i in 0..n{let k=(i+j)%n;z[k]=z[k].add(c.mul(a[i]));}}z}
fn coeff(a:&[C])->String{format!("[{}]",a.iter().map(|c|format!("[{},{}]",c.0,c.1)).collect::<Vec<_>>().join(","))}
fn family(n:usize)->Vec<C>{let mut m=vec![C::default();n];for(i,c)in[(0,C(2,1)),(1,C(7,0)),(3,C(6,2))]{m[i%n]=m[i%n].add(c);}m}
fn evaluate(a:&[C],r:i64,w:i64)->i64 {a.iter().enumerate().map(|(i,c)|(c.0+c.1*w)*if i%2==0{1}else{r}).sum::<i64>().rem_euclid(7)}
fn main(){let out=env::args().nth(1).expect("fresh output dir");let out=Path::new(&out);assert!(!out.exists());fs::create_dir_all(out).unwrap();
 let odds=[3,5,7,11,13,17,19,23,31,41,61,83,127,163,233,283,409,571,1021];let evens=[2,4,6,8,10,12,16,82,162,570];let mut rows=Vec::new();
 for n in odds {let mut one=vec![C::default();n];one[0]=C(1,0);let m=family(n);
  let mut s=vec![C::default();n];for k in 0..n{s[(3*k)%n]=s[(3*k)%n].add(C(if k%2==0{1}else{-1},0));}
  let mut f=one.clone();f[3%n]=f[3%n].add(C(1,0));let mut two=one.clone();two[0]=C(2,0);assert_eq!(product(&f,&s),two);
  let a:Vec<_>=s.iter().map(|c|c.scale(12).red()).collect();let mut b=one.clone();b[3%n]=b[3%n].add(C(2,0));let aab=product(&product(&a,&a),&b);
  let inv:Vec<_>=a.iter().zip(&aab).map(|(x,y)|x.add(C(-3,1).mul(*y).neg()).red()).collect();let p=product(&inv,&m);
  let residual:Vec<_>=one.iter().zip(&p).map(|(x,y)|x.add(y.neg())).collect();assert!(residual.iter().all(|c|c.0%7==0&&c.1%7==0));let other:Vec<_>=residual.iter().map(|c|C(c.0/7,c.1/7)).collect();let seven:Vec<_>=other.iter().map(|c|c.scale(7)).collect();assert_eq!(plus(&product(&m,&inv),&seven),one);
  let mut corrupt=inv.clone();corrupt[0]=corrupt[0].add(C(1,0));assert_ne!(plus(&product(&corrupt,&m),&seven),one);
  // Verify the actual augmentation lattice on every e_j-e_last, both actions.
  for j in 0..n-1 {let mut v=vec![C::default();n];v[j]=C(1,0);v[n-1]=C(-1,0);let mv=action(&m,&v);let im=action(&inv,&mv);let bv:Vec<_>=action(&other,&v).iter().map(|c|c.scale(7)).collect();assert_eq!(plus(&im,&bv),v);}
  let max=inv.iter().chain(&other).flat_map(|c|[c.0.abs(),c.1.abs()]).max().unwrap();let nz=inv.iter().filter(|c|**c!=C::default()).count()+other.iter().filter(|c|**c!=C::default()).count();
  let rec=format!("{{\"n\":{n},\"augmentation_rank\":{},\"least_scalar\":1,\"B1\":{},\"B3\":{},\"max_abs_CM_coordinate\":{max},\"nonzero_coefficients\":{nz},\"augmentation_basis_checks\":{},\"corruption_rejected\":true}}",n-1,coeff(&inv),coeff(&other),n-1);fs::write(out.join(format!("odd-{n}.json")),&rec).unwrap();rows.push(format!("{{\"n\":{n},\"rank\":{},\"d\":1,\"max_abs\":{max},\"nonzero_coefficients\":{nz}}}",n-1));println!("SCALE30_ODD n={n} rank={} least=1 max_abs={max} basis_checks={}",n-1,n-1);
 }
 for n in evens {let m=family(n);assert_eq!(evaluate(&m,-1,3),0);let augmentation=(0..n).map(|i|if i%2==0{1}else{-1}).sum::<i64>();assert_eq!(augmentation,0);assert_eq!(C(-11,-1).0+3*C(-11,-1).1,-14);rows.push(format!("{{\"n\":{n},\"rank\":{},\"d\":7,\"evaluation_r\":-1,\"evaluation_w\":3,\"modulus\":7}}",n-1));println!("SCALE30_EVEN n={n} rank={} least=7 evaluation_obstruction_pass",n-1);}
 fs::write(out.join("results.json"),format!("{{\"schema\":\"koblitz.scale30/v1\",\"scope\":\"generalized polynomial family, not new geometric transfers\",\"records\":[{}],\"all_checks_pass\":true}}\n",rows.join(","))).unwrap();println!("SCALE30_ALL_CHECKS_PASS");}
