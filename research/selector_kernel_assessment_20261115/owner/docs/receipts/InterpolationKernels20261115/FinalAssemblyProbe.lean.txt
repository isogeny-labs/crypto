import CryptoLean.Polynomial.InterpolationKernels
import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
import Mathlib.Data.ZMod.Basic
set_option autoImplicit false
open Polynomial CryptoLean.InterpolationKernels

-- Generic endpoint, applied unchanged to the actual bounded-kernel sets.
example {F : Type*} [Field F] (t b dU dV : ℕ)
    (x y : Fin t × Fin b → F) (hx : Function.Injective x) (ht : t ≤ dU) :
    Nat.card (Set.range (fun choice : Fin t → Fin b =>
      kernel x y dU dV (prefixSupport choice))) = b ^ t :=
  prefix_kernel_count x y hx dU dV ht

-- Prime-field nonsingular Mathlib coordinates and the consumer's actual
-- degree caps. The stronger geometric hypotheses are encoded in q; the
-- shared endpoint only needs the inherited field and clean abscissas.
example (p m t b : ℕ) [Fact p.Prime] (_hp : 3 < p) (_hm : 3 ≤ m)
    (W : WeierstrassCurve.Affine (ZMod p))
    (q : Fin t × Fin b → {q : ZMod p × ZMod p // W.Nonsingular q.1 q.2})
    (hx : Function.Injective (fun i => (q i).val.1)) (ht : t ≤ m/2) :
    Nat.card (Set.range (fun choice : Fin t → Fin b =>
      kernel (fun i => (q i).val.1) (fun i => (q i).val.2)
        (m/2) ((m-3)/2) (prefixSupport choice))) = b ^ t :=
  prefix_kernel_count _ _ hx _ _ ht

-- Symbolic products keep exactly the existential colored-support meaning.
example {F : Type*} [Field F] (t b dU dV : ℕ)
    (x y : Fin t × Fin b → F) (u v : F[X]) :
    (u,v) ∈ symbolic x y dU dV ↔
      ∃ choice : Fin t → Fin b, (u,v) ∈ kernel x y dU dV (prefixSupport choice) :=
  symbolic_mem_iff_exists_kernel x y dU dV u v

example (p m t b : ℕ) [Fact p.Prime] (_hp : 3 < p) (_hm : 3 ≤ m)
    (W : WeierstrassCurve.Affine (ZMod p))
    (q : Fin t × Fin b → {q : ZMod p × ZMod p // W.Nonsingular q.1 q.2}) :
    (∃ uv : (ZMod p)[X] × (ZMod p)[X], uv ≠ 0 ∧
      uv ∈ symbolic (fun i => (q i).val.1) (fun i => (q i).val.2) (m/2) ((m-3)/2)) ↔
      ∃ choice : Fin t → Fin b, ∃ uv : (ZMod p)[X] × (ZMod p)[X],
        uv ≠ 0 ∧ uv ∈ kernel (fun i => (q i).val.1) (fun i => (q i).val.2)
          (m/2) ((m-3)/2) (prefixSupport choice) :=
  symbolic_nonzero_iff_exists_kernel _ _ _ _

#print axioms CryptoLean.InterpolationKernels.mem_kernel
#print axioms CryptoLean.InterpolationKernels.nodal_eval_zero_iff
#print axioms CryptoLean.InterpolationKernels.nodal_pair_mem_iff
#print axioms CryptoLean.InterpolationKernels.nodal_witness
#print axioms CryptoLean.InterpolationKernels.supports_eq_of_kernels_eq
#print axioms CryptoLean.InterpolationKernels.prefixSupport_eq_image
#print axioms CryptoLean.InterpolationKernels.mem_prefixSupport
#print axioms CryptoLean.InterpolationKernels.prefixSupport_card
#print axioms CryptoLean.InterpolationKernels.prefixSupport_injective
#print axioms CryptoLean.InterpolationKernels.prefix_kernel_injective
#print axioms CryptoLean.InterpolationKernels.prefix_kernel_count
#print axioms CryptoLean.InterpolationKernels.colorProduct_eq
#print axioms CryptoLean.InterpolationKernels.colorProduct_zero_iff
#print axioms CryptoLean.InterpolationKernels.colorProducts_zero_iff_choice
#print axioms CryptoLean.InterpolationKernels.mem_prefix_kernel_iff
#print axioms CryptoLean.InterpolationKernels.mem_symbolic
#print axioms CryptoLean.InterpolationKernels.symbolic_mem_iff_exists_kernel
#print axioms CryptoLean.InterpolationKernels.symbolic_nonzero_iff_exists_kernel
