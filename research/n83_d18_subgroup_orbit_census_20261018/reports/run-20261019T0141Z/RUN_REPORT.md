# Direct-subgroup N83 dimension-18 census run

**Protocol:** [PROTOCOL.md](../../PROTOCOL.md), committed before enumeration as `b28bf2904`. **Native source:** `9e483ab2ea4ce4d18b6b4e3bc5070a8db547b8c2`; binary SHA-256 `571f993f4bddc32e2e189d0778ce5b62f883460dff1dcf65a65788cc4f368bee`. **Host:** [Darwin 25.6.0 arm64, macOS 26.6](../../host.txt), shared L0, with another release library suite active. **Curve:** primary `icv1-f2m83-tm6151469093347-debefd74`, a=0, subgroup order 2417851639230796216685689, cofactor 4. The source is the complete polynomial-basis span of bits 0–17, enumerated as x=0..262143, with no target input. The run's [count diagram](counts.svg) has its numeric source in [counts-data.json](counts-data.json).

| attempt | UTC interval | exit | internally measured process wall | disposition |
|:--|:--|--:|--:|:--|
| 1, `/usr/bin/time -l` wrapper | 2026-10-19 01:41:41–01:42:06 | 1 (wrapper) | 24,721.084083 ms | child wrote complete `SIZE_PREMISE_PASS` output; wrapper's post-run `sysctl kern.clockrate` was denied by sandbox |
| 2, direct native binary | 2026-10-19 01:42:39–01:42:58 | 0 | 18,865.460709 ms | complete `SIZE_PREMISE_PASS` output |

The native command in both attempts was `target/release/examples/n83_d18_subgroup_orbit_census NEW_OUTPUT_DIR 18`, with the first prefixed by `/usr/bin/time -l`. The implementation lifted each x on the original curve, checked `[r]P=O` on the packed wide-word curve, walked up to 83 Frobenius squarings, and keyed each retained x by the minimum word in its full orbit. Its complete counts are:

| count | exact value |
|:--|--:|
| enumerated x words | 262,144 |
| x with a curve lift | 130,724 |
| x with a subgroup lift | 32,710 |
| subgroup x in a short Frobenius orbit | 0 |
| distinct full signed-Frobenius orbit classes | 32,643 |
| classes with 1 / 2 / 3 source x values | 32,577 / 65 / 1 |
| selected columns under the frozen rule | 16,627 |
| points in a selected full closure, derived | 2,760,082 |

The 67 repeat source x values across orbit classes account exactly for 32,710 − 32,643. The selected representatives are the first 16,627 orbit keys in ascending order, with the least source-subspace x per key. Their CSV BLAKE3 is `2d5e7b6a5ce1574183d0b236c601f5c2ac5d47008c520b923338fd52e6dc6a44`; all classes CSV BLAKE3 is `6d230ffc20624b8e7ae466210175a6f0ac6dbe45906ac723a1242177586048f0`. Both [attempt 1](../../run-20261019T0141Z/) and [attempt 2](../../run-20261019T0142Z/) wrote byte-identical class and selection CSVs. The [audit](../../audit.log) checks counts, binary hash and equality, while the [21-file SHA-256 manifest](../../raw-files.sha256) binds all raw files. The code checked 17 selected representatives again through the general curve, subgroup and Frobenius APIs. The full enumeration and rerun use the same packed arithmetic, so an independently implemented full replay remains open.

The two process walls were measured under L0 contention and different wrappers; they are not a throughput comparison. No selected point object or membership index was constructed, so the derived point count does not measure its build time or memory. No public target, relation, rank, linear algebra or scalar extraction was executed. The canonical index-calculus leaderboard, progress timeline and curve graphs were checked and left unchanged because this count is not an admitted pipeline or curve result. [RESULT.md](../../RESULT.md) maps the requested work to the remaining gates.
