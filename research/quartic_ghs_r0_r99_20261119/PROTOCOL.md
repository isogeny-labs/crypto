# R0–R99 execution and composition protocol

Status: frozen before range prototypes and receipts were inspected by the
coordinator. Prospective acceptance criteria are not results
`[pcc:unverified]`.

## 1. Bound baseline

| Object | Frozen value |
|:--|:--|
| Campaign-start commit | `67e251c8c60bf8e3a9d5bea5da3fc6c480a16bd8` |
| Composed rho source | `research/quartic_ghs_strong_rho_20261114/strong_rho.rs` |
| Rho source SHA-256 | `724f1c433b465818da6b0ee6d0ad7deb01434c6bb114e87dae4e304fce4bd340` |
| CLI source SHA-256 | `57467c78eac968b22d7fe44c7380652ecabc2398616840f3528c0e561f0e2905` |
| Rust compiler | `rustc 1.100.0 (0ef1bc386 2026-11-09)` |
| Local deterministic control | public synthetic `q=223`, seed 1, target 0 |
| Eventual comparison | frozen cold one-target matched grid in `IMPLEMENTATION_SPEC.md` |

The local cell is a correctness and deterministic-count control only. No
development clock from that cell is admissible as a performance result.

## 2. Requirement-to-evidence contract

| User requirement | Required evidence | Completion rule |
|:--|:--|:--|
| Do R00–R99 | Exactly 100 unique IDs with no gap or duplicate | Native validator accepts all six immutable fragments |
| Run them in parallel | Disjoint batch ownership and retained start state | All six range batches are dispatched before any batch is promoted |
| Implement candidates | Source/prototype, exact command, output, and falsifier | The per-lead implementation gate passes or the failure is retained |
| Preserve correctness | Differential oracle, planted-log recovery, trajectory or state invariant as applicable | Exact equality on the preregistered deterministic controls |
| Optimize the comparator | Isolated source-bound A/A and ABAB measurements | Complete admitted record; local timing is insufficient |
| Establish the frontier | Same-target cold IC/rho grid and complete ECBench inventory | Replay, validator, matched rho, PCC, and publication gates all pass |
| Keep the leaderboard current | Regenerated canonical twins in the same PR | Only a validated eligible measurement may change a row |

## 3. Lead states

Every registry entry uses one of these states:

- `executed-pass`: its frozen deterministic implementation gate passed;
- `executed-reject`: it ran and failed its gate or had a negative count delta;
- `implemented-unmeasured`: implementation passed correctness but awaits
  admitted performance measurement;
- `screened`: a formula/profile/dependency screen completed without a runnable
  performance claim;
- `blocked`: an external capability or missing prerequisite prevents execution;
- `queued-after-dependency`: the lead is specified but mutating the baseline
  before its named predecessor would invalidate the comparison.

No entry may use `complete` unless every gate named by that entry is satisfied.

## 4. Parallel execution rule

R00–R99 are divided into six non-overlapping artifact ranges. All ranges may
run algebraic, static, deterministic, and source-analysis controls at once.
They may not concurrently edit the authoritative composed rho source.

Accepted candidate patches are replayed against the frozen baseline one at a
time. Count-moving patches compose first. Wall-only patches remain separate
until counted output is identical. After each accepted patch, the composed
source becomes the next candidate baseline only after self-test, planted-log,
trajectory, authority, and analyzer gates pass. Standalone deltas are never
summed.

## 5. Deterministic screening envelope

Each runnable prototype records:

1. exact source revision and SHA-256;
2. compiler and complete command;
3. public deterministic inputs and seeds;
4. stdout, stderr, and exit status;
5. an independent oracle or invariant;
6. negative/failure cases rather than only successful output;
7. the lead state justified by that output.

Rust is the default implementation language. Python is not used for field,
curve, walk, solver, benchmark, replay, or result-generation work.

## 6. Performance and promotion gates

At campaign start, `scripts/checkpoint.sh gate` could not upload because this
runtime had no approved profile or upload token, and `conductor` was not
installed `[pcc:unverified]`. Therefore no performance launcher, paid job,
profile-derived ranking, or leaderboard mutation is admitted in the local
wave.

Once `problems/ghs-strong-rho-measurement-admission.md` is resolved, admitted
candidates use the exact q grid, seeds, repetitions, calibration, resource
limits, timeout policy, and bootstrap analysis frozen in
`research/quartic_ghs_strong_rho_parallel_20261118/IMPLEMENTATION_SPEC.md`.
Parallel rho must report elapsed time, total CPU, total steps, overshoot,
distinguished-point traffic, memory, and verification cost together.

## 7. Sources checked

The shared research-KB wrapper reported no executable. The fallback review
used:

- `research/quartic_ghs_strong_rho_parallel_20261118/IMPLEMENTATION_SPEC.md`;
- `research/quartic_ghs_strong_rho_parallel_20261118/NEXT_LEADS.md`;
- `research/quartic_ghs_strong_rho_parallel_20261118/PLAN.md`;
- `research/quartic_ghs_strong_rho_20261114/round-c-integration/RESULTS.md`;
- `problems/ghs-strong-rho-measurement-admission.md`;
- `research/ecbench_e2e_frontier_20261031/LITERATURE.md`;
- `docs/POLLARD_COLLAB_DESIGN.md`.

The primary literature receipts in the ECBench review cover Shoup's generic
lower bound, Gallant–Lambert–Vanstone automorphism use, van Oorschot–Wiener
parallel collision search, F4/F5, Semaev summation polynomials, Gaudry index
calculus, and binary-field Weil descent. They require matched symmetry,
complete pipeline accounting, and separation of solver-local work from an
end-to-end ECDLP claim.
