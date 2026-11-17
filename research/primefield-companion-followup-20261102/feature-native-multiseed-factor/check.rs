//! Separate checker: frozen scalar source, no new consumer/transport import.
#[path="../feature-native-quartic-recovery/routed/source.rs"]mod frozen;
fn mul(a:u64,b:u64,p:u64)->u64{((a as u128*b as u128)%p as u128)as u64}
fn pow(mut a:u64,mut n:u64,p:u64)->u64{let mut z=1;while n>0{if n&1!=0{z=mul(z,a,p);}a=mul(a,a,p);n>>=1;}z}
type Point=Option<[u64;2]>;
fn add(p:u64,a:u64,x:Point,y:Point)->Point{let(Some([x1,y1]),Some([x2,y2]))=(x,y)else{return x.or(y)};if x1==x2&&(y1+y2)%p==0{return None;}let(n,d)=if x1==x2{((3*mul(x1,x1,p)+a)%p,2*y1%p)}else{((y2+p-y1)%p,(x2+p-x1)%p)};let m=mul(n,pow(d,p-2,p),p);let xx=(mul(m,m,p)+2*p-x1-x2)%p;Some([xx,(mul(m,(x1+p-xx)%p,p)+p-y1)%p])}
fn main(){let text=std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();let mut lines=text.lines();assert_eq!(lines.next(),Some("p\tr\tscalar\tx\ty\tsign\tnumerator\tdenominator"));let mut checked=0;
 for(p,a,b,r,base)in[(5,2,1,7,[0,1]),(7,0,3,13,[1,2]),(509,2,10,479,[3,68])]{let source=frozen::Source::new(p,a,b,r,base).unwrap();let mut q:Point=None;
 for k in 0..r{let f:Vec<i64>=lines.next().unwrap().split('\t').map(|x|x.parse().unwrap()).collect();assert_eq!(f.len(),8);assert_eq!(&f[..3],&[p as i64,r as i64,k as i64]);let xy=q.map(|[x,y]|[x as i64,y as i64]).unwrap_or([-1,-1]);assert_eq!(&f[3..5],&xy);let sign=if k==0{0}else if pow(k,(r-1)/2,r)==1{1}else{-1};assert_eq!(f[5],sign);assert!(f[6]>=0&&f[6]<p as i64&&f[7]>0&&f[7]<p as i64);
 if let Some(point)=q{let raw=source.raw(point).unwrap();assert_eq!(mul(f[6]as u64,raw.ratio[1],p),mul(f[7]as u64,raw.ratio[0],p));assert_eq!(source.sign(q).unwrap()as i64,sign);}else{assert_eq!(f[6],0);}q=add(p,a,q,Some(base));checked+=1;}
 assert_eq!(q,None);
 }assert!(lines.next().is_none());assert_eq!(checked,499);println!("checked_rows={checked} point_order=affine raw_reference=frozen_scalar_source");}
