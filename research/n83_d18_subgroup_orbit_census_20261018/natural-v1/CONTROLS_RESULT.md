# Paired N83 Spot import and constructed-control gate

The source- and binary-attested Spot worker imported both exact 16,627-column, 2,760,082-point bases and the same public target under the frozen 4 GiB/zero-swap Docker envelope. U and D then each returned the frozen first-four-representative constructed relation. These are functional controls only; their search times are not natural relation-yield samples, and no total-runtime candidate is selected.

| Arm | Host exit | Worker status | Worker wall | Peak cgroup memory | Result |
| --- | ---: | --- | ---: | ---: | --- |
| First U preflight, pilot-v1 receipts | 1 | no worker receipt | n/a | n/a | `requested primary base absent`; retained input-packaging failure |
| Corrected U-v2 preflight | 0 | `PASS_PREFLIGHT` | 31.850 s | not emitted by preflight | exact U object and public point, solver not entered |
| D18 preflight | 0 | `PASS_PREFLIGHT` | 31.725 s | not emitted by preflight | exact D object and public point, solver not entered |
| U constructed control | 0 | `PASS_CONSTRUCTED_CONTROL` | 32.368 s | 2,570,530,816 B | group-checked first-four relation, rank increment 1 |
| D constructed control | 0 | `PASS_CONSTRUCTED_CONTROL` | 32.268 s | 2,652,684,288 B | group-checked first-four relation, rank increment 1 |

The first U failure is preserved under `spot-arms/p-u/`; [the input correction](INPUT_CORRECTION.md) froze the exact S3 v2 panel receipts before the fresh `p-u-v2` run. Every completed container inspect reports a 4,294,967,296-byte memory cap, no swap allowance beyond it, a one-CPU quota, `NetworkMode=none`, read-only root and read-only binary/input/snapshot mounts, writable output only, and `OOMKilled=false`. Both positive models had 201,598 variables, 5,915,768 clauses and 3,901 XOR rows. Their 9–10 ms pinned SAT times cannot predict unpinned search.

The native arm64 replay binary, SHA-256 `b6574f47066dd5749a06f6feed1a856909f7794f0b19ffc7a8adeaef0e670525`, separately reimported each exact base and its Spot worker receipt. Both replay receipts report `PASS`, the same dense-row BLAKE3 `52b06cef457ecd736ba3f125f4c62bacd31d9bbfc84f2bf6b8d84513f947fe0f`, rank increment 1, and changed-point/changed-row rejection. Four additional forged-receipt subprocesses, one changed point and one changed row per base, exited 1 with no success file; their input and stderr hashes are in `arm64-replay/negative-outcomes.json`. This replay uses a separate architecture and process but shares the repository's field and curve arithmetic; it does not replay SAT assignment bits.

The original host package passed 4,616 SHA-256 entry checks. The 84 retained control/build files are bound by `controls-raw-files.sha256` and its `controls-raw-verify.json` receipt. The two reduced-source build packaging failures and successful 4m16s Linux build are retained under `host-setup/`. The source commit is `fdf36e4b471ec65ea0c77be81d1b266fb417e7b5`, the Linux worker SHA-256 is `67a6e46a1477091f5c8db81b5932b742f4035de6d7397fd15866ead227f50c02`, and the runtime image digest is in [the pre-arm amendment](SPOT_DOCKER_ADDENDUM.md). The first AWS account rejected an EC2 launch at account level; its temporary security group was removed. The second configured account supplied the Spot host. Modal capacity probes did not enter this experiment because they did not expose the required cgroup-v2 swap and peak-memory files.

Natural arms follow the frozen `U1, D1, D2, U2` order on the same host. A single bounded query cannot estimate natural useful-rank yield or full index-calculus cost. `candidate_id`, `selected_best_total_runtime`, and any speedup claim remain null.
