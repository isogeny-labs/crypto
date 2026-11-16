# Note F1.1 to P2 follow-up 1 (2026-11-02, filed before any data of follow-up 1)

Status: filed by the experiment director before any `ecbench run` of follow-up 1.
`runs/followup1_c32A_m3/` does not exist when this is filed (the card creates its
contents; an empty directory is prepared in the run worktree, section 6). It is a
**note**, not an amendment: it changes no value of `FOLLOWUP_c32A_m3.md` (sha256
`4edee7f2379d3c71429415b865374cfa154c01f4bc22ac5d0aedd40ea7c5ff5f`) or of
`protocol_followup1.json` (file sha256
`4de51667746ef027ab5c350cc6a76ae6d82869736d396afdb58f5880d05fd566`; canonical freeze
hash `f382ee669cf010abf871f18ccc8d8378b01fdfa24f099f78da5bf5912fd1313a`), neither of
which is edited. Precedent: note 10.1 to A1 in `PROTOCOL.md`. The protocol itself
(`links.analysis_definition`, `analysis_plan.script` null, FOLLOWUP section 5) says that the
builder commit and the hashes are filed in note F1.1 before the first data point, and the
statistics are defined by the text of FOLLOWUP sections 6 to 8. Binding a script hash into
`analysis_plan.script` would change the protocol hash and need a version-2 amendment; the
protocol chose the note, and this note follows it. The checker's `validate` accepts a null
`analysis_plan.script` (it tests the hash only when a script is set).

## 1. What is bound

| item | value |
|:--|:--|
| builder commit (`p2tool spec --focus`, `p2tool analyze-focus`) | `21aa686118c1e40559a83698f12d526205a94f24`, branch `claude/isogeny-class-ic-yield-20261102` |
| `p2tool/src/analyze.rs` (frozen, unchanged) | `f6cc7286daae471bc40e5860ed8673f1e1676b9fadce3199b8fe67ca8d167678` |
| `p2tool/src/analyze_focus.rs` (new) | `2112d2034503e18bf903283859b4f652835cb79ecc026d0b43bb769ab5343ba3` |
| `p2tool/src/spec.rs` | `850a3b3d7a0bdc976ecc11d3d8b0fefdbcfb929a414c3dfd25b4f4dfd28a486d` |
| `p2tool/src/main.rs` | `ce9a01553ee730b2dc6a8afc253c4e9e36e1e24c971b5291d64ae8b6998bfeb5` |
| `p2tool/src/ec.rs` | `6762cfc839973a16b10e9173c6817813e2dddb492a1739b9f0ecc19d68643ce1` |
| `p2tool/src/pari.rs` | `6adde8853496a94865df6301b7f7ad2539afa5c7d2aacb87ff602fea94d5caf2` |
| `p2tool/Cargo.toml` | `a67a7a8d9b2174d36053be66e0f46d2c044147bcfe74dc33c902b604fac2b933` |
| `p2tool/Cargo.lock` (gitignored, so not in the commit; used for the build) | `25ffb6c9d90640ac60f49b79cce9e9b5f16b37a3faeb992bc8a35fe3bf871efb` |
| **p2tool binary** (release, `--locked --offline`) | `db00363617c26bb9ff6b48fc0fda577d49dac391e256eca4348fd936c3a99120` |
| ecbench binary (the one of the P2 cards, not rebuilt) | `3447ba8a3e410bf816f0fbb13fc7aed235c63a834d31ab32b00f480a930fac1c` |
| `classes.json` | `7e79c642399cdda52e0e6666123cf8abcad27dbf68c9ff7dda14f0d3316adb47` |
| `followup1_focus.json` | `0340494ad36574f7d27283617096cdfe4a9bf8160cad3810c0405484f73a92d7` |

The p2tool binary was built twice: by the builder (target directory in the session
scratchpad) and by the director from a fresh detached checkout of `21aa686118` in a
different directory with the same `Cargo.lock`; both have the hash above (the build is
path independent). The commit that holds this note changes no file under `p2tool/src/`; the
card binds the six source hashes above, so a differing file fails its lint. `cargo test
--release` at the builder commit: 15 tests, 15 passed (13 in `analyze_focus`, 2 in `spec`;
all section 8 branches and the threshold edges). The binary is placed at
`p2tool/target/release/p2tool-f1` so that the P2 binary `p2tool` (hash
`d553ca57...`, hard-linked into the running P2 worktree) is neither read nor overwritten.

Checks on the spec generator (director, before data, outputs not retained in the repository,
not data): `p2tool spec --focus` with the FOLLOWUP section 5 flags printed
`|F|=470 (pc 499) predicted_yield=3.354e-2 attempts=5964 curves=24`; the spec has
`target_seed` 363589115, `measurement.seed` 116, `targets_per_curve` 12, 24 curves in the
order of FOLLOWUP section 3, and its `arms` equal those of `runs/c32A_m3/spec.json` field by
field (the `measurement` object differs only in `seed`, 116 against 103).

Parser check of `analyze-focus` (the one thing no unit test covers: reading a real
`records.jsonl`): `analyze-focus` was run once on a synthetic session made from the committed
P2 c32A m=3 records, relabelled to the 24 curves of this design with 4 targets x 4 rounds
per curve. It parsed, completed and exited 0. The synthetic output has no meaning, was kept
outside the repository, and is not cited.

## 2. The builder's resolutions of the open points of FOLLOWUP section 7: accepted or rejected

Each is accepted or rejected here, before data. Rejected: none.

| # | the builder's resolution | decision | reason |
|:--|:--|:--|:--|
| 1 | Cells of one curve are ordered by (round, target index) before the T1 permutation | **accepted** | the protocol fixes the 48 cell labels, not their order; a fixed order makes the permutation p reproducible. The set of cells does not depend on it |
| 2 | The permutation pool is the 48 cells of curve 80 first, then the 15 comparators in the order of the spec (ascending curve index); Fisher-Yates over the whole pool, splitmix64 (the generator of `analyze.rs`) seeded with 20261102, 20000 permutations, statistic `abs(l)`, add-one p | **accepted** | seed and count are those of FOLLOWUP section 6; the pool order only fixes the stream |
| 3 | A permuted statistic counts as at least the observed one when it is at least `observed - 1e-12` | **accepted** | only guards float equality; no value near the 1.05 or 0.05 thresholds depends on it |
| 4 | Normal tail by the program's own series (`erfc`), no library | **accepted** | unit test `normal_tail_values`; p is compared only to 0.05 |
| 5 | Warm-up rows (`warmup: true`) are dropped; curve identified by `(a, b)` of `workload.curve_spec`; arm by name (`m3-main`, `m3-pc`); positive control uses the `m3-pc` rows of curve 80 only | **accepted** | as `analyze.rs`; FOLLOWUP section 6 |
| 6 | Verdict strings: `not_informative_or_inconclusive_overdispersed` (step 1), `not_replicated_chance_winners_curse` (2), `curve_effect_escalate_to_mechanism_test` (3), `factor_base_presentation_effect` (4), `inconclusive` (5), `not_run_incomplete` | **accepted** | one string per branch of section 8, in the order of section 8 |
| 7 | `not_run_incomplete`: the verdict is overridden when any of the 24 workloads has other than 48 main cells (12 targets x 4 rounds), or curve 80 has other than 48 `m3-pc` cells, or a group has no data or zero relations; the statistics are still written | **accepted** | FOLLOWUP section 12: a session that does not complete is reported as not run, never filled |
| 8 | Step 4 requires `y_P > y_I` as the sign of `ln(y_P / y_I)` greater than 0 | **accepted** | section 8 step 4 |
| 9 | `analyze-focus` exits 1 when `spec.workloads.curves` differs from the focus design in order or content | **accepted** | a spec built from another design is not this experiment |

Binding rule for any later reading: the verdict is the `verdict` key of `focus_analysis.json`
written by the bound binary. Nothing computed by hand replaces it.

## 3. How the card treats a command that has never run on a real session

`analyze-focus` has not run on a session of this experiment; `spec --focus` has produced a
spec only. The card runs four commands in order and stops at the first that fails
(a nonzero exit, a timeout or a missing declared output file).

- A failure of command 1 or 2 means no session: report, keep the output directory, no rerun
  to change a result.
- A failure of command 3 (`ecbench verify`) invalidates the session (stopping rule of
  FOLLOWUP section 12). Command 4 is then not run.
- A failure of command 4 (nonzero exit, a panic, no `focus_analysis.json`) leaves the
  session, `audit.json` and `spec.json` retained and untouched. The session is **not**
  measured again. The director compiles a second card with only the `analyze-focus` command
  and `must_not_exist` on `focus_analysis.json`. If the failure is an infrastructure fault
  (the file system, a missing file), the same binary is rerun on the retained session. If it
  is a defect of the code, a builder fixes the code to the text of FOLLOWUP sections 6 to 8, the
  unit tests are rerun, and a note F1.2 files the new hashes and says that the fix was made
  after the data had been recorded and why. The text of sections 7 and 8 decides, never the
  code. Both the failed and the rerun outputs are reported.
- The session directory must not be compressed before command 4 has finished: `analyze-focus`
  reads `<session>/records.jsonl`.

## 4. Fixed values of the card (the placeholders of FOLLOWUP section 5)

| value | fixed as |
|:--|:--|
| `<run>` | `research/isogeny_class_ic_yield_20261102/p2_prime_analogues/runs/followup1_c32A_m3` |
| worktree | `/home/user/crypto-p2fu`, a fresh detached worktree at the commit that holds this note (sparse: `tools/task_card.py`, this study directory, `target/release/ecbench`, the p2tool binary) |
| command 1 | `p2tool-f1 spec ...` with the flags of FOLLOWUP section 5, timeout 120 s |
| command 2 | `ecbench run --wait --allow-busy ...`, env `ALLOW_BUSY=1`, timeout 7200 s |
| command 3 | `ecbench verify --dir <run>/session --replay 12 --out <run>/audit.json`, timeout 3600 s |
| command 4 | `p2tool-f1 analyze-focus ...` with the flags of FOLLOWUP section 5, timeout 900 s |
| route | A1 local backend, isolation L0 (`--isolation L0` in the spec), `ALLOW_BUSY=1`; counted metrics decide |
| attempts, targets, rounds, aa, seed | 5964 (from the spec), 12, 4, 0, 363589115 |

## 5. Ordering with the running P2 queue

`ecbench run` holds the per-host benchmark lock. The P2 queue in `/home/user/crypto-p2run`
still holds it. **The card is launched only after that queue prints "queue done"**, by the
main session; it is not launched before, although `--wait` would queue behind the lock,
because the wait would be spent against the 7200 s timeout of command 2.

## 6. Preparation of the run worktree (state, not a command of the card)

`runs/followup1_c32A_m3/` is created empty in the worktree because `p2tool spec` writes
`spec.json` with `std::fs::write` and does not create its parent directory. The card's
`must_not_exist` lists `spec.json`, `session`, `audit.json` and `focus_analysis.json` under
it, so a directory that already holds any of them refuses the run.

## 7. What this note does not do

It reads no result, ranks no curve and adds no threshold. Seeds, comparators, T, the 1.05
floor, alpha, D, the decision rule and the route are as in FOLLOWUP sections 3 to 12.
