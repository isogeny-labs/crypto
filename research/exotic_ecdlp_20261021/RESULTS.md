# Results: six falsifiers on prime-field ECDLP hardness

Running tally, updated as runs land. Compute: isolab (EKS/Karpenter Spot, account 104186811093,
us-east-2), SageMath 10.10 image `isolab/tasks@sha256:9ba9edac…`. Each verdict is scored against
the pre-registered Win/Kill in the matching `H*/PROTOCOL.md` and `PROOFS.md`. A Win would trigger
the full validation gate (`docs/VALIDATION_GATE.md`) before any claim; none has.

## Verdict table

| id | route (conjecture) | verdict | the number that decides it | gap to rho |
|----|--------------------|---------|----------------------------|------------|
| H0 | linear-size monoid on E(F_p) (C0) | **KILL** (on cost) | cost per decomposition scales as p^0.50 (median-walk fit); decompositions are O(1) length (median 3) but each step scans all of F (≈√p adds) | at the √p wall, no sub-birthday recursion |
| H1 | symmetrized Semaev small-roots (C1) | **KILL** | β_sym = 0.1125 at p=2^28 vs β_generic = 0.25; never beats the raw system; no growth with p | implied 2^89.4 per relation at 131-bit, vs 2^65 rho and 2^43.7 enumeration null |
| H3 | canonical-lift p-adic digits (C3) | **KILL** (on scale) | min additive-defect 0.99996 (≈1−1/p, no character); MI within null; small-p closure "signal" is a miscalibrated null (z spans −2.9…+5.0 across sibling curves) | no size function; nothing survives to cryptographic scale |
| H2 | Hermitian Néron–Severi spectrum (C2) | in progress | — | — |
| H4 | FPPR descent to F_2, first-fall trend (C4) | in progress | — | — |
| H5 | Ulmer Legendre d=p^2+1 kernel (C5) | in progress | — | — |

Three of six resolved, all Kills, each decided on the correct gate — cost or scale, not a
confounded proxy. In every case the script's strict per-cell rule and the directed final verdict
are both recorded in `H*/out/result.json` (`verdict` vs `verdict_script_output` /
`verdict_literal_3pt_amortized_fit`), so the override trail is auditable.

## What each Kill actually shows

**H0 — the organizing conjecture holds on the only instance we could probe.** The Semaev interval
base `F = {x < √p}` does admit short decompositions (median length 3, O(1)), but finding one costs a
full `|F| ≈ √p` scan per greedy step, so cost per relation scales as `p^0.50` (median-walk fit:
0.501 on p ≥ 2^16). That is birthday search, exactly the `relation_prob × solve_cost ≥ √N` the
conjecture predicts, with no recursive factorization. An earlier "win" from a descent-slope metric
was discarded: `F` is *defined* by small `x` and the metric minimized `x`, so the slope was
circular; a hashed-label control plus the cost measurement settled it.

**H1 — 4-torsion symmetry does not move the Coppersmith threshold.** The FGHR symmetrization drops
the system degree from 4 to 2, but the recoverable root-box exponent `β_sym` is 0.11 at `p = 2^28`,
identical to the raw system and flat in `log p`, against the `β = 1/3` a relation needs and the
`β_gen = 0.25` generic bound. Controls pass (S_3 verified, count-control within 2× of the `1/3!`
model). Extrapolated cost at 131 bits is `2^89.4` per relation — worse than rho and worse than the
naive `2^43.7` box-enumeration null.

**H3 — the canonical lift carries no digit-size.** Using the *exact* CM canonical lift (Hensel root
of the Hilbert class polynomial lifting `j(E)`; Satoh's modular iteration degenerates over a prime
field), no `F_p`-linear functional of the first two p-adic digits is even approximately additive
(min defect `0.99996 ≈ 1 − 1/p`), and mutual information with the discrete log sits inside the
permutation null at every `p`. The lone sub-threshold number — a closure excess at `p = 2^12` — is a
null-calibration artifact: across five independent curves its z-score ranges from −2.9 to +5.0, and
it vanishes by `p = 2^16`. T3's "lift is a homomorphism, hence no linear invariant" is confirmed
empirically.

## In progress

- **H2 (Hermitian spectrum):** first two jobs hit the 132-minute deadline — the line enumeration at
  `p ∈ {7,11,13,31}` is too slow as written. Needs orbit-based enumeration (unitary group action)
  or to accept `p=31` censored and finish `{7,11,13}` within budget. Runner paused on a session rate
  limit (resets 7:50pm PT); resumes with that guidance.
- **H4 (FPPR first-fall trend — the headline):** methodology is correct (in-house Boolean
  Macaulay-matrix profiling, M4RI rank, degree-fall detection, exhaustive `d_solv` check). The debug
  run used a 100-second per-cell budget and hit a worker-pool `KeyError`, so `n=11` censored. Needs
  the pool bug fixed and realistic budgets, then the prime-`n` sweep `{11..41}` and the
  extrapolation to 131. Runner paused on the rate limit; resumes with that guidance.
- **H5 (Ulmer f=2 kernel):** the 8-vCPU pods could not schedule on Spot; resubmitted single-threaded
  at 4 vCPU (`-r3`, running). Deviation from the pre-registered 16 GiB noted; the code is
  single-threaded so the kernel numbers are unaffected.

## Infrastructure notes (for reproducibility)

Two harness-helper bugs were found and fixed mid-run, and are recorded because they invalidated
early attempts (never read as evidence): (1) the submit wrapper wrote stdin to the same temp path
the runners piped from, truncating programs to empty; (2) the MCP command-argument limit (~8 KB)
rejected large scripts, fixed by staging each program through S3 and having the pod pull it. After
the fixes a 17.7 KB script ran clean end to end. AWS Spot returned `MaxSpotInstanceCountExceeded`
under load, which delayed and failed some larger-pod jobs; this is censoring, not evidence.
