module
public import LocalPrecision
public import Mathlib.RingTheory.PowerSeries.Trunc
public import Mathlib.RingTheory.Ideal.Quotient.Defs
public section
namespace FiniteBuffers
open PowerSeries LocalIntegral LocalCharts LocalPrecision

/-- Exactly the finite coefficient criterion for the two-generator jet ideal. -/
theorem jet_mem_iff_coeff_divisible {R : Type*} [CommRing R]
    (c : R) (q : ℕ) (f : PowerSeries R) :
    f ∈ Ideal.span ({PowerSeries.C c, (PowerSeries.X : PowerSeries R)^q} : Set (PowerSeries R)) ↔
      ∀ n < q, c ∣ PowerSeries.coeff n f := by
  classical
  constructor
  · intro hf
    obtain ⟨a,b,heq⟩ := Ideal.mem_span_pair.mp hf
    intro n hn
    have hx : PowerSeries.coeff n (b*(PowerSeries.X : PowerSeries R)^q) = 0 := by
      apply (PowerSeries.X_pow_dvd_iff.mp ?_) n hn
      exact ⟨b, mul_comm _ _⟩
    refine ⟨PowerSeries.coeff n a, ?_⟩
    rw [← heq, map_add, PowerSeries.coeff_mul_C, hx, add_zero]
    exact mul_comm _ _
  · intro h
    let g : PowerSeries R := PowerSeries.mk (fun n => if hn : n<q then Classical.choose (h n hn) else 0)
    have hlow : ∀ n < q, PowerSeries.coeff n (f-PowerSeries.C c*g)=0 := by
      intro n hn
      simp only [map_sub, PowerSeries.coeff_C_mul]
      simp only [g, PowerSeries.coeff_mk]
      rw [dite_eq_left hn, ← Classical.choose_spec (h n hn), sub_self]
    obtain ⟨b,hb⟩ := PowerSeries.X_pow_dvd_iff.mpr hlow
    apply Ideal.mem_span_pair.mpr
    refine ⟨g,b,?_⟩
    calc
      g*PowerSeries.C c+b*(PowerSeries.X : PowerSeries R)^q =
          PowerSeries.C c*g+(PowerSeries.X : PowerSeries R)^q*b := by ring
      _ = PowerSeries.C c*g+(f-PowerSeries.C c*g) := by rw [← hb]
      _ = f := by ring

@[expose] noncomputable def coefficientIdeal (p : ℕ) : Ideal integralRing :=
  Ideal.span {((2 : integralRing)^p)}

/-- A length-q coefficient buffer; machine encoding of residue classes is
not asserted by this mathematical definition. -/
@[expose] noncomputable def buffer (p q : ℕ) (f : PowerSeries integralRing) :
    Fin q → integralRing ⧸ coefficientIdeal p :=
  fun n => Ideal.Quotient.mk (coefficientIdeal p) (PowerSeries.coeff n f)

/-- Input jet congruence is exactly equality of the finite coefficient buffers. -/
theorem buffer_eq_iff_jet (p q : ℕ) (f g : PowerSeries integralRing) :
    buffer p q f = buffer p q g ↔ f-g ∈ jetIdeal p q := by
  unfold jetIdeal
  rw [jet_mem_iff_coeff_divisible]
  constructor
  · intro h n hn
    have he := congrFun h (⟨n,hn⟩ : Fin q)
    change Ideal.Quotient.mk (coefficientIdeal p) (PowerSeries.coeff n f) =
      Ideal.Quotient.mk (coefficientIdeal p) (PowerSeries.coeff n g) at he
    have hd := Ideal.Quotient.eq.mp he
    rw [coefficientIdeal, Ideal.mem_span_singleton] at hd
    simpa only [map_sub] using hd
  · intro h
    funext n
    apply Ideal.Quotient.eq.mpr
    rw [coefficientIdeal, Ideal.mem_span_singleton]
    simpa only [map_sub] using h n n.isLt

/-- The actual source denominator's exact inverse is stable on finite buffers.
The exact approximation-inverse relation remains explicit. -/
theorem actual_inverse_buffer_stability (a : ℕ) (m : Fin (2^(a+1))) (p q : ℕ)
    (E F : PowerSeries integralRing)
    (hinput : buffer p q E = buffer p q (denominatorSeries a m))
    (hEF : E*F=1) : buffer p q F = buffer p q (inverseSeries a m) := by
  apply (buffer_eq_iff_jet p q F (inverseSeries a m)).mpr
  exact inverse_jet_stability a m p q E F ((buffer_eq_iff_jet p q E _).mp hinput) hEF

theorem inverse_low_coefficients {R : Type*} [CommRing R] (q : ℕ)
    (D I E F : PowerSeries R) (hDI : D*I=1) (hEF : E*F=1)
    (hinput : ∀ n < q, PowerSeries.coeff n E = PowerSeries.coeff n D) :
    ∀ n < q, PowerSeries.coeff n F = PowerSeries.coeff n I := by
  have hd : (PowerSeries.X : PowerSeries R)^q ∣ D-E :=
    PowerSeries.X_pow_dvd_iff.mpr (by intro n hn; simp [hinput n hn])
  have hp : (PowerSeries.X : PowerSeries R)^q ∣ F*(D-E)*I :=
    dvd_mul_of_dvd_left (dvd_mul_of_dvd_right hd F) I
  have heq : F*(D-E)*I = F-I := by
    calc
      _ = F*(D*I)-(E*F)*I := by ring
      _ = _ := by rw [hDI,hEF]; ring
  rw [heq] at hp
  intro n hn
  have hz := PowerSeries.X_pow_dvd_iff.mp hp n hn
  simpa only [map_sub, sub_eq_zero] using hz

/-- Zero extension of a finite coefficient vector, not access to a full jet. -/
@[expose] def liftBuffer {R : Type*} [Zero R] (q : ℕ) (v : Fin q → R) : PowerSeries R :=
  PowerSeries.mk (fun n => if hn : n<q then v ⟨n,hn⟩ else 0)

theorem liftBuffer_coeff {R : Type*} [CommRing R] (q : ℕ) (v : Fin q → R) (n : Fin q) :
    PowerSeries.coeff n (liftBuffer q v) = v n := by
  simp [liftBuffer,n.isLt]

/-- The mathematical inverse-buffer operator takes a finite vector and unit.
Coefficient bit encoding and native runtime are separate obligations. -/
@[expose] noncomputable def invertBuffer {R : Type*} [CommRing R]
    (q : ℕ) (v : Fin q → R) (u : Rˣ) : Fin q → R :=
  fun n => PowerSeries.coeff n (PowerSeries.invOfUnit (liftBuffer q v) u)

@[expose] noncomputable def residueUnit (a : ℕ) (m : Fin (2^(a+1))) (p : ℕ) :
    (integralRing ⧸ coefficientIdeal p)ˣ :=
  Units.map (Ideal.Quotient.mk (coefficientIdeal p)).toMonoidHom (rhoUnit a m)

theorem lifted_actual_constant (a : ℕ) (m : Fin (2^(a+1))) (p q : ℕ) (hq : 0<q) :
    PowerSeries.constantCoeff (liftBuffer q (buffer p q (denominatorSeries a m))) =
      (residueUnit a m p : integralRing ⧸ coefficientIdeal p) := by
  rw [← PowerSeries.coeff_zero_eq_constantCoeff_apply]
  have h := liftBuffer_coeff q (buffer p q (denominatorSeries a m)) (⟨0,hq⟩ : Fin q)
  change PowerSeries.coeff 0 (liftBuffer q (buffer p q (denominatorSeries a m))) = _
  rw [h]
  change Ideal.Quotient.mk (coefficientIdeal p) (PowerSeries.coeff 0 (denominatorSeries a m)) = _
  rw [PowerSeries.coeff_zero_eq_constantCoeff_apply, series_constant]
  rfl

/-- The operator now constructs the finite inverse output itself; no caller-
supplied full approximation or exact inverse hypothesis appears. -/
theorem actual_invertBuffer_correct (a : ℕ) (m : Fin (2^(a+1))) (p q : ℕ) :
    invertBuffer q (buffer p q (denominatorSeries a m)) (residueUnit a m p) =
      buffer p q (inverseSeries a m) := by
  by_cases hq : q=0
  · subst q; funext n; exact Fin.elim0 n
  have hqpos : 0<q := Nat.pos_of_ne_zero hq
  let π := Ideal.Quotient.mk (coefficientIdeal p)
  let D := PowerSeries.map π (denominatorSeries a m)
  let I := PowerSeries.map π (inverseSeries a m)
  let E := liftBuffer q (buffer p q (denominatorSeries a m))
  let F := PowerSeries.invOfUnit E (residueUnit a m p)
  have hDI : D*I=1 := by
    dsimp [D,I]
    rw [← map_mul, denominator_mul_inverse, map_one]
  have hEF : E*F=1 := PowerSeries.mul_invOfUnit E _ (lifted_actual_constant a m p q hqpos)
  have hinput : ∀ n<q, PowerSeries.coeff n E = PowerSeries.coeff n D := by
    intro n hn
    have h := liftBuffer_coeff q (buffer p q (denominatorSeries a m)) (⟨n,hn⟩ : Fin q)
    simpa only [E,D,PowerSeries.coeff_map,buffer,π] using h
  funext n
  have h := inverse_low_coefficients q D I E F hDI hEF hinput n n.isLt
  simpa only [invertBuffer,F,I,PowerSeries.coeff_map,buffer,π] using h

end FiniteBuffers
#print axioms FiniteBuffers.jet_mem_iff_coeff_divisible
#print axioms FiniteBuffers.buffer_eq_iff_jet
#print axioms FiniteBuffers.actual_inverse_buffer_stability
#print axioms FiniteBuffers.actual_invertBuffer_correct
