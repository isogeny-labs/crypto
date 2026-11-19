# Results: six falsifiers on prime-field ECDLP hardness

**H4 cost interpretation update (2026-11-19):** The former `2^87.4`
“lower bound” and “Win excluded” language in the H4 row and discussion
below is superseded by the
[H4 cost amendment](H4_prime_descent/COST_INTERPRETATION_AMENDMENT_20261119.md).
That number sizes a conditional dense Macaulay operation, while the
recorded solving-degree trend remains censored from `n=13` onward. The
original summary is retained below as the initial decision record.

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
| H2 | Hermitian Néron–Severi spectrum (C2) | **KILL on a surrogate only** | non-tautological corank 0 at p=7, 11, 13 (rank equals the random baseline; p=31 censored). The matrix has 26 to 82 columns against 2,752 to 30,772 rows, so full column rank was near-certain and the test had little power. The true restriction to a Legendre fibre does not exist (genus and Kodaira mismatch), so a norm-circle model was used | not a rho comparison; the real C2 object is untested |
| H4 | FPPR descent to F_2, first-fall trend (C4) | **INCONCLUSIVE on the trend; Win excluded by a cost bound** | first-fall degree is exactly 7 at all nine prime n from 11 to 41 (62 systems). The metric is uninformative: the equations have degree 6, so the first fall is 6+1 by construction. Solving degree resolved only at n=11, where it equals the 12 variables (no gain over exhaustive search) | any Macaulay solve needs degree at least 7 over 132 variables at n=131: at least 2^87.4 bit-ops per relation, 66 bits over the 2^21 gate and 22 bits over the 2^65 rho floor (recomputed independently; guessing variables first does not lower it at fixed degree) |
| H5 | Ulmer Legendre d=p^2+1 kernel (C5) | **KILL, weak** | corank 48 of 50 at p=7 and 120 of 122 at p=11; only two geometric relations (twice the all-ones and signed sums, both 2-torsion). The corank metric is close to automatic, because the kernel of a map into a finite group always has full rank r. The Néron–Severi height pairing was not computed, so geometric relations beyond Ulmer's two were not searched for | not a rho comparison |

All six have run. Only H0, H1 and H3 are clean Kills. H2 and H5 are weak Kills whose metrics had little power, and H4 is inconclusive on its pre-registered trend, each decided on the correct gate — cost or scale, not a
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

- **H2:** finished as `-r4` (4 CPU, 8Gi) after the earlier jobs never scheduled, failed on JSON serialisation, or were OOM-killed at p=31. See the verdict table for why this Kill is weak. Two protocol errors were found and are corrected in PROOFS.md and the H2 protocol: the line count of H(3,p^2) is (p+1)(p^3+1), not (p^3+1)(p^2+1), which is the point count; and the incidence spectrum of N N^T is {(p+1)(p^2+1), p^2+p, 0}, not the list originally stated.
- **H4:** finished as `-r3` on 4 CPU and 8Gi (the 16 and 8 CPU pods never scheduled on Spot). The pre-registered trend test failed to discriminate, because the first-fall degree is pinned to the equation degree plus one. The informative quantity is the solving degree, which is censored for n of 13 and above. The 130.6-bit figure in the fit block comes from the single n=11 point and should be ignored. The bound that stands is the 2^87.4 lower bound. It is not a measured solving cost. It does not cover algorithms that lower the degree by guessing variables or by other structure, and the FGHR 4-torsion symmetrization was not applied. A real C4 test needs the solving degree measured at n of 13 to 23 with an exact solver, which is a follow-up.
- **H5:** finished as `-r6` on 4 CPU and 8Gi after three infrastructure failures and three bugs in the runner's own code. The recalled Ulmer rank formula predicts one geometric relation and the data show two. That does not change the verdict and was not re-derived. At p=11 the fibre loop stopped at 53 of 70 fibres and the exact function-field check was not run.

## Infrastructure notes (for reproducibility)

Two harness-helper bugs were found and fixed mid-run, and are recorded because they invalidated
early attempts (never read as evidence): (1) the submit wrapper wrote stdin to the same temp path
the runners piped from, truncating programs to empty; (2) the MCP command-argument limit (~8 KB)
rejected large scripts, fixed by staging each program through S3 and having the pod pull it. After
the fixes a 17.7 KB script ran clean end to end. AWS Spot returned `MaxSpotInstanceCountExceeded`
under load, which delayed and failed some larger-pod jobs; this is censoring, not evidence.

## What the program established, and what it did not

Three conjectures survive as measured walls on their probes: the interval-base monoid (C0), symmetrized small roots (C1) and the canonical-lift digits (C3). Two more survive only because their tests were weak (C2, C5). C4, the one route that exists on ECC2-131, is not settled: the pre-registered metric was uninformative, and a lower bound of 2^87 for plain Macaulay solving excludes a win but is not a measured cost.

Nothing here threatens rho on either Certicom curve, and nothing triggers the validation gate. The follow-ups that would turn weak results into real ones are: solving degree at n of 13 to 23 with an exact solver (C4), the Néron–Severi Gram matrix for the Hermitian family (C2 and C5), and a faithful restriction of the surface to a fibre if one exists.
