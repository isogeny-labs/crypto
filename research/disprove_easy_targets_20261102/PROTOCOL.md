# Disproving easy targets from the ECDLP attack graph (2026-11-02)

Frozen before any computation. Source of the targets:
`docs/ecdlp-hardness-corpus/ATTACK_GRAPH.md` and the corpus research targets.

All claims here are about exact finite objects, so each one is settled by exact
computation (and, where small enough, a kernel-checked Lean proof), not by a statistical
experiment. No timing, no cost, no cross-method comparison is claimed.

## Target A: `GAP-LEMMA4-REVERSE` (`RT-LEMMA4-REVERSE`)

**Claim under test (candidate repair of Nagao 2015/984 Lemma 4).** Over `F_2`, equality
reading of Nagao Definitions 5 and 6, field equations OUTSIDE the generating set:
`d'_F <= d_F` for every system.

**Prior evidence.** 7875 pairs of distinct nonzero multilinear polynomials of degree 1
or 2 in 3 variables: 0 counterexamples outside, 224 inside
(`papers/ic-negative-results/evidence/checks.txt`).

**Method.**
1. Exact instrument: the existing `research/blindspots_20261018/instruments/blindspots_19_checks.rs`
   functions (`true_first_fall`, `fake_first_fall`), copied verbatim into a new
   instrument `reverse_census.rs` here, with the original's SHA-256 recorded. No change
   to the fall computations.
2. Populations, in order, each exhaustive (all unordered pairs of distinct nonzero
   polynomials in the stated set, degree cap `dmax = 6`):
   - P1 (control, must reproduce the prior census): multilinear, degree 1-2, n = 3.
   - P2: all polynomials of degree 1-2 in n = 2 (not only multilinear).
   - P3: all polynomials of degree 1-2 in n = 3 (not only multilinear).
   - P4: multilinear, degree 1-3, n = 4, pairs (to test whether the reverse inequality
     survives for multilinear generators beyond the prior population).
3. A counterexample is a system with `d_F` (outside) finite and `d'_F > d_F`, where
   `d'_F > dmax` (no fake fall up to the cap) counts only if a separate argument shows
   no fake fall at any degree (in the multilinear quotient degrees are at most n, so a
   cap `dmax >= n` is exhaustive for `d'_F`).

**Decision rule.**
- Any counterexample in P2 or P3: the claim is REFUTED as stated; record the smallest
  witness, give a hand proof, and kernel-check it in Lean if it fits the existing
  `Lemma4Witness.lean` encoding.
- If all counterexamples have a generator that is not multilinear, and P4 has none:
  record the multilinear restriction as the surviving open form.
- If P1 does not reproduce 0 outside / 224 inside: stop; the instrument copy is wrong.

## Target B: "last fall degree <= degree of regularity" under the BFS definition

**Claim under test.** For a polynomial system F, the last fall degree is at most the
degree of regularity `d_reg` in the Bardet-Faugere-Salvy sense (the least d with
`(F^top)_d = R_d`). HKYY state the inequality for their own "degree of regularity" (a
solving degree), where it holds; the corpus records the BFS reading as the one that
fails (`CEX-GB-SALIZZONI-EX39`, value inferred from Caminata-Gorla Thm 3.1, not
computed).

**Witness.** Salizzoni Example 3.9 at k = 2: `F = {x^2 + y, y^2 + x, xy}`.

**Method.** Exact linear algebra on Macaulay spaces `V_{F,d} = {sum h_i f_i :
deg(h_i f_i) <= d}` over `F_2` and `F_3`, and over `Q` via integer elimination:
1. `d_reg = 2`: `(F^top)_2 = span{x^2, y^2, xy} = R_2`, and `(F^top)_1 = 0 != R_1`.
2. `x, y` are not in `V_{F,2}`; `x, y` are in `V_{F,3}`.
3. Last fall degree `= 3`: by Caminata-Gorla Thm 2.9 it is the largest degree of fall
   among a reduced Groebner basis `{x, y}`, i.e. `max(d_x, d_y) = 3`. The instrument
   also checks `V_{F,d} = (F) ∩ R_{<=d}` for d = 3, 4 directly (ideal (F) = (x, y), so
   `(F) ∩ R_{<=d}` is all polynomials of degree <= d with zero constant term).
4. Kernel-check steps 1-2 over `F_2` in Lean (mask encoding of `Lemma4Witness.lean`).

**Decision rule.** If steps 1-3 hold in every field tried, the BFS-reading claim is
REFUTED (`last fall = 3 > 2 = d_reg`). If any step fails, record the corpus node as
wrong and correct it.

## Target C (only if A and B finish quickly): `RT-SP-T2-TRACE-CHECK`

Not started under this protocol; it needs Semaev's full t = 2 descended system and a
solver-degree trace, and gets its own protocol.

## Host and isolation

Local session container (shared, L0). Exact computations only; no timing is reported,
so isolation does not affect any result. Build: `rustc --edition 2021 -C opt-level=3`.

## Amendment 1 (2026-11-02, after P1-P3, before any P4 computation)

P4 (multilinear, degree 1-3, n = 4) has about 3.3 * 10^4 polynomials and about 5 * 10^8
unordered pairs, which is not feasible as an exhaustive census on the session container.
P4 is not run under this protocol; a seeded sample or three-generator systems go to the
next protocol (RT-LEMMA4-REVERSE). The driver also gained a degeneracy classification
(reduced generators linearly dependent) after P2 showed only degenerate counterexamples;
P1 and P2 were re-run with the final instrument, whose hash is in each results file.
