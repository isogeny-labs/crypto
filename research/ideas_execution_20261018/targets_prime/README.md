These native diagnostics execute parts of audit items 1 and 14. Both original requirements remain **partial**. The runs establish fixture arithmetic and constrained four-point decomposition cells. They do not establish an improved index-calculus pipeline or a small-root method.

| Requirement | Executed evidence | Remaining work |
|---|---|---|
| Item 1: paired Fp² ecbench ladder with torsion, Q-curve folding and double large primes, followed by FourQ extrapolation | 24 exact random-curve fixtures at four primes; same-instance two-sum table diagnostic and verified rho; cofactor/torsion projection checks | ecbench integration, Gaudry algebraic oracle, Q-curve/GLS map and invariant-base certificate, 2LP ablations, full IC extraction, cold cost in a common unit, observed peak memory and family-specific extrapolation |
| Item 14: prime m>=4 constrained multivariate cost cells | 480 natural m=4 targets on 15 prime-order curves; exact MITM versus exhaustive enumeration; separate strict-distinct-abscissa arm; candidate multivariate chained-S3 systems and verified point certificates | lattice/Coppersmith method, GB solving-degree instrument, full S5 expansion if needed, complete ECDLP pricing and additional arities |

The protocol files were hashed before their respective research runs. No autoresearcher contract or frozen record was changed. The historical `EXP-ALPF-016` already includes prime-field m4 S5 cells at 13/15/17 bits; its source verdict is `failed`. Its graded-rank quantities have not been revalidated here. The blanket claim that m4 has zero historical cells is therefore unsupported. `EXP-PFGB-d630f6` is explicitly unapproved for execution; these standalone diagnostics do not run or supersede it.

Read [the descriptive report](reports/REPORT.md), [PDF](reports/REPORT.pdf), and [plot source](render.py). Exact code, pre-run protocols, commands, compiler/host metadata, raw results, systems, certificates and failure records are under this directory and `runs/`.

An [additive internal review](../review/REPORT.md) strengthens verification to enforce target binding, exact declared-hit coverage and strict distinctness. It also fixes a latent prime-subfield constant bug in a new, separately frozen driver under `verification_v2/`; the corrected 24-fixture Fp² replication reproduces all original result objects. Original sources below are retained for historical reproduction; use the amended driver for new sampling.

Reproduce with a C++20 compiler:

```sh
g++ -std=c++20 -O2 -Wall -Wextra -Wpedantic diagnostic.cpp -o /tmp/ideas-targets-prime-diagnostic
g++ -std=c++20 -O2 -Wall -Wextra -Wpedantic verify.cpp -o /tmp/ideas-targets-prime-verify
g++ -std=c++20 -O2 -Wall -Wextra -Wpedantic strict4.cpp -o /tmp/ideas-targets-prime-strict4
/tmp/ideas-targets-prime-diagnostic --test
/tmp/ideas-targets-prime-diagnostic FRESH_OUTPUT_DIRECTORY
/tmp/ideas-targets-prime-verify FRESH_OUTPUT_DIRECTORY
/tmp/ideas-targets-prime-strict4 ANOTHER_FRESH_OUTPUT_DIRECTORY
/tmp/ideas-targets-prime-verify ANOTHER_FRESH_OUTPUT_DIRECTORY
```

Both research commands were limited to 512 MiB address space and 300 CPU seconds. They completed with exit 0. The shared host ran other jobs, so no wall-time or speedup number is reported. Group-operation calls, field multiplication calls and field inversion calls remain separate units. These counters omit native integer/factorization/hash/allocation costs; they are not an end-to-end cold `S`. In particular, a table lookup with zero group calls still has nonzero work.
