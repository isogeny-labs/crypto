//! Independent specialized-fiber correctness gate over the unchanged M31 curve.
//!
//! This module deliberately shares no field arithmetic, polynomial arithmetic,
//! tensor construction or quotient reduction with the candidate solver.  The
//! Sylvester determinant and signed-point product are separate computational
//! paths.  This is a development correctness gate, not a Gröbner benchmark.
use std::collections::BTreeSet;

pub const P: u64 = 2_147_483_647;
pub const CURVE_A: u64 = P - 3;
pub const CURVE_B: u64 = 21;
pub const GATE_SEED: u64 = 0x4d33_315f_2026_1124;

fn add(a: u64, b: u64) -> u64 { (a + b) % P }
fn sub(a: u64, b: u64) -> u64 { (a + P - b) % P }
fn mul(a: u64, b: u64) -> u64 { (a * b) % P }
fn pow(mut a: u64, mut n: u64) -> u64 {
    let mut out = 1;
    while n != 0 {
        if n & 1 != 0 { out = mul(out, a); }
        a = mul(a, a);
        n >>= 1;
    }
    out
}
fn inv(a: u64) -> u64 {
    assert_ne!(a, 0, "zero field denominator");
    pow(a, P - 2)
}
pub fn curve_rhs(x: u64) -> u64 {
    add(add(mul(mul(x, x), x), mul(CURVE_A, x)), CURVE_B)
}

/// a + b*omega, with omega^2=-1. M31 is 3 modulo 4.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Fp2 { pub a: u64, pub b: u64 }
impl Fp2 {
    pub const ZERO: Self = Self { a: 0, b: 0 };
    pub const ONE: Self = Self { a: 1, b: 0 };
    pub fn real(a: u64) -> Self { Self { a: a % P, b: 0 } }
    pub fn plus(self, rhs: Self) -> Self {
        Self { a: add(self.a, rhs.a), b: add(self.b, rhs.b) }
    }
    pub fn minus(self, rhs: Self) -> Self {
        Self { a: sub(self.a, rhs.a), b: sub(self.b, rhs.b) }
    }
    pub fn times(self, rhs: Self) -> Self {
        Self { a: sub(mul(self.a, rhs.a), mul(self.b, rhs.b)),
               b: add(mul(self.a, rhs.b), mul(self.b, rhs.a)) }
    }
    pub fn negative(self) -> Self { Self::ZERO.minus(self) }
    pub fn inverse(self) -> Self {
        let norm = add(mul(self.a, self.a), mul(self.b, self.b));
        let norm_inv = inv(norm);
        Self { a: mul(self.a, norm_inv), b: sub(0, mul(self.b, norm_inv)) }
    }
    pub fn divided_by(self, rhs: Self) -> Self { self.times(rhs.inverse()) }
    pub fn square(self) -> Self { self.times(self) }
    pub fn frobenius(self) -> Self { Self { a: self.a, b: sub(0, self.b) } }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct AffinePoint { pub x: Fp2, pub y: Fp2 }
pub type Point = Option<AffinePoint>; // None is the identity.

pub fn on_curve(point: Point) -> bool {
    match point {
        None => true,
        Some(q) => q.y.square() == q.x.square().times(q.x)
            .plus(Fp2::real(CURVE_A).times(q.x)).plus(Fp2::real(CURVE_B)),
    }
}
pub fn negate(point: Point) -> Point {
    point.map(|q| AffinePoint { x: q.x, y: q.y.negative() })
}
pub fn add_points(left: Point, right: Point) -> Point {
    let (l, r) = match (left, right) {
        (None, q) | (q, None) => return q,
        (Some(l), Some(r)) => (l, r),
    };
    let slope = if l.x == r.x {
        if l.y.plus(r.y) == Fp2::ZERO { return None; }
        assert_eq!(l.y, r.y, "same-x points are neither equal nor opposite");
        Fp2::real(3).times(l.x.square()).plus(Fp2::real(CURVE_A))
            .divided_by(Fp2::real(2).times(l.y))
    } else {
        r.y.minus(l.y).divided_by(r.x.minus(l.x))
    };
    let x = slope.square().minus(l.x).minus(r.x);
    let y = slope.times(l.x.minus(x)).minus(l.y);
    Some(AffinePoint { x, y })
}

/// Deterministic geometric lift. Its ordinate is real or purely imaginary.
pub fn lift_x(x: u64) -> Result<AffinePoint, String> {
    if x >= P { return Err("noncanonical abscissa".into()); }
    let rhs = curve_rhs(x);
    let y = if rhs == 0 {
        Fp2::ZERO
    } else if pow(rhs, (P - 1) / 2) == 1 {
        Fp2::real(pow(rhs, (P + 1) / 4))
    } else {
        Fp2 { a: 0, b: pow(sub(0, rhs), (P + 1) / 4) }
    };
    let point = AffinePoint { x: Fp2::real(x), y };
    if !on_curve(Some(point)) { return Err("lift is off curve".into()); }
    Ok(point)
}

/// One signed sum per sign vector modulo global negation. Duplicates survive.
pub fn signed_sums(xs: &[u64]) -> Result<Vec<Point>, String> {
    let first = xs.first().ok_or("empty input")?;
    let mut states = vec![Some(lift_x(*first)?)];
    for &x in &xs[1..] {
        let point = Some(lift_x(x)?);
        let mut next = Vec::with_capacity(states.len() * 2);
        for state in states {
            next.push(add_points(state, point));
            next.push(add_points(state, negate(point)));
        }
        states = next;
    }
    if !states.iter().all(|&q| on_curve(q)) { return Err("sum is off curve".into()); }
    Ok(states)
}

type Poly = Vec<u64>; // ascending, empty means zero; fixed resultant inputs retain padding.
fn trim(poly: &mut Poly) { while poly.last() == Some(&0) { poly.pop(); } }
fn monic(poly: &[u64]) -> Result<Poly, String> {
    let mut out = poly.to_vec();
    trim(&mut out);
    let leading = *out.last().ok_or("zero polynomial cannot be normalized")?;
    let scale = inv(leading);
    for value in &mut out { *value = mul(*value, scale); }
    Ok(out)
}
fn poly_mul(a: &[u64], b: &[u64]) -> Poly {
    if a.is_empty() || b.is_empty() { return vec![]; }
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() { out[i+j] = add(out[i+j], mul(x,y)); }
    }
    trim(&mut out);
    out
}
fn poly_rem(a: &[u64], b: &[u64]) -> Poly {
    assert!(!b.is_empty());
    let mut out = a.to_vec();
    trim(&mut out);
    let scale = inv(*b.last().unwrap());
    while out.len() >= b.len() {
        let offset = out.len() - b.len();
        let quotient = mul(*out.last().unwrap(), scale);
        for (j, &v) in b.iter().enumerate() {
            out[j+offset] = sub(out[j+offset], mul(quotient, v));
        }
        trim(&mut out);
    }
    out
}
fn poly_gcd(mut a: Poly, mut b: Poly) -> Result<Poly, String> {
    trim(&mut a); trim(&mut b);
    while !b.is_empty() { let r = poly_rem(&a, &b); a = b; b = r; }
    monic(&a)
}
fn poly_power_mod(mut a: Poly, mut exponent: u64, modulus: &[u64]) -> Poly {
    let mut out = poly_rem(&[1], modulus);
    a = poly_rem(&a, modulus);
    while exponent != 0 {
        if exponent & 1 != 0 { out = poly_rem(&poly_mul(&out, &a), modulus); }
        a = poly_rem(&poly_mul(&a, &a), modulus);
        exponent >>= 1;
    }
    out
}
pub fn evaluate(poly: &[u64], x: u64) -> u64 {
    poly.iter().rev().fold(0, |a, &b| add(mul(a, x), b))
}
pub fn derivative(poly: &[u64]) -> Poly {
    let mut out: Poly = poly.iter().enumerate().skip(1)
        .map(|(i, &a)| mul(i as u64, a)).collect();
    trim(&mut out); out
}
pub fn rational_root_polynomial(poly: &[u64]) -> Result<Poly, String> {
    let f = monic(poly)?;
    if f.len() == 1 { return Ok(vec![1]); }
    let mut h = poly_power_mod(vec![0,1], P, &f);
    h.resize(h.len().max(2), 0);
    h[1] = sub(h[1], 1);
    trim(&mut h);
    poly_gcd(f, h)
}
fn linear_product(roots: &BTreeSet<u64>) -> Poly {
    roots.iter().fold(vec![1], |a, &r| poly_mul(&a, &[sub(0,r),1]))
}

fn s3_coefficients(x: u64, y: u64) -> Poly {
    let xy = mul(x,y);
    let sum = add(x,y);
    vec![
        sub(mul(sub(xy,CURVE_A), sub(xy,CURVE_A)), mul(4, mul(CURVE_B,sum))),
        sub(0, mul(2, add(mul(sum,add(xy,CURVE_A)),mul(2,CURVE_B)))),
        mul(sub(x,y),sub(x,y)),
    ]
}
fn determinant(mut matrix: Vec<Vec<u64>>) -> u64 {
    let n = matrix.len();
    let mut result = 1;
    for col in 0..n {
        let row = match (col..n).find(|&row| matrix[row][col] != 0) {
            Some(row) => row,
            None => return 0,
        };
        if row != col { matrix.swap(row, col); result = sub(0,result); }
        let pivot = matrix[col][col];
        result = mul(result,pivot);
        let pivot_inverse = inv(pivot);
        for row in col+1..n {
            let factor = mul(matrix[row][col],pivot_inverse);
            for j in col+1..n {
                matrix[row][j] = sub(matrix[row][j],mul(factor,matrix[col][j]));
            }
            matrix[row][col] = 0;
        }
    }
    result
}
/// Fixed-degree homogeneous Sylvester determinant, including zero high coefficients.
pub fn fixed_resultant(f: &[u64], g: &[u64]) -> u64 {
    assert!(f.len() >= 2 && g.len() >= 2);
    let (m,n) = (f.len()-1,g.len()-1);
    let mut matrix = vec![vec![0;m+n];m+n];
    for row in 0..n {
        for j in 0..=m { matrix[row][row+j] = f[m-j]; }
    }
    for row in 0..m {
        for j in 0..=n { matrix[n+row][row+j] = g[n-j]; }
    }
    determinant(matrix)
}
fn interpolate_consecutive(values: &[u64]) -> Poly {
    assert!(!values.is_empty() && values.len() < P as usize);
    let mut differences = values.to_vec();
    let mut basis = vec![1];
    let mut factorial = 1;
    let mut out = vec![0;values.len()];
    for k in 0..values.len() {
        let scale = mul(differences[0],inv(factorial));
        for (j,&v) in basis.iter().enumerate() { out[j] = add(out[j],mul(scale,v)); }
        for j in 0..differences.len()-1 {
            differences[j] = sub(differences[j+1],differences[j]);
        }
        differences.pop();
        basis = poly_mul(&basis, &[sub(0,k as u64),1]);
        factorial = mul(factorial,(k+1) as u64);
    }
    out // Deliberately retain the known homogeneous degree.
}
/// S_(m+1)(x_1,...,x_m,X) with complete fixed-degree recurrence.
pub fn semaev_fiber(xs: &[u64]) -> Result<Poly,String> {
    if !(2..=5).contains(&xs.len()) { return Err("supported fiber arity is 2..5".into()); }
    if xs.iter().any(|&x| x>=P) { return Err("noncanonical abscissa".into()); }
    let mut f = s3_coefficients(xs[0],xs[1]);
    for &x in &xs[2..] {
        let next_degree = 2*(f.len()-1);
        let values: Vec<_> = (0..=next_degree)
            .map(|z| fixed_resultant(&f,&s3_coefficients(x,z as u64))).collect();
        f = interpolate_consecutive(&values);
    }
    if f.iter().all(|&v|v==0) { return Err("identically-zero Semaev fiber".into()); }
    Ok(f)
}
fn branch_product(states: &[Point]) -> Result<Poly,String> {
    let mut out = vec![Fp2::ONE];
    for point in states.iter().flatten() {
        let mut next = vec![Fp2::ZERO;out.len()+1];
        for (i,&c) in out.iter().enumerate() {
            next[i] = next[i].minus(c.times(point.x));
            next[i+1] = next[i+1].plus(c);
        }
        out=next;
    }
    if out.iter().any(|c|c.b!=0) { return Err("signed product is not Frobenius fixed".into()); }
    Ok(out.iter().map(|c|c.a).collect())
}
fn root_multiplicity(poly: &[u64], root: u64) -> usize {
    let mut f = poly.to_vec(); trim(&mut f);
    let mut count=0;
    while f.len()>1 && evaluate(&f,root)==0 {
        let mut q=vec![0;f.len()-1];
        q[f.len()-2]=*f.last().unwrap();
        for j in (1..f.len()-1).rev() { q[j-1]=add(f[j],mul(root,q[j])); }
        assert_eq!(add(f[0],mul(root,q[0])),0);
        f=q;count+=1;
    }
    count
}

#[derive(Clone,Debug)]
pub struct CellReport {
    pub m: usize,
    pub sector: &'static str,
    pub cases: usize,
    pub roots: usize,
    pub finite_branches: usize,
    pub infinity_branches: usize,
    pub degree_drop_cases: usize,
    pub positive_cases: usize,
    pub simple_root_cases: usize,
    pub repeated_rational_roots: usize,
    pub full_gradient_checks: usize,
    pub coefficient_mutations_rejected: usize,
}
impl CellReport {
    fn new(m:usize,sector:&'static str)->Self {
        Self { m,sector,cases:0,roots:0,finite_branches:0,infinity_branches:0,
            degree_drop_cases:0,positive_cases:0,simple_root_cases:0,
            repeated_rational_roots:0,full_gradient_checks:0,
            coefficient_mutations_rejected:0 }
    }
    fn json(&self)->String {
        format!("{{\"m\":{},\"sector\":\"{}\",\"cases\":{},\"roots\":{},\"finite_branches\":{},\"infinity_branches\":{},\"degree_drop_cases\":{},\"positive_cases\":{},\"simple_root_cases\":{},\"repeated_rational_roots\":{},\"full_gradient_checks\":{},\"coefficient_mutations_rejected\":{}}}",
            self.m,self.sector,self.cases,self.roots,self.finite_branches,self.infinity_branches,
            self.degree_drop_cases,self.positive_cases,self.simple_root_cases,
            self.repeated_rational_roots,self.full_gradient_checks,self.coefficient_mutations_rejected)
    }
}
#[derive(Clone,Debug)]
pub struct GateReport {
    pub seed:u64,
    pub cells:Vec<CellReport>,
    pub no_rational_two_torsion_gcd:Poly,
    pub primitive_zero_sqrt_checks:usize,
    pub zero_polynomial_rejections:usize,
    pub invalid_input_rejections:usize,
    /// A reproducibility checksum only; not a cryptographic evidence signature.
    pub input_fnv1a:u64,
}
impl GateReport {
    pub fn json(&self)->String {
        format!("{{\"schema\":\"m31-specialized-multiplicity-gate/v1\",\"field\":{},\"a\":{},\"b\":{},\"seed\":{},\"scope\":\"specialized_fiber_development_gate_not_groebner\",\"cells\":[{}],\"zero_rhs_curve_branch\":\"absent_by_exact_frobenius_gcd\",\"two_torsion_gcd\":{:?},\"primitive_zero_sqrt_checks\":{},\"zero_polynomial_rejections\":{},\"invalid_input_rejections\":{},\"input_fnv1a\":\"{:016x}\"}}",
            P,CURVE_A,CURVE_B,self.seed,self.cells.iter().map(|c|c.json()).collect::<Vec<_>>().join(","),
            self.no_rational_two_torsion_gcd,self.primitive_zero_sqrt_checks,
            self.zero_polynomial_rejections,self.invalid_input_rejections,self.input_fnv1a)
    }
}
struct Rng(u64);
impl Rng {
    fn next(&mut self)->u64 {
        self.0=self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z=self.0;
        z=(z^(z>>30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z=(z^(z>>27)).wrapping_mul(0x94d049bb133111eb);
        z^(z>>31)
    }
    fn point(&mut self,minus:bool)->Result<AffinePoint,String> {
        for _ in 0..1000 {
            let x=self.next()%P;
            let q=lift_x(x)?;
            if q.y!=Fp2::ZERO && (q.y.b!=0)==minus { return Ok(q); }
        }
        Err("bounded sector sampling exhausted".into())
    }
}
fn hash_inputs(checksum:&mut u64,xs:&[u64]) {
    for n in std::iter::once(xs.len() as u64).chain(xs.iter().copied()) {
        for byte in n.to_le_bytes() {
            *checksum=(*checksum^u64::from(byte)).wrapping_mul(0x100000001b3);
        }
    }
}
fn check_case(xs:&[u64],mixed:bool,positive_target:Option<u64>,report:&mut CellReport)->Result<(),String> {
    let f=semaev_fiber(xs)?;
    let normalized=monic(&f)?;
    let states=signed_sums(xs)?;
    let expected_branches=1usize<<(xs.len()-1);
    if states.len()!=expected_branches { return Err("wrong signed branch count".into()); }
    let infinity=states.iter().filter(|p|p.is_none()).count();
    let finite=states.len()-infinity;
    if mixed {
        let leaves:Vec<_>=xs.iter().map(|&x|lift_x(x)).collect::<Result<_,_>>()?;
        if !leaves.iter().any(|q|q.y.a!=0) || !leaves.iter().any(|q|q.y.b!=0)
            || leaves.iter().any(|q|q.y==Fp2::ZERO) {
            return Err("mixed fixture is outside the nonzero mixed-sector domain".into());
        }
        if infinity%2!=0 { return Err("mixed infinity multiplicity is odd".into()); }
    }
    if normalized.len()-1!=finite { return Err("degree drop does not equal infinity multiplicity".into()); }
    if normalized!=branch_product(&states)? { return Err("full signed multiplicity product mismatch".into()); }
    let roots:BTreeSet<_>=states.iter().flatten().filter(|p|p.x.b==0).map(|p|p.x.a).collect();
    if rational_root_polynomial(&f)?!=linear_product(&roots) {
        return Err("complete projected rational root set mismatch".into());
    }
    let derivative_f=derivative(&f);
    let mut simple=false;
    for &x in &roots {
        let mult=root_multiplicity(&f,x);
        let branch_mult=states.iter().flatten().filter(|q|q.x==Fp2::real(x)).count();
        if mult!=branch_mult { return Err("individual root multiplicity mismatch".into()); }
        if mult==1 { simple=true; }
        if mult>1 { report.repeated_rational_roots+=1; }
        if mixed && (mult%2!=0 || evaluate(&derivative_f,x)!=0) {
            return Err("mixed-sector even multiplicity failed".into());
        }
    }
    if let Some(target)=positive_target {
        if !roots.contains(&target) { return Err("constructed mixed positive was lost".into()); }
        let mut complete=xs.to_vec();complete.push(target);
        if complete.iter().any(|&x|curve_rhs(x)==0) { return Err("gradient fixture has zero RHS".into()); }
        for j in 0..complete.len() {
            let others:Vec<_>=complete.iter().enumerate().filter(|&(i,_)|i!=j).map(|(_,&x)|x).collect();
            let fiber=semaev_fiber(&others)?;
            if evaluate(&fiber,complete[j])!=0 || evaluate(&derivative(&fiber),complete[j])!=0 {
                return Err("mixed positive full-gradient check failed".into());
            }
            report.full_gradient_checks+=1;
        }
    }
    // Coefficient corruption must be detected by an independently supplied root.
    if let Some(&root)=roots.iter().next() {
        let mut corrupted=f.clone();corrupted[0]=add(corrupted[0],1);
        if evaluate(&corrupted,root)==0 || monic(&corrupted)?==branch_product(&states)? {
            return Err("coefficient mutation was not rejected".into());
        }
        report.coefficient_mutations_rejected+=1;
    }
    report.cases+=1;report.roots+=roots.len();report.finite_branches+=finite;
    report.infinity_branches+=infinity;report.degree_drop_cases+=usize::from(infinity>0);
    report.positive_cases+=usize::from(!roots.is_empty());report.simple_root_cases+=usize::from(simple);
    Ok(())
}

/// Fixed 450 sector fixtures plus 50 forced infinity/repeated-x controls.
/// No elapsed times or solver-performance estimates are collected.
pub fn run_multiplicity_gate()->Result<GateReport,String> {
    let mut rng=Rng(GATE_SEED);
    let mut report=GateReport { seed:GATE_SEED,cells:vec![],
        no_rational_two_torsion_gcd:vec![],primitive_zero_sqrt_checks:0,
        zero_polynomial_rejections:0,invalid_input_rejections:0,
        input_fnv1a:0xcbf29ce484222325 };
    for m in 3..=5 {
        for (name,minus) in [("pure_plus",false),("pure_minus",true)] {
            let mut cell=CellReport::new(m,name);
            for _ in 0..50 {
                let xs:Vec<_>=(0..m).map(|_|rng.point(minus).map(|q|q.x.a)).collect::<Result<_,_>>()?;
                hash_inputs(&mut report.input_fnv1a,&xs);
                check_case(&xs,false,None,&mut cell)?;
            }
            // This is the necessary negative control against imposing F'=0 globally.
            if cell.simple_root_cases==0 { return Err("pure-sector simple-root control is vacuous".into()); }
            report.cells.push(cell);
        }
        let mut cell=CellReport::new(m,"mixed_positive");
        for _ in 0..50 {
            let plus_count=if m==3 {1} else {2};
            let mut xs=Vec::new();
            let mut plus_sum=None;
            for _ in 0..plus_count {
                let q=rng.point(false)?;xs.push(q.x.a);plus_sum=add_points(plus_sum,Some(q));
            }
            let target=plus_sum.ok_or("unexpected identity positive target")?.x;
            if target.b!=0 { return Err("base-sector target is not rational".into()); }
            let p=rng.point(true)?;xs.push(p.x.a);
            if m<=4 {
                xs.push(p.x.a); // P + (-P) = O in the minus sector.
            } else {
                let q=rng.point(true)?;
                let r=negate(add_points(Some(p),Some(q))).ok_or("unexpected twist cancellation")?;
                if r.x.b!=0 || r.y.a!=0 || r.y==Fp2::ZERO {
                    return Err("twist triple left the nonzero minus sector".into());
                }
                xs.push(q.x.a);xs.push(r.x.a);
            }
            hash_inputs(&mut report.input_fnv1a,&xs);
            check_case(&xs,true,Some(target.a),&mut cell)?;
        }
        if cell.positive_cases!=50 || cell.full_gradient_checks!=50*(m+1) {
            return Err("mixed positive gate count mismatch".into());
        }
        report.cells.push(cell);
        let mut exceptional=CellReport::new(m,"repeated_x_forced_infinity");
        for _ in 0..10 {
            let p=rng.point(false)?;
            let double=add_points(Some(p),Some(p)).ok_or("unexpected 2-torsion")?;
            let q=rng.point(false)?;
            let xs=match m {
                3=>vec![p.x.a,p.x.a,double.x.a],
                4=>vec![p.x.a,p.x.a,q.x.a,q.x.a],
                5=>vec![p.x.a,p.x.a,double.x.a,q.x.a,q.x.a],
                _=>unreachable!(),
            };
            hash_inputs(&mut report.input_fnv1a,&xs);
            check_case(&xs,false,None,&mut exceptional)?;
        }
        if exceptional.degree_drop_cases!=10 { return Err("forced infinity control missing degree drop".into()); }
        report.cells.push(exceptional);
    }
    for m in 4..=5 {
        let mut exceptional=CellReport::new(m,"mixed_forced_infinity");
        for _ in 0..10 {
            let plus=rng.point(false)?;
            let p=rng.point(true)?;
            let mut xs=vec![plus.x.a,plus.x.a,p.x.a];
            if m==4 {
                xs.push(p.x.a);
            } else {
                let q=rng.point(true)?;
                let r=negate(add_points(Some(p),Some(q))).ok_or("unexpected twist cancellation")?;
                if r.x.b!=0 || r.y.a!=0 || r.y==Fp2::ZERO {
                    return Err("exceptional twist triple left its sector".into());
                }
                xs.push(q.x.a);xs.push(r.x.a);
            }
            hash_inputs(&mut report.input_fnv1a,&xs);
            check_case(&xs,true,None,&mut exceptional)?;
        }
        if exceptional.degree_drop_cases!=10 {
            return Err("mixed forced-infinity control missing degree drop".into());
        }
        report.cells.push(exceptional);
    }
    // No source-curve substitution: zero-RHS leaves are impossible on this curve.
    let cubic=vec![CURVE_B,CURVE_A,0,1];
    report.no_rational_two_torsion_gcd=rational_root_polynomial(&cubic)?;
    if report.no_rational_two_torsion_gcd!=vec![1] {
        return Err("frozen curve unexpectedly has rational 2-torsion".into());
    }
    if pow(0,(P+1)/4)!=0 { return Err("zero square-root primitive failed".into()); }
    report.primitive_zero_sqrt_checks=1;
    if monic(&[0,0,0]).is_ok() || rational_root_polynomial(&[0,0]).is_ok() {
        return Err("zero-polynomial guard accepted invalid input".into());
    }
    report.zero_polynomial_rejections=2;
    if lift_x(P).is_ok() || semaev_fiber(&[0,P,1]).is_ok() || semaev_fiber(&[1]).is_ok() {
        return Err("invalid input was accepted".into());
    }
    report.invalid_input_rejections=3;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn complete_specialized_m31_multiplicity_gate() {
        let report=run_multiplicity_gate().expect("independent M31 multiplicity gate");
        assert_eq!(report.cells.iter().map(|c|c.cases).sum::<usize>(),500);
        assert_eq!(report.cells.iter().map(|c|c.full_gradient_checks).sum::<usize>(),750);
        println!("{}",report.json());
    }
    #[test]
    fn fixed_degree_resultant_keeps_infinity() {
        // Both homogeneous degree-two polynomials have a common infinity root.
        // The reduced-degree resultant of the affine constants would be nonzero.
        assert_eq!(fixed_resultant(&[1,0,0],&[2,0,0]),0);
        assert_eq!(fixed_resultant(&[1,0,0],&[1,0,1]),1);
    }
}
