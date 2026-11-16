# Matched actual F4 instrument

Track 2 of the user's parallel request. This directory contains a native
runner for the **unchanged msolve-neogb F4 implementation**, exact exporters,
verification and raw-trace retention. No elliptic-curve panel was run during
implementation. Execution remains subject to the common protocol and an actual
local/checkpoint admission receipt from the parent session.

## Backend and primary sources

- Official [msolve repository](https://github.com/algebraic-solving/msolve)
  README: input format, reduced degree-reverse-lexicographic bases, verbosity
  and threading conventions.
- Official [msolve site](https://msolve.lip6.fr/): F4 backend and release 0.11.0.
- Inspected upstream `src/neogb/{f4.c,f4.h,engine.h,data.h,meta_data.c,convert.c}`.
  Source revision **3672113ee2c696a2ac9c637636e0f7214f500ba1** is pinned by the
  build script. `neogb_driver.c` follows the same
  `initialize_gba_input_data -> core_f4 -> export_results_from_f4` sequence as
  upstream `export_f4`, then exposes the basis and statistics before cleanup.
  The upstream source itself is not modified.
- Repository context: `SECANT_GRAPH.md` and the existing ordinary-prime F4
  implementations `src/cryptanalysis/{f4_prime_dynamic.rs,f4_fp.rs}`. This
  instrument uses the external backend directly rather than relabeling a
  different Gröbner algorithm as F4.

The full msolve CLI build requires FLINT, MPFR and autotools, which were absent
on the worker. Its stock neogb component builds with the installed GMP/GCC.
The adapter is **msolve-neogb**, not the full CLI, FGLM, Magma or Singular.
No Magma executable/license or Singular executable was found. A single
deterministic msolve-neogb linear-algebra configuration is frozen; no claim
that it is the fastest external configuration is made.

## Build

```sh
git clone https://github.com/algebraic-solving/msolve.git MSOLVE-SOURCE
git -C MSOLVE-SOURCE checkout 3672113ee2c696a2ac9c637636e0f7214f500ba1
bash build-neogb.sh MSOLVE-SOURCE FRESH-BACKEND-DIR
cargo build --release --locked --manifest-path Cargo.toml
```

Build flags: GCC `-std=gnu11 -O3 -march=native -DHAVE_AVX2 -mavx2 -fopenmp`,
linked with GMP and libm. Run only on the matching x86 AVX2 host, or rebuild
there. Stock configuration: DRL variable order `x,z[,m]`, one thread,
linear-algebra option 2, reduced GB enabled, hash-table exponent 12, all pairs
selected. Inputs are ordinary Fp polynomials; no Boolean quotient is used.
Mode 2 dispatches to upstream exact sparse elimination (`io.c`), rather than
the probabilistic modes 42/43/44. The adapter also fixes `srand(315)` and
records that seed; the full CLI's time-based seeding code is not linked.
In DRL with no elimination block, upstream `update.c` sets the selected-pair
degree to the total LCM degree, not an inherited sugar value. That precise
selection statistic is retained without claiming full polynomial-row tracing.

Prepared local paths:

- External source: `/workspace/secant-msolve-src`.
- Adapter: `/workspace/secant-f4-backend/neogb-driver`.
- Runner: `target/release/secant-f4-panel` after the build above.

## Input contract

Consume a flat JSON bundle `{schema,systems:[...]}`. Each record has:

```text
id, p, variables, polynomials,
curve:{p,a,b,...}, target:[r,s], base_x:[...], base_points:[[x,y],...],
arm: candidate-dense|candidate-secant|control-dense|control-secant,
signs:null|[epsilon,eta], h_low_to_high:[...], expected_roots(optional)
```

A polynomial is an array of `{exponents:[...],coefficient:c}` with canonical
nonzero coefficients and distinct monomials. Dense systems use **two variables**
and two exponents; the runner rejects a free third slope coordinate. Secant
systems use three variables. Both signs must be present in the base inventory;
all four sign branches are separate cells. Case-producer metadata such as d,
control name, FB1 and curve identity is retained in each copied `input.json`.

The runner independently reconstructs the complete intended polynomial model,
requires p<=257 for this complete toy population, and checks that every base
includes all affine targets, one dense system and all four distinct secant
sign branches per target. Missing or duplicate branches fail preflight.
The reconstructed model includes exact control membership constraints; every
coefficient is compared before handing it to F4. Candidate secant membership is accepted only when the
complete split-graph factorization holds. It exports both the adapter token
format and official `.ms` text, with no repeated monomials.

## Bounded execution and admission

The parent's admission record must be created only after actual authorization.
Required fields include:

```text
backend:"local", checkpoint_override:true, backend_override:true,
user_authorization_received:true, max_seconds:1800 or a smaller positive cap,
address_space_bytes:2147483648,
source_revision:<actual Git HEAD>, protocol_path:<frozen protocol>,
protocol_sha256:<actual hash of that file>
```

The runner checks those bindings and the process address-space limit before
launching. No example acceptance receipt is supplied here. An admission flag
does not itself constitute user approval.

After admission and source freeze, from the repository root:

```sh
timeout --signal=TERM --kill-after=5s 1800s bash -c '
  ulimit -v 2097152
  exec research/prime_factor_base_degree_20261031/parallel_20261031/f4/target/release/secant-f4-panel \
    run SYSTEMS.json /workspace/secant-f4-backend/neogb-driver FRESH-OUTPUT \
    --admission ADMISSION.json
'
```

The panel is one native process and one F4 thread per CAS child, with a 10-second
timeout per child, a 2 GiB inherited address-space cap and a hard 1,800-second
outer cap. It refuses overwrite, records failures and leaves every partial raw
file. Each case immediately gets its complete `result.json` and raw
`input.json`, `input.tokens`, `input.ms`, `solver.json`, `solver.stderr`.
The final summary is a compact manifest of per-case result paths and hashes;
it is not the only evidence. Large execution trees follow repository artifact
policy; the parent retains/archives them after execution.

## Metrics and verification scope

- Keep input-generator maximum degree, actual selected-pair degree trace/max,
  and maximum exported-basis degree separate. The explicit observed convention
  is `max(input maximum, selected-pair maximum)`. It is not homogeneous
  regularity, sugar degree, or a complete abstract solving-degree theorem.
  If upstream reports reduced pairs but no degree row parses, that metric is
  unavailable rather than silently crediting the input degree alone.
- Preserve every raw F4 round. Parse all numeric matrix shapes, including
  intermediate and final basis reductions, for observed row/column maxima.
  Upstream `mat_max_nrows/ncols` stores the **largest-area matrix's dimensions**,
  not independent peak rows and columns; adapter field names say so.
- Stock pair/row/zero-reduction counts are parsed from the first **core**
  computational-data block in raw
  stderr. `core_f4` frees its local statistics without propagating these
  counters into the export context, so that context's zero counters are never
  credited as workload; raw adapter counter fields are null until the native
  runner fills them from the retained trace. Full field-add/multiply
  totals are **null**, because this configuration does not expose complete
  counters. Process peak RSS and block I/O are whole-process diagnostics.
- Solver internal phase timers exclude process launch/input parsing; outer
  process time includes them. Every clock is **L0 shared-host diagnostic only**.
  No speed claim or timing-ratio acceptance gate is based on them; L2, warmup,
  five A/A pairs and the fastest available matched reference remain necessary
  for a qualified timing comparison.
- Compute normal forms of all input generators against the exported GB and
  verify every S-pair (Buchberger criterion). Extract rational roots from the
  computed GB by recursive univariate specialization, with a two-million-value
  cap; capped extraction is unfinished, not UNSAT.
- Independently enumerate signed base-point group sums and intended ±h branch
  orientation. The reference never filters by input equations; it separately
  rejects equations excluding any valid reference relation. Compare the full
  extracted root set, producer expected roots where provided, SAT/UNSAT class,
  and exact group replays. Doubles are included. Ordered signed relation count
  is not an independent matrix-rank count.
- Input-generator reduction, S-pairs and exact finite-field root-set equality
  are checked. Full ideal equality including multiplicities is **not certified**;
  no claim that exported basis elements have certified membership in the input
  ideal is substituted for that missing direction.

## Development verification

Eight native tests cover DRL order, normal forms/S-pairs, artificial triangular
root extraction, unit ideal, trace parsing (including final/intermediate matrix
width), canonical inputs, regression against filtering the reference by bad
equations, and coefficient equality with the committed secant input fixture.
A ninth test pins a real adapter smoke on the artificial ideal
`x²−1,z−x` over F17, checks its exported GB and exactly two roots. Raw software
smoke stdout/stderr are in `development/`; this is not an elliptic-curve panel.

Preserved preparation failures: missing full-CLI dependencies; first strict
Clippy pass reported type-complexity/manual-is-multiple-of; after adding exact
model reconstruction, the intentionally bad-model test initially still expected
validation to succeed, then was corrected to require rejection while retaining
its independent valid reference. No assertion or research gate was weakened.
An additional artificial two-batch smoke exposed the outer-context counter
trap above: real core counts were two pairs/seven rows, while the outer context
held zeros. The correction and the raw two-batch trace are pinned by a parser
regression. Earlier and corrected software-smoke records are retained separately.

No curve run, degree comparison, natural yield or end-to-end improvement is
reported by this preparation document.
