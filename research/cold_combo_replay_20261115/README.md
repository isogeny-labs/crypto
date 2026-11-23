# Standalone public-target replay for the COMBO candidate

Implementation preparation for the objective: the first independently reproduced
reduction in total ECDLP solving cost. No scientific measurement has been launched
from this directory, and it contains no measured performance result.

The candidate is the existing combined open-addressing hash table and compiled
fixed-exponent square root from `../ic_gw_fasthash_20261114/`. Its domain is the
GHS-weak cubic-extension family `E0/F_(q^3)`, not generic prime-field curves.

## Why a separate driver

The historical `v3.rs` runs D, DL1 and DL in one invocation. Its `solver2.rs`
shares factor-base/relation/matrix phases, and reuses downstream results for equal
normalized kernel vectors. The historical report's total CPU metric sums the DL
phase entries. This implementation instead requests only DL and directly times
the whole solve, including controls, retries, final scalar checking, and solver
state destruction. Phase sums are diagnostics; `total_cpu` and `total_wall` are
the comparison intervals. `unattributed_cpu` retains work outside phase timers.

The new entry point receives a public target point. The scalar used to construct
a known-answer fixture is accepted only by the separate `fixture` command.
Every attempt is retained. A failed six-attempt solve emits its full cost and
exits 2. The solver's own final check is explicitly not independent replay.

## Source binding

Starting source revision: `f47a44756c94f2a70eef0fa4155650e36c8ca2fb`.

- REF relation engine: `../quartic_dl1_cd_sqrt_20261113/src/ic2.rs`.
- COMBO relation engine: `../ic_gw_fasthash_20261114/src/ic2.rs`, compiled with
  `--cfg combo`.
- Both use the identical LA source from the COMBO study.
- `solver_public.rs` copies historical `solver2.rs`, SHA-256
  `f3cbc75c0b65472725ed19b7adb9b1086db6c9502b0ce4e3a469c439088854c8`.
  Its only functional changes replace `k_true` by `&EP`, remove construction of
  the target from the label, and pass that reference to downstream recovery.
- Defaults match the historical invocation: c_D=1.7, gamma=4, block=4,
  rows_mult=1.2, at most six attempts, c_D step 0.25, max_desc=100000.

No historical source, frozen protocol or raw result is edited.

## Development interface

From the repository root, with Rust on PATH:

```sh
bash research/cold_combo_replay_20261115/build.sh /absolute/output/directory
```

The output directory contains `ref`, `combo`, the compiler identity and binary
SHA-256 digests. The CLI forms are:

```text
fixture Q INSTANCE_SEED LABEL [REPLAY_FILE]
solve Q INSTANCE_SEED X0 X1 X2 Y0 Y1 Y2 REPLAY_FILE
```

Fixture output supplies the extension modulus `z^3-extension_c`, generalized
Weierstrass coefficients, subgroup order, generator and public target. Solve
arguments contain no scalar label. Field coordinates must be canonical, the
target must lie on the curve and in the subgroup, and q must be prime, 7 modulo
12, and below 2^31 (the relation engine's packed-key bound).

Instance reconstruction and target validation are input preparation outside the
solve interval. Target-independent factor-base setup is inside the cold interval.
Record external process preparation separately; do not silently relabel the
solve interval as full process cost or suppress input-preparation costs.

Development boundary checks compile without enabling the imported PIC module's
Cargo-only test fixtures:

```sh
rustc --edition 2021 -O --cfg replay_selftest \
  research/cold_combo_replay_20261115/driver.rs -o /absolute/output/selftest
/absolute/output/selftest
```

These checks construct a tiny public point and test input validation. They do not
run the solver or produce scientific timing evidence. Do not use a small known
label from development as the uniform unknown-target evaluation law.

## External scalar replay

`replay.sh REPLAY_FILE` calls PARI/GP (`GP_BIN`, default `gp`). The candidate
exports exact integer coefficients and coordinates in a numeric vector;
no JSON/double conversion touches the 64-bit scalar. A failed solve exports
scalar zero, which the checker rejects. Existing replay files are never replaced.

The adapter bounds input length and allows only digits, commas, brackets, spaces
and newlines before passing the literal vector to GP. `replay.gp` checks shapes,
canonical coordinates, prime field and subgroup orders, an irreducible cubic
extension, a nonsingular curve, point membership, subgroup membership and the
scalar equation with PARI's own arithmetic. Success requires an exact sentinel
and zero exit status; parser errors or diagnostics cannot count as success.
This is an external arithmetic check, not a signed PCC standing or execution
receipt. Preserve the GP version, binary identity and raw replay output when
using it on scientific runs. [pcc:unverified]

Development checks, after building and making PARI/GP available:

```sh
bash research/cold_combo_replay_20261115/check-development.sh /absolute/output/directory
```

The positive fixtures are known-answer target generation, not solver runs. The
negative controls alter the scalar, target coordinate, field, coordinate shape,
GP-code payload and extension irreducibility. A third positive fixture transports
scalar 9007199254740993 without double conversion. The external tool used during development was Debian
PARI/GP 2.17.2; checked its installed `ffgen` and `ellinit` documentation for the
field presentation and generalized Weierstrass coefficient order.

## Remaining work before measurement

1. Apply the external scalar replay to actual candidate-produced recovered
   scalars and retain execution provenance.
   Known-answer fixture controls alone are not an independent solver replay.
2. Qualify REF against the best existing equal-output pipeline. In particular,
   select among the historical LA variants on separate calibration inputs before
   freezing the primary variant; DL alone is not asserted to be the best.
3. Freeze public-instance and independently generated uniform-target holdouts,
   baseline/candidate source and binaries, five or more measured rounds with
   counterbalanced order, warmup, A/A and positive controls, independent replay,
   uncertainty, RSS, and complete accounting. Keep one target per process.
4. Freeze total-cost acceptance before timing: lower cold complete cost than the
   best frozen baseline, paired 95% upper cost ratio below one, unchanged coverage,
   and the repository's residual-work pricing band. Report failures and caps.
   Add matched rho where required by the frozen protocol. Do not replace the
   primary total with relation throughput or an amortized multi-target result.
5. Complete S3 checkpoint admission, live budget readback, finite spending ceiling,
   and live capacity admission through the existing EKS route. The prior source
   receipt `../quartic_dl1_cd_sqrt_20261113/launch/BUDGET-HOLD.md` records an
   exceeded monthly budget. It is historical, not a fresh budget readback.

Sources checked (KB unavailable): `scripts/research-kb status` had no executable;
no KB MCP tools were loaded. Fallback sources: the COMBO frozen protocol and
source, `../ecbench_e2e_frontier_20261031/REPORT.md`, the November 14–15 session
logs, and the current EKS skill/access guide. These identify an existing
engineering mechanism and its missing complete-cost replay; no novelty claim.
