# Attacking fake-degree assumptions with exact witnesses and Lean (2026-11-02)

Frozen before any computation. Follows `research/disprove_easy_targets_20261102/`
(reverse repair refuted as stated; Salizzoni Ex. 3.9). Definitions throughout: Nagao
ePrint 2015/984 Definitions 5 and 6, equality reading, over `F_2`; `d_F` true first fall
degree with the field equations outside or inside the generating set; `d'_F` fake first
fall degree (degrees after reduction modulo `X_i^2 + X_i`).

All claims are about exact finite objects; each is settled by exact computation, with a
core-Lean `decide +kernel` witness where small enough. No cost or speed claim.

## A1. "The fake degree controls the true degree" (outside)

**Claim under test.** There is a function `f` with `d_F <= f(d'_F)` for every system
(field equations outside); in particular any repair of Lemma 4 of the form
`d_F <= d'_F + c`. This is what an estimate "the fake degree is small, so the true first
fall degree is small" needs once Lemma 4 itself is gone.

**Witness family.** `W_k = {X^k Y, XY + X}` in `F_2[X, Y]`, `k >= 2` (`W_2` is witness A).
Hand argument, to be checked: reduced, `X^k Y -> XY`, so `d'_F(W_k) = d'_F(W_2) = 2` for
every k. Outside, `X^{k-1}(XY + X) + X^k Y = X^k` is a fall at `k + 1`; below `k + 1` no
multiple of `X^k Y` is admissible, and multiples of a single generator never fall, so
`d_F = k + 1`.

**Method.** Native: compute `d_F` (outside and inside) and `d'_F` for `k = 2..7`
(`dmax = 10`). Lean: kernel-check `W_3` (`d'_F = 2`, `d_F = 4` outside), which already
refutes `d_F <= d'_F + 1`; the all-k statement stays a hand proof plus the native table.

**Decision rule.** If `d'_F = 2` and `d_F = k + 1` for every `k` computed: A1 is REFUTED
(no function of `d'_F` bounds `d_F`, outside). Otherwise record what was found.

## A2. Bounded gap with the field equations inside

**Question.** Over the exhaustive population P3 of the previous protocol (all pairs of
degree <= 2 in 3 variables), and over `W_k`, what is the largest `d_F(inside) - d'_F`?
If it is at most 1 everywhere, record `d_F(inside) <= d'_F + 1` as a new candidate
repair (open). If some system has gap >= 2, record the smallest as a witness and try to
extend it to a growing family.

## A3. The surviving reverse form, three generators

**Claim under test.** `d'_F <= d_F` (outside) whenever the reduced generators are
linearly independent over `F_2` (open form of `GAP-LEMMA4-REVERSE`).

**Populations.** All unordered triples of distinct polynomials from: T1 all degree <= 2,
n = 2 (62 polynomials); T2 multilinear degree <= 2, n = 3 (126 polynomials).
`dmax = 6`.

**Decision rule.** Any triple with linearly independent reduced generators and
`d_F` finite with `d'_F` larger or absent REFUTES the open form; record the smallest and
kernel-check it if it fits the encoding. Otherwise the open form survives T1 and T2.

## Instrument and host

`fall_gap_census.rs`: lines 1..384 copied byte-for-byte from
`research/blindspots_20261018/instruments/blindspots_19_checks.rs` (sha256 `751277a8...`),
plus a new driver. Session container (shared, L0); exact computations only, no timing
reported as a result.

## Amendment 1 (2026-11-02, after A1-A3 ran, before the steps below)

A2 found gap 2 inside (10752 pairs in P3; smallest `{YZ + 1, XZ + X + 1}`: `d_F(inside) = 3`,
`d'_F = 1`). Added steps:
1. Independent cross-check of that witness by brute force over every admissible
   multiplier tuple (Python, standard library, no shared code with the Rust instrument).
2. The 3-variable witness does not fit the 2-variable mask encoding, so it stays native
   (two implementations), not Lean.
Not added: an n = 4 search for a growing inside gap (left for a later protocol).
