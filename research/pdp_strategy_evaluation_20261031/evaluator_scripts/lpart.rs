// EXPLORATORY toy (not a claim). Tests whether the F_q-points of the GHS Type-1 quartic
// map onto "generic" l-parts of E(K), K=F_{q^3}, q = p prime, p = 3 mod 4.
// E: y^2 = g(x) = (x-a)(x-a^q)(x-b)(x-b^q), a,b in K \ F_p.
// C(F_q) = { (x, y0): x in P^1(F_q), y0^2 = g(x), Norm(y0) = N(x-a) N(x-b) }  (derived in the card).
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct K([u64; 3]);

struct Ctx { p: u64, ca: u64, cb: u64 } // t^3 = ca*t + cb

impl Ctx {
    fn add(&self, x: K, y: K) -> K { K([(x.0[0]+y.0[0])%self.p,(x.0[1]+y.0[1])%self.p,(x.0[2]+y.0[2])%self.p]) }
    fn sub(&self, x: K, y: K) -> K { let p=self.p; K([(x.0[0]+p-y.0[0])%p,(x.0[1]+p-y.0[1])%p,(x.0[2]+p-y.0[2])%p]) }
    fn neg(&self, x: K) -> K { self.sub(K([0,0,0]), x) }
    fn smul(&self, s: u64, x: K) -> K { K([x.0[0]*s%self.p,x.0[1]*s%self.p,x.0[2]*s%self.p]) }
    fn mul(&self, x: K, y: K) -> K {
        let p=self.p;
        let mut c=[0u64;5];
        for i in 0..3 { for j in 0..3 { c[i+j]=(c[i+j]+x.0[i]*y.0[j])%p; } }
        // t^4 = ca t^2 + cb t ; t^3 = ca t + cb
        let c4=c[4]; let c3=c[3];
        let mut r=[c[0],c[1],c[2]];
        r[2]=(r[2]+c4*self.ca)%p; r[1]=(r[1]+c4*self.cb)%p;
        r[1]=(r[1]+c3*self.ca)%p; r[0]=(r[0]+c3*self.cb)%p;
        K(r)
    }
    fn one(&self) -> K { K([1,0,0]) }
    fn pow(&self, x: K, mut e: u64) -> K { let mut r=self.one(); let mut b=x; while e>0 { if e&1==1 { r=self.mul(r,b);} b=self.mul(b,b); e>>=1; } r }
    fn frob(&self, x: K) -> K { self.pow(x, self.p) }
    fn norm(&self, x: K) -> u64 { let f1=self.frob(x); let f2=self.frob(f1); self.mul(self.mul(x,f1),f2).0[0] }
    fn inv(&self, x: K) -> K { let q3=self.p*self.p*self.p; self.pow(x, q3-2) }
    fn fp(&self, a: u64) -> K { K([a%self.p,0,0]) }
}

fn is_sq_mod(p: u64, a: u64) -> bool { if a%p==0 {return true;} let mut r=1u64; let mut b=a%p; let mut e=(p-1)/2; while e>0 { if e&1==1 { r=r*b%p;} b=b*b%p; e>>=1;} r==1 }

struct Rng(u64);
impl Rng { fn next(&mut self)->u64{ self.0^=self.0<<13; self.0^=self.0>>7; self.0^=self.0<<17; self.0 } }

// affine point on y^2 = x^3 + A x^2 + B x + C
#[derive(Clone,Copy,PartialEq,Eq,Debug)]
enum P { O, Aff(K,K) }
struct Cub { a2: K, a4: K, a6: K }
fn cadd(c:&Ctx,e:&Cub,p1:P,p2:P)->P{
    match (p1,p2) {
        (P::O,q)=>q, (q,P::O)=>q,
        (P::Aff(x1,y1),P::Aff(x2,y2))=>{
            let lam;
            if x1==x2 {
                if y1==c.neg(y2) { return P::O; }
                let num=c.add(c.add(c.smul(3,c.mul(x1,x1)), c.smul(2,c.mul(e.a2,x1))), e.a4);
                lam=c.mul(num, c.inv(c.smul(2,y1)));
            } else { lam=c.mul(c.sub(y2,y1), c.inv(c.sub(x2,x1))); }
            let x3=c.sub(c.sub(c.sub(c.mul(lam,lam),e.a2),x1),x2);
            let y3=c.sub(c.mul(lam,c.sub(x1,x3)),y1);
            P::Aff(x3,y3)
        }
    }
}
fn cneg(c:&Ctx,p:P)->P{ match p { P::O=>P::O, P::Aff(x,y)=>P::Aff(x,c.neg(y)) } }
fn cmul(c:&Ctx,e:&Cub,p:P,mut k:u64)->P{ let mut r=P::O; let mut b=p; while k>0 { if k&1==1 { r=cadd(c,e,r,b);} b=cadd(c,e,b,b); k>>=1;} r }
fn factor(mut n:u64)->Vec<(u64,u32)>{ let mut v=vec![]; let mut d=2; while d*d<=n { if n%d==0 { let mut k=0; while n%d==0 { n/=d; k+=1;} v.push((d,k)); } d+=1;} if n>1 { v.push((n,1)); } v }

fn main(){
    let args:Vec<String>=std::env::args().collect();
    let p:u64=args[1].parse().unwrap();
    let ninst:usize=args[2].parse().unwrap();
    let seed:u64=args.get(3).map(|s|s.parse().unwrap()).unwrap_or(1);
    assert!(p%4==3);
    let mut rng=Rng(0x9E3779B97F4A7C15u64 ^ seed.wrapping_mul(0xD1B54A32D192ED03) ^ p);
    // find irreducible cubic t^3 = ca t + cb : no root in F_p
    let (mut ca, mut cb);
    loop { ca=rng.next()%p; cb=rng.next()%p; if cb==0 {continue;}
        let mut ok=true; for t in 0..p { // t^3 - ca t - cb
            if ((t*t%p*t%p + p*2 - ca*t%p - cb)%p)==0 { ok=false; break; } }
        if ok {break;} }
    let c=Ctx{p,ca,cb};
    let q3=p*p*p;
    let sq: Vec<bool>=(0..p).map(|a| is_sq_mod(p,a)).collect();
    let issq_k=|z:K|->i32{ let n=c.norm(z); if n==0 {0} else if sq[n as usize] {1} else {-1} };
    println!("p={} cubic t^3={}t+{}  |K|={}", p,ca,cb,q3);
    let mut done=0;
    while done<ninst {
        // alpha, beta in K \ F_p
        let rk=|rng:&mut Rng|->K{ loop{ let z=K([rng.next()%p,rng.next()%p,rng.next()%p]); if z.0[1]!=0||z.0[2]!=0 {return z;} } };
        let al=rk(&mut rng); let be=rk(&mut rng);
        let al1=c.frob(al); let be1=c.frob(be);
        let su=c.add(al,al1); let pu=c.mul(al,al1); let sv=c.add(be,be1); let pv=c.mul(be,be1);
        // g = (x^2 - su x + pu)(x^2 - sv x + pv)
        let bb=c.neg(c.add(su,sv));
        let cc=c.add(c.add(pu,pv),c.mul(su,sv));
        let dd=c.neg(c.add(c.mul(su,pv),c.mul(sv,pu)));
        let ee=c.mul(pu,pv);
        // squarefree check via discriminant skipped: require al,al1,be,be1 distinct
        if al==be||al==be1||al1==be||al1==be1 {continue;}
        // count N = #E(K) = q^3 + 2 + sum_x chi(g(x))
        let mut tot:i64=(q3 as i64)+2;
        for i in 0..q3 {
            let x=K([i%p,(i/p)%p,i/(p*p)]);
            let g=c.add(c.mul(c.add(c.mul(c.add(c.mul(c.add(x,bb),x),cc),x),dd),x),ee);
            tot+=issq_k(g) as i64;
        }
        let n=tot as u64;
        let fac=factor(n);
        let a2=fac.iter().find(|f|f.0==2).map(|f|f.1).unwrap_or(0);
        // largest prime factor with exponent 1
        let &(l,le)=fac.iter().max_by_key(|f|f.0).unwrap();
        println!("inst {}: N={}={:?}  v2={}", done, n, fac, a2);
        if le!=1 || (l as f64) < (p as f64).powf(1.5) || l==2 { println!("  skip: largest prime factor {} ^{} not suitable", l, le); done+=1; continue; }
        // Weierstrass: Z^2 = F(w), F = -8 w^3 + 4c w^2 + (8e - 2bd) w + d^2 - (4c - b^2) e ; X=-8w, Yw=-8Z
        let a3=c.fp(p-8);
        let two=c.fp(2); let four=c.fp(4); let eight=c.fp(8);
        let f_a2=c.mul(four,cc);
        let f_a1=c.sub(c.mul(eight,ee), c.mul(two,c.mul(bb,dd)));
        let f_a0=c.sub(c.mul(dd,dd), c.mul(c.sub(c.mul(four,cc),c.mul(bb,bb)),ee));
        let e=Cub{ a2:f_a2, a4:c.mul(f_a1,a3), a6:c.mul(f_a0,c.mul(a3,a3)) };
        // S points
        let beta=c.mul(bb,c.inv(two)); // b/2
        let b2q=c.mul(bb,bb);
        let coef=c.sub(c.sub(cc, c.mul(b2q,c.inv(four))),K([0,0,0])); // c - b^2/4
        let mut spts:Vec<P>=vec![];
        for xi in 0..p {
            let x=c.fp(xi);
            let g=c.add(c.mul(c.add(c.mul(c.add(c.mul(c.add(x,bb),x),cc),x),dd),x),ee);
            // sqrt: p=3 mod 4 => p^3 = 3 mod 4 => y=g^((p^3+1)/4)
            let mut y=c.pow(g,(q3+1)/4);
            assert!(c.mul(y,y)==g, "g not square");
            let rx=(c.norm(c.sub(x,al)) * c.norm(c.sub(x,be)))%p;
            let ny=c.norm(y);
            if ny==rx { } else if (p-ny)%p==rx { y=c.neg(y); } else { panic!("norm mismatch"); }
            // map: w = y - x^2 - beta x ; Z = 2(coef - 2w) x + d - b w
            let w=c.sub(c.sub(y,c.mul(x,x)),c.mul(beta,x));
            let zz=c.add(c.mul(c.mul(two,c.sub(coef,c.mul(two,w))),x), c.sub(dd,c.mul(bb,w)));
            let xx=c.mul(a3,w); let yy=c.mul(a3,zz);
            // check on curve
            let lhs=c.mul(yy,yy);
            let rhs=c.add(c.add(c.add(c.mul(c.mul(xx,xx),xx), c.mul(e.a2,c.mul(xx,xx))), c.mul(e.a4,xx)), e.a6);
            assert!(lhs==rhs,"map off curve");
            spts.push(P::Aff(xx,yy));
        }
        // group order sanity: [N] on first point is O
        let chk=cmul(&c,&e,spts[0],n);
        assert!(chk==P::O,"N*P != O (order mismatch)");
        // lambda projection
        let h=n/l;
        let q0=spts[0];
        let mut lam_set:HashSet<(K,K)>=HashSet::new();
        let mut zero=0usize; let mut cnt=0usize;
        for &pt in &spts {
            let t=cadd(&c,&e,pt,cneg(&c,q0));
            let lt=cmul(&c,&e,t,h);
            cnt+=1;
            match lt { P::O=>{zero+=1;}, P::Aff(x,y)=>{lam_set.insert((x,y));} }
        }
        // control: random K-points of E, same count
        let mut ctl:HashSet<(K,K)>=HashSet::new(); let mut ctl_zero=0usize; let mut cc_cnt=0usize;
        while cc_cnt<cnt {
            let x=K([rng.next()%p,rng.next()%p,rng.next()%p]);
            let g=c.add(c.mul(c.add(c.mul(c.add(c.mul(c.add(x,bb),x),cc),x),dd),x),ee);
            if issq_k(g)!=1 {continue;}
            let y=c.pow(g,(q3+1)/4); if c.mul(y,y)!=g {continue;}
            let w=c.sub(c.sub(y,c.mul(x,x)),c.mul(beta,x));
            let zz=c.add(c.mul(c.mul(two,c.sub(coef,c.mul(two,w))),x), c.sub(dd,c.mul(bb,w)));
            let pt=P::Aff(c.mul(a3,w),c.mul(a3,zz));
            let t=cadd(&c,&e,pt,cneg(&c,q0));
            let lt=cmul(&c,&e,t,h); cc_cnt+=1;
            match lt { P::O=>{ctl_zero+=1;}, P::Aff(x,y)=>{ctl.insert((x,y));} }
        }
        println!("  l={} (prime, exp 1, cofactor {}): S has {} finite pts; l-part of (Q-Q0): zero for {} ; distinct nonzero lambda values {} ; control(random K-pts of E): zero {} distinct {}",
            l, n/l, cnt, zero, lam_set.len(), ctl_zero, ctl.len());
        done+=1;
    }
}
