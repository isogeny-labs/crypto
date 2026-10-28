# Original-scope coverage and remaining work

This is an implementation handoff, not a completed-corpus certificate. Read the
committed test log, frozen campaign manifest, per-cell outcomes and S3 receipts
for executed coverage. The first centralized dataset test run found five
failures; fixes for even-degree lifting, repeated-prime subgroup extraction and
small-field cardinality have been applied, with the subsequent centralized run
still responsible for validating them. No performance comparison follows from
these correctness tests.

## Preserve the whole campaign

Keep every requested cell: Koblitz, independent general binary and prime fields;
both quadratic-twist positions; n=23..31 and 47..61; p=2^61-1; m=2..15;
multiple curve seeds, generators and whole-subgroup nonzero target scalars.
Unsupported methods, empty domains, trace caps, timeouts and failed construction
remain rows in this campaign. A successful known-answer construction is not an
independent ECDLP solve, and a completed bounded wave is not the requested
enormous corpus.

## Concrete implementation obligations

| Area | Current implementation boundary | Remaining acceptance condition |
|---|---|---|
| Curve construction | Native arithmetic, Koblitz trace recurrence, exact Hasse certificates for general curves, exhaustive small-field certificates, trace-one binary twists and nonsquare prime twists | Run requested-boundary tests and a frozen all-degree correctness wave; independently replay every order and scalar certificate, including repeated largest-prime factors and rank-two rational torsion |
| Prime cofactors | `CurveParams.h` is a u32; larger exact cofactors are explicitly unsupported | Introduce an exact-cofactor representation for generic solver adapters, without truncating or replacing the selected curve |
| Repeated prime factors | Remove all r factors from the clearing scalar, then trim the sampled r-primary point to exact order r; repeated-r membership uses bounded BSGS in the actual generated cyclic subgroup | Preserve and charge this diagnostic setup separately; membership logs remain private; validate membership against the generated subgroup rather than accepting all rational r-torsion |
| Factor bases | Small-x lifting, per-point prime-primary projection, signed membership and exact orbit records | Add independently declared polynomial-subspace, Frobenius-union, quotient, PKM and learned base policies as separate populations, with rejection probabilities, selection costs and matched solver domains |
| General ECDLP recovery | Independent bounded BSGS exists for all three families; native large-prime scalar recovery initially admits actual F2-coefficient Koblitz models | Add actual index-calculus relation collection, modular rank, descent and scalar recovery for general binary and prime models; all m and twist cells remain visible until admitted |
| Double large primes | Explicit public residual pool, exact pair lookup, native signed-Frobenius elimination, source coefficients, rank and independent recovery verification | Generalize residual discovery/column representation to non-F2 binary twists and prime curves; export internal pivot snapshots and every reduction operation; retain ineffective and capped graph runs |
| Binary SAT | Chained S3 finite-domain circuits are admitted only under explicit variable/clause/model/work caps, with independently checked point-sign lifts | Export CDCL decision/trail/reason/conflict/learned/deleted-clause/restart streams and proof artifacts; independently check UNSAT proofs; extend practical admitted coverage at m=2..15 without interpreting a cap as UNSAT |
| Prime SAT | No prime-field SAT adapter is yet declared complete | Integrate an actual prime-field bit-vector/modular encoding with exact domain maps, verified models and proof-checkable outcomes |
| F4/F5 | Native Boolean matrix engines and prime F4 have scoped adapters; Boolean exact selector systems currently use m=2 | Integrate larger-m native algebra with exact ring/quotient/order maps and export matrices, row provenance, pivots, syzygies, F5 signatures and criteria; distinguish Boolean matrix F5 from general incremental F5 |
| F6 | Native geometric four-sum index has its own declared m=4 lane | Expose each implemented F6 algebraic/geometric variant separately, document its exact operation, and integrate the remaining arities without changing what the F6 name denotes |
| Root solvers | Native S3 univariate root routes are m=2 adapters | Export factorization/GCD/intermediate root states; integrate higher-m elimination/root extraction and certify completeness separately from finding one valid root |
| Weil descent and PKM | Boolean exact selector restriction is scoped to m=2; native prime Kummer-tower PKM adapter has m=2/3 and modulus restrictions | Preserve full tower/basis/modulus/quotient maps; extend legitimate native coverage to requested higher m and larger primes, including p=2^61-1; do not substitute a PKM audit score or a different factor base without recording it |
| Fall degrees | Exact first and last fall degree remain null | Define each invariant for the actual ring and field equations; export degree-fall witnesses, complete lower-degree exclusion/closure evidence and independently checked counterexamples; formalize universal claims separately |
| Measurements | Typed native counters and worker-level measurement mechanisms have distinct scopes | Admit and verify PMU instructions with event configuration, enabled/running time, thread/process coverage and readback; preserve unavailable/failed counter collection explicitly; add missing phase/peak-memory/I/O/checkpoint accounting |
| Complete intermediate artifacts | Curve construction and several native solver adapters still export summaries rather than every internal state | Add incremental event/chunk sinks inside long native calls, export Hasse baby/giant tables and integer-factorization internals, and test truncation/exhaustion as explicit terminal outcomes |
| Durable publication | Plan/lease/shard/content-addressed publication machinery is under integration | Demonstrate actual bounded generation, kill/resume without duplication, immutable S3 upload plus download-and-hash receipts, source/binary identity, finite execution limits and committed indexes |
| ML consumer | Public arithmetic inputs and private scalar/provenance artifacts are separated; current split component conservatively groups an entire n | Implement a strict feature allowlist that excludes seeds, key/generator indices, private scalars, membership logs, post-decision state and scalar-search tables; construct merged transfer/isomorphism/isogeny components across field sizes and frozen holdouts |
| Training and novel solvers | No trained-model efficacy or novel SAT solver is established by the generator | Materialize a validated training view, freeze held-out benchmarks and controls, train candidate policies, verify each proposed action with exact solvers, charge inference and fallback, and report failures and uncertainty |

## Next discriminating validation

1. Complete centralized native tests, including n23/24/31/47/61 for each family
   and twist, the n11/n12 repeated-prime controls, prime n7 cardinality, and a
   genuine large-prime recovery test whose private witness is absent.
2. Re-run tamper checks on wrong curves/twists/scalars, factor-base membership,
   source coefficients, residual-pool setup, unused pair-table entries, arity,
   declared budgets, recovered factor-base logs and subgroup identity.
3. Freeze a correctness pilot that names all requested cells even when only a
   bounded subset is executed in the first wave. Export outcomes by family,
   degree, twist, m, solver, subgroup size and construction/replay status.
4. Run that pilot on the authorized finite backend, checkpoint actual artifacts
   to the approved S3 bucket, verify round trips, and commit receipts and a
   source-linked report. Only then expand finite waves using verified cells and
   preserve the unsuccessful cells as research data.

The immediate reproducible failure modes are implementation limits, not claims
that index calculus, a twist, a fall-degree conjecture or machine learning cannot
work at the requested parameters.
