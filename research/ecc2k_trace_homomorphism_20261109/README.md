# Trace homomorphism integration for binary point decompositions

Implementation and local correctness work; performance evaluation pending.
No signed PCC verdict has been obtained. Implementation and analysis statements
below are producer assertions [pcc:unverified].

## Scope and implementation

User request: implement the screenshot's trace homomorphism in
Isogeny-labs/crypto, validate it, and evaluate its utility for ECC2K-130.
Source baseline: `e472218c8b14a770c38a1198bc2073edbc03d480`.

For the supported binary model `y² + xy = x³ + a x² + b`,
`delta(P) = Tr(x(P) + a)` and `delta(O) = 0`. A rational-point
relation `R = P_1 + ... + P_m` therefore satisfies

```
sum_i Tr(x_i) + Tr(x_R) + (m+1) Tr(a) = 0.
```

The shared builder adds this affine Boolean equation before any variable
permutation. Intermediate coordinates do not occur. Identically zero rows
are omitted; a constant-one contradiction is retained; duplicate rows are
not inserted. This uses the actual field coefficient `curve.a`, including
subfield models [pcc:unverified].

| Path | Baseline source | This patch |
| --- | --- | --- |
| Ordinary SAT decomposition | aggregate trace on by default | reuse shared builder |
| Ambient S4 SAT | native XOR trace already present | unchanged |
| Gröbner decomposition | no explicit trace equation | default-on, explicit on/off control |
| F6 and pair-index F6 | no explicit trace equation | default-on |
| Root prebuild/export | no explicit trace equation | match solver equations and permutation |
| Crossbred decomposition | no explicit trace equation | add before preprocessing |
| Raw summation-polynomial builders | algebraic root semantics | unchanged |

`groebner_decompose_with_trace` and
`decomposition_solver_equations_with_trace` retain the unmodified equation
system when their final argument is false. Returned relations still undergo
full rational-point lifting and group-sum checks [pcc:unverified].

## ECC2K-130 applicability

The exact model is `E_0: y² + xy = x³ + 1` over GF(2^131), using
`z^131 + z^13 + z² + z + 1`. The published curve, generator and target
come from `BinaryCurve::certicom_challenge("ECC2K-130")`; do not substitute
the random binary ECC2-131 challenge. The checked-in campaign configuration
is `docs/ic/params/ecc2k130-fixed.json`.

The trace constraint has rank at most one. When all summand coordinates
already have trace zero and the target is in the odd-order subgroup, the
aggregate equation is identically zero. Thus inserting that row adds no
restriction. More generally it can remove at most one binary dimension;
this is a statement about this row, not a bound on a solver's runtime
[pcc:unverified].

The fixed campaign builds a subgroup factor base of normal-basis Hamming
weight at most two (`ecc2k130/codegen/indexcalc_fixed.py`, `Parameters` and
`the base method`). In any normal basis, trace is coordinate parity: each
basis element has trace one. Its subgroup points on E_0 have trace zero,
so nonzero admitted coordinates of weight at most two have weight exactly
two. The aggregate row is therefore redundant for that factor-base domain
[pcc:unverified]. This is a source/mathematical applicability analysis,
not a measured campaign speed result.

In an ambient-coordinate SAT encoding this means logical redundancy modulo
the finite domain, not a literal zero polynomial. An explicit redundant XOR
can still strengthen propagation. Only a matched measurement can determine
that effect. Literal zero-row omission applies when the linear coordinate
basis itself has trace zero [pcc:unverified].

The generic polynomial frontend's `FieldStructure::is_exact` admits field
degrees at most 64. The patch preserves that guard. Its new arithmetic
checks at degree 83 and 131 do not turn this frontend into a wide-field
Gröbner solver. Existing separate wide SAT paths retain their own contracts.
A full degree-83 baseline/candidate comparison remains required before a
scaling claim; none is established here [pcc:unverified].

## Correctness checks

Native Rust tests in `koblitz_index_calculus.rs` cover:

- the original group homomorphism test, including identity and torsion;
- both coefficients a=0/1, decomposition sizes 2/3/4, all assignments of a
  small coordinate system, and independence from intermediate variables;
- zero-row omission, contradictory-row retention and duplicate suppression;
- trace-on/off decomposition agreement with exhaustive factor-base search,
  including a two-torsion target, plus group-sum checks and export ordering;
- published ECC2K-130 and degree-83 E_0 fixture arithmetic, without target
  discrete logarithms;
- construction and independent rank checking of a normal basis in the exact
  challenge field, checking trace-one basis elements and trace-zero adjacent
  weight-two coordinates. This basis is not represented as the campaign's
  specific type-II basis conversion [pcc:unverified].

Commands and their outcomes are recorded in `VALIDATION.md`. Local self-checks
are not independent signed verification.

## Performance protocol and remaining dependencies

No benchmark is launched and no performance comparison is reported.
`PROTOCOL.md` retains the required matched comparison. `scripts/checkpoint.sh
gate` rejected an experiment launch because no task checkpoint was pushed.
This environment has no checkpoint upload token or local AWS credentials.
The GitHub workflow exists, but that alone does not discharge the checkpoint
or finite-resource admission requirements. Repository-variable inspection
also returned HTTP 403. Do not bypass these gates or relabel local tests as
an EKS experiment.

| Requested deliverable | Status |
| --- | --- |
| Repository integration | implemented and locally compiled; publication pending |
| Local correctness | 5 new + 6 existing focused tests pass; whole-library checks incomplete |
| Independent signed verification | pending |
| Matched solver comparison | blocked on experiment checkpoint/access |
| Degree-83 full-DLP comparison | not run |
| ECC2K-130 campaign measurement | not run; aggregate row redundant for the configured subgroup base by the analysis above |

No leaderboard, timing graph or progress metric is changed: this patch has no
admitted performance measurement. `flow.svg` is an applicability diagram,
not a performance plot.

## Sources checked

- Michiel Kosters and Sze Ling Yeo, *Notes on summation polynomials*,
  [arXiv:1503.08001v3](https://arxiv.org/pdf/1503.08001), Proposition 4.2
  (homomorphism and kernel), Proposition 4.7 (S3 non-2-torsion target),
  Remark 4.8 (higher-polynomial caveat and adding the condition directly).
  Primary PDF read; the screenshot is from this paper.
- `src/cryptanalysis/koblitz_index_calculus.rs`: existing SAT row, lifting,
  decomposition frontends and curve fixtures.
- `src/cryptanalysis/koblitz_groebner.rs`: exactness and variable limits.
- `src/cryptanalysis/koblitz_symmetrised.rs`: historical trace-disabled
  comparison with a different coordinate system, not a missing SAT feature.
- `src/binary_ecc/curve.rs`, `docs/ic/params/ecc2k130-fixed.json`, and
  `ecc2k130/codegen/indexcalc_fixed.py`: exact challenge and configured base.
- Sources checked (KB): KB unavailable: no MCP tools loaded;
  `scripts/research-kb status` exited 127, no executable. The above source
  paths and primary paper are the fallback. No novelty claim is made.

## Local outcome and discovered baseline limitation

All five focused trace tests and six existing decomposition/status regressions
pass in the final diagnostic build (release profile with package crypto
opt-level=0; native dependencies unchanged) [pcc:unverified]. The initial
ordinary optimized build passed four tests and exposed a baseline failure in
the fifth. Its output is preserved in checks/ rather than overwritten.

The failure was target (0,1), a=0, degree 7, m=3, with a valid group witness
consisting of three copies of the order-two point. The initial affine chain
reported no decomposition without exhaustion, even with trace disabled. Its
first pair sums to infinity, which the affine intermediate coordinates cannot
encode. The patch conservatively labels chained two-torsion targets unsupported
for m>=3 in Gröbner/F6, Crossbred, ordinary SAT and equation export. It does not
claim to implement missing infinity branches [pcc:unverified].

The whole-library suite is incomplete: its earlier-binary run was interrupted
after about 13 minutes, already carrying failures. Isolated reproductions
identify missing GPU source and a missing historical targetwalk record; other
reported failures remain unclassified. The final source has not been rerun at
ordinary root-package optimization. See VALIDATION.md and checks/ for exact
commands and outputs. These gaps are retained as publication dependencies.
