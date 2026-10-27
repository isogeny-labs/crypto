# Proof map — lift-dense factor bases

Required by AGENTS.md, "Mathematical results should include Lean proofs".
Status as of 2026-10-27: **formalization attempted, not built.**

This session could not install a Lean toolchain: the elan installer
download was refused by the session's permission policy. The draft
[`LiftDense.lean.unbuilt`](LiftDense.lean.unbuilt) has never been
type-checked. No statement below is a verified Lean proof. The `.unbuilt`
suffix keeps the repository proof gate from mistaking it for one.

| report statement | Lean declaration (draft) | exact hypotheses in the draft | status |
|:--|:--|:--|:--|
| Lemma 1, trace tower for y·u | `LiftDense.trace_mul_algebraMap` | fields K ⊆ L ⊆ M, scalar tower, finite-dimensional | written, **unbuilt** |
| Theorem 2, twisted-line trace constancy (Tr of x + a + c/x is Tr(a) on u·L \ {0}) | `LiftDense.twisted_line_trace_const` | Tr_{M/L}(u) = 0, Tr_{M/L}(c u⁻¹) = 0, y ≠ 0 | written, **unbuilt** |
| Theorem 2, lift form: every x ∈ u·F_{2^d} \ {0} lifts iff Tr(a) = 0 | — | needs O1 and O2 | **open** |
| Proposition 3, u-count: n = 2 {0, q−1}; n = 3 {0, q−1, 2(q−1), q²−1}; n = 4 a multiple of q−1 | — | — | **open** (O3); checked exhaustively at (12,6), (15,5), (18,6), (18,9), (16,4) and on 4096 sampled b at (21,7), (20,5) |
| Yield identity: mean ordered yield is \|F\|^m/#E, or (\|F\|^m − C_O − C_T)/(#E − 2) on targets ≠ O, T | — | — | **open** (counting argument; not attempted) |
| Weil cap: all-lift dim ≤ N/2 + 1 | — | needs Weil's Kloosterman bound (not in Mathlib) | **open** |

Open obligations:
- **O1. Artin–Schreier criterion in characteristic 2:** ∃z, z² + z = w ⟺
  Tr_{F_q/F_2}(w) = 0. It is the bridge from trace constancy to lifting.
- **O2.** Tr(b/x²) = Tr(√b/x), by Frobenius invariance of the absolute
  trace.
- **O3.** The projective-zero-set classification for the u-count.

To check the draft:
1. Install elan.
2. Create a Lake project with the lakefile shown in the draft header (the
   repository Mathlib pin).
3. Rename the file to `LiftDense.lean` and run `lake build`.
4. Run `#print axioms` on both theorems. Only `propext`,
   `Classical.choice` and `Quot.sound` are acceptable.
5. Record the output here and add the project to the proof gate.
