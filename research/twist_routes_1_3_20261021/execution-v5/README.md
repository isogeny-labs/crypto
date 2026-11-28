# Exact all-shift overlap and fixed genus-two factor checks

[Results, editable graphs and inspected three-page PDF](../reports/overlap-20261027/RESULTS.md).
The complete fixed panel is executed: **27,140** shifts, including **27,130**
smooth nonzero cases and ten singular zero-shift controls. Independent
Euler-character and square-table point counts agree throughout; every
plus-one false count is rejected. The classical identity is
`4*N_c = #D_c(Fp)` for `D_c: y²=f(u)f(u+c)/d`, c nonzero. Full-universe
density does not establish short-orbit mixing or solving cost.

Exactly c=1,3 on all ten original fixtures received PARI Frobenius-polynomial
checks. **0/20** have the source elliptic factor; **0/20** have the twist
factor. All **10/10** known covers detect the source factor; **10/10**
corrupted polynomials fail that factor test. Other shifts were not tested for
Frobenius factors. No scalar map, absolute simplicity, novelty, NIST or IC
speedup is claimed.

## Frozen evidence

* [Protocol](PROTOCOL.md), [structured freeze](protocol.json),
  [literature/access gaps](LITERATURE.md), [source/binary/input hashes](FROZEN.json).
  Scientific source is `069fb43cd486b24b604ce9320d01caa9f8fb64b3`.
* [Independent census](overlap-audit-v1/AUDIT.json),
  [strict final admission and all 30 polynomial rows](ADMISSION.json),
  [actual GP receipt](pari-replay-v1/RECEIPT.json).
* [Unified export](experiment.export.json), original native export, and
  `checks/export-binding.*`: full schema/protocol/file binding passes with
  zero findings. Two actual full-panel producer/verifier executions are
  exported; do not pool the same shifts as independent targets.
* [Pre-PARI compatibility amendment](GP_COMPATIBILITY_AMENDMENT_V2.md) and
  [pinned helper/transport hashes](GP_COMPATIBILITY.json). The explicit
  `polrem(a,b)=a%b` preload corrects an installed API spelling assumption
  before GP outputs. The main script, native data, cases and gates are unchanged.
* [Three checked conditional Lean theorems](../lean-v4/README.md) and
  IDC1ha81899274c038263. Fresh final axiom replay is
  `../lean-v4/receipts/final-v2/`; only standard Lean foundations occur.
  Legendre/genus/projective/Weil bridges are not formalized. A copied Lake
  manifest's root package name was corrected before that final replay;
  theorem source and dependency revisions are unchanged.

The full native experiment and identity packages pass serial release tests.
The broader root library still has the six unchanged incoming regressions
documented in [the predecessor check record](../execution-v4/ROOT_CHECKS.md).
No new root library source or validated leaderboard row changes here.

## Reproduction and publication

The raw run tree is deterministic `runs.tar.xz`, **102,176 bytes**, SHA256
`d66823361504cce4e14bf4e8f265982eb4cb939fdca1cd0cf8c5ea085e075d23`.
The maintained Setec manifest preserves its path and immutable bytes.
[Fresh download verification](RAW_VERIFICATION.json) checks all **17** member
hashes and an identical repack. Large compiled proof auditors and plots use
their own manifests; new report PDFs stay in Git. No workflow-owned reports
prefix, bucket policy, funding or credentials are changed/persisted.

From the repository root, hydrate, check and extract:

```sh
python3 tools/artifacts/artifacts.py pull research/twist_routes_1_3_20261021/execution-v5
cd research/twist_routes_1_3_20261021/execution-v5
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
```

Build `twist-shift-overlap` from the frozen source with Rust 1.90.0 and pinned
Cargo.lock using `--release --offline --locked`. Verify FROZEN.json hashes.
The native transport commands and actual limits/outputs are in the extracted
`producer-transport.json` and `verifier-transport.json`. The original input
path/hash is in FROZEN.json. Use fresh output directories for `produce`,
`verify`, `admit` and `export`. GP replay reads polynomial-remainder.gp before
the unchanged overlap-audit-v1/verify.gp; receipt argv records both files.

Producer/verifier each have 600 seconds, 4 GiB and 64 MiB/file; GP has
300 seconds, initial 64 MB stack and 4 GiB maximum. No native timing, GAE,
fastest-external or whole-IC claim. The helper's semantic checks, all cases
and strict stderr admission must remain. Final E28 labels 32–63 stay unopened.

The next discriminator is twist-generator order and finite-prefix selection:
full-universe support may be ample while one chosen generator explores a
proper subgroup. Discovery, order testing and any subsequent complete solver
work must be charged, with target-blind selection and matched ordinary controls.
