# Vieta/coset Rust scale study — prospective protocol v1

Status: **not launched; not frozen for timing**. The implementation and
correctness checks are producer evidence [pcc:unverified]. The performance
request remains open. `protocol.json` is the machine-readable design; a valid
freeze, source publication, checkpoint and backend admission precede data.

User request: put the quadratic Vieta / PKM smooth prime-field coset construction
in `isogeny-labs/crypto`, implement it in Rust, and measure bigger sizes and scale.

| Requirement | Deliverable / acceptance |
|---|---|
| Native implementation | `cryptanalysis::vieta_coset`; standalone crate compiles the exact same source |
| Avoid interpolation | Composed reduction of Z^m modulo a quadratic; no evaluation/interpolation step |
| Complete two-sum output | Every ordered finite-point pair; separate discriminant and identity branches |
| Strong controls | Scalar subtraction, batched-inversion lookup and A/A; full output equality |
| Three-sum comparison | Streaming Vieta, nested batched lookup, pair-table MITM and A/A; setup charged |
| Bigger fields / bases | 17-, 31-, 62-, 224-bit fields; m=32,128,512,2048,8192 |
| Scale evidence | Retained raw rows, failures, matched controls, replay, host/isolation/custody and signed PCC standing |

## Construction and scope

On E: y²=x³+A*x+B, for finite R=(t,r), set s=L²-t and
v=t*L²+2*r*L+A+t². Reduce Z^m to a(L)Z+b(L) modulo Z²-sZ+v.
Distinct x coordinates come from roots of gcd(a,b-c^m). Repeated x coordinates
come from gcd(s²-4v,(s/2)^m-c^m), with the power reduced modulo s²-4v.
Recover y_i=L(x_i-t)-r, check membership and the group sum, and return both
orders. R=O explicitly enumerates P,-P. Finite two-torsion targets are covered
by the same formulas and the exhaustive tests, without dividing by r.

`m=2^d` makes the membership lift quadratic with d gates, while dense elimination
polynomials still have degree O(m). This implementation uses schoolbook dense
polynomial multiplication/division. It does not establish a last-fall-degree
bound or polynomial complexity in log(m).

The 17/31/62-bit fixtures have A=2,B=3 and p=65537, 2013265921,
4179340454199820289. They are component curves over prime fields; no prime
subgroup order is asserted. P-224 uses p=2^224-2^96+1, A=-3, standard B.
Pure quadratic towers are deliberately tested on P-224; the large 2-adicity
does not transfer to P-256 or secp256k1. No secret keys are inputs.

## Workload, controls and stopping

- Two-term panel: all four fields × m=32,128,512,2048,8192; 12 target calls
  per configuration, of which two warmups and ten retained calls (five per
  population). Fixed seed and alternating arm order are in `native/main.rs`.
- Populations are separate: deterministic random x conditioned on rational
  lift (canonical Tonelli sign, **not uniform points**), and known sums of
  fixed factor-base pairs. Random targets are not presumed unsatisfiable;
  the lookup reference determines the complete relation count. Identity targets
  are explicitly marked by null target and analyzed separately.
- Three-term panel: four fields × m=16,64,128; same warmups/round counts,
  half random targets, half known three-sums. Full ordered triples are compared.
  MITM construction is a separate charged phase; preserve construction failures.
- Controls: identical batched lookup A/A for two-term; identical MITM A/A
  for three-term; conditioned targets are positive controls. Reject a row on
  any relation-set mismatch. A bounded candidate failure must retain all control
  rows; it cannot disappear from a success-only table.
- Per stage: <=1,000,000,000 primitive multiplications, <=4096 split attempts,
  <=120 seconds; checks at loop boundaries can overshoot by one loop body.
  Maximum m=16384 is an API allocation guard, not a measured size.
- Whole attempt: 3600-second absolute deadline, one attempt, no retries; <=4
  CPUs, <=8 GiB RAM, <=1 GiB raw output. A hard deadline preserves completed
  rows and marks every unattempted row missing. No shrinking the matrix after
  seeing results. No builds or heavy tests while timings run.
- Cloud spending ceiling: USD 3 including build, failed/provisioning/idle and
  storage costs, subordinate to live shared daily/monthly balances. Do not launch
  while balances, capacity, source/image custody or checkpoint upload are unknown.

## Accounting and interpretation

Emit descriptor, factor-base/hash setup, query, and pair-table setup separately.
For one target, candidate cold cost is descriptor + extraction; lookup cold cost
is descriptor + factor-base/hash setup + lookup; MITM cold cost includes its table.
Warm comparisons omit explicitly reusable setup and are secondary. Fixture
generation is outside every arm, identically shared, and recorded by frozen rule.
The candidate's returned-witness group checks are inside its interval.

Primitive totals include arithmetic inside inverses and square roots. The
inverse/sqrt call counts are diagnostic; do not add their prices again to nested
primitive totals. Hashing, allocation, comparisons, serialization and sorting
are not priced by the field-multiplication counter; clocks are secondary and
must carry their actual isolation standing. This reference uses u128 reduction
and BigUint, not the repository's optimized Montgomery field backends. A generic
batched control does **not** establish comparison to the fastest existing engine.
No fastest-backend or end-to-end ECDLP claim is in this study.

`native/analyze.rs` reports counts, sample sizes, failures and descriptive
quartiles by configuration/population/arm. It does not pool fields or target
populations, fit an exponent, impute failures, or perform a significance test.
Ratios are interpretable only for exact matched inputs with admitted A/A and
replay. No result is promoted to a frontier or leaderboard here.

## Launch gates and current blockers

The default is EKS Spot, following `.agents/skills/isolab-eks/SKILL.md` and
`taskq/deploy/eks-karpenter/COST_CONTROLS.md`. `scripts/checkpoint.sh gate`
currently fails because this session has no AWS profile/upload token. The
read-only repository-variable endpoint returns 403 and live budget/capacity
receipts are unavailable. No paid build or job has been submitted.

The existing `isolab-experiment.yml` image builds ecbench/fresh-owner, not this
standalone component CLI. A component-image builder plus an admitted command
runner is also required before calling this study remotely runnable. Do not
dispatch its ecbench fixture and misreport it as this study. A local backend
override, if the user supplies one, must also resolve the S3 checkpoint rule;
write an amendment before measurements and retain the original EKS requirement.

## Sources checked

- Petit–Kosters–Messeng, *Algebraic approaches for the ECDLP over prime fields*,
  §3.3, https://christophe.petit.web.ulb.be/files/16PKC_primeECDLP.pdf:
  smooth subgroup/coset power maps; no novelty claim.
- `RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md`, `prime_fast.rs`,
  `ic_framework/prime_tower.rs`, `pkm_smooth_tower.rs`, `fp_backend.rs`:
  existing roots/lookup/factor-base backends and stronger native controls.
- NIST SP 800-186 §G.1.2.2, P-224 domain parameters:
  https://doi.org/10.6028/NIST.SP.800-186.
- KB unavailable (no MCP; wrapper exits 127); sources above were read directly.
- Protocol tool/schema pinned to crypto-autoresearcher
  `0ca36c9d5b49f2fcba56f79a51456a715b7aae5e`, last plaintext parent of the
  encryption rollout; current encrypted tree cannot be unlocked here.
