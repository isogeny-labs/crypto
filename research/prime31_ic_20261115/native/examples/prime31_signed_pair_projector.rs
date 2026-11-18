//! Denominator-safe, target-conditioned algebraic B31 signed-pair projector.
 //! This is a Semaev three-point identity as a homogeneous polynomial screen,
 //! NOT a six-summand ECDLP solver and NOT a rho/BSGS advantage.
 use prime31_algebraic_ic::{check_curve, Arithmetic, Counts, Point, PRIME31};
 use serde::Serialize;
 use std::time::Instant;

 const P: u64 = 2_147_483_647;
 const BASES: [usize; 3] = [4, 8, 31];
 const RNG_SEED: u64 = 0xDEC0_DE12_34AB_CDEF;

 #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
 struct Witness { i: usize, j: usize, sign: i8 }

 #[derive(Default, Clone, Copy, Debug, Serialize)]
 struct FieldCounts { mul: u64, homogeneous_evals: u64, tangent_cases: u64 }

 impl FieldCounts {
     fn mul(&mut self, x: u64, y: u64) -> u64 {
         self.mul += 1;
         ((x as u128 * y as u128) % P as u128) as u64
     }
     fn add(&self, x: u64, y: u64) -> u64 { (x + y) % P }
     fn sub(&self, x: u64, y: u64) -> u64 { (x + P - y) % P }
 }

 fn base_points(ar: &mut Arithmetic) -> Vec<(u64, u64)> {
     let mut out = Vec::new();
     for x in 0..10_000 {
         let rhs = ar.rhs(PRIME31, x);
         if let Some(y) = ar.sqrt(rhs, P) {
             if y != 0 {
                 let pt = (x, y.min(P - y));
                 assert!(ar.on_curve(PRIME31, Some(pt)));
                 out.push(pt);
             }
         }
         if out.len() == 31 { break; }
     }
     assert_eq!(out.len(), 31);
     assert_eq!(&out.iter().take(8).map(|q| q.0).collect::<Vec<_>>(), &[0,1,2,3,5,6,8,9]);
     out
 }

 fn base_polynomial(base: &[(u64, u64)], fp: &mut FieldCounts) -> Vec<u64> {
     let mut h = vec![1u64];
     for &(x, _) in base {
         let mut g = vec![0; h.len() + 1];
         for (k, &c) in h.iter().enumerate() {
             let xc = fp.mul(x, c);
             g[k] = fp.sub(g[k], xc);
             g[k+1] = fp.add(g[k+1], c);
         }
         h = g;
     }
     assert_eq!(h.last(), Some(&1u64));
     h
 }

 /// H_hom(N,D) = D^degree H(N/D), valid with D != 0.
 /// Unlike an unguarded formula, a zero denominator is NEVER fed here.
 fn homogeneous(h: &[u64], n: u64, d: u64, fp: &mut FieldCounts) -> bool {
     assert_ne!(d, 0);
     fp.homogeneous_evals += 1;
     let mut d_power = 1u64;
     let mut acc = *h.last().unwrap();
     for &coefficient in h[..h.len()-1].iter().rev() {
         d_power = fp.mul(d_power, d);
         let term = fp.mul(coefficient, d_power);
         let up = fp.mul(acc, n);
         acc = fp.add(up, term);
     }
     acc == 0
 }

 /// The normalized x-coordinate of Q - P_i is N/D when u != x_i.
 /// At Q = -P_i it is x(-2P_i), from the tangent.
 /// At Q = P_i the result is infinity and has no x-coordinate.
 fn screen_one(q: (u64,u64), pt: (u64,u64), h: &[u64], fp: &mut FieldCounts) -> bool {
     let (u,v) = q;
     let (x,y) = pt;
     let (n,d) = if u == x {
         if v == y { return false; } // Q-P_i = O, exclude from finite x support
         assert_eq!(v, (P-y)%P);
         fp.tangent_cases += 1;
         let y2 = fp.mul(y,y);
         let d = fp.mul(4,y2);
         let x2 = fp.mul(x,x);
         let tangent = fp.add(fp.mul(3,x2), PRIME31.a);
         let n = fp.sub(fp.mul(tangent,tangent), fp.mul(fp.mul(2,x),d));
         (n,d)
     } else {
         let dx = fp.sub(u,x);
         let d = fp.mul(dx,dx);
         let sumy = fp.add(v,y);
         let n = fp.sub(fp.mul(sumy,sumy),fp.mul(fp.add(u,x),d));
         (n,d)
     };
     homogeneous(h,n,d,fp)
 }

 fn witness_from_difference(q: Point, i: usize, base: &[(u64,u64)], ar: &mut Arithmetic) -> Option<Witness> {
     let point = Some(base[i]);
     let diff = ar.add(PRIME31, q, ar.neg(PRIME31, point));
     for (j,&candidate) in base.iter().enumerate() {
         if diff == Some(candidate) { return Some(Witness{i,j,sign:1}); }
         if diff == ar.neg(PRIME31,Some(candidate)) {
             return Some(Witness{i,j,sign:-1});
         }
     }
     None
 }

 /// Never builds a table of point sums. Only the exact algebraic screen sees
 /// every base candidate; an EC subtraction is charged only on a positive hit.
 fn projected(q: Point, base: &[(u64,u64)], h: &[u64], fp: &mut FieldCounts, ar: &mut Arithmetic) -> Option<Witness> {
     if q.is_none() { return Some(Witness{i:0,j:0,sign:-1}); }
     for (i,&pt) in base.iter().enumerate() {
         if screen_one(q.unwrap(),pt,h,fp) {
             let w = witness_from_difference(q,i,base,ar);
             assert!(w.is_some(), "algebraic screen produced an unsigned false positive");
             return w;
         }
     }
     None
 }

 /// Optimized loop structure (O(B) subtractions and x lookups) rather than
 /// O(B^2) pair-sum enumeration; uses the reused Rust group arithmetic.
 fn group_control(q: Point, base: &[(u64,u64)], ar: &mut Arithmetic) -> Option<Witness> {
     if q.is_none() { return Some(Witness{i:0,j:0,sign:-1}); }
     for i in 0..base.len() {
         if let Some(d) = ar.add(PRIME31,q,ar.neg(PRIME31,Some(base[i]))) {
             if let Some(j) = base.iter().position(|pt| pt.0 == d.0) {
                 if d == base[j] { return Some(Witness{i,j,sign:1}); }
                 if d == ar.neg(PRIME31,Some(base[j])) {
                     return Some(Witness{i,j,sign:-1});
                 }
                 panic!("x-coordinate matched but neither sign was valid");
             }
         }
     }
     None
 }

 fn verify(q: Point, base: &[(u64,u64)], w: Witness, ar: &mut Arithmetic) {
     let (i,j) = (w.i,w.j);
     let rhs = if w.sign == 1 { Some(base[j]) } else { ar.neg(PRIME31,Some(base[j])) };
     assert_eq!(ar.add(PRIME31,Some(base[i]),rhs), q);
 }

 fn targets(base: &[(u64,u64)], ar: &mut Arithmetic) -> Vec<Point> {
     let k = base.len().min(8);
     let mut qs = Vec::new();
     for i in 0..k {
         for j in 0..k {
             qs.push(ar.add(PRIME31,Some(base[i]),Some(base[j])));
             qs.push(ar.add(PRIME31,Some(base[i]),ar.neg(PRIME31,Some(base[j]))));
         }
     }
     qs.push(None);
     for &p in &base[..k] { qs.push(Some(p)); }
     for &p in &base[..k] { qs.push(ar.neg(PRIME31,Some(p))); }
     let mut state = RNG_SEED;
     for _ in 0..128 {
         state ^= state << 13;
         state ^= state >> 7;
         state ^= state << 17;
         let mut scalar = state % PRIME31.n;
         if scalar == 0 { scalar = 1; }
         qs.push(ar.scalar(PRIME31,scalar,Some(PRIME31.g)));
     }
     qs
 }

 #[derive(Debug, Serialize)]
 struct Panel {
     b: usize,
     targets: usize,
     positives: usize,
     exceptional_target_abscissae: usize,
     algebraic: FieldCounts,
     algebraic_ec: Counts,
     control_ec: Counts,
     source_only_algebraic_time_ns: u128,
     control_time_ns: u128,
     interpretation: &'static str,
 }

 fn panel(full: &[(u64,u64)], b: usize, check_per_i: bool) -> Panel {
     let base = &full[..b];
     let mut setup_fp = FieldCounts::default();
     let h = base_polynomial(base, &mut setup_fp);
     let mut maker = Arithmetic::default();
     let qs = targets(base,&mut maker);
     let mut fp = FieldCounts::default();
     let mut projected_ec = Arithmetic::default();
     let mut group_ec = Arithmetic::default();
     let mut checker = Arithmetic::default();
     let mut positives = 0;
     let mut exceptions = 0;
     let mut algebraic_time = 0;
     let mut group_time = 0;
     for &q in &qs {
         if let Some((u,_))=q {
             if base.iter().any(|p|p.0==u) { exceptions+=1; }
         }
         let started = Instant::now();
         let found = projected(q,base,&h,&mut fp,&mut projected_ec);
         algebraic_time += started.elapsed().as_nanos();
         let started = Instant::now();
         let expected = group_control(q,base,&mut group_ec);
         group_time += started.elapsed().as_nanos();
         assert_eq!(found.is_some(),expected.is_some(),"projection/control disagreement at B={b}, Q={q:?}");
         if let Some(w)=found { verify(q,base,w,&mut checker); positives+=1; }
         if let Some(w)=expected { verify(q,base,w,&mut checker); }
         if check_per_i {
             if let Some(qp)=q {
                 for (i,&p) in base.iter().enumerate() {
                     let algebraic = screen_one(qp,p,&h,&mut FieldCounts::default());
                     let diff = checker.add(PRIME31,q,checker.neg(PRIME31,Some(p)));
                     let direct = diff.is_some_and(|d|base.iter().any(|r|r.0==d.0));
                     assert_eq!(algebraic,direct,"pointwise x-projection mismatch at B={b}, i={i}, Q={q:?}");
                 }
             }
         }
     }
     Panel {
         b, targets:qs.len(), positives, exceptional_target_abscissae:exceptions,
         algebraic:fp, algebraic_ec:projected_ec.counts,control_ec:group_ec.counts,
         source_only_algebraic_time_ns:algebraic_time,control_time_ns:group_time,
         interpretation:"component diagnostic; unbatched control, NOT a competitive ECDLP benchmark",
     }
 }

 fn study() -> Vec<Panel> {
     let mut ar=Arithmetic::default();
     check_curve(PRIME31,&mut ar).unwrap();
     let base=base_points(&mut ar);
     BASES.iter().map(|&b|panel(&base,b,true)).collect()
 }

 fn main() {
     println!("{}",serde_json::to_string_pretty(&study()).unwrap());
 }

 #[cfg(test)]
 mod tests {
     use super::*;
     #[test]
     fn original_prime31_denominator_safe_signed_pair_panel() {
         let result=study();
         assert_eq!(result.iter().map(|x|x.targets).collect::<Vec<_>>(),vec![169,273,273]);
         assert_eq!(result.iter().map(|x|x.positives).collect::<Vec<_>>(),vec![33,129,129]);
         assert!(result.iter().all(|x|x.algebraic.homogeneous_evals>0));
     }
     #[test]
     fn unguarded_vertical_denominator_is_unsound() {
         let mut ar=Arithmetic::default();
         check_curve(PRIME31,&mut ar).unwrap();
         let base=base_points(&mut ar);
         let mut fp=FieldCounts::default();
         let h=base_polynomial(&base,&mut fp);
         let (x,y)=base[0];
         let q=(x,(P-y)%P);
         let dx=0u64;
         let d=fp.mul(dx,dx);
         let n=fp.sub(fp.mul(fp.add(q.1,y),fp.add(q.1,y)), fp.mul(fp.add(x,x),d));
         assert_eq!((n,d),(0,0));
         // Unguarded homogeneous evaluation is zero, regardless of support.
         let mut raw=FieldCounts::default();
         let mut power_d=1u64;
         let mut v=*h.last().unwrap();
         for &c in h[..h.len()-1].iter().rev() {
             power_d=raw.mul(power_d,d);
             v=raw.add(raw.mul(v,n),raw.mul(c,power_d));
         }
         assert_eq!(v,0);
         assert!(!screen_one(q,base[0],&h,&mut fp));
     }
 }
