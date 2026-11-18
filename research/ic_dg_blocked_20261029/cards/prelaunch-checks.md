# Pre-launch checks P1 to P5, XP-20261029-dg-blocked

author_role: experiment-director (Sonnet), read-only; run 2026-10-29 about 12:50 to 13:00 UTC
freeze commit: cd04cfd2d1df985e34f59b6a19bb8354e0f9a5ae (branch claude/ic-dg-blocked-20261029, pushed)
source revision: be2969cebf30a0cfb79088fd10094324c3efb3bb (read with `git show <rev>:<path>` from /dev/shm/wt-blk)
binary: /dev/shm/isolab-build/ecbench, sha256 88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c
checks as written in backlog/XP-20261029-dg-blocked/APPROVAL.md, "Pre-launch checks" (fail closed).

## Verdicts

| check | verdict |
|:--|:--|
| P1 tool behaviour at the frozen revision | PASS |
| P2 the oracle | **FAIL on one clause** (the refusal clause), the other clauses pass; see below |
| P3 the specs | PASS |
| P4 checkpoint and scope | PASS for the checkpoint; Conductor not installed (known gap, recorded, not a fail) |
| P5 edits applied | PASS |

Because P2 fails on a clause that APPROVAL.md says fails closed, no launch card was written.

## P1 tool behaviour at the frozen revision: PASS

`git show be2969ce:taskq/deploy/eks-karpenter/run-ecbench` (69 lines):

    64: reference=$(jq -r '.arms[] | select(.role=="reference") | .name' "$spec" | head -1)
    65: control=$(jq -r '.arms[] | select(.role=="control") | .name' "$spec" | head -1)
    66: ecbench compare --dir "$ISOLAB_OUTPUT_DIR/session" --a "$reference" --b "$control" --json \
    68: jq -e '.wall.status=="admitted" and .wall.pairs_below_level==0 and .wall.aa_pairs>=5
    69:   and .wall.aa_ci95!=null' "$ISOLAB_OUTPUT_DIR/control-comparison.json" >/dev/null || exit 43

- first `reference` arm and first `control` arm in spec order: yes (lines 64-65, `head -1` over `.arms[]` in spec order). PASS.
- exit 43 conditioned on exactly `.wall.status=="admitted" and .wall.pairs_below_level==0 and .wall.aa_pairs>=5 and .wall.aa_ci95!=null`: yes (lines 68-69). PASS.
- nothing requires the reference to be at index 0: the only spec gate (lines 39-42) asks `[.arms[] | select(.role == "reference")] | length > 0` and the same for `control`. PASS.

`git show be2969ce:src/cryptanalysis/ecbench/compare.rs` (2506 lines), `choose_aa` at lines 1395-1440:

- tries arm A's same-method twin first: `cands.iter().find(|(x, y)| *x == a || *y == a)` returns `(a, other, "includes arm A")`; then the reference arm's pair (`"includes the reference arm"`); then `cands.first()` (`"first alphabetical"`, candidates built from the name-sorted arm list). PASS.
- `self_aa` keyed on `method_id`: lines 1705-1714, `arms.entry(r.arm).or_insert_with(|| (r.method.method_id.clone(), r.role.clone()))`, then `if method_of(a).is_some() && method_of(a) == method_of(b) { w.self_aa = true; ...}`. PASS.
- `method_id` derived from the method id and params, not from the arm name: `src/cryptanalysis/ecbench/methods.rs` `resolve()`, `short_id("ECM1", &json!({"schema": "ecbench.method/v1", "id": d.id, "params": params}))`, defaults filled in. `ecbench plan` below prints one method id for `dg` and `dg-aa` and another for `dgb` and `dgb-edge`. PASS.
- `corrected_ratio` is `ratio_b_over_a / aa_ratio`: compare.rs lines 1730-1738, `divide_independent(&fig.ratio, &af.ratio)` sets `corrected_ratio = c.estimate`, and `stats.rs` `divide_independent` has `est = n.estimate / d.estimate`. PASS.

Consequence of the code for the six files (derived by reading `choose_aa`, not run): `--a dg --b dgb` gives `aa_arms` (`dg`, `dg-aa`) "includes arm A" (condition 7); `--a dgb-edge --b dg-aa` gives (`dgb-edge`, `dgb`) "includes arm A" (condition 6, native admission); `--a dgb-edge --b dgb` is `self_aa` true.

The `alternate` order at be2969ce (`src/cryptanalysis/ecbench/spec.rs`, expansion of executions: `Order::Alternate => if round % 2 == 1 { order.reverse(); }`). `ecbench plan --spec <wall spec> --doc`, measured (non-warm-up) blocks, run on all three wall specs:

    b=24 measured_blocks 288 forward 144 reversed 144 positions_by_arm_index {0: {0: 144, 3: 144}, 1: {1: 144, 2: 144}, 2: {1: 144, 2: 144}, 3: {0: 144, 3: 144}}
    b=28 measured_blocks 288 forward 144 reversed 144 positions_by_arm_index {0: {0: 144, 3: 144}, 1: {1: 144, 2: 144}, 2: {1: 144, 2: 144}, 3: {0: 144, 3: 144}}
    b=32 measured_blocks 288 forward 144 reversed 144 positions_by_arm_index {0: {0: 144, 3: 144}, 1: {1: 144, 2: 144}, 2: {1: 144, 2: 144}, 3: {0: 144, 3: 144}}

Arm indices 0 (`dg`) and 3 (`dgb-edge`) sit at {0, 3}; indices 1 (`dg-aa`) and 2 (`dgb`) sit at {1, 2}, 144 blocks at each position. PASS.

## P2 the oracle: FAIL on the refusal clause

1. `git merge-base --is-ancestor e59331c96 be2969cebf30a0cfb79088fd10094324c3efb3bb` printed `ANCESTOR_OK`, exit 0. PASS.

2. Routing at be2969ce. PASS.
   - ecbench maps the oracle string to the oracle object (`src/cryptanalysis/ecbench/methods.rs` lines 4163-4195): `"mitm" => &mut mitm` (`MitmOracle`) and `"mitm-prime-blocked" => &mut prime_blocked` (`PrimeBatchMitmOracle::blocked()`).
   - `src/cryptanalysis/ic_framework/plugins.rs` line 2090: `MitmOracle::prepare` with `params.flag("negation_folded")` calls `PairTable::build_negation_folded(ctx.group, fb)` (the scalar builder).
   - `plugins.rs` line 2267: `PrimeBatchMitmOracle::prepare` with `self.blocked` calls `PairTable::build_negation_folded_prime_blocked(ctx.group, fb)?`.
   - `src/cryptanalysis/ic_boundary.rs` lines 5760-5762: when `g.p <= u64::from(u32::MAX) && fb.points.len() <= 1 << 16`, that builder returns `PackedPairTable::build(g, fb, &mut ops)?` wrapped in the `PairTable`.
   - The oracle accepts only `m=2` (`plugins.rs`, `params.u64_or("m", 2)? != 2 || params.0.keys().any(|key| key != "m")` returns an error).

3. **The blocked oracle must refuse, with an error rather than a silent fallback, a field with p >= 2^32 or more than 2^16 signed points: FAIL (the code does the opposite).** `ic_boundary.rs` lines 5753-5777, `build_negation_folded_prime_blocked`:

        /// ... a quarter of the memory of the maps.  Larger fields keep the small maps.
        if g.p <= u64::from(u32::MAX) && fb.points.len() <= 1 << 16 {
            let packed = PackedPairTable::build(g, fb, &mut ops)?;
            ...
            return Ok(Self { kind: "negation_folded_prime_blocked", ... packed: Some(packed), ... });
        }
        Self::build_negation_folded_prime_blocked_maps(g, fb, start, ops)

   For p >= 2^32 or more than 2^16 signed points the oracle does not refuse. It builds the same table in the small-maps layout (`build_negation_folded_prime_blocked_maps`), and its `kind` string, `negation_folded_prime_blocked`, is the same in both layouts. The only refusals in the path are in `for_each_folded_prime_sum` (`ic_boundary.rs` line 5414): `g.p < 5 || g.p & 1 == 0 || g.p >= (1u64 << MAX_BITS)` with `prime_fast::MAX_BITS = 62`, a 2-torsion base point, and a repeated abscissa.
   So the check's premise (refusal at p >= 2^32 or more than 2^16 signed points) is false at be2969ce.

   Facts that bear on whether the six curves reach the packed layout (not a waiver; the Coordinator decides):
   - `ecbench plan --spec spec-P<b>.json --doc` gives `field_bits` 24, 28 and 32 for the six curves, so p < 2^32 on all six (the slugs `icv1-fp24-...`, `icv1-fp28-...`, `icv1-fp32-...`).
   - The factor base has N_inc abscissae, at most 2 N_inc = 2,048 signed points (the pilot record at 32 bits: `"signed_points": 2048, "abscissae": 1024`), far below 2^16.
   - So, by the code read, the six curves take the `PackedPairTable` branch. No part of the run records which layout was used.

4. Pilot clause: the pilot binary is 88070375...412c, so no smoke run is needed. PASS.
   - `pilot/run1/host.json` line 140: `"binary_sha256": "88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c"`, `"binary_path": "/dev/shm/isolab-build/ecbench"`.
   - `grep -o '"binary_sha256":"[0-9a-f]*"' pilot/run1/records.jsonl | sort | uniq -c` printed `6 "binary_sha256":"88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c"`.
   - `sha256sum /dev/shm/isolab-build/ecbench` printed the same hash.
   - Only these fields of the pilot were read: `outcome.status` and `outcome.matches_target` (6 of 6 `verified`, `True`), `cost.total_gae` (pairs `dg`/`dgb` equal: 1506948.0, 1671658.0, 1664684.0), `isolation.level` (`L0`) and the binary hash. No wall field.

## P3 the specs: PASS

`ecbench plan --spec <spec>` with the binary 88070375...412c exited 0 on all six (`spec-P24`, `spec-P24-rho`, `spec-P28`, `spec-P28-rho`, `spec-P32`, `spec-P32-rho`). Each printed 1344 executions and 48 workloads (24 per curve). Slugs (identical `workload` lines between the wall and counted plan of each size, `diff` empty):

    24: icv1-fp24-t1973-f876d133, icv1-fp24-tm995-ee6c519b
    28: icv1-fp28-t9805-a382b8a8, icv1-fp28-tm3463-6d2f5699
    32: icv1-fp32-tm43175-08b04f34, icv1-fp32-tm81759-3ad5e8c5

These equal the slugs of PROTOCOL.md section 3. Arms in spec order (name, role, method id), wall specs:

    24: dg baseline ECM1h63972f6444cd | dg-aa control ECM1h63972f6444cd | dgb candidate ECM1h7130702fe80a | dgb-edge reference ECM1h7130702fe80a
    28: dg baseline ECM1h06aa2f4e545c | dg-aa control ECM1h06aa2f4e545c | dgb candidate ECM1h0214efbf2ff5 | dgb-edge reference ECM1h0214efbf2ff5
    32: dg baseline ECM1h1e90ba7ea59e | dg-aa control ECM1h1e90ba7ea59e | dgb candidate ECM1h1ece5a06377d | dgb-edge reference ECM1h1ece5a06377d

Counted specs: `rho-neg` reference ECM1hefea4e0ebe79, `dg` baseline, `dgb` candidate, `rho-neg-aa` control ECM1hefea4e0ebe79 (same `dg` and `dgb` method ids as the wall spec at each size).

`jq` checks, run per size (all printed OK at 24, 28, 32; final line `ALLOK=1`):
- arms `["dg","dg-aa","dgb","dgb-edge"]` with roles `["baseline","control","candidate","reference"]`;
- `.arms[1].method == .arms[0].method` and `.arms[3].method == .arms[2].method`;
- `dgb` params minus `oracle` equal `dg` params minus `oracle`, the `oracle` values differ, and the method ids are equal;
- `jq -S` of the `dg` arm's method equals that of the `dg` arm of `backlog/XP-20261029-dg-wall/spec-P<b>.json` (`{"factor_base":"prime-abscissa:size=162|407|1024","oracle":"mitm:negation_folded=1","max_trials":"4243777|26479085|167871873","targets":"stride-renew-deferred-guard"}`);
- counted spec: `dg` and `dgb` arms equal the wall spec's, `rho-neg` and `rho-neg-aa` equal those of dg-wall's `spec-P<b>-rho.json`, `workloads` and `measurement` equal the wall spec's.

Spec sha256 at the working tree (`sha256sum backlog/XP-20261029-dg-blocked/spec-P*.json`) equal the six lines of the freeze commit message; `git diff --quiet cd04cfd2d1 -- backlog/XP-20261029-dg-blocked` exit 0:

    62267dc13d78ff7fbbf285933dc25581fa1de207464a3377bac4200441e3b54c  spec-P24.json
    f1e1a8081f58a57e2c6bfe6041cab9597c5aa16a4f6b03a0da981e511c01e669  spec-P24-rho.json
    8f1908c39e46344f3d1a4edc64d4a50d1e3ef7a6eba04c8fb368ef9e154a68f6  spec-P28.json
    3e69c4f9501b4d43291492da12bae551811d1bf54659f414c2cd6a81315cbfba  spec-P28-rho.json
    1e0f5fb9979e8916b625594a868f54fb201fa85a0161027ec0c83c12e39ba120  spec-P32.json
    9a23e873c9fd2d852601d691a9a89c19ef9d9116ccddcf70eb4f12b7167572e5  spec-P32-rho.json

## P4 checkpoint and scope: PASS (checkpoint); Conductor a known gap

- `cd /dev/shm/wt-blk && PATH=/tmp/claude-0/awsvenv/bin:$PATH bash scripts/checkpoint.sh gate` printed `checkpoint: unchanged since 20261029T124848Z-cd04cfd2d1df s3://setec-astronomy-artifacts/checkpoints/crypto/claude__ic-dg-blocked-20261029/20261029T124848Z-cd04cfd2d1df/` and `gate_exit=0`. `.resume/RESUME.md` is filled (Goal, Lead, Spec, State, Next steps sections present). PASS.
- `scripts/checkpoint.sh` sha256 56be5ef986ec87092f19434f5a527b3e14ffabf8d9e84afa0153a7928dd36d4f (equal to the one the dg-wall cards pinned).
- `which conductor` printed nothing and `/dev/shm/wt-blk/.conductor` does not exist: Conductor is not installed in this environment. No Conductor task holds the `backlog/XP-20261029-dg-blocked/` and `research/ic_dg_blocked_20261029/` scopes. Recorded as a known gap (the same for dg-wall), not a fail.

## P5 edits applied: PASS

- `grep -n 'dgb-lead'` on PROTOCOL.md and the six specs printed exactly two lines, both in section 2 "Why the drafted order fails" (E1's own text describing the replaced draft): `PROTOCOL.md:50` and `PROTOCOL.md:51`. No hit in any spec (`grep -c` printed 0 for each of the three wall specs; the counted specs were checked by the same grep, no output). APPROVAL.md also holds the string; it is not in P5's scope.
- E2's three fields are filled: PROTOCOL.md section 3 names the pilot binary `88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c`, the pilot output path `backlog/XP-20261029-dg-blocked/pilot/`, and the wall ratio `0.9146`; no placeholder text (`<pilot ...>`, `<value as seen>`) remains.
- E7's `git grep` output is listed: PROTOCOL.md lines 221-224 ("Output of that search ...") and `prior-data-search.txt` in the protocol directory.
