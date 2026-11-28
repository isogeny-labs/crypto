//! Matched, exact M31 polynomial presentations. No solver is launched here.
//!
//! Original coordinates are affine forms in two parameters. Every arm uses
//! the same section t(t-1)=u(u-1)=0; this is a small zero-dimensional test
//! section over M31, not a replacement of M31 by a smaller field. The target
//! coordinate is constant-folded in every presentation.
use crate::quotient::{field_add, field_mul, field_sub, Error, Polynomial as Poly};
use crate::multiplicity::{self, Fp2, Point};
use crate::M31;
use std::collections::BTreeSet;

const A: u64 = M31 - 3;
const B: u64 = 21;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Arm { Monolithic, PreclosedSequential, PreclosedBalanced, MultiplicityBalanced }
impl Arm {
    pub const ALL: [Self; 4] = [Self::Monolithic, Self::PreclosedSequential,
        Self::PreclosedBalanced, Self::MultiplicityBalanced];
    pub fn name(self) -> &'static str {
        match self {
            Self::Monolithic => "monolithic",
            Self::PreclosedSequential => "preclosed_sequential",
            Self::PreclosedBalanced => "preclosed_balanced",
            Self::MultiplicityBalanced => "multiplicity_balanced",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WitnessMode { Geometric, ExplicitFp2 }

#[derive(Clone, Debug)]
pub struct Plane {
    /// x_i = forms[i][0]*t + forms[i][1]*u + forms[i][2].
    pub forms: Vec<[u64; 3]>,
    pub target_x: u64,
    pub seed: u64,
    pub family: String,
}

#[derive(Clone, Debug)]
pub struct System {
    pub variables: Vec<String>,
    pub equations: Vec<Poly>,
    pub parameters: [usize; 2],
    pub arm: Arm,
    pub witness_mode: WitnessMode,
    /// Number of Euler-symbol exponentiations used to prepare the guard.
    pub guard_euler_powers: usize,
    pub plane: Plane,
}

fn one(n: usize) -> Poly { Poly::constant(n, 1) }
fn embed(poly: &Poly, n: usize, positions: &[usize]) -> Result<Poly, Error> {
    if positions.len() != poly.variables || positions.iter().any(|&i| i >= n) {
        return Err(Error::Invalid("polynomial embedding".into()));
    }
    Poly::from_terms(n, poly.terms.iter().map(|(m, &c)| {
        let mut e = vec![0; n];
        for (i, &j) in positions.iter().enumerate() { e[j] += m[i]; }
        (e, c)
    }))
}
fn affine(n: usize, t: usize, u: usize, c: [u64; 3]) -> Result<Poly, Error> {
    Poly::variable(n, t).scale(c[0]).add(&Poly::variable(n, u).scale(c[1]))?
        .add(&Poly::constant(n, c[2]))
}

/// The ordinary affine S_3 polynomial, without any division or saturation.
pub fn s3(xs: &[Poly; 3]) -> Result<Poly, Error> {
    let n = xs[0].variables;
    let [x, y, z] = xs;
    x.sub(y)?.pow(2)?.mul(&z.pow(2)?)?
        .sub(&x.add(y)?.mul(&x.mul(y)?.add(&Poly::constant(n, A))?)?
             .add(&Poly::constant(n, 2 * B))?.mul(z)?.scale(2))?
        .add(&x.mul(y)?.sub(&Poly::constant(n, A))?.pow(2)?)?
        .sub(&x.add(y)?.scale(4 * B))
}

fn coefficients_last(poly: &Poly, fixed_degree: usize) -> Result<Vec<Poly>, Error> {
    let n = poly.variables.checked_sub(1).ok_or_else(|| Error::Invalid("no elimination variable".into()))?;
    let mut result = vec![Poly::zero(n); fixed_degree + 1];
    for (m, &c) in &poly.terms {
        let d = usize::try_from(m[n]).map_err(|_| Error::ExponentOverflow)?;
        if d > fixed_degree { return Err(Error::Invalid("resultant input degree exceeded".into())); }
        let term = Poly::from_terms(n, [(m[..n].to_vec(), c)])?;
        result[d] = result[d].add(&term)?;
    }
    Ok(result)
}

/// Division-free Sylvester determinant. Input lengths retain their declared
/// degrees even when specialization kills a leading coefficient.
pub fn fixed_resultant(f: &[Poly], g: &[Poly]) -> Result<Poly, Error> {
    if f.len() < 2 || g.len() < 2 { return Err(Error::Invalid("fixed resultant degrees".into())); }
    let d = f.len() - 1;
    let e = g.len() - 1;
    let width = d + e;
    if width > 10 { return Err(Error::Limit("Sylvester width > 10")); }
    let n = f[0].variables;
    let mut matrix = vec![vec![Poly::zero(n); width]; width];
    for i in 0..e { for j in 0..=d { matrix[i][i+j] = f[d-j].clone(); } }
    for i in 0..d { for j in 0..=e { matrix[e+i][i+j] = g[e-j].clone(); } }
    let mut states = vec![Poly::zero(n); 1usize << width];
    states[0] = one(n);
    for mask in 0..states.len()-1 {
        let row = mask.count_ones() as usize;
        if states[mask].is_zero() { continue; }
        for col in 0..width {
            if mask & (1 << col) != 0 || matrix[row][col].is_zero() { continue; }
            let mut term = states[mask].mul(&matrix[row][col])?;
            if (mask >> (col+1)).count_ones() & 1 != 0 { term = term.scale(M31 - 1); }
            let next = mask | (1 << col);
            states[next] = states[next].add(&term)?;
        }
    }
    Ok(states.pop().unwrap())
}

/// Monolithic S_N by fixed-degree recursive resultants, N=3..=6.
/// This construction is separately charged in a future solver comparison.
pub fn semaev(xs: &[Poly]) -> Result<Poly, Error> {
    if xs.len() == 3 { return s3(&[xs[0].clone(), xs[1].clone(), xs[2].clone()]); }
    if !(4..=6).contains(&xs.len()) { return Err(Error::Invalid("Semaev arity must be 3..6".into())); }
    let n = xs[0].variables;
    let positions: Vec<_> = (0..n).collect();
    let lifted: Vec<_> = xs.iter().map(|x| embed(x, n+1, &positions)).collect::<Result<_,_>>()?;
    let v = Poly::variable(n+1, n);
    let mut left = lifted[..xs.len()-2].to_vec();
    left.push(v.clone());
    let f = semaev(&left)?;
    let g = s3(&[lifted[xs.len()-2].clone(), lifted[xs.len()-1].clone(), v])?;
    fixed_resultant(&coefficients_last(&f, 1 << (xs.len()-3))?, &coefficients_last(&g, 2)?)
}

#[derive(Clone)]
struct Cx { re: Poly, im: Poly }
impl Cx {
    fn real(re: Poly) -> Self { Self { im: Poly::zero(re.variables), re } }
    fn add(&self, rhs: &Self) -> Result<Self, Error> {
        Ok(Self { re: self.re.add(&rhs.re)?, im: self.im.add(&rhs.im)? })
    }
    fn scale(&self, c: u64) -> Self { Self { re: self.re.scale(c), im: self.im.scale(c) } }
    fn mul(&self, rhs: &Self) -> Result<Self, Error> {
        Ok(Self { re: self.re.mul(&rhs.re)?.sub(&self.im.mul(&rhs.im)?)?,
                  im: self.re.mul(&rhs.im)?.add(&self.im.mul(&rhs.re)?)? })
    }
}
type Node = [Cx; 3];

fn tensor(nodes: [&Node; 3]) -> Result<Cx, Error> {
    let n = nodes[0][0].re.variables;
    let mut out = Cx::real(Poly::zero(n));
    for i in 0..3 {
        let j = (i+1)%3;
        let k = (i+2)%3;
        out = out.add(&nodes[j][0].mul(&nodes[k][0])?.mul(&nodes[i][2])?)?;
        out = out.add(&nodes[i][0].mul(&nodes[j][1])?.mul(&nodes[k][1])?.scale(M31-2))?;
        out = out.add(&nodes[j][1].mul(&nodes[k][1])?.mul(&nodes[i][2])?.scale(field_sub(0,field_mul(2,A))))?;
        out = out.add(&nodes[i][1].mul(&nodes[j][2])?.mul(&nodes[k][2])?.scale(field_sub(0,4*B)))?;
    }
    out.add(&nodes[0][2].mul(&nodes[1][2])?.mul(&nodes[2][2])?.scale(field_mul(A,A)))
}

fn link_tree(
    leaves: &[Node], nodes: &[Node], used: &mut usize, output: Option<&Node>,
    balanced: bool, equations: &mut Vec<Poly>, mode: WitnessMode,
) -> Result<Node, Error> {
    if leaves.len() == 1 { return Ok(leaves[0].clone()); }
    let split = if balanced { leaves.len()/2 } else { leaves.len()-1 };
    let left = link_tree(&leaves[..split], nodes, used, None, balanced, equations, mode)?;
    let right = link_tree(&leaves[split..], nodes, used, None, balanced, equations, mode)?;
    let out = if let Some(q) = output { q.clone() } else {
        let q = nodes.get(*used).ok_or_else(|| Error::Invalid("tree node accounting".into()))?.clone();
        *used += 1;
        q
    };
    let value = tensor([&left,&right,&out])?;
    equations.push(value.re);
    if mode == WitnessMode::ExplicitFp2 { equations.push(value.im); }
    Ok(out)
}

/// Exact normal form modulo the *common* section t²=t, u²=u. This is used
/// only for guarded derivative preparation, and never changes the section.
pub fn section_normal_form(poly: &Poly) -> Result<Poly, Error> {
    if poly.variables != 2 { return Err(Error::Invalid("two-parameter section required".into())); }
    Poly::from_terms(2, poly.terms.iter().map(|(m,&c)| (m.iter().map(|&v| v.min(1)).collect(),c)))
}
fn section_mul(x: &Poly, y: &Poly) -> Result<Poly, Error> { section_normal_form(&x.mul(y)?) }
fn section_pow(x: &Poly, mut n: u64) -> Result<Poly, Error> {
    let mut out = one(2);
    let mut base = section_normal_form(x)?;
    while n != 0 {
        if n&1 != 0 { out = section_mul(&out,&base)?; }
        n >>= 1;
        if n != 0 { base = section_mul(&base,&base)?; }
    }
    Ok(out)
}

/// Indicator of the nonzero mixed Frobenius sector on the common section.
/// e_i = f(x_i)^((p-1)/2) in {0,+1,-1}; the guard is
/// prod(e_i²) - prod((e_i²+e_i)/2) - prod((e_i²-e_i)/2).
/// Zero-ordinate and both pure sectors have guard zero. No derivative is
/// imposed on those branches. The indicator is computed by quotient powers,
/// without a search for solutions or replacing the factor-base map.
pub fn mixed_sector_guard(plane: &Plane) -> Result<Poly, Error> {
    let mut xs = plane.forms.iter().map(|&c| affine(2,0,1,c)).collect::<Result<Vec<_>,_>>()?;
    xs.push(Poly::constant(2,plane.target_x));
    let mut nonzero = one(2);
    let mut plus = one(2);
    let mut minus = one(2);
    let inv2 = (M31+1)/2;
    for x in xs {
        let f = x.pow(3)?.add(&x.scale(A))?.add(&Poly::constant(2,B))?;
        let e = section_pow(&f,(M31-1)/2)?;
        let square = section_mul(&e,&e)?;
        nonzero = section_mul(&nonzero,&square)?;
        plus = section_mul(&plus,&square.add(&e)?.scale(inv2))?;
        minus = section_mul(&minus,&square.sub(&e)?.scale(inv2))?;
    }
    section_normal_form(&nonzero.sub(&plus)?.sub(&minus)?)
}

pub fn build(plane: &Plane, arm: Arm, mode: WitnessMode) -> Result<System, Error> {
    let m = plane.forms.len();
    if !(3..=5).contains(&m) || plane.target_x >= M31 || plane.forms.iter().flatten().any(|&x| x>=M31) {
        return Err(Error::Invalid("canonical M31 plane with 3..5 summands required".into()));
    }
    let rank_two = plane.forms.iter().enumerate().any(|(i,x)| plane.forms[i+1..].iter().any(|y|
        field_mul(x[0],y[1]) != field_mul(x[1],y[0])));
    if !rank_two { return Err(Error::Invalid("original plane must have rank two".into())); }
    let mut names = Vec::new();
    if arm != Arm::Monolithic {
        for i in 0..m { names.push(format!("q{i}")); }
        for i in 0..m-2 {
            for name in if mode == WitnessMode::Geometric { vec!["z","w","s"] } else { vec!["z","wr","wi","sr","si"] } {
                names.push(format!("{name}{i}"));
            }
        }
    }
    let n = names.len()+2;
    let ti = n-2;
    let ui = n-1;
    names.extend(["t".into(),"u".into()]);
    let xs = plane.forms.iter().map(|&c| affine(n,ti,ui,c)).collect::<Result<Vec<_>,_>>()?;
    let mut equations = Vec::new();
    let target = Poly::constant(n,plane.target_x);
    if arm == Arm::Monolithic {
        let mut all = xs;
        all.push(target);
        equations.push(semaev(&all)?);
    } else {
        let mut leaves = Vec::new();
        for (i,x) in xs.iter().enumerate() {
            let q = Poly::variable(n,i);
            equations.push(q.sub(&x.pow(2)?)?);
            leaves.push([Cx::real(q),Cx::real(x.clone()),Cx::real(one(n))]);
        }
        let target_node = [Cx::real(target.pow(2)?),Cx::real(target),Cx::real(one(n))];
        let mut nodes = Vec::new();
        for i in 0..m-2 {
            let offset = m + i*if mode==WitnessMode::Geometric {3} else {5};
            let z = Poly::variable(n,offset);
            equations.push(z.pow(2)?.sub(&z)?);
            if mode == WitnessMode::Geometric {
                let w = Poly::variable(n,offset+1);
                let s = Poly::variable(n,offset+2);
                equations.extend([z.mul(&w)?.sub(&w)?, z.mul(&s)?.sub(&s)?, w.pow(2)?.sub(&s)?]);
                nodes.push([Cx::real(one(n).sub(&z)?.add(&s)?),Cx::real(w),Cx::real(z)]);
            } else {
                let wr = Poly::variable(n,offset+1);
                let wi = Poly::variable(n,offset+2);
                let sr = Poly::variable(n,offset+3);
                let si = Poly::variable(n,offset+4);
                for x in [&wr,&wi,&sr,&si] { equations.push(z.mul(x)?.sub(x)?); }
                equations.push(wr.pow(2)?.sub(&wi.pow(2)?)?.sub(&sr)?);
                equations.push(wr.mul(&wi)?.scale(2).sub(&si)?);
                nodes.push([Cx {re:one(n).sub(&z)?.add(&sr)?,im:si}, Cx {re:wr,im:wi},Cx::real(z)]);
            }
        }
        let mut used = 0;
        link_tree(&leaves,&nodes,&mut used,Some(&target_node),arm!=Arm::PreclosedSequential,&mut equations,mode)?;
        if used != m-2 { return Err(Error::Invalid("unused projective node".into())); }
    }
    let mut guard_euler_powers = 0;
    if arm == Arm::MultiplicityBalanced {
        let mut all = plane.forms.iter().map(|&c| affine(2,0,1,c)).collect::<Result<Vec<_>,_>>()?;
        all.push(Poly::constant(2,plane.target_x));
        let mono = semaev(&all)?;
        let guard = mixed_sector_guard(plane)?;
        for i in 0..2 {
            let derivative = section_mul(&guard,&mono.derivative(i)?)?;
            equations.push(embed(&derivative,n,&[ti,ui])?);
        }
        guard_euler_powers = m+1;
    }
    for i in [ti,ui] {
        let x = Poly::variable(n,i);
        equations.push(x.pow(2)?.sub(&x)?);
    }
    // Canonical polynomial maps aggregate repeated terms before msolve export.
    for p in &equations { p.validate()?; }
    Ok(System { variables:names,equations,parameters:[ti,ui],arm,witness_mode:mode,
                guard_euler_powers,plane:plane.clone() })
}

impl System {
    pub fn max_generator_degree(&self) -> u64 { self.equations.iter().map(Poly::degree).max().unwrap_or(0) }
    /// Exact variable permutation for the geometric chart quotient. Chart
    /// blocks move first; leaf squares and original parameters stay ordinary
    /// free variables. Explicit Fp2 equations require a different quotient
    /// and are deliberately refused instead of dropping cross relations.
    pub fn quotient_presentation(&self) -> Result<(crate::Ring,Vec<Poly>,Vec<String>),Error> {
        if self.arm==Arm::Monolithic || self.witness_mode!=WitnessMode::Geometric {
            return Err(Error::Invalid("only geometric projective arms use the z,w,s quotient".into()));
        }
        let m=self.plane.forms.len();
        let blocks=m-2;
        let ring=crate::Ring::new(blocks,m+2)?;
        let mut permutation=vec![0;self.variables.len()];
        for i in 0..m {permutation[i]=3*blocks+i;}
        for i in 0..3*blocks {permutation[m+i]=i;}
        permutation[self.parameters[0]]=3*blocks+m;
        permutation[self.parameters[1]]=3*blocks+m+1;
        let equations=self.equations.iter().map(|p|embed(p,p.variables,&permutation)).collect::<Result<Vec<_>,_>>()?;
        let mut names=vec![String::new();self.variables.len()];
        for (old,&new) in permutation.iter().enumerate(){names[new]=self.variables[old].clone();}
        Ok((ring,equations,names))
    }
    /// Export the same chart-first variable order as the geometric quotient
    /// engine. Only the declaration order changes: polynomial terms retain
    /// their original variable names, so their algebra is unchanged.
    /// Explicit Fp2 and monolithic presentations keep their existing order.
    pub fn msolve_variable_order(&self) -> Vec<String> {
        if self.arm==Arm::Monolithic || self.witness_mode!=WitnessMode::Geometric {
            return self.variables.clone();
        }
        let m=self.plane.forms.len();
        let chart_end=m+3*(m-2);
        let mut names=Vec::with_capacity(self.variables.len());
        names.extend_from_slice(&self.variables[m..chart_end]);
        names.extend_from_slice(&self.variables[..m]);
        names.extend_from_slice(&self.variables[chart_end..]);
        names
    }
    pub fn msolve_input(&self) -> String {
        let mut out = format!("{}\n{}\n",self.msolve_variable_order().join(","),M31);
        let nonzero: Vec<_> = self.equations.iter().filter(|p| !p.is_zero()).collect();
        for (i,p) in nonzero.iter().enumerate() {
            if i != 0 { out.push_str(",\n"); }
            let mut first = true;
            for (m,&c) in &p.terms {
                if !first { out.push('+'); }
                first = false;
                out.push_str(&c.to_string());
                for (j,&e) in m.iter().enumerate() {
                    if e != 0 {
                        out.push('*'); out.push_str(&self.variables[j]);
                        if e != 1 { out.push('^');out.push_str(&e.to_string()); }
                    }
                }
            }
        }
        out.push('\n');
        out
    }
}

pub fn original_coordinates(plane: &Plane, t: u64, u: u64) -> Vec<u64> {
    plane.forms.iter().map(|c| field_add(field_add(field_mul(c[0],t),field_mul(c[1],u)),c[2])).collect()
}

/// Independent signed-sum oracle for the four points of the shared section.
/// This enumerates signs for a correctness oracle; it is not a decomposition
/// algorithm or a replacement for extracting a solver's projected output.
pub fn expected_projection(plane: &Plane) -> Result<BTreeSet<[u64;2]>,String> {
    let mut out = BTreeSet::new();
    for t in 0..=1 { for u in 0..=1 {
        let xs = original_coordinates(plane,t,u);
        if multiplicity::signed_sums(&xs)?.iter().any(|q| q.is_some_and(|q| q.x==Fp2::real(plane.target_x))) {
            out.insert([t,u]);
        }
    }}
    Ok(out)
}

fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13; *seed ^= *seed >> 7; *seed ^= *seed << 17;
    *seed
}
fn sample_point(seed: &mut u64, imaginary: bool) -> Result<multiplicity::AffinePoint,String> {
    for _ in 0..1000 {
        let q = multiplicity::lift_x(next(seed)%M31)?;
        if q.y!=Fp2::ZERO && ((q.y.b!=0)==imaginary) { return Ok(q); }
    }
    Err("finite fixture sampling limit".into())
}

/// Deterministic positive fixtures in three equally rotated families. The
/// offset tuple has a disclosed signed sum with the fixed target; it is used
/// only to avoid vacuous all-empty correctness panels. Construction costs
/// belong in a future generation interval, not in solver timings.
pub fn fixture_plane(m: usize, case: usize) -> Result<Plane,String> {
    if !(3..=5).contains(&m) { return Err("fixture arity must be 3..5".into()); }
    let seed = 0x4d33_315f_1123_0000u64 ^ ((m as u64)<<32) ^ (case as u64+1);
    let mut rng = seed;
    let family = case%3;
    let mut points = Vec::<Point>::new();
    if family==2 {
        let q=Some(sample_point(&mut rng,true)?);
        points.extend([q,multiplicity::negate(q)]);
        for _ in 2..m { points.push(Some(sample_point(&mut rng,false)?)); }
    } else {
        for _ in 0..m { points.push(Some(sample_point(&mut rng,family==1)?)); }
    }
    let mut sum = None;
    for &q in &points { sum=multiplicity::add_points(sum,q); }
    let target=sum.ok_or("fixture target unexpectedly infinite")?;
    if target.x.b!=0 { return Err("fixture target coordinate is not rational".into()); }
    let mut forms=points.iter().map(|p| [next(&mut rng)%M31,next(&mut rng)%M31,p.unwrap().x.a]).collect::<Vec<_>>();
    // Fix two independent directions in every cell, without changing offsets.
    forms[0][0]=1; forms[0][1]=0;
    forms[1][0]=0; forms[1][1]=1;
    let mut family=["pure_plus","pure_minus","mixed"][family].to_string();
    // Alternate the ordering of the mixed fixture's canceling negative-sector
    // pair. Both tree shapes then encounter infinity and nonrational partial
    // sums. This shared fixture function binds exported inputs, projection
    // checking and all-equation witness qualification to the identical plane.
    if case%6==5 {
        forms.swap(0,m-1);
        family.push_str("_permuted");
    }
    Ok(Plane {forms,target_x:target.x.a,seed,family})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quotient::field_pow;

    #[test]
    fn monolithic_polynomial_matches_independent_numeric_resultants_and_roots() {
        for m in [3,4] { for case in 0..6 {
            let plane=fixture_plane(m,case).unwrap();
            let model=build(&plane,Arm::Monolithic,WitnessMode::Geometric).unwrap();
            let expected=expected_projection(&plane).unwrap();
            assert!(expected.contains(&[0,0]));
            for t in 0..=1 { for u in 0..=1 {
                let zero=model.equations[0].evaluate(&[t,u]).unwrap()==0;
                assert_eq!(zero,expected.contains(&[t,u]),"m={m}, case={case}");
            }}
        }}
    }

    #[test]
    fn exact_guard_is_zero_on_pure_sectors_and_one_on_mixed_nonzero_sectors() {
        for m in [3,4,5] { for case in 0..12 {
            let plane=fixture_plane(m,case).unwrap();
            let guard=mixed_sector_guard(&plane).unwrap();
            assert_eq!(guard.evaluate(&[0,0]).unwrap(),u64::from(case%3==2));
            assert_eq!(section_mul(&guard,&guard).unwrap(),guard);
            for t in 0..=1 { for u in 0..=1 {
                let mut xs=original_coordinates(&plane,t,u);xs.push(plane.target_x);
                let chars:Vec<_>=xs.into_iter().map(|x|field_pow(multiplicity::curve_rhs(x),(M31-1)/2)).collect();
                let mixed=!chars.contains(&0)&&chars.contains(&1)&&chars.contains(&(M31-1));
                assert_eq!(guard.evaluate(&[t,u]).unwrap(),u64::from(mixed));
            }}
        }}
    }

    #[test]
    fn guarded_derivatives_preserve_projected_zero_set_on_two_parameter_sections() {
        for m in [3,4] { for case in 0..9 {
            let plane=fixture_plane(m,case).unwrap();
            let mono=build(&plane,Arm::Monolithic,WitnessMode::Geometric).unwrap();
            let guard=mixed_sector_guard(&plane).unwrap();
            let derivs=(0..2).map(|i|section_mul(&guard,&mono.equations[0].derivative(i).unwrap()).unwrap()).collect::<Vec<_>>();
            for point in expected_projection(&plane).unwrap() {
                assert!(derivs.iter().all(|f|f.evaluate(&point).unwrap()==0));
            }
        }}
    }

    #[test]
    fn cubic_preclosed_system_inventory_and_canonical_export() {
        for m in [3,4] { for arm in [Arm::PreclosedSequential,Arm::PreclosedBalanced] {
            for mode in [WitnessMode::Geometric,WitnessMode::ExplicitFp2] {
                let model=build(&fixture_plane(m,0).unwrap(),arm,mode).unwrap();
                assert!(model.max_generator_degree()<=3);
                let auxiliaries=m+(m-2)*if mode==WitnessMode::Geometric {3}else{5};
                assert_eq!(model.variables.len(),auxiliaries+2);
                assert_eq!(model.parameters,[auxiliaries,auxiliaries+1]);
                let text=model.msolve_input();
                let exported_order=model.msolve_variable_order();
                assert_eq!(text.lines().next(),Some(exported_order.join(",").as_str()));
                if mode==WitnessMode::Geometric {
                    assert_eq!(exported_order,model.quotient_presentation().unwrap().2);
                } else {
                    assert_eq!(exported_order,model.variables);
                }
                assert_eq!(text.lines().nth(1),Some("2147483647"));
                assert!(!text.ends_with(",\n"));
                assert!(!text.contains("+0*"));
            }
        }}
    }

    #[test]
    fn rejects_rank_one_planes_and_retains_fixed_resultant_degree_drop() {
        let plane=Plane {forms:vec![[1,0,1],[2,0,2],[3,0,3]],target_x:4,seed:0,family:"invalid".into()};
        assert!(build(&plane,Arm::Monolithic,WitnessMode::Geometric).is_err());
        // Fixed quadratics 1 and t² have nonzero projective resultant even
        // though the first specialized polynomial has affine degree zero.
        let z=Poly::zero(0);let o=one(0);
        let r=fixed_resultant(&[o.clone(),z.clone(),z.clone()],&[z.clone(),z,o.clone()]).unwrap();
        assert_eq!(r,o);
    }

    #[test]
    fn quotient_adapter_keeps_all_definitions_and_rejects_wrong_coefficient_model() {
        let plane=fixture_plane(4,2).unwrap();
        let model=build(&plane,Arm::PreclosedBalanced,WitnessMode::Geometric).unwrap();
        let (ring,equations,names)=model.quotient_presentation().unwrap();
        assert_eq!(&names[..3],["z0","w0","s0"]);
        assert_eq!(model.msolve_input().lines().next(),Some(names.join(",").as_str()));
        assert_eq!(equations.len(),model.equations.len());
        let zero_count=equations.iter().filter(|f|ring.normal_form(f).unwrap().is_zero()).count();
        assert_eq!(zero_count,4*(plane.forms.len()-2));
        assert!(build(&plane,Arm::PreclosedBalanced,WitnessMode::ExplicitFp2).unwrap().quotient_presentation().is_err());
        let monolithic=build(&plane,Arm::Monolithic,WitnessMode::Geometric).unwrap();
        assert_eq!(monolithic.msolve_variable_order(),monolithic.variables);
        assert!(monolithic.quotient_presentation().is_err());
    }
}
