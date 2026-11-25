# Prime31 APS/rho V2 held-out input registration

Status: **registered inputs, performance run not admitted**
`[pcc:unverified]`. This amendment follows
`docs/experiment-seeds.md` (`isogeny-labs/experiment-seeds/v1`) and the
same-target equal-core comparison protocol in
`../aps_parallel_equal_core_20261124/PROTOCOL.md`.

`generate_heldout_v2.py` fixes the prime31 curve
`p=2147483647, a=847514846, b=2087444724, n=2147446831`, generator
`G=(625037715,2100627251)`, and 48 ordered public Q. It uses a recorded
256-bit master seed selected once with `openssl rand -hex 32` and the exact
HMAC-SHA-256 domain/framing in `heldout_v2_seed_manifest.json`. Case and
replicate IDs, five independent replicate streams, 64 logical rho lane
restart seeds, the paired folded/unfolded rho walk seed, APS source seed,
schedule seed, and analysis seed are all explicit in that manifest. Target
scalars are in `heldout_v2_caller_answers.json` for evaluator replay and
are absent from the public input file passed to a solver. The target
selection recorded zero rejections in 48 attempts. V2 public Q are unique
and disjoint from the archived 48 development Q and the superseded V1 Q.

SHA-256:

| Artifact | Digest |
| --- | --- |
| `generate_heldout_v2.py` | `1e5bc6bd68ed68f53129e073d30d56276fa18c7c61a65926bd83bf846a3e75fc` |
| `verify_heldout_v2.sage` | `70e1e6aab1fc672b0a42713f7c3051503f1343afcd9ccda74c82e2721d9e41fd` |
| `heldout_v2_seed_manifest.json` | `b9d5cb5dca1f46131616f5703284edfc769645ee48a5e187f929a1c3e187f64d` |
| `heldout_v2_public_targets.json` | `e02d4497a17a0d5830a4f4728d58c8eec914b4934f53810ef66fb9a74b1927ff` |
| `heldout_v2_caller_answers.json` | `c927f97c9d9c5ccbf4282bb7e085a1f9059f8392d2fc1400feec3eff61420119` |

`sage -python verify_heldout_v2.sage` independently checked the 48
caller-known scalar multiples on the stated curve, all 240 solver-stream
records, every HMAC-derived seed and schedule rotation, artifact digests,
and disjointness from development and V1 public Q. This verifies input
registration only [pcc:unverified]; no APS or rho solve used a V2 Q.

The native single-target entry points now accept the V2 seed streams. The
Linux runner, admission template, and analysis are specified in
`HELDOUT_V2_RUNBOOK.md`. A same-host A/A control, reviewed host isolation,
binary hash and actual resource admission remain outstanding. The existing
48 development Q remain the pilot population. Keep every timeout, failure,
cancelled cell and worker report; do not replace a held-out case. These
registered inputs make no timing or crossover claim.
