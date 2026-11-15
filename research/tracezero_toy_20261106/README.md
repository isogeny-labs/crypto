# Trace-zero factor-base toy test (2026-11-06)

Question: does the factor base F_V = { Tr(Q) : x(Q) ∈ V ⊂ F_{p^k} } ⊂ E(F_p) behave
differently from random points of E(F_p)? Protocol and amendments A1–A4: `PROTOCOL.md`.
Instrument: `tracezero.py`; analysis: `analyze.py`. All results [pcc:unverified]
(no claim registered, no replay certificate, no validator report, no matched rho).

## Result

- Scale: p ∈ {11,…,31}, k ∈ {2,3}, d ∈ {1,2}, 5,064 curve rows × 20 V-pairs each.
  Rows (`rows.jsonl`, over the 64 KiB limit) are not in Git. SHA-256, registered run:
  `8c571702…da69781`; symmetric-null run: `df50c1dd…1197a`. Both are in the scratchpad
  only, so they cannot be re-derived from Git except by rerunning (deterministic seeds).
- **Registered run** (`summary.json`): verdict "no structure detected" for cov_2 and cov_3.
  Positive control and A/A band passed (A/A half-width ≈ 0.003–0.004).
  Only the two d = 1 cells moved: Δcov_2 ≈ −0.027, Δ|S| ≈ −0.16 (trace below random).
- **Post hoc A4** (`summary_symnull.json`): that shortfall came mostly from the null, which
  was not sign-symmetric like the lifts (Q, −Q). With a symmetric null: k=3, d=1: Δcov_2 =
  +0.0005 [−0.0025, +0.0039]; k=2, d=2 and k=3, d=2 contain 0. k=2, d=1 keeps a small
  negative Δcov_2 = −0.0086 [−0.0125, −0.0050] (about 0.9 points of coverage). One cell of
  four, unadjusted for multiple looks, unexplained, not replicated at k=3.
- d = 2 cells are saturated in the full base (random cov_2 = 1.00 at p ≥ 17); only the
  3-point small-base arm carries information there.

## Reading

At these sizes the trace map on lifts looks equidistributed in E(F_p) apart from one small
k=2, d=1 residual. This gives no sign of structure to exploit, consistent with the
original expectation. It says nothing about asymptotics, cost, or a comparison with rho.

## Not done

No figure or PDF (matplotlib is not installed here), no benchmark record, no Conductor task
(not installed), no research-kb query. The k=2, d=1 residual is untested beyond this run.

## Replication of the k = 2, d = 1 residual (amendment A5)

`TZ_SALT=rep1 TZ_SYM=1`, d = 1, p ∈ {37,…,61}, 2,800 curve rows, independent seeds
(`summary_rep1.json`; rows SHA-256 `3c24c106…f88a298`, scratchpad only). Positive control
excluded 0 and A/A half-width was ≤ 0.003 in coverage.

- k = 2: pooled Δcov_2 = −0.0026 [−0.0039, −0.0012], Δcov_3 = −0.0083 [−0.0115, −0.0053].
  By the A5 rule this **replicates in sign**: the interval excludes 0 on the negative side.
- k = 3 (reference): Δcov_2 = −0.0002 [−0.0013, +0.0010]; contains 0, as in A4.
- Per prime the k = 2 effect shrinks as p grows: −0.0086 for p = 17–31 (A4), −0.0052 at
  p = 37, −0.0061 at 43, then −0.0005 to −0.0027 for p = 47–61 (most intervals contain 0).
  This is the pattern of a small-p effect that vanishes, not a stable bias. Only seven
  primes and one cell, so the trend is an observation and not a fit or a claim.
- No cost claim follows. A coverage deficit of 0.003 is far below anything usable.

## Explaining the k = 2, d = 1 residual (post hoc exploration)

Hypothesis, stated before the run: for k = 2 and d = 1, V = F_p·g is one of p + 1 lines of
F_{p²} over F_p, and two are degenerate. If g ∈ F_p ("rational"), Q and πQ have the same
x, so Tr(Q) is O or a doubling 2P. If g^p = −g ("pure"), x(πQ) = −x(Q). A uniform random g
lands on each with probability (p − 1)/(p² − 1) ≈ 1/(p + 1), which shrinks with p. For
k = 3 the chance that g ∈ F_p is about 1/p², matching no residual there.

`explore_line_classes.py` (output `explore_line_classes.out.txt`) enumerates all p + 1 lines
for 60 curves and 4 draws each, small-base cov_2 against the sign-symmetric null:

| p | generic Δcov_2 | pure | rational | weighted by 1/(p+1) exposure |
|---|---|---|---|---|
| 23 | +0.0015 | −0.061 | −0.138 | −0.0070 |
| 37 | −0.0026 | −0.039 | −0.051 | −0.0049 |
| 53 | −0.0014 | −0.009 | −0.029 | −0.0021 |

The generic lines (almost all of them) sit near 0. The two degenerate lines carry the
deficit, and the exposure-weighted mix (−0.0070, −0.0049, −0.0021) is the size and the
decay of the pooled residual (−0.0086 for p 17–31, −0.0052 at p = 37, −0.0005 to −0.0027
for p 47–61). So the residual is the fraction of random V that are degenerate, not a
property of the trace map on generic V. No confidence intervals were computed here (60
curves, one seed), and a small negative generic value at p = 37 and 53 is not excluded.

## Rerun without degenerate V (amendments A6, A7)

`TZ_EXCL=1 TZ_SYM=1 TZ_SALT=rep2`; independent seeds; positive control excluded 0 and A/A
half-widths were ≤ 0.004 in coverage in every cell. Rows are not in Git (SHA-256 in the
summary files).

| part | cell (pooled p ≥ 17) | Δcov_2 [95%] | Δcov_3 [95%] |
|---|---|---|---|
| (a) `summary_a6a.json`, p 17–61, 4,800 rows, `fa1111c1…a92ec6` | k=2, d=1 | −0.0009 [−0.0025, +0.0007] | **−0.0037 [−0.0064, −0.0010]** |
| (a) | k=3, d=1 | −0.0002 [−0.0017, +0.0014] | +0.0012 [−0.0012, +0.0036] |
| (b) `summary_a6b.json`, p 17–31, 1,000 rows, `89bc282b…dc635` | k=3, d=2 | −0.0010 [−0.0040, +0.0018] | −0.0003 [−0.0039, +0.0034] |

- The k = 2, d = 2 cell was dropped (A7): there V = F_{p²}, always degenerate. This also
  means the earlier registered k = 2, d = 2 cell used V = F_{p²} on every draw.
- The A6 prediction ("every interval contains 0") is **falsified in one interval**: k = 2,
  d = 1, cov_3. Excluding degenerate V shrinks that residual about fourfold (−0.0151 with them
  in, p 17–31, A4) but does not remove it. It is about 0.4 points of coverage, one of five
  intervals in cov_3, unexplained, with no multiplicity correction applied.
- The other four intervals contain 0, and the registered rule's verdict for the cells
  stays "no structure detected".
- Reading: degenerate V account for most of the k = 2, d = 1 deficit, not all. A residual of
  this size is irrelevant to any cost question, and nothing here is a result about rho.

## Where the last k = 2, d = 1 residual comes from (post hoc stratification)

`stratify_curve_class.py` splits the A6 part (a) rows (rows SHA-256 `fa1111c1…a92ec6`, checked
before use) by curve class; output in `stratify_curve_class.out.txt`. Small-base Δ = trace −
sign-symmetric random.

| k = 2 class | n | Δcov_2 [95%] | Δcov_3 [95%] |
|---|---|---|---|
| b = 0 (j = 1728) | 75 | −0.0965 [−0.1148, −0.0796] | **−0.2032 [−0.2250, −0.1832]** |
| a = 0 (j = 0) | 75 | +0.0016 [−0.0086, +0.0113] | +0.0031 [−0.0110, +0.0177] |
| generic | 2,250 | +0.0022 [+0.0008, +0.0037] | +0.0027 [+0.0004, +0.0051] |

- b = 0 curves are 3.1% of the rows. The weighted mix, 0.969 × 0.0027 − 0.031 × 0.2032 =
  −0.0037, is the pooled A6 cov_3 residual. So that residual is the b = 0 curves.
- The b = 0 deficit is the same for p ≡ 1 and p ≡ 3 (mod 4): −0.2034 and −0.2031 in cov_3. So
  it does not track whether the automorphism (x, y) → (−x, iy) is defined over F_p. The
  mechanism is untested. The V = F_p·g line is closed under x → −x, which this
  automorphism preserves, and that is the obvious thing to look at.
- Generic k = 2 curves are slightly positive (+0.0027), not zero: the trace image covers a
  little more than a random base. Interval excludes 0 for cov_2 and just for cov_3.
- k = 3: b = 0 and generic intervals contain 0 for cov_3. The a = 0 class has Δcov_2 =
  −0.0133 [−0.0252, −0.0014] (n = 70). It is one exploratory look among about a dozen and
  is not read as a result.
- This is a post hoc stratification of already-seen rows, with no multiplicity correction. The
  registered verdicts are unchanged. A rerun should exclude j = 1728 (b = 0), or stratify it
  in the protocol.

## Why b = 0 curves lose coverage: the 2-isogeny descent class (post hoc, checked on toy fields [pcc:unverified])

`subgroup_index_b0.py` and `descent_coset_test.py` (outputs `*.out.txt`; p = 23, 37, 53; k = 2).

- **Observation.** On b = 0 curves the subgroup of E(F_p) generated by the trace image has
  index 2 in exactly the (curve, line) cases where the norm N(g) = g^(p+1) is a square in F_p,
  and index 1 otherwise. On generic curves the index is 1 in all 4,400 cases. Half the lines
  give index 2, which with the index-2 positive control (about −0.4 in cov_3) accounts for the
  b = 0 deficit of about −0.2.
- **General law.** If E has a rational 2-torsion point (r, 0), the map delta(P) = x(P) − r
  mod squares is a homomorphism E(F_p) → F_p*/squares (2-isogeny descent). For V = r + F_p·g
  and Q = (x, y) with x ∈ V, delta(Tr Q) = N_{F_{p²}/F_p}(x − r) = t²·N(g): a constant, the
  square class of N(g). So the whole trace image lies in one coset of ker(delta): in the
  subgroup ker(delta) (index 2) when N(g) is a square, in the nontrivial coset (which generates
  everything) when it is not. `descent_coset_test.py` checks this on ordinary curves with a
  rational root r: the class of delta is constant on every image set and equals the class of
  N(g) in every case (no mixed case), and the index is 2 exactly when N(g) is a square.
  For b = 0 the root is r = 0, which is why V = F_p·g through 0 is the degenerate case there.
  Other curves are not affected for the random lines used here because those do not pass
  through a root of the cubic.

### What this says about cofactor-8 curves (analysis, not measured on any cofactor curve)

- The gain is a filter, not an exponent change. delta kills the odd part of the group, so it is
  trivial on the prime-order subgroup. It sorts the cosets of the small 2-power part.
- In the repo's cofactor-lift oracle (`pkm-semaev:lifts=1`,
  `research/curve25519_cofactor_algebraic_20261102`), the target R is shifted by each of the 8
  torsion points K. If every factor-base point had a fixed delta class, only the shifts R + K with the
  right delta(R + K) could ever decompose, so at most half (up to a quarter with full rational
  2-torsion) of the 8 attempts could succeed and the rest are wasted work. That is a constant factor of 2 (or 4) on a
  sub-step. Whether the miniature's base points have a fixed class is not checked: it would
  need the x-coordinates of base points in F_{q^n} to lie on a line through the 2-torsion
  abscissa with n even, so that F_q* lies in the squares. This is a hypothesis, not a result.
- It matches your reading: a constant gain on the order of the cofactor, nothing asymptotic.

Sources checked: `repo:research/curve25519_cofactor_algebraic_20261102/README.md` (head),
`repo:research/curve25519_cofactor_shared_20261102/README.md` (head),
`repo:research/lift_dense_torsion_pairing_20261106/README.md` (head; the characteristic-2
analogue found a 2× effect from 2-torsion translation). research-kb unavailable.

**Update 2026-11-11:** the premise above was checked on the k = 20 Curve25519 miniature and
fails for the "bits" base: its points fall in both descent classes (36%/64% at t = 5, 40%/60% at
t = 6, 47%/53% at t = 9), so no lift is excluded. Only a halving of candidate pairs per lift
remains, of unmeasured value. See
[`research/curve25519_miniature_descent_class_20261111`](../curve25519_miniature_descent_class_20261111/README.md).
