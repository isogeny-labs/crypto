//! CT13 development regression for Galois-stable non-split Richelot kernels.
//! Public fixture E/F17: y²=x³+2x+2, C: y²=x⁶+2x²+2.
//! These are independently replayable necessary arithmetic gates, NOT
//! an implemented Richelot correspondence or signed PCC certification.

const P: i64 = 17;
fn m(x: i64) -> i64 { x.rem_euclid(P) }
fn fpow(mut x: i64, mut n: usize) -> i64 {
    let mut z = 1;
    while n > 0 {
        if n & 1 != 0 { z = m(z*x); }
        x = m(x*x);
        n >>= 1;
    }
    z
}
fn chi(x: i64) -> i64 {
    let a = m(x);
    if a == 0 { 0 } else if fpow(a, 8) == 1 { 1 } else { -1 }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct L([i64; 3]); // F17[t]/(t³+6t²+t+7), irreducible.
impl L {
    fn c(x: i64) -> Self { Self([m(x), 0, 0]) }
    fn zero() -> Self { Self::c(0) }
    fn one() -> Self { Self::c(1) }
    fn plus(self, q: Self) -> Self {
        Self(std::array::from_fn(|i| m(self.0[i]+q.0[i])))
    }
    fn neg(self) -> Self { Self(std::array::from_fn(|i| m(-self.0[i]))) }
    fn minus(self, q: Self) -> Self { self.plus(q.neg()) }
    fn times(self, q: Self) -> Self {
        let mut v = [0_i64; 5];
        for i in 0..3 {
            for j in 0..3 { v[i+j] = m(v[i+j]+self.0[i]*q.0[j]); }
        }
        for d in (3..=4).rev() {
            let a = v[d];
            for (j, coefficient) in [7, 1, 6].iter().enumerate() {
                v[d-3+j] = m(v[d-3+j]-a*(*coefficient));
            }
        }
        Self([v[0],v[1],v[2]])
    }
    fn pow(mut self, mut n: usize) -> Self {
        let mut z = Self::one();
        while n > 0 {
            if n & 1 != 0 { z = z.times(self); }
            self = self.times(self);
            n >>= 1;
        }
        z
    }
}
fn frobenius_roots() -> [L; 3] {
    let t = L([0,1,0]);
    [t, t.pow(17), t.pow(289)]
}
// G_i=(X-beta_i)(X+beta_(i+k)), descending coefficients.
fn rows(k: usize) -> [[L; 3]; 3] {
    let b = frobenius_roots();
    std::array::from_fn(|i| [
        L::one(), b[(i+k)%3].minus(b[i]), b[i].times(b[(i+k)%3]).neg()
    ])
}
fn det3(a: [[L; 3]; 3]) -> L {
    a[0][0].times(a[1][1].times(a[2][2]).minus(a[1][2].times(a[2][1])))
    .minus(a[0][1].times(a[1][0].times(a[2][2]).minus(a[1][2].times(a[2][0]))))
    .plus(a[0][2].times(a[1][0].times(a[2][1]).minus(a[1][1].times(a[2][0]))))
}
fn multiply(a: &[L], b: &[L]) -> Vec<L> {
    let mut out = vec![L::zero(); a.len()+b.len()-1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            out[i+j] = out[i+j].plus(x.times(*y));
        }
    }
    out
}
// For monic quadratics a,b descending: a' b - a b' in ASCENDING order.
fn bracket(a: [L; 3], b: [L; 3]) -> [L; 3] {
    [a[1].times(b[2]).minus(a[2].times(b[1])),
     L::c(2).times(b[2].minus(a[2])),
     b[1].minus(a[1])]
}
fn richelot_raw(k: usize) -> Vec<i64> {
    let r = rows(k);
    let d = det3(r);
    assert_eq!((d.0[1],d.0[2]),(0,0));
    assert_ne!(d.0[0],0);
    let mut h = vec![L::one()];
    for i in 0..3 {
        let b = bracket(r[(i+1)%3],r[(i+2)%3]);
        h = multiply(&h, &b);
    }
    let di = fpow(d.0[0],15);
    h.into_iter().map(|a| {
        assert_eq!((a.0[1],a.0[2]),(0,0));
        m(a.0[0]*di)
    }).collect()
}
fn has_monic_factor(f: &[i64], degree: usize) -> bool {
    assert!(degree > 0 && degree < f.len());
    for code in 0..(P as usize).pow(degree as u32) {
        let mut g=vec![0_i64;degree+1];
        let mut n=code;
        for coeff in g.iter_mut().take(degree) {
            *coeff = (n % P as usize) as i64;
            n /= P as usize;
        }
        g[degree]=1;
        let mut rem=f.to_vec();
        for shift in (0..=f.len()-g.len()).rev() {
            let top=rem[shift+degree];
            for (j,b) in g.iter().enumerate() {
                rem[shift+j] = m(rem[shift+j]-top*b);
            }
        }
        if rem.into_iter().all(|v|v==0) { return true; }
    }
    false
}
fn eval(f: &[i64], x: i64) -> i64 {
    f.iter().rev().fold(0, |z,&a|m(z*x+a))
}
fn count_e(f: &[i64]) -> i64 {
    P+1+(0..P).map(|x|chi(eval(f,x))).sum::<i64>()
}
fn count_c_fp(f: &[i64]) -> i64 {
    P+1+chi(*f.last().unwrap())+(0..P).map(|x|chi(eval(f,x))).sum::<i64>()
}
#[derive(Copy,Clone)]
struct L2(i64,i64); // F_17[s]/(s²-3); 3 is a nonsquare.
impl L2 {
    fn plus(self, q: Self) -> Self {Self(m(self.0+q.0),m(self.1+q.1))}
    fn times(self, q: Self) -> Self {
        Self(m(self.0*q.0+3*self.1*q.1),m(self.0*q.1+self.1*q.0))
    }
    fn chi(self) -> i64 {chi(self.0*self.0-3*self.1*self.1)}
}
fn eval2(f: &[i64], x: L2) -> L2 {
    f.iter().rev().fold(L2(0,0), |z,&a|z.times(x).plus(L2(a,0)))
}
fn count_c_fp2(f: &[i64]) -> i64 {
    P*P+1+L2(*f.last().unwrap(),0).chi()
      +(0..P).flat_map(|a|(0..P).map(move |b|L2(a,b)))
        .map(|x|eval2(f,x).chi()).sum::<i64>()
}
fn card_j(f: &[i64]) -> (i64,i64,i64) {
    let n1=count_c_fp(f);
    let n2=count_c_fp2(f);
    let a1=n1-P-1;
    let a2=(n2-P*P-1+a1*a1)/2;
    (n1,n2,P*P+1+(P+1)*a1+a2)
}
fn matchings(v: &[u8]) -> Vec<Vec<(u8,u8)>> {
    if v.is_empty() {return vec![vec![]];}
    let mut result=Vec::new();
    for j in 1..v.len() {
        let remaining:Vec<_>=v.iter().copied().enumerate()
            .filter_map(|(i,x)|if i==0 || i==j {None} else {Some(x)}).collect();
        for mut rest in matchings(&remaining) {
            rest.push((v[0],v[j]));
            rest.sort_unstable();
            result.push(rest);
        }
    }
    result
}
fn invariant(pairs:&[(u8,u8)], perm:&[u8;6]) -> bool {
    let mut mapped:Vec<_>=pairs.iter().map(|&(a,b)|{
        let x=perm[a as usize];let y=perm[b as usize];
        (x.min(y),x.max(y))
    }).collect();
    mapped.sort_unstable();
    mapped.as_slice() == pairs
}
#[test]
fn f17_source_and_stable_kernels() {
    assert_eq!(count_e(&[2,2,0,1]),19);
    assert_eq!(count_e(&[1,0,2,2]),15);
    assert_eq!(fpow(3,8),16); // nonsquare for Fp².
    assert_eq!(fpow(-2,8),1); // -constant is square; cubic+ cubic.
    let a=[7,1,6,1];let b=[10,1,11,1];
    let product=multiply(&a.map(L::c), &b.map(L::c));
    assert_eq!(product,[2,0,2,0,0,0,1].map(L::c).to_vec());
    assert!(!has_monic_factor(&a,1));
    assert!(!has_monic_factor(&b,1));
    let roots=frobenius_roots();
    assert_eq!(roots[2].pow(17),roots[0]);
    for k in 0..3 {
        let r=rows(k);
        let mut product=vec![L::one()];
        for g in &r { product=multiply(&product,&[g[2],g[1],g[0]]); }
        assert_eq!(product,[2,0,2,0,0,0,1].map(L::c).to_vec());
        for i in 0..3 {
            assert_eq!(r[i][1].pow(17),r[(i+1)%3][1]);
            assert_eq!(r[i][2].pow(17),r[(i+1)%3][2]);
        }
        assert_eq!(det3(r),L::c([0,11,6][k]));
    }
    let all=matchings(&[0,1,2,3,4,5]);
    assert_eq!(all.len(),15);
    let source=[1,2,0,4,5,3];
    let target=[1,2,3,4,5,0];
    assert_eq!(all.iter().filter(|q|invariant(q,&source)).count(),3);
    assert_eq!(all.iter().filter(|q|invariant(q,&target)).count(),1);
}
#[test]
fn f17_descended_codomain_and_zeta_gates() {
    let f=[2,0,2,0,0,0,1];
    let h1=richelot_raw(1);
    let h2=richelot_raw(2);
    assert_eq!(h1,vec![14,12,6,4,10,3,16]);
    assert_eq!(h2,vec![14,5,6,13,10,14,16]);
    // Involution X↦-X interchanges the two resulting curves.
    for i in 0..7 {
        assert_eq!(h2[i], if i.is_multiple_of(2) {h1[i]} else {m(-h1[i])});
    }
    // Source sextic has Frobenius type (3,3); targets have type (6).
    for d in 1..=2 {assert!(!has_monic_factor(&f,d));}
    for h in [&h1,&h2] {
        for d in 1..=3 {assert!(!has_monic_factor(h,d));}
        assert_eq!(card_j(h),(16,348,285));
        // -1 is square in F17, so opposite overall sign is an F17
        // coordinate isomorphism, not a detectable quadratic twist.
        let neg:Vec<_>=h.iter().map(|&c|m(-c)).collect();
        assert_eq!(card_j(&neg),(16,348,285));
    }
    assert_eq!(fpow(4,2),16); // y↦4y realizes the sign change.
    assert_eq!(card_j(&f),(16,348,285));
    assert_eq!(19*15,285);
}

 
/// The usual two-term Richelot correspondence is written in a chart where
/// the kernel quadratics are defined over the base field. Here Frobenius
/// cyclically permutes all three quadratics. The *raw* two-term support
/// polynomial cannot be interpreted as a rational degree-two divisor on
/// the F17 target, even though its bracket-product target is rational.
/// This is a descent guard, not a proof that the rational isogeny fails.
#[test]
fn f17_raw_two_term_correspondence_requires_descent() {
    let xs: Vec<_> = (0..P)
        .filter(|&x| chi(m(fpow(x, 6) + 2 * x * x + 2)) >= 0)
        .collect();
    assert_eq!(xs, vec![0, 3, 4, 8, 9, 13, 14]);
    // Each affine x supports two rational points (none is a branch point).
    for &x in &xs {
        assert_ne!(m(fpow(x, 6) + 2 * x * x + 2), 0);
    }
    assert_eq!(2 * xs.len() + 2, 16); // two points at infinity
    for k in [1, 2] {
        let gs = rows(k);
        let delta = det3(gs);
        let delta_inv = L::c(fpow(delta.0[0], 15));
        let hs: [[L; 3]; 3] = std::array::from_fn(|i| {
            bracket(gs[(i + 1) % 3], gs[(i + 2) % 3])
                .map(|c| c.times(delta_inv))
        });
        for &x in &xs {
            let z = L::c(x);
            let eval_g = |g: [L; 3]| {
                g[0].times(z.times(z))
                    .plus(g[1].times(z))
                    .plus(g[2])
            };
            let a = eval_g(gs[0]);
            let b = eval_g(gs[1]);
            // Coefficients of G0(x) H0(Z) + G1(x) H1(Z).
            let c: [L; 3] = std::array::from_fn(|j| {
                a.times(hs[0][j]).plus(b.times(hs[1][j]))
            });
            assert_ne!(c[2], L::zero());
            // Inversion-free rationality test for monic support:
            // c_j/c_2 is F17-rational iff c_j^17*c_2 == c_j*c_2^17.
            for j in 0..2 {
                assert_ne!(
                    c[j].pow(17).times(c[2]),
                    c[j].times(c[2].pow(17)),
                    "unexpected F17 support for kernel {k}, x={x}, coeff={j}"
                );
            }
        }
    }
}

/// Each nonzero geometric 2-torsion class is indexed by an unordered
/// pair of six branch roots (even subsets modulo complement).
/// No pair is Frobenius-invariant for either (3)(3) or (6).
/// If a base-field-defined kernel K has abstract type (2,2,4), then
/// 2K is a characteristic order-two subgroup, hence its nonzero member
/// would be an F17-rational 2-torsion point. Such a kernel is impossible.
#[test]
fn f17_absent_rational_two_torsion_rules_out_double_richelot_type() {
    let f = [2, 0, 2, 0, 0, 0, 1];
    let t = [14, 12, 6, 4, 10, 3, 16];
    assert_eq!(card_j(&f).2, 285);
    assert_eq!(card_j(&t).2, 285);
    for frob in [[1_u8, 2, 0, 4, 5, 3], [1, 2, 3, 4, 5, 0]] {
        let mut fixed = 0;
        for a in 0_u8..6 {
            for b in (a + 1)..6 {
                let pair = [a, b];
                let mut moved = [frob[a as usize], frob[b as usize]];
                moved.sort_unstable();
                if moved == pair { fixed += 1; }
            }
        }
        assert_eq!(fixed, 0);
    }
}
