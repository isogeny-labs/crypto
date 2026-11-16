import WeightedProjNonbranchDVR
import WeightedProjQuinticHead
import Common22SourceHead

set_option autoImplicit false
noncomputable section
namespace ActualNonbranchNorm
open Polynomial Common22SeparateChartMap Common22BranchStalk
  WeightedProjNonbranchZeroOrder WeightedProjSmoothPoints
variable {K : Type*} [Field K]

abbrev A (a3 a2 a1 a0 : K) := AffineRing a3 a2 a1 a0
abbrev L (a3 a2 a1 a0 x0 y0 : K) (h : y0^2 = fAt a3 a2 a1 a0 x0) :=
 Localization.AtPrime (pointIdeal a3 a2 a1 a0 x0 y0 h)

def rho (a3 a2 a1 a0 x0 y0 : K) (h : y0^2 = fAt a3 a2 a1 a0 x0) :
 A a3 a2 a1 a0 →+* ProjPointRing a3 a2 a1 a0 x0 y0 h :=
 (pointRingEquiv a3 a2 a1 a0 x0 y0 h).symm.toRingHom.comp
  (algebraMap (A a3 a2 a1 a0) (L a3 a2 a1 a0 x0 y0 h))
def iota (a3 a2 a1 a0 x0 y0 : K) (h : y0^2 = fAt a3 a2 a1 a0 x0) :=
 (rho a3 a2 a1 a0 x0 y0 h).comp (algebraMap K (A a3 a2 a1 a0))
def affineEval (a3 a2 a1 a0 : K) : K[X] →+* A a3 a2 a1 a0 :=
 eval₂RingHom (algebraMap K (A a3 a2 a1 a0)) (xbar a3 a2 a1 a0)
def germEval (a3 a2 a1 a0 x0 y0 : K) (h : y0^2 = fAt a3 a2 a1 a0 x0) :=
 (rho a3 a2 a1 a0 x0 y0 h).comp (affineEval a3 a2 a1 a0)

theorem affine_curve (a3 a2 a1 a0 : K) :
 ybar a3 a2 a1 a0 ^ 2 = affineEval a3 a2 a1 a0 (quintic a3 a2 a1 a0) := by
 simpa [affineEval, quintic] using quotient_affine_relation a3 a2 a1 a0

theorem germ_curve (a3 a2 a1 a0 x0 y0 : K) (h : y0^2 = fAt a3 a2 a1 a0 x0) :
 pointY a3 a2 a1 a0 x0 y0 h ^ 2 =
 germEval a3 a2 a1 a0 x0 y0 h (quintic a3 a2 a1 a0) := by
 simpa [germEval,rho,pointY] using congrArg (rho a3 a2 a1 a0 x0 y0 h) (affine_curve a3 a2 a1 a0)

theorem point_scalar (a3 a2 a1 a0 x0 y0 k : K) (h : y0^2 = fAt a3 a2 a1 a0 x0) :
 pointEval a3 a2 a1 a0 x0 y0 h (algebraMap K (A a3 a2 a1 a0) k) = k := by
 change pointEval a3 a2 a1 a0 x0 y0 h
  (Ideal.Quotient.mk (affineIdeal a3 a2 a1 a0) (MvPolynomial.C k)) = k
 simp [pointEval,pointPreEval]

theorem point_affine_eval (a3 a2 a1 a0 x0 y0 : K) (h : y0^2 = fAt a3 a2 a1 a0 x0) (n : K[X]) :
 pointEval a3 a2 a1 a0 x0 y0 h (affineEval a3 a2 a1 a0 n) = n.eval x0 := by
 have hh : (pointEval a3 a2 a1 a0 x0 y0 h).comp (affineEval a3 a2 a1 a0) = evalRingHom x0 := by
  apply Polynomial.ringHom_ext
  · intro k
    simpa [affineEval] using point_scalar a3 a2 a1 a0 x0 y0 k h
  · simp [affineEval,pointEval,pointPreEval,xbar]
 exact congrArg (fun f : K[X] →+* K => f n) hh

theorem scalar_unit (a3 a2 a1 a0 x0 y0 b : K) (h : y0^2 = fAt a3 a2 a1 a0 x0) (hb : b≠0) :
 IsUnit (iota a3 a2 a1 a0 x0 y0 h b) :=
 (isUnit_iff_ne_zero.mpr hb).map (iota a3 a2 a1 a0 x0 y0 h)

theorem numerator_nonunit (a3 a2 a1 a0 x0 y0 b : K) (h : y0^2 = fAt a3 a2 a1 a0 x0)
 (n : K[X]) (hn : n.eval x0 = b*y0) :
 germEval a3 a2 a1 a0 x0 y0 h n -
 iota a3 a2 a1 a0 x0 y0 h b * pointY a3 a2 a1 a0 x0 y0 h ∈
 nonunits (ProjPointRing a3 a2 a1 a0 x0 y0 h) := by
 let t := affineEval a3 a2 a1 a0 n - algebraMap K (A a3 a2 a1 a0) b * ybar a3 a2 a1 a0
 have ht : t ∈ pointIdeal a3 a2 a1 a0 x0 y0 h := by
  change pointEval a3 a2 a1 a0 x0 y0 h t = 0
  dsimp [t]
  rw [map_sub,map_mul,point_affine_eval,pointEval_ybar]
  rw [point_scalar]
  simp [hn]
 have hnloc : ¬ IsUnit (algebraMap (A a3 a2 a1 a0) (L a3 a2 a1 a0 x0 y0 h) t) := by
  intro hu
  have hc := (IsLocalization.AtPrime.isUnit_to_map_iff
   (L a3 a2 a1 a0 x0 y0 h) (pointIdeal a3 a2 a1 a0 x0 y0 h) t).mp hu
  exact hc ht
 change ¬ IsUnit _
 intro hu
 apply hnloc
 have hm := hu.map (pointRingEquiv a3 a2 a1 a0 x0 y0 h).toRingHom
 simpa [germEval,iota,rho,pointY,t,map_sub,map_mul] using hm

theorem stalk_norm (a3 a2 a1 a0 x0 y0 b : K) (h : y0^2 = fAt a3 a2 a1 a0 x0)
 (n u1 u2 U : K[X])
 (hnorm : n^2-u1*u2*U=C (b^2)*quintic a3 a2 a1 a0) :
 (germEval a3 a2 a1 a0 x0 y0 h n-iota a3 a2 a1 a0 x0 y0 h b*pointY a3 a2 a1 a0 x0 y0 h)*
 (germEval a3 a2 a1 a0 x0 y0 h n+iota a3 a2 a1 a0 x0 y0 h b*pointY a3 a2 a1 a0 x0 y0 h)=
 germEval a3 a2 a1 a0 x0 y0 h u1*germEval a3 a2 a1 a0 x0 y0 h u2*germEval a3 a2 a1 a0 x0 y0 h U := by
 have hh := congrArg (germEval a3 a2 a1 a0 x0 y0 h) hnorm
 have hc : germEval a3 a2 a1 a0 x0 y0 h (C b) = iota a3 a2 a1 a0 x0 y0 h b := by
  simp [germEval,affineEval,iota]
 simp only [map_sub,map_pow,map_mul,hc] at hh
 rw [←germ_curve] at hh
 linear_combination hh

theorem concrete_norm_order (a3 a2 a1 a0 x0 y0 b : K)
 (h : y0^2 = fAt a3 a2 a1 a0 x0)
 (hs : (quintic a3 a2 a1 a0).Separable) (h2 : (2:K)≠0) (hy : y0≠0) (hb : b≠0)
 (n u1 u2 U : K[X]) (hn : n.eval x0=b*y0)
 (hnorm : n^2-u1*u2*U=C (b^2)*quintic a3 a2 a1 a0) :
 let R := ProjPointRing a3 a2 a1 a0 x0 y0 h
 letI := Common22SeparableAffineDomain.projective_point_domain_of_separable a3 a2 a1 a0 x0 y0 h hs
 letI := WeightedProjNonbranchDVR.projective_point_dvr a3 a2 a1 a0 x0 y0 h hs (mul_ne_zero h2 hy)
 IsDiscreteValuationRing.addVal R (germEval a3 a2 a1 a0 x0 y0 h n-
 iota a3 a2 a1 a0 x0 y0 h b*pointY a3 a2 a1 a0 x0 y0 h) =
 IsDiscreteValuationRing.addVal R (germEval a3 a2 a1 a0 x0 y0 h u1)+
 IsDiscreteValuationRing.addVal R (germEval a3 a2 a1 a0 x0 y0 h u2)+
 IsDiscreteValuationRing.addVal R (germEval a3 a2 a1 a0 x0 y0 h U) := by
 dsimp only
 letI := Common22SeparableAffineDomain.projective_point_domain_of_separable a3 a2 a1 a0 x0 y0 h hs
 letI := WeightedProjNonbranchDVR.projective_point_dvr a3 a2 a1 a0 x0 y0 h hs (mul_ne_zero h2 hy)
 apply point_norm_order a3 a2 a1 a0 x0 y0 h hy
 · simpa only [map_ofNat] using scalar_unit a3 a2 a1 a0 x0 y0 2 h h2
 · exact scalar_unit a3 a2 a1 a0 x0 y0 b h hb
 · exact numerator_nonunit a3 a2 a1 a0 x0 y0 b h n hn
 · exact stalk_norm a3 a2 a1 a0 x0 y0 b h n u1 u2 U hnorm

def numerator (q1 r1 s1 t1 q2 r2 s2 t2 : K) : K[X] :=
 let (b,alpha,c,_,_) := Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2
 C (-c)*X^3+C (-(c*q1+alpha))*X^2+C (b*s1-c*r1-alpha*q1)*X+C (b*t1-alpha*r1)
def residual (q1 r1 s1 t1 q2 r2 s2 t2 : K) : K[X] :=
 let (_,_,c,q,r) := Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2
 C (c^2)*X^2+C q*X+C r

theorem source_norm_identity (a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2 : K)
 (hb : (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1≠0)
 (v1 : Common22Coprime.firstSupport q1 r1 ∣ quintic a3 a2 a1 a0-(C s1*X+C t1)^2)
 (v2 : Common22Coprime.secondSupport q2 r2 ∣ quintic a3 a2 a1 a0-(C s2*X+C t2)^2) :
 numerator q1 r1 s1 t1 q2 r2 s2 t2 ^2-
 Common22Coprime.firstSupport q1 r1*Common22Coprime.secondSupport q2 r2*
 residual q1 r1 s1 t1 q2 r2 s2 t2 =
 C ((Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1^2)*quintic a3 a2 a1 a0 := by
 simpa [numerator,residual,Common22SourceHead.sourceE,Common22Coprime.firstSupport,Common22Coprime.secondSupport] using
 Common22SourceHead.source_curve_eq q1 r1 s1 t1 q2 r2 s2 t2 (quintic a3 a2 a1 a0) hb v1 v2
  (WeightedProjQuinticHead.quintic_natDegree_le_five a3 a2 a1 a0)
  (WeightedProjQuinticHead.quintic_coeff_five a3 a2 a1 a0)
  (WeightedProjQuinticHead.quintic_coeff_four a3 a2 a1 a0)

theorem source_actual_norm_order (a3 a2 a1 a0 x0 y0 q1 r1 s1 t1 q2 r2 s2 t2 : K)
 (h : y0^2 = fAt a3 a2 a1 a0 x0)
 (hs : (quintic a3 a2 a1 a0).Separable) (h2 : (2:K)≠0) (hy : y0≠0)
 (hb : (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1≠0)
 (v1 : Common22Coprime.firstSupport q1 r1 ∣ quintic a3 a2 a1 a0-(C s1*X+C t1)^2)
 (v2 : Common22Coprime.secondSupport q2 r2 ∣ quintic a3 a2 a1 a0-(C s2*X+C t2)^2)
 (hn : (numerator q1 r1 s1 t1 q2 r2 s2 t2).eval x0=
 (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1*y0) :
 let R := ProjPointRing a3 a2 a1 a0 x0 y0 h
 letI := Common22SeparableAffineDomain.projective_point_domain_of_separable a3 a2 a1 a0 x0 y0 h hs
 letI := WeightedProjNonbranchDVR.projective_point_dvr a3 a2 a1 a0 x0 y0 h hs (mul_ne_zero h2 hy)
 IsDiscreteValuationRing.addVal R (germEval a3 a2 a1 a0 x0 y0 h (numerator q1 r1 s1 t1 q2 r2 s2 t2)-
 iota a3 a2 a1 a0 x0 y0 h (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1*
 pointY a3 a2 a1 a0 x0 y0 h) =
 IsDiscreteValuationRing.addVal R (germEval a3 a2 a1 a0 x0 y0 h (Common22Coprime.firstSupport q1 r1))+
 IsDiscreteValuationRing.addVal R (germEval a3 a2 a1 a0 x0 y0 h (Common22Coprime.secondSupport q2 r2))+
 IsDiscreteValuationRing.addVal R (germEval a3 a2 a1 a0 x0 y0 h (residual q1 r1 s1 t1 q2 r2 s2 t2)) := by
 exact concrete_norm_order a3 a2 a1 a0 x0 y0 _ h hs h2 hy hb _ _ _ _ hn
  (source_norm_identity a3 a2 a1 a0 q1 r1 s1 t1 q2 r2 s2 t2 hb v1 v2)
end ActualNonbranchNorm
