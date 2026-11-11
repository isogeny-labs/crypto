//! Standalone CT11 regression: rustc --edition=2021 --test frobenius_partition.rs -o ct11_test && ./ct11_test
//! Small-field development assertions, NOT independently signed PCC standing.
const P: i64 = 11;
fn m(x: i64) -> i64 { x.rem_euclid(P) }
fn powmod(mut a:i64,mut n:usize)->i64 {
    let mut r=1;
    while n>0 { if n&1==1 {r=m(r*a);} a=m(a*a); n>>=1; }
    r
}
fn chi(x:i64)->i64 { let x=m(x); if x==0 {0} else if powmod(x,5)==1 {1} else {-1} }
#[derive(Copy,Clone,Eq,PartialEq,Debug)]
struct K([i64;3]);  // F_11[t]/(t^3+5t^2+7t+7).
impl K {
    fn c(x:i64)->Self {Self([m(x),0,0])}
    fn zero()->Self {Self::c(0)}
    fn one()->Self {Self::c(1)}
    fn add(self,r:Self)->Self {Self(std::array::from_fn(|i|m(self.0[i]+r.0[i])))}
    fn neg(self)->Self {Self(std::array::from_fn(|i|m(-self.0[i])))}
    fn sub(self,r:Self)->Self {self.add(r.neg())}
    fn mul(self,r:Self)->Self {
        let mut q=[0;5];
        for i in 0..3 {for j in 0..3 {q[i+j]=m(q[i+j]+self.0[i]*r.0[j]);}}
        // t^3=-7-7t-5t^2.
        for d in (3..=4).rev() {
            let a=q[d];for i in 0..3 {q[d-3+i]=m(q[d-3+i]-a*[7,7,5][i]);}
            q[d]=0;
        }
        Self([q[0],q[1],q[2]])
    }
    fn pow(mut self,mut n:usize)->Self {
        let mut r=Self::one();
        while n>0 {if n&1==1 {r=r.mul(self);} self=self.mul(self);n>>=1;}
        r
    }
}
fn det(a:[[K;3];3])->K {
    a[0][0].mul(a[1][1].mul(a[2][2]).sub(a[1][2].mul(a[2][1])))
    .sub(a[0][1].mul(a[1][0].mul(a[2][2]).sub(a[1][2].mul(a[2][0]))))
    .add(a[0][2].mul(a[1][0].mul(a[2][1]).sub(a[1][1].mul(a[2][0]))))
}
fn conj_roots()->[K;3] {
    let t=K([0,1,0]);
    [t,t.pow(11),t.pow(121)]
}
fn rows(k:usize)->[[K;3];3] {
    let b=conj_roots();std::array::from_fn(|i| {
        let j=(i+k)%3;[K::one(),b[j].sub(b[i]),b[i].mul(b[j]).neg()]
    })
}
fn poly_mul(a:&[K],b:&[K])->Vec<K> {
    let mut out=vec![K::zero();a.len()+b.len()-1];
    for (i,v) in a.iter().enumerate() {
        for (j,w) in b.iter().enumerate() {out[i+j]=out[i+j].add(v.mul(*w));}
    }
    out
}
fn bracket(a:[K;3],b:[K;3])->[K;3] {
    // a and b are coefficients in descending powers: x^2 + bx + c.
    [a[1].mul(b[2]).sub(a[2].mul(b[1])),
     K::c(2).mul(b[2].sub(a[2])),
     b[1].sub(a[1])]
}
fn richelot_raw(k:usize)->Vec<i64> {
    let r=rows(k);let d=det(r);
    assert_eq!(&d.0[1..], &[0,0]);
    assert_ne!(d.0[0],0);
    let h: Vec<[K;3]>=(0..3).map(|i|bracket(r[(i+1)%3],r[(i+2)%3])).collect();
    let mut prod=vec![K::one()];
    for g in h {prod=poly_mul(&prod,&g);}
    let inv=powmod(d.0[0],9); // inverse in F11: x^(11-2)
    prod.into_iter().map(|c| {assert_eq!(&c.0[1..], &[0,0]);m(c.0[0]*inv)}).collect()
}
fn div_monic(mut a:Vec<i64>,b:&[i64])->bool {
    if a.len()<b.len() {return false;}
    for i in (0..=a.len()-b.len()).rev() {
        let c=a[i+b.len()-1];
        for j in 0..b.len() {a[i+j]=m(a[i+j]-c*b[j]);}
    }
    a.into_iter().all(|x|x==0)
}
fn has_factor(f:&[i64],degree:usize)->bool {
    for code in 0..11_usize.pow(degree as u32) {
        let mut k=code;let mut g=vec![0;degree+1];
        for x in g.iter_mut().take(degree) {*x=(k%11) as i64;k/=11;}
        g[degree]=1;
        if div_monic(f.to_vec(),&g) {return true;}
    }
    false
}
fn eval(f:&[i64],x:i64)->i64 {f.iter().rev().fold(0,|a,&b|m(a*x+b))}
fn count_e(a:i64,b:i64)->i64 {
    1+(0..P).map(|x|(0..P).filter(|&y|m(y*y)==m(x*x*x+a*x+b)).count() as i64).sum::<i64>()
}
fn count_c_fp(f:&[i64])->i64 {
    P+1+chi(*f.last().unwrap())+(0..P).map(|x|chi(eval(f,x))).sum::<i64>()
}
#[derive(Copy,Clone)]
struct K2(i64,i64); // F121=F11[s], s²=2.
impl K2 {
    fn add(self,y:Self)->Self{Self(m(self.0+y.0),m(self.1+y.1))}
    fn mul(self,y:Self)->Self{Self(m(self.0*y.0+2*self.1*y.1),m(self.0*y.1+self.1*y.0))}
    fn chi(self)->i64{chi(m(self.0*self.0-2*self.1*self.1))}
}
fn eval2(f:&[i64],x:K2)->K2 {
    f.iter().rev().fold(K2(0,0),|v,&a|v.mul(x).add(K2(a,0)))
}
fn count_c_fp2(f:&[i64])->i64 {
    P*P+1+K2(*f.last().unwrap(),0).chi()
    +(0..P).flat_map(|a|(0..P).map(move|b|K2(a,b)))
        .map(|x|eval2(f,x).chi()).sum::<i64>()
}
fn jcard(f:&[i64])->(i64,i64,i64) {
    let c1=count_c_fp(f);let c2=count_c_fp2(f);
    let a1=c1-P-1;
    let a2=(c2-P*P-1+a1*a1)/2;
    (c1,c2,P*P+1+(P+1)*a1+a2)
}
#[test]
fn prime_order_elliptic_inputs_and_sextic_irreducibility() {
    assert_eq!(count_e(1,6),13);
    assert_eq!(count_e(2,4),17);
    assert_eq!(count_e(0,0),12); // singular control only; not an elliptic-curve claim
    let f_plus=[6,0,1,0,0,0,1];
    let f_minus=[4,0,2,0,0,0,1];
    assert_eq!(f_plus.iter().len(),7);
    for d in [1,2] {assert!(!has_factor(&f_plus,d));}
    for d in [1,2,3] {assert!(!has_factor(&f_minus,d));}
    let m1=[7,7,5,1];
    let m2=[4,7,6,1];
    let mut prod=vec![0;7];
    for (i,&a) in m1.iter().enumerate(){
        for (j,&b) in m2.iter().enumerate(){prod[i+j]=m(prod[i+j]+a*b);}
    }
    assert_eq!(prod,f_plus);
    assert_eq!(powmod(m(0-6),5),1); // Norm(alpha)=-b square.
    assert_eq!(powmod(m(0-4),5),10); // Norm(alpha)=-b nonsquare.
}
#[test]
fn frobenius_stable_nonrational_quadratic_partitions() {
    let betas=conj_roots();
    assert_ne!(betas[0],betas[1]);assert_ne!(betas[1],betas[2]);
    assert_eq!(betas[2].pow(11),betas[0]);
    for k in 0..3 {
        let r=rows(k);
        let mut prod=vec![K::one()];
        for g in &r {prod=poly_mul(&prod,&[g[2],g[1],g[0]]);}
        let expected=[6,0,1,0,0,0,1].map(K::c);
        assert_eq!(prod,expected);
        for i in 0..3 {
            assert_eq!(r[i][1].pow(11),r[(i+1)%3][1]);
            assert_eq!(r[i][2].pow(11),r[(i+1)%3][2]);
        }
    }
    assert_eq!(det(rows(0)),K::zero()); // opposite pairing: split endpoint
    assert_eq!(det(rows(1)),K::c(5)); // B*C-9D=5
    assert_eq!(det(rows(2)),K::c(6));
    assert_eq!(richelot_raw(1),vec![6,6,6,6,6,6,5]);
    assert_eq!(richelot_raw(2),vec![6,5,6,5,6,5,5]);
}
#[test]
fn sign_is_a_required_zeta_control_not_an_isogeny_certificate() {
    let src=[6,0,1,0,0,0,1];
    let raw1=richelot_raw(1);let raw2=richelot_raw(2);
    assert_eq!(jcard(&src),(10,156,117));
    assert_eq!(jcard(&raw1),(14,156,165));
    assert_eq!(jcard(&raw2),(14,156,165));
    let corr1:Vec<i64>=raw1.iter().map(|&v|m(-v)).collect();
    let corr2:Vec<i64>=raw2.iter().map(|&v|m(-v)).collect();
    assert_eq!(jcard(&corr1),(10,156,117));
    assert_eq!(jcard(&corr2),(10,156,117));
    let e2=1+(0..P).map(|x|(0..P).filter(|&y|m(y*y)==m(6*x*x*x+x*x+1)).count() as i64).sum::<i64>();
    assert_eq!(e2,9); assert_eq!(count_e(1,6)*e2,117); // second elliptic quotient
}
