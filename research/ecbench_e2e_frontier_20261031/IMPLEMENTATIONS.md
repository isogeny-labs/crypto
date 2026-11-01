# Closed implementation roster

This file defines what **all implementations** means for the 2026-10-31
frontier refresh. It is bound to source
`ff7ad61cfed11908fb79de9dd485b696d0ada779`, static x86-64 `ecbench`
SHA-256 `924d090c1d83a20ce954848c04b116cee4d5a7cb04187bb618dff60958bfd64d`,
and registry SHA-256
`85fa142069c6eb9a0ae65d112dac133fc8f6d606c15efe348476659f6b81b157`.

## Closure rule

The finite implementation census is the union of:

1. every one of the 74 top-level IDs in `method-registry.json`;
2. every named factor-base, decomposition-oracle, algebraic-solver,
   relation-matrix, target-source, and symmetry implementation reachable from
   the exact executable;
3. the specialized F_(p^4), F_((2^m)^4), transfer, Jacobian, GPU, and
   diagnostic paths already represented by a top-level ID or an explicitly
   separate study.

Numeric settings such as factor-base size, dimension, node budget, lane count,
or timeout are configurations, not new implementations. Their space is not
finite. The closed fresh campaign will therefore measure every named implementation on a
canonical applicable cell for correctness and accounting, but promotes only
the best prospectively selected configurations to the multi-size L2 frontier.
It never treats an unrun configuration as slow.

The 74 top-level dispositions and all 11 nested solver dispositions are in
`coverage.json` and `COVERAGE.md`. A top-level registry count alone is not a
closed census because `ic.pipeline` multiplexes the stages below.

## Factor-base implementations

| domain | names in the exact executable | interpretation |
|:--|:--|:--|
| prime | `explicit-x`, `prime-abscissa`, `prime-projected-abscissa`, `glv-orbit`, `prime-kummer`, `prime-isogeny`, `cm-scalar-orbit`, `known-log` | eight named builders |
| binary/Koblitz built-in | `explicit-x`, `binary-subspace`, `koblitz-orbit`, `compact-orbit-scan`, `cyclotomic`, `onb-weight` | six named paths; prime and binary `explicit-x` are distinct implementations |
| binary-linked plug-ins | `koblitz-standard-subspace`, `binary-standard-subspace`, `koblitz-closure`, `structured-s4-base-v1` | the two standard-subspace names alias one builder; closure and structured S4 are separate |
| specialized extension fields | `fghr-y-line`, `bin-t-line` | dedicated F_(p^4) and two-word F_((2^m)^4) pipeline bases |

Every measured factor-base row must retain its `FB1` identity, raw/effective
column counts, construction cost, known-log/advice status, and whether the
base is closed under the symmetry used by the oracle and reference.

## Decomposition and relation-oracle implementations

| domain | names / engines |
|:--|:--|
| prime | `pkm-semaev`, `subtract`, `mitm`, `ksum`, `mitm-prime-batch`, `mitm-prime-blocked`, `mitm-fiber`, `rootfind-mitm`, `rootfind-pairs`, `rootfind-subtract` |
| binary/Koblitz built-in | `subtract`, `mitm`, `mitm-frobenius`, `mitm-frobenius-counted`, `mitm-frobenius-lazy-counted`, `mitm-frobenius-layout-counted`, `mitm-fiber`, `mitm-frobenius-fiber`, `rootfind-mitm`, `rootfind-pairs`, `descent-algebraic`, `descent-algebraic-fiber` |
| binary-linked plug-ins | `pdp3-koblitz` / `pdp3-binary` (aliases), `pdp-split`, `structured-s4-oracle-v1` |
| PDP3 engines | `inherited-f4`, `f6-ic`, `matrix-f4`, `matrix-f5`, `rootfind`, `rootfind-window`, `rootfind-negfold`, `rootfind-window-negfold` |
| specialized extension fields | `gaudry-d4`, `gaudry-d4-bin` |

`subtract`, `mitm`, `rootfind-*`, pair tables, and `ksum` are retained in the
generic table/search track. A faster row there is engineering progress but is
not an algebraic sub-square-root result. `pkm-semaev`, `descent-algebraic`, and
the Groebner PDP3 engines form the algebraic track. The tracks may be compared
on the same complete E2E object but ratchet separately.

## Algebraic solver implementations

The exact finite solver registry has 11 backends:

| backend | role and native boundary |
|:--|:--|
| `f4-f2` | Boolean F4; elimination word XORs |
| `buchberger-f2` | reference Buchberger; polynomial reductions |
| `matrix-f4` | matrix F4; partial matrix word work |
| `matrix-f5` | F5-signature-filtered matrix F4; partial matrix word work |
| `inherited-f4` | inherited-variable F4; partial word work |
| `crossbred-f2` | Crossbred hybrid; backend-native work |
| `xl-f2` | XL; modelled monomial work |
| `sat-cdcl` | CDCL SAT with CNF/native-XOR choices; conflicts are local units |
| `fes-f2` | scalar fast exhaustive search; Gray-code word XORs |
| `fes-f2-wide` | SIMD FES; vector XORs, at most 32 equations |
| `exhaustive` | general-degree reference; short-circuited monomial tests |

F6 is an oracle engine around inherited F4 rather than a twelfth
`SystemSolver`. WDSAT is historical repository evidence, not a live solver in
this exact registry; it remains a separately labelled failed/diagnostic path.
No ranking converts SAT conflicts, FES vector XORs, or partial F4 word XORs
into one synthetic unit. Cross-backend rank requires complete common pricing
or matched verified L2 E2E wall.

## Relation-matrix implementations

The exact pipeline accepts nine names representing eight underlying choices:

- `incremental-gauss`;
- `incremental-gauss-full-rank`;
- `incremental-gauss-full-rank-checked`;
- `structured-gauss`;
- `sparse-cado` and its alias `filter+wiedemann`;
- `filter+lanczos`;
- `wiedemann` without filtering;
- `lanczos` without filtering.

The stopping rule (target pin versus full rank) is part of the implementation
identity and cannot be changed after observing a result.

## Target-source implementations

The exact pipeline has 14 target sources:

`random`, `walk`, `walk-prime-fast`, `walk-prime-serial-4`,
`walk-prime-serial-8`, `walk-prime-serial-16`, `walk-prime-batch-4`,
`walk-prime-batch-8`, `walk-prime-batch-16`, `residual`, `stride`,
`stride-renew`, `stride-generator`, and `stride-renew-deferred-guard`.

These are not interchangeable target laws. In particular,
`stride-generator` uses a known generator-orbit column and belongs to a
fixed-setup engineering panel, not the uniform-target scaling panel.

## Symmetry and strongest-reference rule

The census includes plain/negation, cubic (j=0), quartic (j=1728), Frobenius,
signed-Frobenius, GLS, strong-walk, escape, full-price, and word-RAM paths
through the top-level IDs in `method-registry.json`. Every candidate travels
with the strongest applicable accounting-correct reference on the same curve,
subgroup, target law, advice scope, and unit:

| cell | required reference set |
|:--|:--|
| generic prime or random binary | at least `rho.negation_v2`; retain finite-size BSGS/kangaroo where applicable |
| j=0 | `rho.cubic_v2` plus its A/A control |
| j=1728 | `rho.quartic_v2` plus its A/A control |
| Koblitz | `rho.signed_frobenius_strong_v2` and `rho.signed_frobenius_strong_escape`; choose only by preregistered common accounting, not post-hoc wall time |
| F_((2^m)^4) | `rho.negation_strong` in F_(2^m)-multiplication equivalents |
| transferred/Jacobian object | a reference on the same resulting object plus every charged transfer cost; never silently substitute it for the source-curve problem |

## Measurement tiers

| tier | scope | what it can establish |
|:--|:--|:--|
| census | all 74 IDs and every named nested stage above on one applicable fixture; retain unsupported, diagnostic, timeout, OOM, and wrong-answer outcomes | reachability, correctness, accounting, and explicit non-applicability |
| component | matched factor-base/oracle/solver/matrix changes with identical inputs | local implementation frontier only |
| E2E finite-size | complete cold one-target candidate/reference/A/A shards | a frontier point in that exact domain |
| scaling | at least four sizes, same law and implementation | qualified trend; still no cryptographic extrapolation without a validated model |

The current nine-spec draft covers only a subset of these cells and plans
4,704 process executions. It is therefore a planning artifact, not the closed
campaign. It must be split into one-hour candidate/reference/A-A shards and
expanded with random-binary, j=0, j=1728, generator-orbit, wide F_((2^m)^4),
and specialized-domain panels before prospective freeze.

## Current status

The exact executable and image are published and the registry is snapshotted.
No fresh L2 measurement has been launched: the mandatory checkpoint gate
cannot upload because this workspace has no checkpoint token or AWS transport.
All fresh states therefore remain `blocked_checkpoint_gate`; none is assigned
zero cost, a timeout, or a loss.
