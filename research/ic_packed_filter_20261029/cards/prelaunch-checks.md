# Pre-launch checks P1 to P6, XP-20261029-packed-filter

author_role: experiment-director (Sonnet), read-only; run 2026-10-29 about 17:45 to 18:45 UTC
freeze commit: f870408c1762a52dd133e8f1e0bca181cc2a76f5 (branch claude/ic-packed-filter-20261029, pushed: `git ls-remote origin refs/heads/claude/ic-packed-filter-20261029` printed this SHA)
candidate code commit: 3271a25d02dcef6da148293fb8f9e2149ce59858
worktree: /dev/shm/src-pf (clean: `git status --short` empty before this directory was created)
binary: /dev/shm/pf-bundle/ecbench, sha256 21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503 (the image's binary, see P2)
checks as written in backlog/XP-20261029-packed-filter/APPROVAL.md, "Pre-launch checks" (fail closed). Scratch output was kept under /dev/shm/pf-work; the 0.9 GB of fetched PR blobs in /dev/shm/prscan2 was removed after P4.
no pilot records.jsonl and no wall field was opened by any check.

## Verdicts

| check | verdict |
|:--|:--|
| P1 the candidate commit | PASS (with two notes for the Coordinator, below) |
| P2 image and binary | PASS |
| P3 the specs | PASS |
| P4 fresh curves and seeds | PASS (scope stated below) |
| P5 checkpoint and scope | PASS for the checkpoint; Conductor is not installed (the known gap XP-20261029-dg-blocked recorded in its prelaunch-checks.md P4 row), recorded, not a fail |
| P6 edits applied | PASS |

All of P1 to P6 pass, so the launch card was written (cards/card-launch.md) and an analysis card (cards/card-analysis.md). No job was launched by this task.

## P1 the candidate commit: PASS

`git show --stat 3271a25d02` printed four changed files, and `git diff --name-only 3271a25d02 f870408c17` printed nothing outside `backlog/XP-20261029-packed-filter/`:

    docs/units/BITOPS.md                      |  11 +++
    src/cryptanalysis/ecbench/work.rs         |   4 +
    src/cryptanalysis/ic_boundary.rs          | 157 +++++++++++++++++++++++++++++-
    src/cryptanalysis/ic_framework/plugins.rs |  34 ++++++-
    4 files changed, 200 insertions(+), 6 deletions(-)

`git show 3271a25d02` read in full for the three source files. Every hunk belongs to the filter:

- `ic_boundary.rs`: the `PackedPairTable` fields `filter`, `filter_shift`, the constant `PACKED_FILTER_BITS_PER_ENTRY = 8`, `filter_bit`, the `filtered: bool` parameter of `PackedPairTable::build` and the filter build after the slots are filled, the guard in `PackedPairTable::get`, `PairTable::filter_bit_sets` with its entry `table_filter_bit_sets` in `work_counters`, `build_negation_folded_prime_blocked_with` with its refusal (`filter=1 needs the packed pair table …`), and two unit tests (`prime_blocked_filter_answers_every_key_as_the_unfiltered_table`, `prime_blocked_filter_refuses_the_small_maps_layout`).
- `plugins.rs`: parsing of `filter` (0 or 1) in `PrimeBatchMitmOracle::parameters` and `prepare`.
- `work.rs`: `"table_filter_bit_sets" => Prims(&[(WordOp, 6)])`.
- `BITOPS.md`: the `table_filter_bit_sets` row; the only change beyond `src/`.

On the unfiltered path (`filter` absent or 0) the behavioural changes are exactly: the guard `if !self.filter.is_empty()` in `PackedPairTable::get` (the vector is empty when off), and the call `build_negation_folded_prime_blocked` → `build_negation_folded_prime_blocked_with(g, fb, false)`. Two notes for the Coordinator, neither a behavioural change of lookups:

1. data plumbing the guard needs: the struct fields `filter: Vec::new()` and `filter_shift: 64` when off, and `filter_bit_sets: 0` in four of the five `PairTable` constructors (the fifth sets it to `packed.entries` when filtered); `kind` stays `"negation_folded_prime_blocked"` when `filtered` is false; `work_counters` adds the new key only when `filter_bit_sets > 0`;
2. the parameter-error text of `mitm-prime-blocked` changes to `accepts only m=2 and filter=0|1` (error path only); `mitm-prime-batch` keeps its text.

Commands and outputs:

    git diff --quiet 3271a25d02 -- src/cryptanalysis/ic_boundary.rs src/cryptanalysis/ic_framework/plugins.rs src/cryptanalysis/ecbench/work.rs ; echo $?
    0
    git diff --quiet 3271a25d02 f870408c17 -- src docs taskq scripts Cargo.toml Cargo.lock tools ; echo $?
    0

So the working tree equals `3271a25d02` in the three source files, and the freeze commit adds nothing outside `backlog/XP-20261029-packed-filter/`.

`record.rs` at `3271a25d02` (`git show 3271a25d02:src/cryptanalysis/ecbench/record.rs`, 2,174 lines): `grade_v3` is the default grader (`grade_for_version` 3 and `pub fn grade` call it, lines 1489 to 1504). Its `l2.push` sites are lines 1266 to 1439. The strings behind the seven regular expressions of PROTOCOL.md section 4 are, verbatim:

    1322  "{f} ticks of foreign work on the run CPU during the run"
    1331  "{st} ticks of hypervisor steal on the run CPU"
    1344  "{sb} busy ticks on the run CPU's idle siblings and the rest of the reservation"
    1356  "run-queue delay {} µs is {:.3}% of the solve"
    1373  "{n} involuntary context switches during a {:.3} ms solve (at most {allowed})"
    1387  "{n} involuntary context switches over the process (at most {allowed})"
    1400  "{} interrupts on the run CPU during the solve ({} device, {} IPI/other, {} softirq; at most {allowed})"
    1423  "{foreign_arrivals} foreign task slices on the run CPU during the solve ({} µs)"

The `µ` in the file is U+00B5 (`python3 -I` over the file: the set of its characters among {µ, μ} was {0xb5}). The prefix `L1: `, `L2: ` or `L3: ` is added at lines 1472 to 1475 (`format!("{lv}: {b}")`). The nine other `L2` strings of `grade_v3` (`preflight not quiet: …`, `no preflight`, `preflight thresholds looser than the defaults: …`, `solve too short to admit wall time: …` twice, `no /proc/stat ticks for the run CPU`, `no schedstat for the solve`, `… of memory stall system-wide`, `no memory PSI`) match none of the seven expressions, as PROTOCOL.md section 4 requires.

`cards/tools/test_trigger.py` renders each of the eight format strings above with numbers and checks `cards/tools/trigger.jq` (the jq program on the launch card, an anchored-regex implementation of the seven expressions): `python3 -I test_trigger.py trigger.jq /dev/shm/pf-work /dev/shm/pf-work/dgb-ref/research/ic_dg_blocked_20261029` printed 32 lines (output kept at cards/tools/test_trigger.out), the last `ALL_FIXTURES_AS_EXPECTED True`. Eight renderings (including both involuntary-switch forms) are accepted; the nine other `L2` strings of the list above, an `L1: ` string, a bare string, a trailing suffix, the Greek mu (U+03BC) in place of U+00B5, a level `L2` or `L0` entry, an empty `below_l2`, a non-empty or absent `invalid_outcomes` and an absent `blockers` are all refused. On the previous study's real `l2-gate.json` files (`git show ee55de347e74b66e61dfced83fa10358b63805a1:research/ic_dg_blocked_20261029/P<b>/wall/eks-results/l2-gate.json`, the head of PR #2855): the 24-bit and 28-bit wall sessions give exit status 1 (false) and the 32-bit wall session, the one the clause was written for, gives exit status 0 (true).

`taskq/deploy/eks-karpenter/run-ecbench` at `3271a25d02` (identical in the freeze commit: `git diff --quiet 3271a25d02 f870408c17 -- taskq` exit 0). In this file the lines the APPROVAL names are 57 to 67 (not 54 to 67):

    57  jq -s '{required:"L2",executions:length,
    58    invalid_outcomes:[.[] | select(.outcome.status != "verified" or .outcome.matches_target != true) | {seq,arm,outcome}],
    59    below_l2:[.[] | select(.isolation.level != "L2" and .isolation.level != "L3") | {seq,arm,isolation}],
    60    grades:(group_by(.isolation.level)|map({level:.[0].isolation.level,count:length}))}
    61    | .accepted=(.executions > 0 and (.below_l2|length)==0 and (.invalid_outcomes|length)==0)' \
    62    "$ISOLAB_OUTPUT_DIR/session/records.jsonl" > "$ISOLAB_OUTPUT_DIR/l2-gate.json"
    63  jq -e '.accepted' "$ISOLAB_OUTPUT_DIR/l2-gate.json" >/dev/null || exit 42
    64  reference=$(jq -r '.arms[] | select(.role=="reference") | .name' "$spec" | head -1)
    65  control=$(jq -r '.arms[] | select(.role=="control") | .name' "$spec" | head -1)
    66  ecbench compare --dir "$ISOLAB_OUTPUT_DIR/session" --a "$reference" --b "$control" --json \
    67    > "$ISOLAB_OUTPUT_DIR/control-comparison.json"

- it writes `l2-gate.json` with `invalid_outcomes` and `below_l2[] = {seq, arm, isolation}`: yes (57 to 62);
- it exits 42 before it writes `control-comparison.json`: yes (63 against 66 to 67);
- it takes the first `reference` and the first `control` arm in spec order: yes (64 and 65, `head -1`).

Limit: the script that runs in the cluster is the one compiled into the deployed controller (`taskq/deploy/eks-karpenter/mcp/src/main.rs` line 365 and `resume/controller/src/cloud.rs` line 562 `include_str!`); it cannot be read from here. The job's own files (`job-exit.json`, `l2-gate.json`, `run.stderr`, `input-spec.json`) are what evidence the behaviour, and the previous study's six sessions behaved as lines 57 to 69 say.

## P2 image and binary: PASS

    aws ecr get-login-password | docker login --username AWS --password-stdin 104186811093.dkr.ecr.us-east-2.amazonaws.com
    Login Succeeded
    docker pull 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:32144a29e437376b9bf8b902aef4ba6b9cc723a15f01d296fb7b09eb131b817b
    Digest: sha256:32144a29e437376b9bf8b902aef4ba6b9cc723a15f01d296fb7b09eb131b817b
    Status: Image is up to date for 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:32144a29e437376b9bf8b902aef4ba6b9cc723a15f01d296fb7b09eb131b817b
    aws ecr describe-images --repository-name isolab/tasks --image-ids imageDigest=sha256:32144a29… :
      digest sha256:32144a29e437376b9bf8b902aef4ba6b9cc723a15f01d296fb7b09eb131b817b, tags ["local-20261029-pf-3271a25d02dcef6da148293fb8f9e2149ce59858"], size 153301362
    docker run --rm --entrypoint sh <image> -c 'sha256sum /usr/local/bin/ecbench; cat /opt/isolab/build-info.json':
      21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503  /usr/local/bin/ecbench
      "source_revision": "3271a25d02dcef6da148293fb8f9e2149ce59858", "ecbench_sha256": "21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503",
      "architecture": "linux/amd64", "target": "x86_64-unknown-linux-musl", "default_features": false, "rustflags": "-C target-cpu=x86-64"
    sha256sum /dev/shm/pf-bundle/ecbench
    21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503  /dev/shm/pf-bundle/ecbench
    the image's /opt/isolab/build-info.json and /dev/shm/pf-bundle/build-info.json are byte-identical (diff printed nothing).

`validate-request`: `cards/tools/build_requests.py` wrote eighteen `isolab.eks.request/v1` files (the six first launches and, for the reruns of PROTOCOL.md section 4, the host-loss reruns `-r2` and `-rho-r2`, the noise rerun `-nr` and its host-loss rerun `-nr-r2`; only the job name differs within a size), each embedding its frozen spec, the image digest, `source_revision` 3271a25d02dcef6da148293fb8f9e2149ce59858, `ecbench_sha256` 21701be6…4503, c8a.2xlarge, 3600 s. `taskq/deploy/eks-karpenter/remote/validate-request` (sha256 dc6b59d32e937db5c049704102fe0d5ca0db7af9ac2bd8de67270bcafc230426, the same as the previous study's) exited 0 on all eighteen; for each of the twelve contingent requests, `diff <(jq -S 'del(.job.job_name)' request-P<b>[-rho].json) <(jq -S 'del(.job.job_name)' <the rerun request>)` printed no line (only the job name differs). The six first-launch requests are among the eighteen:

    VALID request-P24.json  request-P24-rho.json  request-P28.json  request-P28-rho.json  request-P32.json  request-P32-rho.json  (and the twelve contingent ones)

L0 run with the image's binary (the bundle's `ecbench`, same sha256) on `pilot/spec32.json`, output `/dev/shm/pf-work/p2run32` (`--cpus none --allow-busy`, arms `dg`, `dgb`, `dgbf`, 3 targets, 1 round). Summary line: `session ECBS1hb2108533e193 complete (9 records: {"verified": 9})`. Only the fields P2 lists were read:

- `outcome.status` and `outcome.matches_target`: `jq -r '[.arm,.outcome.status,.outcome.matches_target]|@tsv' records.jsonl | sort | uniq -c` printed `3 dg verified true`, `3 dgb verified true`, `3 dgbf verified true`: every outcome verified.
- `phases[oracle_setup].work_native.table_filter_bit_sets`: `null` on every `dg` and `dgb` record; 1,048,173 on each of the three `dgbf` records; no other phase carries the counter.
- the `ops` block: `ecbench compare --dir p2run32 --a dgb --b dgbf --weight W --json | jq .ops` at W = 0, 0.1 and 1: `ratio_b_over_a` 1.0 (`ci95` [1.0, 1.0], `band.ratio_b_over_a` [1.0, 1.0, 1.0]). Counted `ops` dgbf / dgb is exactly 1 at every weight.
- `ecbench work p2run32` totals (Σ `work.Ok.bitops` per arm): `dg` 326,608,566,048; `dgb` 326,608,566,048; `dgbf` 327,816,061,344 (ratio dgbf / dgb 1.003697). Matched by target, the per-record difference dgbf − dgb is 402,498,432 on all three, equal to 384 × 1,048,173; the phase bitops that differ are `oracle_setup` only (`{"oracle_setup": 402498432}` for each).

No `time`, `online`, `solve_wall_ns` or `wall` field was read; `compare` was piped through `jq .ops`.

## P3 the specs: PASS

`bash /dev/shm/pf-work/p3.sh` (script and output kept at cards/tools/p3.sh and p3.out; 39 checks, `P3_FAIL=0`). With the image's binary, `ecbench plan --spec backlog/XP-20261029-packed-filter/spec-P<b>[-rho].json` exited 0 on all six and printed 1344 executions each; the plan text of every spec is byte-identical to the committed `plan-P*.txt`. The previous study's specs used for the comparison are `backlog/XP-20261029-dg-blocked/spec-P*.json` as committed at its freeze commit cd04cfd2d1df985e34f59b6a19bb8354e0f9a5ae (extracted with `git show cd04cfd2d1:…` into /dev/shm/pf-work/dgb-ref, because the /dev/shm/wt-chain worktree was moved to another branch while this task ran; an earlier run of the same checks against that worktree's files gave the same 39 OK lines, and those files were byte-identical (`cmp`) to the frozen copies `research/ic_dg_blocked_20261029/validation/frozen/spec-P*.json`). Their sha256 are the six values 62267dc1…, f1e1a808…, 8f1908c3…, 3e69c4f9…, 1e0f5fb9…, 9a23e873….

Slugs (§3), identical between the wall and counted plan of each size (`diff` of the `workload` lines empty):

    24: icv1-fp24-t217-e1fbffd2, icv1-fp24-tm567-4accd0e8
    28: icv1-fp28-tm295-85243301, icv1-fp28-tm8273-f1778659
    32: icv1-fp32-t89141-95879fe3, icv1-fp32-tm68303-f902d2f0

Arms in spec order (name, role, method id):

    wall 24: dgb baseline ECM1h7130702fe80a | dgb-aa control ECM1h7130702fe80a | dgbf candidate ECM1h72ec76c15b6d | dgbf-edge reference ECM1h72ec76c15b6d
    wall 28: dgb baseline ECM1h0214efbf2ff5 | dgb-aa control ECM1h0214efbf2ff5 | dgbf candidate ECM1h517b28481d4b | dgbf-edge reference ECM1h517b28481d4b
    wall 32: dgb baseline ECM1h1ece5a06377d | dgb-aa control ECM1h1ece5a06377d | dgbf candidate ECM1h201ac28dd4c3 | dgbf-edge reference ECM1h201ac28dd4c3
    counted (each size): rho-neg reference ECM1hefea4e0ebe79 | dgb baseline | dgbf candidate | rho-neg-aa control ECM1hefea4e0ebe79

`jq` checks, each printed OK at 24, 28 and 32: arm names and roles in order; `dgb-aa` method equals `dgb`'s and `dgbf-edge`'s equals `dgbf`'s; `dgbf` params equal `dgb`'s except `oracle`, which is `dgb`'s `oracle` plus `,filter=1` (`mitm-prime-blocked:m=2` and `mitm-prime-blocked:m=2,filter=1`); `jq -S` of the `dgb` method equals that of the `dgb` arm of the previous study's `spec-P<b>.json`; counted `dgb` and `dgbf` arms equal the wall spec's; counted `rho-neg` and `rho-neg-aa` equal those of the previous study's `spec-P<b>-rho.json`; counted `workloads` and `measurement` equal the wall spec's. Measurement per size: `{"rounds":6,"warmup":1,"order":"alternate","seed":202610296<b>8,"isolation_required":"L2","timeout_seconds":1800}`.

## P4 fresh curves and seeds: PASS

Strings (18): the six slugs above and the twelve seeds 202610296243, 202610296244, 202610296246, 202610296248, 202610296283, 202610296284, 202610296286, 202610296288, 202610296323, 202610296324, 202610296326, 202610296328 (each equals the spec value: `curves[].seed`, `target_seed`, `measurement.seed`). Fixed-string search, `git grep -alF -e S1 … -e S18` on trees and, for blobs fetched from the GitHub remote, a byte-substring scan of each blob (`cards/tools/p4scan.py`, the same fixed strings, `git cat-file --batch`).

What was searched:

1. `origin/main` as fetched into the local clone, 117c9c4f3ee85d4dc32f275893f6e50b591e4a47 (the parent of the candidate commit): the whole tree, 176,122 entries, `git grep -alF` over 18 strings: 0 files (23 s).
2. `origin/main` now, 78f84234e18facfc46be52741d7fa6acadc92b88 (`git ls-remote origin refs/heads/main` printed this SHA, equal to the head the Coordinator fetched): 86 commits after 117c9c4f; the 855 added or modified paths (774 distinct blobs) fetched and scanned: 0 hits.
3. The 112 open PR heads fetched by the Coordinator (`refs/pr/<n>` in /dev/shm/prscan): for each, every blob it adds or modifies against its merge base with main, `git diff-tree -r --no-renames --diff-filter=AM` (68,775 path entries, 33,292 distinct blobs, of which 28,881 were fetched for this check): 0 hits. `git ls-remote origin 'refs/pull/*/head'` (2,851 pull heads) shows that none of the 112 heads has moved since the fetch. Pull requests 2870 and 2871, which are numbered above the fetched set, were fetched and are ancestors of main (contained in item 2).
4. 24 pull requests numbered 2400 or above that were not among the 112 and are not ancestors of main (closed, or not in the Coordinator's open set): 2402, 2449, 2473, 2476, 2487, 2491, 2492, 2535, 2560, 2590, 2634, 2713, 2714, 2715, 2725, 2726, 2769, 2777, 2782, 2799, 2801, 2802, 2824, 2825. Their changed blobs (5,749 distinct) were scanned too: 0 hits.
5. The branch itself: `git diff-tree -r --name-only --diff-filter=AM 117c9c4f f870408c17` (32 paths), `git grep -alF` at f870408c17: 13 files, all under `backlog/XP-20261029-packed-filter/` (PROTOCOL.md, the six `plan-P*.txt`, the six `spec-P*.json`); 0 hits outside that directory.

Totals of the byte scan (items 2, 3): 34,061 blobs, 2,510,242,024 bytes, 0 missing, 0 hits; item 4: 5,749 blobs, 684,081,943 bytes, 0 hits.

Re-scan at the end of the task (about 18:40 UTC), because the remote moved while this task ran: `git ls-remote origin refs/heads/main` then printed 8171fb53733b43b3174abf66c839d43037a30fc2 (6 commits after 78f84234, the merge of PR #2868), and 11 of the 112 scanned pull-request heads had moved (2381, 2642, 2666, 2706, 2826, 2845, 2849, 2855, 2856, 2868, 2869); no pull request numbered above 2871 exists. The new main and the 11 new heads were fetched; of their changed blobs against the new merge bases, 284 had not been scanned before (422,488,496 bytes): 0 hits (the control string `mitm-prime-blocked:m=2` hit 12 of them). So the result holds for main 8171fb53 and the heads of that moment. The branch head was still f870408c (`git ls-remote`).

Positive controls (the same scanner with other strings): `icv1-fp28-t9805-a382b8a8` (a curve of XP-20261029-dg-blocked) 43 blobs, `202610295283` (its seed) 9 blobs, `mitm-prime-blocked:m=2` 128 blobs, all in pull requests such as #2855 (`research/ic_dg_blocked_20261029/P28/wall/...`). A control for the whole-tree grep: a 19-digit string from the middle of a 424 KB `.jsonl` and a token from the 96 MB `docs/curves/traits.json` were each found at 117c9c4f.

Every hit lies under `backlog/XP-20261029-packed-filter/` on this branch; there is no other hit.

Scope limits (stated, not waived): (a) a PR head's blobs that equal its merge base are not rescanned (they are old main content; items 1 and 2 cover main); (b) compressed or binary blobs (`.gz`, `.png`, `.pdf`) cannot contain these strings in plain text; (c) the open-PR list is the Coordinator's fetch of 112 heads; the GitHub API was not reachable from this session (HTTP 403), so open versus closed was not re-read, and item 4 widens the scan instead.

## P5 checkpoint and scope: PASS (checkpoint); Conductor a known gap

- `cd /dev/shm/src-pf && PATH=/tmp/claude-0/awsvenv/bin:$PATH bash scripts/checkpoint.sh gate` printed `checkpoint: unchanged since 20261029T174410Z-f870408c1762 s3://setec-astronomy-artifacts/checkpoints/crypto/claude__ic-packed-filter-20261029/20261029T174410Z-f870408c1762/` and `gate_exit=0`. `aws s3 ls` of that prefix lists `RESUME.md`, `checkpoint.tar.gz`, `meta.json`. `.resume/RESUME.md` is filled (sections Goal, Lead / hypothesis, Spec, State, Next steps, Runs in flight, Stop and success conditions, Open questions). PASS.
- `scripts/checkpoint.sh` sha256 56be5ef986ec87092f19434f5a527b3e14ffabf8d9e84afa0153a7928dd36d4f (equal to the one the previous study's cards pinned).
- `which conductor` printed nothing and `/dev/shm/src-pf/.conductor` does not exist: Conductor is not installed in this environment. No Conductor task holds the `backlog/XP-20261029-packed-filter/` and `research/ic_packed_filter_20261029/` scopes. Recorded as the known gap XP-20261029-dg-blocked recorded (its prelaunch-checks.md P4 row), not a fail.

## P6 edits applied: PASS

- `grep -n 'wall-r2' backlog/XP-20261029-packed-filter/PROTOCOL.md` printed nothing (exit status 1).
- `grep -c 'conditions 1–10' PROTOCOL.md` printed 1.
- E7's two fields are filled: PROTOCOL.md §8 states, for the pilot's `dg` comparisons, "not computed and not seen" with the reason, and, for the `filter=1` search over open PR heads, "none" with the method; `grep -n '<filled\|<author\|<pilot' PROTOCOL.md` printed nothing.
- `sha256sum` of the frozen files equals the values in the dispatch for PROTOCOL.md (703bde26a197859c02e2e5f293ba6719cb5b71871f3044a7ac336d346d067e51) and APPROVAL.md (0a35923cca184b68c212b66a70d2d42e2e2546e1cd50fd6ebe99d854889a6a40). The six specs are in the freeze commit with no earlier version on the branch; the freeze commit message says they are byte-identical to the reviewed drafts, and the working tree equals the commit (`git diff --quiet f870408c17 -- backlog/XP-20261029-packed-filter` exit 0). The drafts themselves are not in the repository, so identity with them rests on that statement; the Coordinator's checks of the specs (§5 of the APPROVAL "Reasons") were re-derived above in P3. The spec sha256:

        c98bf0c914efceef796be5f00a86e18c75e426a1ca8cf9e63fab748c23db0321  spec-P24-rho.json
        ce9510367b95c5038b4b200675943b2e81bc9f395921cfe10f87973ab7f151e7  spec-P24.json
        55a403714d9cb353eb1fe644eb9e04e628ef46b1a1ada5b8575709db40cc349a  spec-P28-rho.json
        9f54fdd0305c789f6b12202fbb81bee50a05135fca674fae65c7557f531053d9  spec-P28.json
        2095978592b5d21713d5de32dec6c1b862cc7e0d3810c3f82f8399c4a2c45389  spec-P32-rho.json
        9f950f3bcba5e4729798c82bcca1a1d665104329b5ed9368c9d98b6addbc30ab  spec-P32.json
