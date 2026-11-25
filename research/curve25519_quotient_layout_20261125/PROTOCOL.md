# Protocol: quotient-first grevlex layout for the Curve25519 remainder circuit

**Frozen on 2026-11-25 before implementing a layout option or observing any
quotient-layout solver row.**

Study id: `EXP-C25519-QLAYOUT-001`. Parent revision: `41dd1c910e`. The
instrument and analyzer will be native Rust in the Isogeny Labs `crypto`
repository. No JavaScript is used.

## 1. Question and prior evidence

The active objective is a classical prime-field ECDLP algorithm with expected
total work `r^o(1)`. This study is only a relation-system solver diagnostic.

The completed selective-cut study retained explicit quotient variables at the
last two nonlinear squares of the `d=19` cubic remainder schedule. Its B arm
lowered maximum productive degree from 5 to 4 on 8/8 random targets, but B/D
counted F4 work was `1.3292420766` and peak RSS reached `2.2876` times D. The
membership builder itself was slightly cheaper than D. The extra solver state,
not equation construction, erased the degree benefit.

In B's current variable numbering, elementary symmetric variables come first,
then every residue, then quotient variables. Repository grevlex treats variable
0 as largest. In the quotient equations, quadratic convolution terms in prior
residues can therefore lead over bilinear quotient terms, leaving the local
auxiliaries in polynomial tails. A quotient-first permutation puts all four
quotient variables before the elementary-symmetric and residue variables, so
the bilinear quotient terms can lead whenever total degree ties.

**Question.** Does this quotient-first grevlex variable layout preserve the
degree-4 benefit while consuming local quotient state early enough to beat the
direct system on counted work and memory?

This is a variable permutation under ordinary grevlex, not a true block
monomial order and not local elimination. The repository already contains
binary-chain variable-interleaving work, including a historical 3.57x counted
F4 reduction on one `m=3` cell. Repository search found no prime-field
quotient-layout experiment. The research-KB wrapper remains unavailable, so no
novelty claim is made.

## 2. Fixed system and fresh targets

- Curve: `curve25519_family_instance(20)`, short-Weierstrass
  `y^2 = x^3 + 63,700 x + 872,826` over `p=1,048,573`, prime subgroup order
  `r=131,203`, generator `(349,837,122)`.
- Factor base: `x = 349,833 + 3 w`, `w in mu_19`, distinct roots only.
- Relation: the same symmetrized `S4`, cubic root polynomial
  `q(X)=X^3-e1 X^2+e2 X-e3`, and B cut set `{2,4}` as the completed study.
- Arity `m=3`; qualification order `d=19`.
- Fresh random seed `2026112502`: 8 random subgroup targets per arm. Four
  planted distinct-root targets per arm follow as correctness controls.
- Solver: repository `f4_fp`, ordinary grevlex, release mode,
  `RAYON_NUM_THREADS=1`, degree ceiling 24, 300-second per-system cap.
- Primary unit: counted F4 `F_p` multiply-adds. Secondary units: productive
  degree, rows, columns, steps, exact membership-build coefficient operations,
  peak RSS, and descriptive wall time.

No trace learning, guessing, target reuse from the preceding random panel,
post-data layout selection, or change to the equations is allowed.

## 3. Frozen layouts

Logical variables are `e=(e1,e2,e3)`, six three-coefficient output residues
`r0,...,r5`, and two quotient coefficients `h2=(h20,h21)` and
`h4=(h40,h41)` at schedule cuts 2 and 4. A layout is only a bijection from
these logical variables to exponent-vector positions.

| arm | grevlex positions, largest first | role |
|:--|:--|:--|
| D | `e, r0, r1, r2, r3, r4, r5` | direct baseline, no quotients |
| T | `e, r0, r1, r2, r3, r4, r5, h2, h4` | trailing-quotient reproduction |
| F | `h2, h4, e, r0, r1, r2, r3, r4, r5` | **primary quotient-first arm** |
| E | `e, h2, h4, r0, r1, r2, r3, r4, r5` | quotient block after `e` |
| I | `e, r0, r1, h2, r2, r3, h4, r4, r5` | local interleaving diagnostic |

Within every residue or quotient block, coefficients retain ascending order;
quotient blocks retain schedule order. Only F may qualify. E and I are
mechanistic controls and cannot replace F after data.

## 4. Correctness and identity gates

Before cost interpretation:

1. Every layout permutation and inverse are bijections and round-trip every
   exponent vector and planted assignment.
2. Inverse-permuting each T/F/E/I polynomial reproduces T's canonical logical
   polynomial multiset byte for byte. Thus all four quotient-layout arms define
   exactly the same ideal with renamed variables.
3. Every arm's planted assignment satisfies every equation and re-adds to the
   exact target group point; invalid returned relations equal zero.
4. The predecessor tiny exhaustive positive/negative truth panel passes for
   D and all four quotient layouts.
5. On the fresh panel T, F, E, and I use identical scalar and affine targets,
   return identical verdicts, and have identical input term/degree counts.
6. T reproduces a separately rerun canonical B-layout control on every target's
   counted work, degrees, matrix extrema, steps, and verdict. No prior random
   result is reused as a measurement row.

Any failure invalidates the instrument. Failures and partial rows are retained;
no layout, seed, threshold, or target changes after a row exists.

## 5. Qualification and stop rule

Run 12 rows per arm at `d=19`, 60 total, in target-major D/T/F/E/I order. A
timeout, allocation failure, memory kill, or degree-ceiling result is censored
and never assigned a completed work value.

For random pairs completed by both arms, compute paired geometric-mean work
ratios and descriptive 95% paired bootstrap intervals using 10,000 resamples
and frozen seed `2026112502`.

F qualifies for scaling only if all conditions hold:

1. every correctness and identity gate passes;
2. F has no greater censoring fraction than D or T, with at least 6 random
   D/F/T triples completed;
3. the upper 95% endpoint of F/D counted-work ratio is below **0.75**;
4. the upper 95% endpoint of F/T counted-work ratio is below **0.60**;
5. F's maximum productive degree is at most 4 on at least 6/8 random pairs;
6. F/D peak RSS is at most **1.25** on every completed random pair;
7. layout permutation adds zero membership-build coefficient operations, and
   F/D build operations are at most **1.25** on every pair.

Missing any condition closes F before larger orders or `m=4`. A favorable E
or I row is diagnostic only and requires a separately frozen confirmation.

## 6. Gated scaling and claim boundary

Only after F passes section 5 may D/T/F run at `d in {7,9,21,27,57}` with the
same target counts, seed, caps, semantic last-two-nonlinear cut policy, and
quotient-first layout. At each order F must retain all work, degree, censoring,
and resource limits. Fit `ln(F/D)` on `ln(d)` over completed cell geometric
means; call the advantage strengthening only if the upper two-sided 95% slope
endpoint is below zero. These finite cells remain descriptive.

No `m=4` study is authorized here. A total ECDLP claim additionally requires
relation probability, enough independent relations, setup, filtering, sparse
linear algebra, individual logarithms, and target recovery all bounded by
`r^o(1)`.

## 7. Evidence receipt

Retain exact revisions, compile-repair provenance, toolchain, commands, binary
hashes, per-target JSON, failures, native analyzer source, paired CSV,
SVG/PNG, Typst/PDF, complete hashes, and `experiment.export/v1`. Results remain
`[pcc:unverified]` unless separately registered and signed.
