# ECBench end-to-end frontier refresh (2026-10-31)

This directory is the evidence-backed refresh of the elliptic-curve discrete-log
performance frontier requested on 2026-10-31.  It binds the review to source
commit `48c8a4767d2d1eed6a7b379f74410bc729239380`, inventories the live ECBench
registry, separates comparable domains, and prepares the next isolated run.

## Current answer

No admitted result on the repository's generic prime, random-binary, or
Koblitz ladders beats the matched, one-target, cold Pollard-rho reference.
That is the generic frontier boundary, not a claim that every component is
slower in every unit or that every curve family has the same boundary:

- the 74 live ECBench method IDs contain 31 IC, 32 rho/transfer, seven BSGS,
  two kangaroo, and two claw methods;
- the algebraic framework contains 11 nested solver engines, including F4,
  matrix F4, matrix F5, inherited F4, SAT-CDCL, FES/Gray code, wide FES,
  Crossbred, XL, Buchberger, and exhaustive search;
- current whole-pipeline IC results remain above matched rho on every admitted
  prime, random-binary, and Koblitz cell;
- F5 and F6 can reduce their own native solver work without moving the E2E
  frontier because relation generation, closure, setup, and unpriced work still
  dominate;
- prime `rootfind-subtract` is a generic table/search algorithm bounded by the
  square-root frontier.  It is not evidence of an algebraic sub-square-root
  attack;
- the 62-bit GPU explicit-relation result is a complete standalone L0
  measurement, but it has no matched ECBench rho arm and is therefore not a
  leaderboard win.

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
| `LITERATURE.md` | primary-source and repository evidence reviewed before the refresh |
| `PROTOCOL.md` / `protocol.json` | frozen comparison rules and machine-readable protocol |
| `specs/` | L2 ECBench specs for the best prime and Koblitz cells and their rho A/A controls |
| `execution/` | exact build, EKS request, checkpoint, and current blocker receipts |
| `REPORT.md` / `REPORT.pdf` | comprehensive frontier and bottleneck report |

## Execution status

The source builds locally as release binary SHA-256
`91f5895637501c6de257bbaf3afc9cbc2c0f5e964964d3dc4fc3bf78a427d96b`
with Rust 1.99.1.  Local execution is limited to spec validation,
correctness, and counted-operation preflight.  This workspace has no AWS CLI,
no AWS identity, no Iso Lab connector, and an invalid GitHub token, so it
cannot submit the frozen EKS run or publish a PR.  That infrastructure failure
is retained in `execution/BLOCKER.md`; no local wall result is substituted.

## Reproduction

From the repository root, after building `ecbench`:

```text
target/release/ecbench methods --json
target/release/ecbench plan --spec research/ecbench_e2e_frontier_20261031/specs/prime-p20.json --json
target/release/ecbench plan --spec research/ecbench_e2e_frontier_20261031/specs/koblitz-n13.json --json
```

The campaign uses six measured rounds so alternating arm order is balanced.
The EKS executor must run every spec in `execution/manifest.json`, preserve
failed and censored rows, replay all records, and only then rebuild the
frontier and report.
