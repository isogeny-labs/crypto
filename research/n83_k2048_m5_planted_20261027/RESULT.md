# N83 K2048 five-summand circuit control

The exact primary N83 chained-S3 five-summand circuit returned `Sat` for a
constructed five-point group sum in **both** 2,048-orbit factor bases when its
three intermediate x coordinates were pinned. The same target, base, circuit,
and five pinned summand x coordinates returned `Unknown` at 5,000 conflicts
when those three chain coordinates were left free. The [independent group
replay](evidence/audit.json) accepts all four original records and the
[negative control](evidence/negative-control.json) rejects an altered target.

This unregistered diagnostic follows the [frozen natural-query
panel](../n83_k2048_m5_pair_20261027/reports/run-20261027T2012Z/REPORT.md).
It isolates an internal-chain search gate; its two pin modes present different
solver information and their durations are not a speedup comparison. The
separate U and D fixtures have different constructed targets, so they do not
rank factor bases. Candidate ID, natural relation yield, matrix rank, verified
target log, same-point rho and single-target online speedup remain `null`.

## Exact construction and outcomes

Curve: `EC1N83Ckb0h252dacc102a4`, `a=0`, subgroup order
`2417851639230796216685689`. U is the retained `public_x_hash` K2048
archive (compressed SHA-256
`aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a`);
D is `direct_subgroup_x_d18_prefix2048` (SHA-256
`2406a7105bff162fc80e448d441709cc9f725e36da9751a2e9feb6f3e23512c7`).
The verified [base construction](../n83_d18_k2048_20261027/RESULT.md)
records 339,968 distinct subgroup points and 2,048 signed-Frobenius columns
per base, with zero point-set overlap. The source archives remain at their
content-addressed locations recorded there; only compact result receipts are
copied into this study.

For each base, representative indices `[0,17,53,97,211]` select five
points. The worker checked the fast and generic group sums, their subgroup
membership, and the three intermediate partial sums before building the
unchanged 111,478-variable, 1,140,557-initial-clause, 4,980-XOR-row model.
The solver had a 5,000-conflict and 240-second cap. The table reports one
run per cell on a shared Apple M4 Pro Darwin arm64 host; it is a correctness
and solver-gate diagnostic without a CPU timing tier or uncertainty estimate.

| Base | Pin mode | SAT status | Conflicts | Search wall (s) | Model verified |
| --- | --- | --- | ---: | ---: | --- |
| U | five summand x values | `Unknown` | 5,000 | 87.680 | no model |
| D | five summand x values | `Unknown` | 5,000 | 115.556 | no model |
| U | summands plus three chain x values | `Sat` | 510 | 32.789 | yes |
| D | summands plus three chain x values | `Sat` | 204 | 32.643 | yes |

The first two `Unknown` records preserve the conflict cap as a censored
outcome, not an unsatisfiability verdict. Both `Sat` records were checked by
the worker's full SAT-model verifier. The separate Rust auditor reloaded the
exact archived base objects, used generic field/group arithmetic to replay
the five-point target sum and the pinned intermediate x coordinates, checked
subgroup membership, and matched receipt/source identities. It does not
independently rerun the SAT solver. Its negative control changes one target
coordinate and fails before writing an audit result.

The worker source BLAKE3 digests are
`38f17cda8762f5755e3326d1151770537b1a011cb621df29499dd3e705a97a3a`
(summands) and
`f5623a5b6f90c3f642cf7f06f2776ea964d5c8ecf2223da13f2481dd57ffd697`
(full chain). The audit source BLAKE3 is
`439239b4951b7411294483d1fbfa66c16b428904266f926ae9929ae7a475deae`.
Individual receipts retain binary SHA-256, object hash, policy, target,
selected indices/points, model dimensions, status, and each timing phase.

## Reproduce and next gate

Build with `cargo build --release --locked --example n83_k2048_m5_planted
--example n83_k2048_m5_fullchain --example n83_k2048_m5_planted_verify`.
The workers take the exact archived U or D object and an output JSON path;
the verifier takes the four records and both archive paths. The [run
report](reports/run-20261027T2057Z/REPORT.md) contains the command template,
[figure](reports/run-20261027T2057Z/pin_gates.svg), and
[PDF](reports/run-20261027T2057Z/REPORT.pdf). The sources are
[`n83_k2048_m5_planted.rs`](../../examples/n83_k2048_m5_planted.rs),
[`n83_k2048_m5_fullchain.rs`](../../examples/n83_k2048_m5_fullchain.rs), and
[`n83_k2048_m5_planted_verify.rs`](../../examples/n83_k2048_m5_planted_verify.rs).

The next experiment should reduce the free intermediate-chain search cost
without supplying a planted path, then seek and independently lift a natural
five-summand relation on the same public points in U and D. The end-to-end
factor-base decision requires relation yield/rank, final log recovery and a
verified one-target IC/rho pair under a frozen Tier 1 or Tier 0 envelope.
