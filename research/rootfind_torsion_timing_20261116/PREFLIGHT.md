# K13 torsion timing: preflight and admission receipt

Recorded 2026-11-16 before any timed execution of
`SPEC-K13-LOCAL.json`. Status: `admission_blocked`. The frozen diagnostic
remains 4 arms × 4 public targets × 9 rounds, including one warmup and eight
measured rounds. This record is source/build/plan validation, not a timing
result or a performance claim. [pcc:unverified]

## Source and correctness

- Opt-in torsion engine ancestor: `f24788b1b0a2ec5ff0857bb96006b2a7c1e4c3f1`.
  Protocol/spec/analyzer freeze: `a7469e341cb0a1c14c7366e74d6f3ce10d25eb8a`.
  Pre-data registry-index repair: `6bd6f373bdc6db0e85ead0d701148c2d61cdbccc`.
- `docs/curves/registry.json` generates exactly the 22,278-line committed
  `src/cryptanalysis/ecbench/registry_index.txt` via
  `scripts/curve_id.py:registry_index_text`; byte equality was checked.
  Before repair the index had 22,300 lines and the first release plan marked
  K13 unregistered. Its rejected plan and binary digest are preserved in
  [amendment 1](AMENDMENT-1.md).
- Focused Rust 1.99.1 release library tests
  `registry_entry_matches_the_full_parse` and
  `registered_prime_curve_resolves_its_slug_and_ec1` both returned exit 0.
  The build emitted an unrelated `rust-objcopy` missing-`libLLVM.dylib`
  warning but returned exit 0.
- Rebuilt `target/release/ecbench` from the repair commit with
  `CRYPTO_BUILD_GIT_COMMIT=6bd6f373bdc6db0e85ead0d701148c2d61cdbccc`
  and `CRYPTO_BUILD_GIT_DIRTY=0`: binary SHA-256
  `1b4a2bde70d22541cb1628307a6b727e8a0feab59d5bd6d332b8edd7c2ed6fdc`.
  Spec SHA-256 is
  `8a40c31be62b0472e83f6316c76138bf6adb3e5f6b9c7f71af1227301c260457`;
  analyzer SHA-256 is
  `810ce94540c80c1669450c4039b776791602b642e0caea09ab01c0eb471059e2`.

## Corrected plan identity

`target/release/ecbench plan --spec SPEC-K13-LOCAL.json --doc` returned exit 0
and wrote [PLAN.json](PLAN.json), SHA-256
`3294c6589e3760f8888bcb2bf53f97f132d32d2367491c0e5dc991b8f128e13a`.
The ID remains `ECS1h5840d67387b7`; there are four arms, four workloads and
144 executions. Each workload has `registered=true`, subgroup order 2003,
cofactor four, EC1 `EC1N13Ce0h1dfa32cdf623`, and full curve UID
`urn:ec-record:1:sha256:1dfa32cdf623e34fb258128415cab472abaa51695380fa5d4f5d636ed5a3da60`.
Relative to [the rejected plan](PLAN-PRE-INDEX.json), all four target points,
generator, curve parameters, arms, execution order and algorithm seeds are
identical. The target points by index are `(0x1132,0x778)`,
`(0x1c48,0x16eb)`, `(0xe05,0x14d6)` and `(0x55e,0x18d7)`.

## Admission obstruction and exit

`scripts/checkpoint.sh gate` returned exit 0 and uploaded to
`s3://setec-astronomy-artifacts/checkpoints/crypto/codex__rootfind-pair-collision-census-20261116/20261116T093628Z-6bd6f373bdc6/`.
Its checker requires nonempty headings but does not check that the resume
content describes the active study. The local `.resume/RESUME.md` begins with
a hyperelliptic IC task and contains merge-conflict markers at lines 66, 198
and 202. It does not give a new session the K13 protocol, binary, current
state and exact next commands. Conductor T-6342 reserves `dir:.resume` for
another task, so this session cannot replace that file. The mechanically
successful checkpoint is not a complete resume record for this experiment.
No `ecbench run` command was launched. [pcc:unverified]

Exit condition: after T-6342 releases `.resume` through its normal workflow,
write a branch-specific complete resume record with no conflict markers;
verify it names this protocol, spec, source, binary and run/custody paths;
push a fresh S3 checkpoint and inspect its uploaded record; pass
`scripts/checkpoint.sh gate`; then run the unchanged protocol with its
600-second outer watchdog. Keep warmups, failed and censored rows, replay
receipt, analysis, graph and PDF. The [existing complete-cost problem](../../problems/rootfinding-complete-cost-calibration.md)
retains the broader native-pricing, L2 and independent-replay requirements.
