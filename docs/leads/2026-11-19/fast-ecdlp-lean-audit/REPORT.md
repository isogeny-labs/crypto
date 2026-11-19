# Kaarelson v2: Lean statement-fidelity audit of the decomposition oracle

2026-11-19 UTC · follows the merged [fast-ecdlp-v2-assessment](../../2026-11-18/fast-ecdlp-v2-assessment/REPORT.md) (PR #4672)
and the coverage reproduction (PR #4745). `[pcc:unverified]`

**Scope.** A statement-level audit of the Lean theorems the paper's Theorem 1 rests on, read
directly from the source at the pinned commit `kaarelkaarelson/fast-ecdlp@d1deb5ab` via
`raw.githubusercontent.com`. This checks whether the Lean *statements* say what the paper claims
(statement fidelity), the `sorry`/axiom mechanism, and where the fidelity guarantee actually
lives. It does **not** run the Lean build, so it does not confirm the proofs go through or that
the axiom prints come out clean; those need a ~12-minute, ~10 GB Mathlib build (noted below).
Source reads were via a summarizing fetch tool, so quoted Lean is reconstructed, not byte-exact.

## What the Lean project claims

- **Theorem 2(ii), the search oracle:** `ECDLP.L104R.subexponentialRate_all`, the statement being
  audited is `ECDLP.subexponentialRate` in `lean/Challenge.lean` (imports only Mathlib,
  intentionally `sorry`), with a proof in `lean/Solution.lean`.
- **Theorem 2(i), counting:** `ECDLP.L132.countingRate_all` (v2, `ECDLPv2/L132_Final`), audited
  against the `sorry` statement `ECDLP109.countingRate` in `ECDLPv2/L109_CountChallenge.lean`.
- **Theorem 1 reduction:** `relation_lattice`, `recover_from_span`, `crt_correct`,
  `planted_kernel`, `runFinal_sound`, `runFinal_complete` (`L114_*`), with `cap_bound` (Chernoff),
  `gen_log_explicit` (any finite abelian group), `totient_ge`; analytic inputs (coverage, group
  order and factorization, sparse linear algebra cost) are **named hypotheses** rather than theorems.
- **Coverage, Theorems 1 and 3:** `L40.sq_card_le_sums_mul_energy`, `L46.las_vegas_success`,
  `L46.coverage_oracle`, `L49.dilEquiv`, `L49.dlp_transfer`, `L49.corollary4_coverage`.
- **Engine:** the Rudich-Rousseau "counting engine" is **vendored** in-repo at
  `lean/vendor/rudich-factoring/`; the Lean search theorem rests on it building correctly, plus
  Mathlib. This is the unrefereed foundation the assessment flagged.

## Finding 1 — the search statement is faithful and non-vacuous

`ECDLP.subexponentialRate m` states: there exists `M : TM2ComputableInTime enc encOut (lexLeast m)`
with `∀ k, Finite (M.tm.Γ k)` and `∀ k ≥ 1, ∃ C, ∀ ℓ, M.time ℓ ≤ C * 2 ^ (ℓ / k)`.

- `TM2ComputableInTime enc encOut (lexLeast m)` is Mathlib's genuine computability type: `M` is
  **bound to compute the function `lexLeast m`** (the lexicographically least bit string encoding
  a generic decomposition, or `none`) via the encodings `enc`/`encOut`. `M` cannot return a
  trivial function.
- The time form `∀ k ≥ 1, ∃ C, ∀ ℓ, time ℓ ≤ C·2^(ℓ/k)` is exactly `time = 2^{o(ℓ)}`: it forces
  `g(ℓ)/ℓ → 0` for `time = 2^{g(ℓ)}`. With `ℓ = O(log p)` for valid instances this is `p^{o(1)}`.
  The bound is genuinely subexponential, not a weaker or constant-ridden restatement.
- **Curve membership is type-enforced:** each summand `P i : W.toAffine.Point` for
  `W = ⟨0,0,0,a,b⟩` over `ZMod p`, so points are on `y²=x³+ax+b` by construction — no off-curve
  cheating. `p` prime (`Fact.mk hp`) and nonsingularity (`W.Δ ≠ 0`) are required; non-prime `p`,
  `Δ = 0`, `p = 2`, and `m = 0` all give the empty set (output `none`), matching the paper.
- **No vacuity.** `strings m x` is empty iff no real decomposition of `R` exists; `m = 1` is
  degenerate (bound vacuous, nonempty iff `R` is a nonzero curve point) but handled; `m ≥ 2`
  requires an actual generic decomposition. The paper's earlier "vacuous statement" error (an
  average-over-curves lemma) is not present in this statement.

This is the central result, and at the statement level it is a correct formalization of the
paper's Theorem 2(ii).

## Items to verify before relying on the full claim

2. **Factor base differs from the paper's prose.** The Lean `strings` uses `x(P_i).val < B` with
   `B = sInf {B | p ≤ B^m} = ⌈p^{1/m}⌉` — a half-open interval `[0, ⌈p^{1/m}⌉)` that **includes
   `x = 0`** — whereas the paper's text and the coverage theorem use `I = [1, ⌊p^{1/m}⌋]`. Same
   asymptotic size `p^{1/m}`, but the exact set differs (ceiling vs floor, half-open, `x=0`
   included). Note this also differs from the interval `[1, ⌊√p⌋]` used in the coverage
   reproduction (PR #4745); a like-for-like comparison should use the Lean set.
3. **Genericity vs Definition 3.1 (the main check).** The Lean generic condition is, for each
   partial sum `S` of the first `k` points (`0 < k < m`): `S ≠ 0 ∧ x(S) ≠ x(P_k)`. This matches
   the intent of Definition 3.1 ("every partial addition adds two affine points with distinct
   `x`-coordinates", excluding doubling and inverses), but the exact indexing of the partial sum
   against the next summand should be confirmed equivalent, because a mismatch would make the
   counted set differ from true generic decompositions. Reassuringly, the authors' own in-kernel
   check (`L105`: `two_iff/E1/E2/E4`) decides `lexLeast_2` on concrete instances at `p = 13` and
   matches a brute-force search.
4. **Challenge↔Solution identity rests on the Comparator, not `Verify.lean` alone.**
   `Verify.lean` contains `example : ∀ m, type_of% (ECDLP.subexponentialRate m) :=
   ECDLP.L104R.subexponentialRate_all`, a sound type-equality binding — **but only if the audited
   `subexponentialRate` resolves to the `sorry` challenge declaration.** `Verify.lean` imports an
   aggregate `ECDLPArtifact`, not `Challenge`/`Solution` directly, so on its own it cannot rule
   out resolving to the *Solution-side* (already-established) declaration (circularity). The anti-circularity guarantee is the
   Comparator (`lean/comparator.json`) plus the second kernel `nanoda`, which re-checks that
   `Solution.lean` derives exactly the `Challenge.lean` statement. That mechanism should be
   inspected and run to close the audit.
5. **Axiom audit is comprehensive but unconfirmed here.** `Verify.lean` runs `#print axioms` over
   the whole set (the main theorem, the `m = 0/1` cases, the engine/network/gadget lemmas, and
   the coverage / Theorem 1 & 3 lemmas `L40/L46/L49`), each expected to report
   `[propext, Classical.choice, Quot.sound]`. The mechanism is sound; confirming the output needs
   the build.

## Overall and next step

At the statement level the headline Lean claim (Theorem 2(ii), the decomposition oracle) is a
faithful, non-vacuous formalization of the paper's search problem, with curve membership
type-enforced and a correctly subexponential time bound. Closing the audit fully needs, in an
environment with Lean/Mathlib `v4.35.0` and ~10 GB disk: (a) `lake build` + `lake env lean
Verify.lean` to confirm one `sorry` (only in the challenge) and clean axioms across the set;
(b) running the Comparator (`comparator.json` + `nanoda`) for Challenge↔Solution statement
identity; (c) confirming the genericity condition equals Definition 3.1; and (d) independent
review of the vendored Rudich-Rousseau engine, which the whole search theorem rests on. This
runtime was not used for the build. The statement-level audit above stands on its own.

## Sources checked

- `kaarelkaarelson/fast-ecdlp@d1deb5ab`: `README.md`, `v2/README.md`, `lean/Challenge.lean`
  (statement, `strings`/`lexLeast`/`enc` structure), `lean/Verify.lean` (the `type_of%` example,
  the `#print axioms` set, the `ECDLPArtifact` import), read via `raw.githubusercontent.com`.
- Paper §8 (Formal verification), as assessed in the 2026-11-18 fast-ecdlp-v2-assessment lead.
