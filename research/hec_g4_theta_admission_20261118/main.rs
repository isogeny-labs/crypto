type E=[u64;9];type P=Vec<E>;
const Z:E=[0;9];
fn scalar(a:u64)->E{let mut v=Z;v[0]=a;v}
fn trim(mut a:P)->P{while a.last()==Some(&Z){a.pop();}a}
#[derive(Clone)]struct F{p:u64,n:usize,m:Vec<u64>,q:u64}
impl F{
 fn new(p:u64,m:Vec<u64>)->Self{let n=m.len()-1;assert!(n<=9&&m[n]==1);Self{p,n,m,q:p.pow(n as u32)}}
 fn add(&self,a:E,b:E)->E{let mut c=Z;for i in 0..self.n{c[i]=(a[i]+b[i])%self.p}c}
 fn sub(&self,a:E,b:E)->E{let mut c=Z;for i in 0..self.n{c[i]=(a[i]+self.p-b[i])%self.p}c}
 fn mul(&self,a:E,b:E)->E{let mut c=[0u64;17];for i in 0..self.n{for j in 0..self.n{c[i+j]=(c[i+j]+a[i]*b[j])%self.p}}for i in(self.n..2*self.n-1).rev(){let v=c[i];for j in 0..self.n{c[i-self.n+j]=(c[i-self.n+j]+self.p-(v*self.m[j])%self.p)%self.p}c[i]=0}let mut out=Z;out[..self.n].copy_from_slice(&c[..self.n]);out}
 fn pow(&self,mut a:E,mut n:u64)->E{let mut b=scalar(1);while n>0{if n&1!=0{b=self.mul(b,a)}a=self.mul(a,a);n>>=1}b}
 fn inv(&self,a:E)->E{assert_ne!(a,Z);if a==scalar(1){a}else{let b=self.pow(a,self.q-2);assert_eq!(self.mul(a,b),scalar(1));b}}
 fn pa(&self,a:&P,b:&P)->P{let mut c=vec![Z;a.len().max(b.len())];for(i,x)in c.iter_mut().enumerate(){*x=self.add(*a.get(i).unwrap_or(&Z),*b.get(i).unwrap_or(&Z))}trim(c)}
 fn ps(&self,a:&P,b:&P)->P{let mut c=vec![Z;a.len().max(b.len())];for(i,x)in c.iter_mut().enumerate(){*x=self.sub(*a.get(i).unwrap_or(&Z),*b.get(i).unwrap_or(&Z))}trim(c)}
 fn pm(&self,a:&P,b:&P)->P{if a.is_empty()||b.is_empty(){return vec![]}let mut c=vec![Z;a.len()+b.len()-1];for(i,&v)in a.iter().enumerate(){for(j,&w)in b.iter().enumerate(){c[i+j]=self.add(c[i+j],self.mul(v,w))}}trim(c)}
 fn div(&self,a:&P,b:&P)->(P,P){assert!(!b.is_empty());if a.len()<b.len(){return(vec![],a.clone())}let mut r=a.clone();let mut q=vec![Z;a.len()-b.len()+1];let iv=self.inv(*b.last().unwrap());while !r.is_empty()&&r.len()>=b.len(){let j=r.len()-b.len();let v=self.mul(*r.last().unwrap(),iv);q[j]=v;for(k,&w)in b.iter().enumerate(){r[j+k]=self.sub(r[j+k],self.mul(v,w))}r=trim(r)}(trim(q),r)}
 fn rem(&self,a:&P,b:&P)->P{self.div(a,b).1}
 fn monic(&self,a:&P)->P{if a.is_empty(){return a.clone()}let v=self.inv(*a.last().unwrap());a.iter().map(|&x|self.mul(x,v)).collect()}
 fn gcd(&self,mut a:P,mut b:P)->P{while !b.is_empty(){let r=self.rem(&a,&b);a=b;b=r}self.monic(&a)}
 fn ppow(&self,mut a:P,mut n:u64,m:&P)->P{let mut b=vec![scalar(1)];while n>0{if n&1!=0{b=self.rem(&self.pm(&b,&a),m)}a=self.rem(&self.pm(&a,&a),m);n>>=1}b}
 fn eval(&self,a:&P,x:E)->E{a.iter().rev().fold(Z,|s,&v|self.add(self.mul(s,x),v))}
 fn elements(&self,i:u64)->E{let mut r=Z;let mut i=i;for v in &mut r[..self.n]{*v=i%self.p;i/=self.p}assert_eq!(i,0);r}
}
fn poly(v:&[u64])->P{trim(v.iter().copied().map(scalar).collect())}
fn irreducible(p:u64,a:&[u64]){let f=F::new(p,vec![0,1]);let m=poly(a);let n=a.len()-1;let x=poly(&[0,1]);let mut y=x.clone();for i in 1..=n{y=f.ppow(y,p,&m);if i<n&&n%i==0{assert_eq!(f.gcd(m.clone(),f.ps(&y,&x)).len(),1)}}assert!(f.ps(&y,&x).is_empty())}
struct R(u64);impl R{fn next(&mut self,n:u64)->u64{self.0=self.0.wrapping_add(0x9e3779b97f4a7c15);let mut z=self.0;z=(z^(z>>30)).wrapping_mul(0xbf58476d1ce4e5b9);z=(z^(z>>27)).wrapping_mul(0x94d049bb133111eb);(z^(z>>31))%n}}
fn split_roots(f:&F,a:P,rng:&mut R,attempts:&mut usize)->Vec<E>{if a.len()==2{let x=f.mul(f.sub(Z,a[0]),f.inv(a[1]));assert_eq!(f.eval(&a,x),Z);return vec![x]}
 for _ in 0..128{*attempts+=1;let h: P=(0..a.len()-1).map(|_|f.elements(rng.next(f.q))).collect();let h=f.ps(&f.ppow(h,(f.q-1)/2,&a),&poly(&[1]));let b=f.gcd(a.clone(),h);if b.len()>1&&b.len()<a.len(){let(q,r)=f.div(&a,&b);assert!(r.is_empty());let mut v=split_roots(f,b,rng,attempts);v.extend(split_roots(f,q,rng,attempts));return v}}
 panic!("root split bound");}
fn orbit(f:&F,a:E,n:usize)->Vec<E>{let mut v=vec![];let mut b=a;for _ in 0..n{assert!(!v.contains(&b));v.push(b);b=f.pow(b,f.p)}assert_eq!(b,a);let first=*v.iter().min().unwrap();let mut out=vec![first];for i in 1..n{out.push(f.pow(out[i-1],f.p))}out}
fn support(a:u8)->u16{let s=a as u16;if s.count_ones()>4{s^511}else{s}}
fn divisor(f:&F,roots:&[E],mask:u16)->P{let mut out=poly(&[1]);for(i,&r)in roots.iter().enumerate(){if mask&(1<<i)!=0{out=f.pm(&out,&vec![f.sub(Z,r),scalar(1)])}}out}
fn action(a:u8,perm:&[usize])->u8{let mut full=a as u16;if a.count_ones()%2!=0{full|=1<<9}let mut out=0;for(i,&j)in perm.iter().enumerate(){if full&(1<<i)!=0{out|=1<<j}}if out&(1<<8)!=0{out^=1023}out as u8}
fn thomae(f:&F,roots:&[E])->(E,E,usize){assert_eq!(roots.len(),9);let mut s=Z;let mut t=Z;let mut count=0;let mut weights=vec![];for a in 0..512u16{if a.count_ones()!=4{continue}let mut v=scalar(1);for i in 0..9{for j in i+1..9{if(a>>i)&1==(a>>j)&1{v=f.mul(v,f.sub(roots[i],roots[j]))}}}assert_ne!(v,Z);let v2=f.mul(v,v);weights.push(v2);s=f.add(s,v2);t=f.add(t,f.mul(v2,v2));count+=1}assert_eq!(count,126);assert_eq!(f.sub(f.mul(s,s),f.mul(scalar(16),t)),Z);let rejected=weights.iter().filter(|&&w|{let sn=f.add(s,scalar(1));let tn=f.add(t,f.add(f.add(w,w),scalar(1)));f.sub(f.mul(sn,sn),f.mul(scalar(16),tn))!=Z}).count();assert!(rejected>0);println!("MUTATIONS p={} field_degree={} attempted=126 rejected={rejected} invalid_theta_tuples_not_ppav_classification=true",f.p,f.n);(s,t,count)}
fn curve_count(f:&F,cs:&[u64])->u64{let pp=poly(cs);let mut n=1;for i in 0..f.q{let z=f.eval(&pp,f.elements(i));if z==Z{n+=1}else{let l=f.pow(z,(f.q-1)/2);assert!(l==scalar(1)||l==scalar(f.p-1));if l==scalar(1){n+=2}}}n}
fn main(){
 let mut rng=R(2026112800);let base=F::new(31,vec![0,1]);let coeff=[1,16,0,1,0,0,0,0,0,1];let ff=poly(&coeff);let r=(0..31).find(|&x|base.eval(&ff,scalar(x))==Z).unwrap();let(eight,rem)=base.div(&ff,&poly(&[(31-r)%31,1]));assert!(rem.is_empty());let mut factor=None;let mut factor_attempts=0;
 for _ in 0..128{factor_attempts+=1;let a=(0..8).map(|_|scalar(rng.next(31))).collect();let h=base.ps(&base.ppow(a,(31u64.pow(4)-1)/2,&eight),&poly(&[1]));let d=base.gcd(eight.clone(),h);if d.len()==5{factor=Some(d);break}}
 let first=factor.expect("quartic split bound");let(second,rem)=base.div(&eight,&first);assert!(rem.is_empty());let first_cs=first.iter().map(|a|a[0]).collect::<Vec<_>>();let second_cs=second.iter().map(|a|a[0]).collect::<Vec<_>>();irreducible(31,&first_cs);irreducible(31,&second_cs);
 let f=F::new(31,first_cs.clone());let mut alpha=Z;alpha[1]=1;let mut root_attempts=0;let other=split_roots(&f,second.clone(),&mut rng,&mut root_attempts);let mut roots=vec![scalar(r)];roots.extend(orbit(&f,alpha,4));roots.extend(orbit(&f,other[0],4));assert_eq!(roots.len(),9);
 for i in 0..9{assert_eq!(f.eval(&ff,roots[i]),Z);for j in 0..i{assert_ne!(roots[i],roots[j])}}assert_eq!(divisor(&f,&roots,511),ff);
 println!("FIELD p=31 modulus={first_cs:?} other_quartic={second_cs:?} factor_attempts={factor_attempts} root_split_attempts={root_attempts} roots={roots:?}");
 let perm=[0,2,3,4,1,6,7,8,5,9];for i in 0..9{assert_eq!(f.pow(roots[i],31),roots[perm[i]])}
 let divs:Vec<_>=(0..256u16).map(|a|divisor(&f,&roots,support(a as u8))).collect();for a in 0..256u16{let raised=divs[a as usize].iter().map(|&v|f.pow(v,31)).collect::<P>();assert_eq!(raised,divs[action(a as u8,&perm)as usize]);}
 let old=include_str!("../hec_residual_span_20261116/native-two-kernel/CONTROL.txt");let(mut kernels,mut pairs,mut identities)=(0,0,0);
 for line in old.lines().filter(|l|l.starts_with("KERNEL genus=4 p=31 ")){let entries=line.split("abstract_vectors=[").nth(1).unwrap().trim_end_matches(']');let v=entries.split(',').map(|s|s.trim().parse::<u8>().unwrap()).collect::<Vec<_>>();assert_eq!(v.len(),16);assert!(v.iter().all(|&a|v.contains(&action(a,&perm))));let fixed=v.iter().filter(|&&a|action(a,&perm)==a).count();let degree=[1,2,4].into_iter().find(|&d|v.iter().all(|&a|{let mut b=a;for _ in 0..d{b=action(b,&perm)}b==a})).unwrap();
 for &a in &v{let point=&divs[a as usize];for coefficient in point{assert_eq!(f.pow(*coefficient,31u64.pow(degree)),*coefficient)}for &b in &v{let ma=support(a);let mb=support(b);let common=divisor(&f,&roots,ma&mb);let lhs=f.pm(&divs[a as usize],&divs[b as usize]);let common2=f.pm(&common,&common);let result=&divs[(a^b)as usize];if(ma^mb).count_ones()>4{assert_eq!(f.pm(&lhs,result),f.pm(&common2,&ff));}else{assert_eq!(lhs,f.pm(&common2,result));}identities+=1}println!("POINT kernel={kernels} abstract={a} u={point:?} v=0");}
 let pair=line.contains("pair_partition=true");pairs+=pair as usize;println!("KERNEL id={kernels} pair={pair} fixed_points={fixed} full_point_field_degree={degree} abstract={v:?}");kernels+=1;
 }assert_eq!((kernels,pairs,identities),(11,5,2816));let(s,t,n)=thomae(&f,&roots);let shifted=roots.iter().map(|&x|f.add(f.mul(scalar(2),x),scalar(3))).collect::<Vec<_>>();let(ss,tt,_)=thomae(&f,&shifted);assert_eq!(ss,f.mul(f.pow(scalar(2),32),s));assert_eq!(tt,f.mul(f.pow(scalar(2),64),t));println!("THOMAE p=31 nonzero={n} even_zero=10 sum8={s:?} sum16={t:?} schottky=0 affine_covariance=true");
 let cs61=[1,2,0,1,0,0,0,0,0,1];irreducible(61,&cs61);let f61=F::new(61,cs61.to_vec());let roots61=orbit(&f61,alpha,9);assert_eq!(divisor(&f61,&roots61,511),poly(&cs61));let(s,t,n)=thomae(&f61,&roots61);println!("THOMAE p=61 field_degree=9 nonzero={n} even_zero=10 sum8={s:?} sum16={t:?} schottky=0 roots={roots61:?}");
 let n1=curve_count(&base,&coeff);let mut cubic=None;for a in 0..31{for b in 1..31{let pp=poly(&[b,a,0,1]);if(0..31).all(|x|base.eval(&pp,scalar(x))!=Z){cubic=Some(vec![b,a,0,1]);break}}if cubic.is_some(){break}}let cubic=cubic.unwrap();irreducible(31,&cubic);let f3=F::new(31,cubic.clone());let n3=curve_count(&f3,&coeff);let s1=32i64-n1 as i64;let s3=31i64.pow(3)+1-n3 as i64;println!("TRACE p=31 points1={n1} trace1={s1} field3_modulus={cubic:?} points3={n3} trace3={s3} geometric_polarization_indecomposable_by_simple_odd_trace={}",s1!=0||s3!=0);
 let b61=F::new(61,vec![0,1]);let n61=curve_count(&b61,&cs61);println!("TRACE p=61 points1={n61} trace1={} geometric_polarization_indecomposable_by_simple_odd_trace={}",62i64-n61 as i64,n61!=62);
 println!("PASS exact fields, labelled original kernels, torsion polynomial identities, source Schottky controls; quotient Schottky not computed");
}
