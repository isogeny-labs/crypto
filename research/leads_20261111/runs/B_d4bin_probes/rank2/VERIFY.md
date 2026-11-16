# VERIFY: d4bin-rank2-elimination (independent skeptic)

Date 2026-11-12. Reviewer: an independent skeptic subagent. It did not write the probe.
Every result statement is **[pcc:unverified]**. No pcc claim exists for this result.
Local only. No RunPod pod was created.

## Verdict

**The key number is confirmed, within the stated limits.** The frob_roots arm paired counted-cost
ratio (elimination / current) is 0.6891. My own bootstrap gives a 95% interval of [0.6885, 0.6896],
from per-target rows I dumped myself. The solution sets are identical on 300/300 targets at m = 12.
The frozen success rule held. I found no refutation. Several caveats and one reporting defect are
listed below. [pcc:unverified]

## What I did

1. **Raw records.** The producer saved only aggregated stdout (`rank2_m12.txt`, `rank2_m8.txt`).
   No per-target rows exist, so the paired bootstrap could not be re-derived from the saved data.
   To get rows, I copied the probe into my scratchpad as `audit.rs`, sha256
   `09b3e20f34f1a4ac355c73ca9999a582f8a717760802eb103a095b29ae485d44`. I made three mechanical changes,
   none of them to the algorithm:
   - every `g.mul`/`g.inv` goes through a wrapper that counts the real calls;
   - `roots_with` goes through a wrapper that records the library's own tally;
   - each paired row is appended to a CSV.

   I compiled it with `rustc` against the existing `libcrypto_lib-42993a31e73be122.rlib`. Library
   source: `git diff HEAD -- src Cargo.toml Cargo.lock` is empty. No repository file was added or changed.
2. **Re-run.** I ran `RAYON_NUM_THREADS=1 ./audit_bin 12 300` (2 min 29 s) and `./audit_bin 8 300`.
   After timing lines are dropped, both stdouts are **byte-identical** to the producer's
   `rank2_m12.txt` and `rank2_m8.txt`.
3. **Own analysis** (`ana.py`, my Python, 10,000 paired resamples with my own seed), run on the CSVs:

| m | Arm | Current muls/trial | Elimination muls/trial | Ratio | My 95% bootstrap | Per-target ratio min / median / max | Cheaper on |
|---|---|---|---|---|---|---|---|
| 12 | frob_roots | 191,046.2 | 131,644.6 | **0.6891** | [0.6885, 0.6896] | 0.6727 / 0.6895 / 0.7075 | 300/300 |
| 12 | baseline | 223,574.6 | 225,123.3 | 1.0069 | [1.0061, 1.0077] | 0.9800 / 1.0078 / 1.0287 | 47/300 |
| 8 | frob_roots | 181,451.0 | 101,600.2 | 0.5599 | [0.5593, 0.5605] | 0.5342 / 0.5602 / 0.5721 | 300/300 |
| 8 | baseline | 189,598.7 | 109,973.8 | 0.5800 | [0.5794, 0.5807] | 0.5544 / 0.5804 / 0.5928 | 300/300 |

   Solutions at m = 12: agree on 300/300, 324 = 324 solutions. At m = 8: agree on 297/300; current
   finds 307 solutions, elimination 310. All of these match the producer. [pcc:unverified]

4. **Counting audit.** This is the main risk in a counted-cost claim. Outside `roots_with`, which uses
   the library's own counter on both arms, I compared the probe's manual tally with the real number of
   `GfTab::mul` + `GfTab::inv` calls:
   - The only uncounted calls are the 7,260 Newton node-difference inverses per trial
     (= 121·120/2). They are documented in RESULT.md caveat 1, and they can be precomputed because the
     nodes are fixed at 0..120.
   - With those set aside, the manual tally is **at least** the real count on every m = 12 target
     (excess 2..15, mean 5.8): it overcounts slightly. At m = 8 it ranges from -4 to +503.
   - Charging the 7,260 Newton inverses at one multiplication each moves the frob ratio to
     (131,645 + 7,260) / 191,046 = **0.727**. That is still well below 1.

   No undercounting that would change the verdict was found. [pcc:unverified]
5. **Control.** The current arm equals the committed lever record:
   - `research/ic_d4_char2_lever_20261023/REPORT.md` line 51 gives totals of 224,196 and 191,435;
   - `../control_profile_m12.txt` gives split phases of 621 and 389;
   - 224,196 - 621 = 223,575 and 191,435 - 389 = 191,046, matching the current arm exactly.

   The component sums in RESULT.md add up (frob elimination: 1,431 + 131 + 1,428 + 40,161 + 14,520 +
   72,640 + 871 + 225 + 238 = 131,645).
6. **Descriptive wall time** (not in the producer's report). `audit_wall.rs` adds `Instant` timers
   around each solve; it ran on the first 60 m = 12 targets. The VM is shared, there was one run, and
   other agents were active:

   | Path | Wall time per trial |
   |---|---|
   | current, baseline | 5.96 ms |
   | current, frob | 5.61 ms |
   | elimination, baseline | 0.67 ms |
   | elimination, frob | 0.54 ms |

   So the wall time points the same way as the counted metric, and more strongly. This is descriptive
   only and is not a benchmark. The current path includes uncounted Macaulay construction and shape
   tracing. [pcc:unverified]

## Checks against the preregistration (PREREG.md, frozen 20:46 UTC)

- **Success rule.** It needs identical solutions on 300/300 at m = 12, and a paired 95% interval for
  the frob ratio wholly below 1.0. Both hold.
- **Comparison.** The frozen comparison was used unchanged: the same targets (curve
  `find_bin_curve(12,1,16)`, RNG seeds 1^0x9e and 77), the same phase set, and the split phase
  excluded.
- **Predictions** were kept and reported honestly:
  - frob ratio: 0.689 measured, outside and below the 80% interval [0.8, 1.7];
  - baseline ratio: 1.007 measured, inside [1.0, 2.2];
  - deg R: 120 measured, inside [64, 144];
  - root finding on R: 72.6k measured, inside the 70–120k prediction;
  - resultant: 40k measured, against a predicted ~70k.
- **Variant.** The implemented variant matches the frozen steps 1–7. Two additions are both disclosed:
  the separate s4 = 0 line and the D = 0 rank-1 handling.
- **m = 8 disagreements.** Under the frozen correctness wording, any extra or missing solution counts
  as a disagreement. The m = 8 run, which the frozen verdict does not cover, has 3 disagreements. They
  are reported. In each case the extra solution is on the elimination side, has s4 = 0, and agreed with
  the producer's brute force. I did **not** independently re-run the brute force: it is the producer's
  `brute` function, and my copy is the same code.

## Source changes and default behaviour

- The only change is the new, untracked `examples/d4bin_rank2_elim_probe.rs`, sha256
  `fe990609…3e963`, which matches `binary.sha256`.
- The existing binary has sha256 `7e582393…3fa7`, which also matches.
- `git diff HEAD -- src Cargo.toml Cargo.lock` is empty, so the default path is unchanged. The
  current arm reproduces the committed numbers bit for bit.
- I did not re-run the 11-minute `cargo test`. Since the library has no diff, that test only re-checks
  HEAD.

## RunPod

I listed the account's pods with `GET https://rest.runpod.io/v1/pods`: 28 pods. None is named `bdr-*`.
This task created no pod, and I created none.

## Issues found (none refutes the verdict)

1. **No per-target raw records were saved.** The bootstrap could only be re-derived by re-running.
   That re-run is now done: CSV rows are in the reviewer's scratchpad. A claim should keep the rows,
   for example as a bench.record raw file.
2. **The reporting line is wrong.** `rank2_m8.txt` says "rational solutions 307 … with s4 = 0: 0", but
   that count covers only the *current* path's solutions. The elimination path returned 310 solutions,
   3 of them with s4 = 0. RESULT.md's text describes this correctly; only the stdout line is misleading.
3. **The Newton inverses are treated as free.** That is justified for fixed nodes, but it is a
   modelling choice. Including them gives a ratio of 0.727.
4. **Scope is narrow.**
   - One curve per m.
   - Counted multiplications only. The producer measured no wall time; my 60-target wall reading is
     descriptive.
   - The probe is not wired into `solve_d4_bin`, and no relation-rate or end-to-end measurement exists.
   - At m = 12 the gain comes only from the SqMod root-finding arm.
5. **The ~0.42 projection in "Next steps" is unmeasured.** It should stay labelled as a projection.
6. The m = 8 brute-force completeness evidence comes from the producer's own code. I did not
   reproduce it independently.

## Commands (reviewer)

```sh
S=/tmp/claude-0/-home-user/5d26ea47-4c79-5b59-bd9c-e47e6411a571/scratchpad/audit
D=/home/user/crypto/target/release/deps
rustc --edition 2021 -C opt-level=3 $S/audit.rs -o $S/audit_bin \
  --extern crypto_lib=$D/libcrypto_lib-42993a31e73be122.rlib \
  --extern rand=$D/librand-995139fa871833e1.rlib -L dependency=$D
AUDIT_CSV=$S/m12.csv RAYON_NUM_THREADS=1 $S/audit_bin 12 300   # stdout identical to rank2_m12.txt
AUDIT_CSV=$S/m8.csv  RAYON_NUM_THREADS=1 $S/audit_bin 8 300    # stdout identical to rank2_m8.txt
python3 $S/ana.py $S/m12.csv $S/m8.csv
AUDIT_CSV=/dev/null RAYON_NUM_THREADS=1 $S/audit_wall 12 60    # descriptive wall
```

Reviewer files are in the scratchpad, not the repository. Their sha256:

| File | sha256 |
|---|---|
| `audit.rs` | `09b3e20f…5d44` |
| `audit_wall.rs` | `3b4122aa…bea` |
| `ana.py` | `ff03b876…6b46` |
| `m12.csv` | `fbae7d34…bd8a` |
| `m8.csv` | `c170322f807fd88cfa20dc5c3ac95dd41fdfb244d59bccfa6423867de56ed041` |
