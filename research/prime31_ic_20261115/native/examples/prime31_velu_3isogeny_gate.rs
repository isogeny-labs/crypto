//! Exact normalized rational three-isogeny between two *ordinary* curves
//! of prime order n over F_(2^31-1). Scalar-blind, no DLP oracle.
//! This is an algebraic transport / CM-order gate, NOT a faster DLP solver.
//! Reuses prime31-algebraic-ic's original Curve, Arithmetic, check_curve.
//! [pcc:unverified]
use prime31_algebraic_ic::{check_curve, Arithmetic, Curve, Point, PRIME31};
use serde_json::json;

const P: u64 = 2_147_483_647;
const SOURCE_ROOT: u64 = 533_591_203;
const TARGET_ROOTS: [u64; 4] = [1_890_447_397, 1_697_269_373, 546_710_038, 160_540_486];
const DUAL_ROOT: u64 = 546_710_038;
const DEST_A: u64 = 323_891_735;
const DEST_B: u64 = 1_985_248_359;
const DEST_G: (u64, u64) = (1_767_020_261, 634_125_456);

#[derive(Default, Clone, Copy, Debug)]
struct FieldMeter { multiplications: u64, inversions: u64 }

impl FieldMeter {
    fn mul(&mut self, x: u64, y: u64) -> u64 {
        self.multiplications += 1;
        ((u128::from(x) * u128::from(y)) % u128::from(P)) as u64
    }
    fn pow(&mut self, mut a: u64, mut e: u64) -> u64 {
        let mut out = 1;
        while e > 0 {
            if e & 1 != 0 { out = self.mul(out, a); }
            a = self.mul(a, a);
            e >>= 1;
        }
        out
    }
    fn inv(&mut self, a: u64) -> Result<u64, String> {
        if a == 0 { return Err("point belongs to geometric isogeny kernel".to_string()); }
        self.inversions += 1;
        Ok(self.pow(a, P - 2))
    }
}
fn add(x: u64, y: u64) -> u64 { (x + y) % P }
fn sub(x: u64, y: u64) -> u64 { (x + P - y) % P }

fn rhs(c: Curve, x: u64, f: &mut FieldMeter) -> u64 {
    let x_squared=f.mul(x,x);
    let x_cubed=f.mul(x_squared,x);
    let ax=f.mul(c.a,x);
    add(add(x_cubed,ax),c.b)
}
fn psi3(c: Curve, x: u64, f: &mut FieldMeter) -> u64 {
    let x2 = f.mul(x, x);
    let x4 = f.mul(x2, x2);
    let a2 = f.mul(c.a, c.a);
    let ax2 = f.mul(c.a, x2);
    let bx = f.mul(c.b, x);
    sub(add(add(f.mul(3, x4), f.mul(6, ax2)), f.mul(12, bx)), a2)
}
#[derive(Clone, Copy)]
struct Kernel { x: u64, t: u64, v: u64 }

fn kernel(c: Curve, x: u64, f: &mut FieldMeter) -> Result<Kernel, String> {
    if psi3(c, x, f) != 0 { return Err("x is not a 3-division abscissa".to_string()); }
    let v = rhs(c, x, f);
    // n is prime and not 3, so any rational 3-torsion point is impossible.
    // Its Fp-rational kernel polynomial can have a root x while y lies in Fp2.
    if f.pow(v, (P-1)/2) != P-1 {
        return Err("kernel x must have a non-square nonzero ordinate square".to_string());
    }
    let x_squared=f.mul(x,x);
    let triple=f.mul(3,x_squared);
    let t=add(triple,c.a);
    Ok(Kernel { x, t, v })
}

fn dest_coefficients(c: Curve, k: Kernel, f: &mut FieldMeter) -> (u64,u64) {
    // Normalized Velu for K={O,T,-T}, with x(T)=k.x:
    // a' = a - 10(3u^2+a), b' = b - 14(5u^3+3au+2b).
    // Equivalently w=u*t+2*f(u).
    let w = add(f.mul(k.x, k.t), add(k.v,k.v));
    (sub(c.a, f.mul(10,k.t)), sub(c.b, f.mul(14,w)))
}

// The only input for target transport is a *public* point, never its log.
// For a nonrational geometric kernel point x=u the rational map has a pole.
// Since f(u) is nonsquare, no affine Fp-point has x=u.
fn velu_map(q: Point, k: Kernel, f: &mut FieldMeter) -> Result<Point,String> {
    let Some((x,y)) = q else { return Ok(None); };
    let h = sub(x, k.x);
    let inv_h = f.inv(h)?;
    let inv_h2 = f.mul(inv_h,inv_h);
    let inv_h3 = f.mul(inv_h2,inv_h);
    let twice_t = add(k.t,k.t);
    let four_v = add(add(k.v,k.v),add(k.v,k.v));
    let eight_v = add(four_v,four_v);
    let x_image = add(x, add(f.mul(twice_t, inv_h),f.mul(four_v,inv_h2)));
    let derivative = sub(sub(1,f.mul(twice_t,inv_h2)),f.mul(eight_v,inv_h3));
    let y_image = f.mul(y,derivative);
    Ok(Some((x_image,y_image)))
}

fn scaled_three(q: Point) -> Point {
    q.map(|(x,y)| ((9*x)%P,(27*y)%P))
}

fn target() -> Curve {
    Curve {p:P,a:DEST_A,b:DEST_B,n:PRIME31.n,g:DEST_G}
}

// psi3=3 product_i (X-r_i). Check ALL coefficients against native
// short-Weierstrass formula; four distinct rational roots means Frobenius
// preserves all four E'[3] lines, hence is scalar on E'[3].
fn check_target_division(c: Curve, f: &mut FieldMeter) {
    let mut poly = vec![3u64];
    for &r in &TARGET_ROOTS {
        let mut next = vec![0u64;poly.len()+1];
        for (i,&v) in poly.iter().enumerate() {
            next[i] = sub(next[i],f.mul(v,r));
            next[i+1] = add(next[i+1],v);
        }
        poly = next;
        assert_eq!(psi3(c,r,f),0);
    }
    assert_eq!(poly,vec![
        sub(0,f.mul(c.a,c.a)),f.mul(12,c.b),f.mul(6,c.a),0,3
    ]);
    let mut roots=TARGET_ROOTS.to_vec();roots.sort_unstable();roots.dedup();
    assert_eq!(roots.len(),4);
}

fn exact_gate() -> serde_json::Value {
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar).expect("original native curve certificate");
    let mut setup=FieldMeter::default();
    let k=kernel(PRIME31,SOURCE_ROOT,&mut setup).expect("certified rational kernel");
    assert_eq!(k.t,1_555_600_864);
    assert_eq!(k.v,259_447_350);
    assert_eq!(dest_coefficients(PRIME31,k,&mut setup),(DEST_A,DEST_B));
    let e2=target();
    let mut transfer=FieldMeter::default();
    assert_eq!(velu_map(None,k,&mut transfer).unwrap(),None);
    assert_eq!(velu_map(Some(PRIME31.g),k,&mut transfer).unwrap(),Some(DEST_G));
    check_curve(e2,&mut ar).expect("codomain original Rust full-order certificate");
    check_target_division(e2,&mut setup);

    let dual_k=kernel(e2,DUAL_ROOT,&mut setup).expect("certified dual kernel");
    let scaled_a=(81*PRIME31.a)%P;
    let scaled_b=(729*PRIME31.b)%P;
    assert_eq!(dest_coefficients(e2,dual_k,&mut setup),(scaled_a,scaled_b));
    // No scalar enters the map. Scalars below only construct/validate public Q.
    const SCALARS: [u64; 16] = [
        1,2,3,5,7,11,13,17,19,31,37,101,257,65537,1_234_567,2_147_446_830
    ];
    for &s in &SCALARS {
        let pub_q=ar.scalar(PRIME31,s,Some(PRIME31.g));
        let image=velu_map(pub_q,k,&mut transfer).expect("F_p point is not kernel");
        assert!(ar.on_curve(e2,image),"transported point outside codomain");
        let expected=ar.scalar(e2,s,Some(e2.g));
        assert_eq!(image,expected,"scalar-blind homomorphism failure");
        let back=velu_map(image,dual_k,&mut transfer).expect("dual map");
        let tripled=ar.scalar(PRIME31,3,pub_q);
        assert_eq!(back,scaled_three(tripled),
            "normalized dual composition must be [3] under x9/y27 scaling");
    }
    assert_eq!(velu_map(None,dual_k,&mut transfer).unwrap(),None);
    // A full group-homomorphism control not limited to generator multiples
    // evaluated by the same scalar in the expected result.
    let p1=ar.scalar(PRIME31,113,Some(PRIME31.g));
    let p2=ar.scalar(PRIME31,271,Some(PRIME31.g));
    let lhs=velu_map(ar.add(PRIME31,p1,p2),k,&mut transfer).unwrap();
    let i1=velu_map(p1,k,&mut transfer).unwrap();
    let i2=velu_map(p2,k,&mut transfer).unwrap();
    assert_eq!(lhs,ar.add(e2,i1,i2));
    // Negative mutation controls.
    assert!(kernel(PRIME31,SOURCE_ROOT+1,&mut setup).is_err());
    assert!(!ar.on_curve(e2,Some((e2.g.0,add(e2.g.1,1)))));
    assert_ne!(velu_map(Some(PRIME31.g),k,&mut transfer).unwrap(),Some(PRIME31.g));

    json!({
        "standing":"[pcc:unverified]",
        "scope":"actual prime31 scalar-blind degree-three transfer and normalized dual; not a DLP speed benchmark",
        "p":P, "n":PRIME31.n,
        "source_curve":[PRIME31.a,PRIME31.b],
        "source_generator":PRIME31.g,
        "kernel_x":SOURCE_ROOT,
        "kernel_ordinate_square":k.v,
        "target_curve":[e2.a,e2.b],
        "target_generator":e2.g,
        "target_three_division_x_roots":TARGET_ROOTS,
        "dual_kernel_x":DUAL_ROOT,
        "dual_codomain_coefficients":[scaled_a,scaled_b],
        "dual_identity":"dual_normalized(phi(Q))=(9*x([3]Q),27*y([3]Q))",
        "endomorphism_conductors":{"source":3,"target":1},
        "source_min_nonscalar_norm":1_808_610_775u64,
        "target_min_nonscalar_norm":200_956_753u64,
        "public_scalar_controls":SCALARS.len(),
        "transport_setup_fp_multiplications":setup.multiplications,
        "transport_setup_fp_inversions":setup.inversions,
        "transport_controls_fp_multiplications":transfer.multiplications,
        "transport_controls_fp_inversions":transfer.inversions,
        "full_ic_target_logs_recovered":0,
        "full_ic_timing_performed":false,
        "speedup_claim":false
    })
}

fn main() { println!("{}",exact_gate()); }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actual_prime31_velu_dual_and_cm_transition() {
        let v=exact_gate();
        assert_eq!(v["public_scalar_controls"],16);
        assert_eq!(v["full_ic_timing_performed"],false);
    }
    #[test]
    fn no_affine_rational_point_in_kernel() {
        let mut f=FieldMeter::default();
        let k=kernel(PRIME31,SOURCE_ROOT,&mut f).unwrap();
        assert_eq!(f.pow(k.v,(P-1)/2),P-1);
        assert!(velu_map(Some((SOURCE_ROOT,0)),k,&mut f).is_err());
    }
}
