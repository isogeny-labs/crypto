# Prime31 shared-coefficient and selector-SAT experiments

This directory resumes PR #4692 with two deliberately separated predicates:

- an exact selector-SAT circuit for eight ordered, weighted public point
  tables; and
- the recovered RR9 common-support remainder tower
  `M_(A,V,R) | H_B^8`.

They are not interchangeable.  Read `PROTOCOL_SELECTOR_SAT_20261120.md` and
`PROTOCOL_RR9_F4_20261120.md` before running either track.  Generated CNF,
solver logs, and large raw artifacts are not committed; deterministic
metadata, hashes, summaries, compact RR9 assignments, and replay instructions
are.

No speedup is claimed.  All local evidence is `[pcc:unverified]` until a
separate evidence gate says otherwise.

The selector launchers are:

- `run_registered_case.sh`, for one frozen B4/B6/B8 candidate/solver row; and
- `run_registered_grader.sh`, for the isolated generic four-plus-four truth
  panel.

The separate common-support launchers are:

- `run_registered_rr9_case.sh`, for a frozen B4/B8 RR9 case and the
  source-linked repository F4 engine; and
- `run_registered_rr9_grader.sh`, for the isolated signed-eight-multiset truth
  panel.

All four call `scripts/checkpoint.sh gate` before generating or reading any
registered solver artifact, enforce 16 GiB and finite wall caps, refuse to
overwrite evidence, and bind their local release binaries to
`toolchain/build-receipt.json`.  A typical post-gate smoke pair is generated
with `native/target/release/prime31-selector-sat generate`, then passed to the
selector case runner.  RR9 cases use the same binary's `rr9-generate` command
and are passed only to the RR9/F4 runner.  Grader output is never passed to a
candidate process.
