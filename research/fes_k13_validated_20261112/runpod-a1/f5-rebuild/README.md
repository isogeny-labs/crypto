# F5 rebuild: ecbench from e9d5dbdccd

Finding F5 of the validator report said the session's binary (sha256 8f9576b5…) was a
producer-local build. Its link to commit e9d5dbdccd therefore rested on the producer's word.
**Verdict: closed** `[pcc:unverified]`. See [`F5-ASSESSMENT.md`](F5-ASSESSMENT.md) and
[`F5-ASSESSMENT.json`](F5-ASSESSMENT.json), written by an independent validator.

- **Validator's own rebuild.** The validator built ecbench from its own `git archive` of
  e9d5dbdccd, with rustc 1.97.0 checked file for file against the official tarballs and
  `cargo build --release --locked`. `verify --replay-all` on the committed session gave
  288 of 288 identical. Disassembly shows the measured binary is the same program; the only
  differences are link layout and the install path embedded in panic strings.
- **Producer's pod rebuild, this directory.** A Runpod pod built from a minimal `git archive`
  tree (`source-recipe.txt`, `source-extra-paths.txt`, `source-tree.sha256`). Every file of
  that tree equals its blob at e9d5dbdccd. The rebuilt binary (d012489f…) replayed 288 of 288
  (`replay-rebuilt.stdout`). It also re-ran the frozen spec (`rerun-session/`), and all 336
  records equal the session's in every counted, cost, answer, workload, method and seed field.
  - **Differing fields:** only `record_id`, `session_id`, `binary_sha256`, `time.*`,
    `isolation.*`, `online.*`, `phases[].wall_ns`, `solver.wall_ns` and
    `detail.decomposition.solver.wall_ns`. This corrects the producer's first list, which named
    `allocator` (it does not differ) and missed the last field (finding F5-N2).
- **Failed first pod attempt (finding F5-N1, disclosed here).** Pod 6s16l5yuxcc2wf used the
  same inputs. Its build failed after 9 s: `openssl-sys` could not find `pkg-config`. Its pod
  logs and results tarball were delete-marked in S3 before the second attempt. They can still
  be recovered by the version ids listed in `F5-ASSESSMENT.json`. The second attempt, pod
  2vz9wqml2kwjkk, added the `apt-get install pkg-config libssl-dev build-essential cmake` line
  to `pod-f5.sh`; nothing else changed. Runpod cost of both pods: about 0.20 USD.

Residuals: the bundled C parts (sqlite, blake3) were compiled by each host's C compiler,
which was not checked; their machine code is identical across the three binaries, and they
are outside the counting path. `session.json` `git_commit` is still null. This finding does
not change F2 (H1 unresolved inside the band) or F3/F4 (no usable wall figure).
