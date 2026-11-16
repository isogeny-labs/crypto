# Protocol: dynamic constant-time audit of the `crypto` crate — 2026-11-02

Status: **frozen before data** at the commit that changes this line (recorded in the PR). The controls dry run on
scratch output (§8) preceded freezing.

## 1. Question

The 2026-10-08 source inventory (`docs/audits/SIDE_CHANNEL_AUDIT_20261008.md`) classified secret-handling paths as
constant-time (CT) or variable-time (VT) *by reading the source*; it measured nothing and named that limitation. Since
then one finding was fixed and measured (`research/scalar_mul_ct_timing_20261022`), and X25519 was rewritten on
fixed limbs. This study asks, for each path below, two questions that an execution can answer exactly:

1. **Taint:** does any conditional jump/move or memory address depend on the secret? (Memcheck, secret marked
   undefined: the ctgrind method.)
2. **Instruction count:** does the number of executed instructions per call depend on the secret? (Callgrind Ir,
   fixed-vs-random classes plus a bit-length class and a low-Hamming-weight class.)

## 2. Claims and their classes

| claim | class | what licenses it |
|:--|:--|:--|
| "target T executed a secret-dependent branch/address at site S" | exact computation | a Memcheck report on the target with the positive control detected and the negative control clean |
| "target T's Ir depends on the secret" | exact computation | two calls with different secrets and different Ir, with the A/A arm deterministic |
| "no secret-dependent branch/address was reported for T" | comparative (bounded) | only with the positive control detected; scoped to these inputs, this build and Memcheck's model |
| "no Ir variation was observed for T across N inputs" | comparative (bounded) | only with a deterministic A/A arm and the positive control varying; scoped to these classes |
| "T is constant-time" / "T is safe against timing attacks" | **out of scope** | not established: Ir equality does not exclude cache, port or micro-architectural timing; taint covers only this build |

## 3. Targets (the paths the 2026-10-08 audit ranked, plus its claimed-CT stacks)

`ctrl_pos_branch`, `ctrl_neg_masked` (controls, §4), `x25519`, `smul_p256`, `smul_secp256k1`, `smul_sm2_generic`,
`inv_prime_ct`, `inv_euclid`, `ed25519_sign`, `ecdsa_p256_sign`, `sm2_sign`. Exact entry points and secret inputs:
`src/main.rs` `TARGETS` and `call()`. Release profile with `debug = line-tables-only` (codegen as shipped plus line
tables), the crate at this branch's base `46f01e51d`.

## 4. Controls

- **Positive control** `ctrl_pos_branch`: `if secret[31] & 1` around a 64-iteration loop, plus a table read indexed by
  `secret[0]`, compiled in the same binary. Expected: ≥ 1 Memcheck report and Ir that varies across random secrets.
  If either is missed, the corresponding instrument is **invalid** for this run and no "nothing found" is reported.
- **Negative control** `ctrl_neg_masked`: a fixed 32-iteration loop of masked u64 arithmetic with no secret-dependent
  branch or index. Expected: zero reports and constant Ir. A report here measures the instrument's false-positive
  behaviour (for example, if the compiler emits a conditional move Memcheck reports); it is reported and every target
  report at the same kind of site is read against it.
- **A/A arm** `aa`: one fixed secret per target repeated 16 times in one process. Expected: identical Ir
  (determinism, not variance: Ir is exact). A target whose A/A arm varies has internal randomness (scalar blinding,
  OsRng nonces) and its Ir class test is **not applicable**.
- **Null population** for the Ir comparison: the `random` class, 32 uniformly drawn secrets in [1, n−1] (or 32 random
  bytes for seed/scalar-byte targets). Matched on everything except the secret: same target, process, build, warm-up.

## 4a. Seeds and sampling

Secrets come from `StdRng::seed_from_u64(target_seed(target, class))`, where `target_seed` is FNV-1a over
`"target/class"` XOR `0x5ca1ab1ec7d11102`, fixed in source before any data. Classes and sizes: `aa` 16, `random` 32,
`short` 16 (scalar < 2^32 before reduction; the bit-length class of the 2026-10-22 leak), `lowhw` 16 (Hamming
weight 8). Taint: 4 calls with independent random secrets per target. Sizes are fixed; Ir is exact, so one differing
pair is a witness and no sampling-variance power calculation applies. The bounded negative claim's reach is stated as
"N = 80 inputs across 4 classes".

## 5. Instrument and execution

`run.sh <binary> <raw> <out>`: per target, one Memcheck process
(`--xml=yes --track-origins=no --num-callers=40 --error-limit=no`) and one Callgrind process
(`--collect-atstart=no`; collection toggled around each call; one `DUMP_STATS_AT` per call), then
`ct_dynamic_audit analyze`. `ENV.txt` records binary SHA-256, Valgrind and rustc versions, commit and host.

**Execution backend.** AGENTS.md §9a routes x86 experiments to Iso Lab on EKS. This session has no AWS identity
(`aws` absent, no profile), so the EKS route is unavailable; that is recorded as an integration gap. Both metrics here
are properties of the instruction stream (Valgrind's synthetic CPU), not clocks: they do not depend on the host's
load, frequency or co-tenants, so the run is executed in the session container. **No wall-clock timing claim is made**;
a dudect-style clock study on an isolated host is listed as follow-up and needs the EKS route.

## 6. Decision rule (applied by `analyze`, unchanged after freezing)

- Taint verdict: "N secret-dependent sites" (distinct first in-crate frame and line) or "no secret-dependent branch or
  address reported". Valid only if the positive control reports ≥ 1 site.
- Ir verdict: if A/A min ≠ max → "not applicable: A/A not deterministic"; else if min = max over `random ∪ short ∪
  lowhw` → "no Ir variation observed across classes"; else "Ir depends on the secret (spread, % of min)". Valid only
  if the positive control's verdict is "depends".
- No multiple-comparison correction: every verdict is an exact witness or its absence, not a significance test.

## 7. What does not count

Changing targets, classes, sizes, seeds, Valgrind options or the decision rule after seeing data. Any change is a
dated amendment below, saying whether it preceded the data it affects; both versions are reported.

## 8. Dry run (before freezing, scratch output: the two controls and `x25519` only)

1. The first positive control (`if secret & 1 { 64 iterations of constant work }` plus a secret-indexed table read)
   produced one taint site (the table index) and **no** Ir variation. The optimizer had compiled the branch to a
   conditional move: Memcheck does not report a conditional move on undefined data, and the instruction count does
   not change. Under §4 the Ir instrument would have been invalid. **Changed**: the positive control now runs a
   secret-dependent trip count (0–7 iterations, body pinned with `black_box`). It then gave 2 taint sites
   (conditional jump `main.rs:125`, secret-indexed load `main.rs:128`) and Ir from 69 to 122 across classes.
   Consequence for reading targets: **cmov-based selection is invisible to both instruments**. That is consistent
   with cmov being data-independent on current x86 cores, but it also means these instruments cannot see leakage
   that a compiler introduces only as cmov.
2. Negative control: no taint site; Ir constant within each class. The `aa` class reads 506 against 508 for the
   other classes, a constant 2-instruction offset from the harness's clone path at the toggle boundary. The decision
   rule tests `aa` only for internal determinism (16/16 identical) and compares `random ∪ short ∪ lowhw` among
   themselves, so the offset does not affect any verdict.
3. `x25519` (the only target in the dry run): no taint site, no Ir variation. Its result is reported from the frozen
   run, not from this dry run.

## Amendments

(none)
