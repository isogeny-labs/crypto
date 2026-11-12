//! First both-component F101 H0 stage in the existing forward Psi route.
//! Original SevenChain::forward remains an unchanged reference.
use super::ct28_direct_norm::DirectQuartic;
use super::SevenChain;
use crate::balanced::Class;
use isogeny_algos::curve::Pt;
use num_traits::ToPrimitive;
use std::collections::HashSet;

struct DirectH0 {
    covers: [DirectQuartic; 2],
}
impl DirectH0 {
    fn new() -> Self {
        Self {
            covers: [DirectQuartic::first(), DirectQuartic::second()],
        }
    }
    fn norms(&self, d: &Class) -> Result<[Pt<u64>; 2], String> {
        Ok([self.covers[0].norm(d)?.0, self.covers[1].norm(d)?.0])
    }
}
impl SevenChain {
    fn forward_direct_h0(&self, h0: &DirectH0, d: &Class) -> Result<Class, String> {
        self.r1
            .forward_closed(&self.quarter_phi_points(h0.norms(d)?, false)?)
    }
}

fn key(d: &Class) -> (Vec<u64>, Vec<u64>, u8) {
    let (u, v, n) = d.components();
    let coefficients =
        |p: &crate::reference::FpPoly| p.coeffs.iter().map(|c| c.to_u64().unwrap()).collect();
    (coefficients(u), coefficients(v), n)
}

#[test]
fn complete_forward_psi_with_both_prime_field_h0_components() {
    let c = SevenChain::new();
    let h0 = DirectH0::new();
    let all = c.r0.target.classes().unwrap();
    assert_eq!(all.len(), 12317);
    let mut coordinates = HashSet::new();
    let mut images = HashSet::new();
    for (i, d) in all.iter().enumerate() {
        let direct = h0.norms(d).unwrap();
        let reference =
            c.t.source_coordinates(&c.r0.dual_closed(d).unwrap())
                .unwrap();
        assert_eq!(direct, reference, "ordered H0 component pair{i}");
        assert!(coordinates.insert(direct), "H0 rational bijection{i}");
        let fast = c.forward_direct_h0(&h0, d).unwrap();
        assert_eq!(fast, c.forward(d).unwrap(), "complete forward Psi class{i}");
        assert!(
            images.insert(key(&fast)),
            "forward Psi rational bijection{i}"
        );
    }
    assert_eq!(coordinates.len(), 12317);
    assert_eq!(images.len(), 12317);
    assert!(c.forward_direct_h0(&h0, &c.r1.target.zero()).is_err());
    println!("PASS both F101 H0 components/all12317 ordered old-coordinate pairs/bijection and full forwardPsi exact outputs/bijection/wrong-model rejection. The new H0 stage calls no R0 dual;the existing R1 evaluator and all original chain/reference arithmetic remain. No measured cost or whole-prime-field Psi claim");
}
