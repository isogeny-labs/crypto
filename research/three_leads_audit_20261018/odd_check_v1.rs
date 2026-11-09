// Independent finite-field and coverage checks; no timing claims.
use std::collections::HashSet;
struct Field { p:u32,n:usize,size:u32,tail:u32,exp:Vec<u32>,log:Vec<u32> }
impl Field {
    fn add(&self,mut a:u32,mut b:u32)->u32{
        let(mut r,mut k)=(0,1);for _ in 0..self.n{r+=((a%self.p+b%self.p)%self.p)*k;a/=self.p;b/=self.p;k*=self.p;}r
    }
    fn neg(&self,mut a:u32)->u32{
        let(mut r,mut k)=(0,1);for _ in 0..self.n{r+=((self.p-a%self.p)%self.p)*k;a/=self.p;k*=self.p;}r
    }
    fn sub(&self,a:u32,b:u32)->u32{self.add(a,self.neg(b))}
    fn rawmul(&self,mut a:u32,mut b:u32)->u32{
        let(mut aa,mut bb,mut c)=([0i64;5],[0i64;5],[0i64;10]);
        for i in 0..self.n{aa[i]=(a%self.p)as i64;bb[i]=(b%self.p)as i64;a/=self.p;b/=self.p;}
        for i in 0..self.n{for j in 0..self.n{c[i+j]+=aa[i]*bb[j];}}
        let mut t=self.tail;let mut co=[0i64;5];for x in co.iter_mut().take(self.n){*x=(t%self.p)as i64;t/=self.p;}
        for i in (self.n..2*self.n-1).rev(){let v=c[i].rem_euclid(self.p as i64);for j in 0..self.n{c[i-self.n+j]-=v*co[j];}}
        let(mut r,mut k)=(0,1);for &v in c.iter().take(self.n){r+=v.rem_euclid(self.p as i64)as u32*k;k*=self.p;}r
    }
    fn rawpow(&self,mut a:u32,mut e:u32)->u32{let mut r=1;while e>0{if e&1!=0{r=self.rawmul(r,a);}a=self.rawmul(a,a);e>>=1;}r}
    fn mul(&self,a:u32,b:u32)->u32{if a==0||b==0{0}else{self.exp[((self.log[a as usize]+self.log[b as usize])%(self.size-1))as usize]}}
    fn pow(&self,a:u32,e:u32)->u32{if e==0{1}else if a==0{0}else{self.exp[((self.log[a as usize]as u64*e as u64)%(self.size-1)as u64)as usize]}}
    fn inv(&self,a:u32)->u32{assert_ne!(a,0);self.pow(a,self.size-2)}
    fn new(p:u32,n:usize)->Self{
        let size=p.pow(n as u32);let mut f=Self{p,n,size,tail:0,exp:Vec::new(),log:Vec::new()};
        // A monic polynomial is irreducible iff it has no monic divisor
        // of degree <= n/2. This tiny-degree independent test is exact.
        'candidate:for tail in 0..size{
            if n>1&&tail%p==0{continue;}
            let mut coeff=[0u32;6];let mut t=tail;for v in coeff.iter_mut().take(n){*v=t%p;t/=p;}coeff[n]=1;
            for d in 1..=n/2{for v in 0..p.pow(d as u32){
                let mut divisor=[0u32;6];let mut v0=v;for x in divisor.iter_mut().take(d){*x=v0%p;v0/=p;}divisor[d]=1;
                let mut rr=coeff;for i in(d..=n).rev(){let c=rr[i];for j in 0..=d{rr[i-d+j]=(rr[i-d+j]+p-(c*divisor[j])%p)%p;}}
                if rr[..d].iter().all(|&c|c==0){continue 'candidate;}
            }}f.tail=tail;break;
        }
        assert!(n==1||f.tail!=0);
        let mut primes=Vec::new();let mut k=size-1;let mut q=2;while q*q<=k{if k%q==0{primes.push(q);while k%q==0{k/=q;}}q+=1;}if k>1{primes.push(k);}
        let g=(1..size).find(|&g|primes.iter().all(|&q|f.rawpow(g,(size-1)/q)!=1)).unwrap();
        f.log=vec![u32::MAX;size as usize];let mut x=1;
        for i in 0..size-1{assert_eq!(f.log[x as usize],u32::MAX);f.log[x as usize]=i;f.exp.push(x);x=f.rawmul(x,g);}assert_eq!(x,1);
        for a in 1..size{assert_eq!(f.mul(a,f.inv(a)),1);assert_eq!(f.mul(a,a),f.rawmul(a,a));}
        f
    }
}
type P=Option<(u32,u32)>;
struct Curve<'a>{f:&'a Field,a:u32,b:u32}
impl Curve<'_>{
    fn valid(&self,p:P)->bool{match p{None=>true,Some((x,y))=>self.f.mul(y,y)==self.f.add(self.f.add(self.f.mul(self.f.mul(x,x),x),self.f.mul(self.a,x)),self.b)}}
    fn add(&self,p:P,q:P)->P{
        let(x,y)=match p{None=>return q,Some(p)=>p};let(u,v)=match q{None=>return p,Some(q)=>q};let f=self.f;
        if x==u&&f.add(y,v)==0{return None;}
        let l=if x==u{f.mul(f.add(f.mul(3,f.mul(x,x)),self.a),f.inv(f.mul(2,y)))}else{f.mul(f.sub(y,v),f.inv(f.sub(x,u)))};
        let z=f.sub(f.sub(f.mul(l,l),x),u);Some((z,f.sub(f.mul(l,f.sub(x,z)),y)))
    }
    fn points(&self)->Vec<P>{
        let f=self.f;let mut roots=vec![None;f.size as usize];for y in 0..f.size{roots[f.mul(y,y)as usize]=Some(y);}
        let mut r=vec![None];for x in 0..f.size{let y2=f.add(f.add(f.mul(f.mul(x,x),x),f.mul(self.a,x)),self.b);if let Some(y)=roots[y2 as usize]{r.push(Some((x,y)));if y!=0{r.push(Some((x,f.neg(y))));}}}r
    }
    fn sumset(&self,base:&[P],m:usize)->HashSet<P>{
        let mut s=HashSet::from([None]);for _ in 0..m{let mut out=HashSet::new();for &p in &s{for &q in base{let t=self.add(p,q);assert!(self.valid(t));out.insert(t);}}s=out;}s
    }
}
fn next(seed:&mut u64)->u64{*seed=seed.wrapping_add(0x9e3779b97f4a7c15);let mut z=*seed;z=(z^(z>>30)).wrapping_mul(0xbf58476d1ce4e5b9);z=(z^(z>>27)).wrapping_mul(0x94d049bb133111eb);z^(z>>31)}
fn compare(c:&Curve,points:&[P],xs:&[u32],m:usize,kind:&str,index:u32,seed:&mut u64){
    let base:Vec<_>=points.iter().copied().filter(|p|p.is_some_and(|(x,_)|xs.contains(&x))).collect();
    let torsion:Vec<_>=base.iter().copied().filter(|p|p.unwrap().1==0).collect();let pairs=(base.len()-torsion.len())/2;
    let mut pool=Vec::new();let mut seen=HashSet::new();for &p in points{if let Some((x,y))=p{if y!=0&&seen.insert(x){pool.push(x);}}}
    let original=c.sumset(&base,m).len();let mut nulls=Vec::new();
    for _ in 0..32{for i in (1..pool.len()).rev(){let j=next(seed)as usize%(i+1);pool.swap(i,j);}
        let keep:HashSet<_>=pool[..pairs].iter().copied().collect();
        let mut control=torsion.clone();control.extend(points.iter().copied().filter(|p|p.is_some_and(|(x,y)|y!=0&&keep.contains(&x))));
        assert_eq!(base.len(),control.len());nulls.push(c.sumset(&control,m).len());
    }
    let mean=nulls.iter().sum::<usize>()as f64/32.;let sd=(nulls.iter().map(|&x|(x as f64-mean).powi(2)).sum::<f64>()/31.).sqrt();
    let raw=nulls.iter().map(|x|x.to_string()).collect::<Vec<_>>().join(";");
    println!("{kind},{},{},{},{},{},{index},{m},{},{},{original},{:.9},{:.9},{:.9},{raw}",c.f.p,c.f.n,c.f.tail,c.a,c.b,points.len(),base.len(),original as f64/points.len()as f64,mean/points.len()as f64,sd/points.len()as f64);
}
fn main(){
    println!("kind,p,degree,modulus_tail,a,b,index,summands,group_size,fb_size,reachable,coverage,null_mean_coverage,null_sd_coverage,null_reachable_counts");let mut seed=0x20261018;
    for p in [5,7]{for n in 2..=5{let f=Field::new(p,n);for i in 0..3{
        let a=(p+3*i+1)%f.size;let mut b=(2*p+5*i+1)%f.size;while f.add(f.mul(4,f.mul(f.mul(a,a),a)),f.mul(27%p,f.mul(b,b)))==0{b=(b+1)%f.size;}
        let c=Curve{f:&f,a,b};let pts=c.points();for &p in &pts{assert!(c.valid(p));}
        if f.size<=49{let mut direct=HashSet::from([None]);for x in 0..f.size{for y in 0..f.size{if c.valid(Some((x,y))){direct.insert(Some((x,y)));}}}assert_eq!(direct,pts.iter().copied().collect());}
        compare(&c,&pts,&(0..p).collect::<Vec<_>>(),n,"odd_subfield",i,&mut seed);
    }}}
    for p in [257,769,1153]{let f=Field::new(p,1);let c=Curve{f:&f,a:2,b:3};let pts=c.points();for t in 2..=4{
        let d=1 << t;let kummer:Vec<_>=(0..p).filter(|&x|f.pow(x,d)==1).collect();assert_eq!(kummer.len(),d as usize);
        let z=f.exp[((p-1)/d)as usize];let generated:HashSet<_>=(0..d).map(|i|f.pow(z,i)).collect();assert_eq!(generated,kummer.iter().copied().collect());
        compare(&c,&pts,&kummer,2,"prime_kummer",t,&mut seed);
        // D_2(X,1)=X^2-2; composition gives D_(2^t)(X,1).
        let target=2;let dickson:Vec<_>=(0..p).filter(|&x|{let mut y=x;for _ in 0..t{y=f.sub(f.mul(y,y),2);}y==target}).collect();
        for u in 1..p{let x=f.add(u,f.inv(u));let mut y=x;for _ in 0..t{y=f.sub(f.mul(y,y),2);}assert_eq!(y,f.add(f.pow(u,d),f.inv(f.pow(u,d))));}
        compare(&c,&pts,&dickson,2,"prime_dickson_split",t,&mut seed);
    }}
}
