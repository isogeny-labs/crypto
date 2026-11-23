# F5 assessment (independent validator)

**Verdict: closed.** Disposition `validated`. Machine-readable record with every number:
[`F5-ASSESSMENT.json`](F5-ASSESSMENT.json). This file does not change the status in
`validation/findings.json` or `VALIDATION.json`; the owner of those files records it.

F5 said the binding between the measured ecbench binary (sha256 `8f9576b5…e5130f4`) and
commit `e9d5dbdccd` rested on the producer's word. Its stated close condition was to build
ecbench from `e9d5dbdccd` on another host, replay the session, and get 288/288 identical.

## What closes it

The closure rests on the validator's own rebuild, not on the producer's pod alone.

| check | result |
| --- | --- |
| Validator `git archive` of `e9d5dbdccd` (on `origin/main`); fresh `CARGO_HOME` from crates.io under `--locked`; `cargo build --release --locked --offline --bin ecbench` | exit 0, 600 s, binary `fbb83c40…`, same 231 compilation units |
| Toolchain used for that build | rustc 41/41, rust-std 62/62 and cargo 44/44 files identical to the official 1.97.0 tarballs (hashes match the channel manifest); rustup proxies equal official rustup-init 1.28.2 |
| `verify --replay-all` of the committed session with the validator binary | 288 listed, 288 `reproduced`, 288 `identical`; the replayed set equals the 288 non-warm-up records; file hashes match the committed session; receipt sha256 `46c42213…` (embedded in the JSON) |
| Is `8f9576b5` the program `e9d5dbdccd` compiles to? | Yes. Its code matches both rebuilds instruction for instruction (4,674,413 instructions). Every operand difference is a layout address, apart from 27/29 references, each traced to `"}"`, `""`, `"Err"` or a relative table whose entries resolve to equal targets. Its data also matches: 3714 named `.rodata` symbols are identical and 60 more become identical once `CARGO_HOME` is normalised; all 18,529 relocations correspond. The only remaining difference is 441 path-length fields. |

## The producer's pod evidence (checked; it would only narrow F5 on its own)

- The source tree is exactly a subset of `e9d5dbdccd`. All 9372 files equal their git blobs, and all 23 symlink targets and executable bits match. Nothing in it is absent from the commit. `src.tgz` hashes to the staged value and matches the manifest.
- The build is clean. It used rustc 1.97.0 (2d8144b78). All 226 downloads and 224 compiled registry crates are pinned crates.io packages with checksums. The 7 path crates are under `/work/src`. There are no `[patch]`, `[replace]` or git sources. `--locked` appears in `pod-f5.sh` and is consistent with `build.log`.
- The S3 objects and `results.tgz` hashes match, and the local files are byte-identical to `results.tgz`. `session.tgz` equals the committed session.
- `replay-rebuilt.stdout`, recomputed from its JSON: 288/288 `identical`.
- The re-run against the original, compared field by field by validator code: 336/336 records have no counted, cost, answer, workload, method or seed difference. Only identity, binary, time, isolation, online and `*wall_ns` fields differ. Mean S per arm is identical: `ic-descent-fes-f2` 86.066899, `ic-descent-inherited-f4` 121.544209, `rho-sfstrong-v2` 2.584483.

## New findings (minor; they do not block closure)

- **F5-N1 (disclosure).** An earlier pod attempt is missing from `f5-rebuild/`. Its build failed at 03:23Z because `pkg-config` was missing for `openssl-sys`; the inputs were the same. Its logs and `results.tgz` were delete-marked in the versioned bucket, and the script was re-uploaded with an `apt-get` line. The failed attempt should be disclosed. Its objects can still be recovered by version id.
- **F5-N2 (accuracy).** The producer's list of differing fields names `allocator`, which does not differ. It omits `detail.decomposition.solver.wall_ns`, a wall-clock field that does differ. No number is affected.

## Residuals

- The fresh re-run exists only from the producer's pod. It is corroboration, not the basis of the verdict.
- The C parts (bundled sqlite, blake3) were compiled by each host's system `cc`, which was not checked. Their code is identical across all three binaries, and they are not in the counting path.
- `git_commit` and `git_dirty` remain null in `session.json`. The binding is now this rebuild equivalence, and the gate documents must cite it.
- Closing F5 does not change F2 (H1 unresolved inside the band) or F3/F4 (no admissible wall figure).
