# ECBench end-to-end frontier refresh (2026-10-31)

This directory is the evidence-backed refresh of the elliptic-curve discrete-log
performance frontier requested on 2026-10-31. It binds the executable audit to
source commit `ff7ad61cfed11908fb79de9dd485b696d0ada779`, inventories both the
top-level registry and the named implementations multiplexed by `ic.pipeline`,
separates comparable domains, and prepares the next isolated run. Historical
performance rows keep the source and binary that actually produced them.

## Current answer

No admitted result on the repository's generic prime, random-binary, or
medium/scaling Koblitz ladders beats the matched, one-target, cold Pollard-rho
reference. That is the generic frontier boundary, not a claim that every
component is slower in every unit or that every curve family has the same
boundary:

- the 74 live ECBench method IDs contain 31 IC, 32 rho/transfer, seven BSGS,
  two kangaroo, and two claw methods, but that count alone is not a census of
  the factor-base, oracle, target-source, and linear-algebra implementations
  hidden behind the pipeline IDs;
- the algebraic framework contains 11 nested solver engines, including F4,
  matrix F4, matrix F5, inherited F4, SAT-CDCL, FES/Gray code, wide FES,
  Crossbred, XL, Buchberger, and exhaustive search;
- current whole-pipeline IC results remain above matched rho on every admitted
  prime, random-binary, and Koblitz cell;
- on the historical F4/F5/F6 experiment binary, F5 and F6 reduce native solver
  work without moving the E2E frontier because relation generation, closure,
  setup, and unpriced work still dominate. A fresh current-binary audit
  reproduces only 72/144 deterministic rows, so those historical counts are
  not relabelled as current-source measurements;
- prime `rootfind-subtract` is a generic table/search algorithm bounded by the
  square-root frontier.  It is not evidence of an algebraic sub-square-root
  attack;
- the 62-bit GPU explicit-relation result is a complete standalone L0
  measurement, but it has no matched ECBench rho arm and is therefore not a
  leaderboard win.

A narrow Koblitz engineering crossover must remain explicit. On the n=29,
one-column, fixed-setup generator-orbit panel, the Frobenius-layout pipeline's
S band is 0.546/0.611/1.196 versus 1.569 for rho-frob, with paired ratio 0.366
[0.241, 0.523]. The repository classifies that pipeline at one column as
Frobenius-folded BSGS; the crossing rests on rho's fixed setup at log2(r)=15.4
and the gap grows with r. It is not a uniform-target, medium-tier, or
asymptotic IC win.

There is one important, deliberately separate exception.  The plane-quartic
attack on the special GHS-weak `E_0/F_{q^3}` family is below matched rho in its
own cold one-target, `F_q`-multiplication-equivalent comparison.  It is a real
frontier for that restricted family, but not for generic curves, prime-order
`E/F_{q^3}`, any standard curve, or ECC2K-130.  Combining that result with the
generic ladders would compare different problem classes and give a false
global ranking.

The newest validated factor-base improvement is the 20-bit prime row.  On the
second-host EKS replay, `prime-abscissa:size=45` costs 0.836, 0.817, and 0.746
times the N=32 incumbent at native-operation prices 0, 0.1, and 1 GAE.  It is
still 7.95, 8.24, and 9.62 times matched rho.  This is one curve and is not a
scaling result.

## What “frontier” means here

A row may enter a ranked frontier only when all of the following travel with
it: the same ECDLP object and target law, a cold one-target scope, a strongest
applicable rho arm, complete or explicitly banded accounting, verified answers,
retained failures/timeouts, at least eight targets, at least five measured
rounds, an A/A control, and L2 isolation for wall time.  Scaling additionally
needs at least four sizes.  Missing or unpriced evidence is unranked; it is not
zero cost and not a loss.

The ranked object is Pareto, not a single time column: counted work, isolated
wall time, peak memory, preprocessing/advice, success/censoring, and evidence
level are all retained.  Native F4 word XORs, SAT conflicts, FES Gray-code XORs,
and GAE do not become comparable merely because they are numbers.

## Files

| file | purpose |
|:--|:--|
| `method-registry.json` | exact `ecbench methods --json` snapshot from the pinned release binary |
| `coverage.json` / `COVERAGE.md` | disposition for every live method and every nested solver backend |
| `IMPLEMENTATIONS.md` | closed roster of factor bases, oracles, solver engines, matrix engines, target sources, and symmetry references |
| `LITERATURE.md` | primary-source and repository evidence reviewed before the refresh |
| `PROTOCOL.md` / `protocol.json` | draft comparison rules and machine-readable protocol; not yet frozen |
| `specs/` | nine exact-build planning specs; incomplete and not yet queue-safe |
| `execution/` | exact publication identity, plan-only receipt, checkpoint, and current blocker receipts |
| `establishment/` | executable claim certificate and fresh all-replay receipts |
| `REPORT.md` / `REPORT.pdf` | comprehensive frontier and bottleneck report |

## Execution status

Trusted publication run 41241104247 built source
`ff7ad61cfed11908fb79de9dd485b696d0ada779` as static x86-64 release binary
SHA-256 `924d090c1d83a20ce954848c04b116cee4d5a7cb04187bb618dff60958bfd64d`
and ECR image digest
`sha256:b8c20f9c151360b78f505360e8d47068c4068f00a8f6faa7ebd3086fa0a791c6`.
That workflow was publication-only and launched no experiment.

Local execution is limited to registry capture, spec validation, correctness,
and counted-operation preflight. GitHub publication now works, but the
mandatory `scripts/checkpoint.sh gate` cannot pass: this workspace has no
`CHECKPOINT_UPLOAD_TOKEN`, checkpoint token file, AWS CLI, or AWS identity.
The existing nine specs also require expansion and one-hour sharding before
freeze. Both blockers are retained in `execution/BLOCKER.md`; no local wall
result is substituted.

The follow-up establishment audit passed 19/19 cross-artifact checks. The
optimized-rho session reproduced 720/720 deterministic rows with the current
binary. The historical F4/F5/F6 sessions reproduced 72/144, with all failures
retained in `establishment/receipts/`; this narrows those solver claims to the
original measured binary until the closed current-source campaign runs.

## Reproduction

From the repository root, after building `ecbench`:

```text
target/release/ecbench methods --json
target/release/ecbench plan --spec research/ecbench_e2e_frontier_20261031/specs/prime-p20.json --json
target/release/ecbench plan --spec research/ecbench_e2e_frontier_20261031/specs/koblitz-n13.json --json
```

The final campaign will use six measured rounds so alternating arm order is
balanced. Before submission, the executor must close the roster in
`IMPLEMENTATIONS.md`, split candidate/reference/A-A shards to fit the one-hour
queue envelope, prospectively freeze them on `main`, pass the checkpoint gate,
preserve failed and censored rows, replay all records, and only then rebuild
the frontier and report.
