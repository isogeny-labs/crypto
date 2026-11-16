# Validation and provenance

## Research runs

| Panel | Requested / completed / failed | Recorded probes | Native seals / witnesses |
| --- | --- | --- | --- |
| Primary | 5 / 5 / 0 | 125 | 5 / 5 |
| Secondary | 43 / 42 / 1 | 2,058 | 43 / 596 |
| Fresh confirmation | 6 / 6 / 0 | 1,158 | 6 / 135 |
| Representation | 6 / 6 / 0 | 582 | 6 / 270 |
| Total | 60 / 59 / 1 | 3,923 | 60 / 1,006 |

The failed secondary trace-zero/Frobenius seed29 dimension3 has zero
materialized points and fatal `empty or over-budget materialized inventory`.
Its hash-sealed failure remains in the raw tree. The final native analysis
preserves null phase data for this case. No timeout/cap occurred in the
research panels. Root `pipeline.exit` and `followup.exit` are zero, meaning
orchestration completed, not that every candidate qualified.

Original native CLI SHA256:
`dd9c1e582effe751777081fc4d7fc6ba1acd90e1696797ef210d20da58c3c3f0`.
Its source snapshot is `b3ed888964100f1bce4780a4c0500a399c7aee50`.
Protocol commits: `846cc9433` screen, `e549f4d86` fresh follow-up,
`9d5336b28` independent census. Final auditor source is
`392129454a1668b3c4eb15014fe6c15b2fdaf205`.
Source/config snapshots and command scripts are retained with the records.

The independent polynomial-basis Rust arithmetic audit checks every original
truth flag and witness, and enumerates exact supported nonidentity subgroup
targets. Lint-only edits reproduce the earlier successful output byte for byte.
It checks on-curve points, prime subgroup order and rG=O, without solving a
discrete log. This is a second implementation on the same host, not an
independent-host measurement certificate.

## Corrected production CLI

Source: `392129454a1668b3c4eb15014fe6c15b2fdaf205`.
New production/installed SHA256:
`5272caabfe93a42b76a307d3ce5effbeb1cc8fe7aa7e5b1209f4afd4c67141e4`.

- Seven release CLI unit tests pass, including the target-subgroup control
  rejection and acceptance regressions.
- All six actual CLI subprocess integration tests pass unchanged.
- Eight native audit tests pass; strict all-targets Clippy for its standalone
  crate passes.
- Strict release Clippy for `factor-base-sweep` passes.
- Root `cargo check --locked --all-targets` passes.
- The four-case new-CLI regression retains two expected unqualified failures
  and two completed supported cases. Native verifier checks four seals;
  independent arithmetic checks six recorded trials and two point witnesses.
- Native validation of the new draft backlog card passes with its two
  intentional TODO warnings. It is not promoted, funded, published or leased.

The installed executable is updated on PATH; the original executable is
preserved locally for old byte-bound resumes. The new control search is
included in setup point operations and shares the existing supervisor budget.
No workflow, assertion, resource budget, coverage or ignore annotation is
weakened by this follow-up.

## Full root-library command

`cargo test --locked --release --lib -- --test-threads=1`, Rust1.98,
networking/loopback enabled: **4,380 passed, four failed, 117 existing ignores,
zero filtered**, exit101, one complete command. The four failures are the
same documented base failures: three curve-traits query tests deserialize
`missing field levels`; the Jacobian trace-of-a case gets Boundary instead
of FormulaMarginal. Full raw output is retained as `runs/root-library.log`.
No full-pass claim is made. Library source and compile-time fixtures are
unchanged by the study/CLI guard; the unit binary SHA256 remains
`f6ec0b235135f0d41eee4f9163dcba33ce70a8042dfa4036f0107d3c427a9593`.
The new guard is separately covered by the CLI tests above.

## Failed analysis attempts and environment limits

The initial auditor refused an incomplete candidate instead of silently
counting it as zero yield. The additive repair preserves the failed row;
both the initial failure and accepted output remain. Strict analysis Clippy
initially found three style diagnostics (modulo/range/type complexity); fixes
do not change the scientific output, and replay is byte-identical.
Sparse staging of the new standalone Cargo.lock first refused the path;
the explicit --sparse retry succeeded. No frozen lockfile was changed.
Font-cache warnings during rendering are retained; all three PDFs build and
all seven final pages are visually inspected before publication.

Conductor check is unavailable (exit127); the pinned workflow was404 in
the maintained PR, not a pass. No paid environment or S3 operation was used.
The observed host is shared Linux x86-64, Intel Xeon Platinum8573C,
five visible logical CPUs; plans use one worker. This earns no timing
isolation. Live rates/ETAs remain operational telemetry. Missing complete
native counter emission prevents a total ranking at every symmetric pricing
weight. Magma/msolve/FGb/Singular are not on PATH; no external comparison or
fastest claim is made.

Existing unrelated PR blockers (workflow duplicate keys, historical replay
lockfiles, naming/registry disagreement, backlog placeholder and provider
review availability) remain governed by current-head CI, not bypassed.
Historical #1898/#1975 evidence, every registry/alias/store entry and the
draft/unrun/zero-funded/unleased #1963 card are untouched.

## Publication and future CI

All 8,546 archived member hashes and byte-identical deterministic repacking
pass. The new CI replay step is executed locally without changing its shell
body: all five native run directories verify and both original observations
and independent coverage outputs match byte for byte, including the new
control regression's separate receipt. All nine CI-scope tests pass with
selection coverage for the standalone auditor and research records. The
updated workflow YAML and shell snippets parse. See
[publication checks](PUBLICATION_CHECKS.md) for archive identity and receipts.
These are local checks, not a claim that fresh GitHub CI has passed.
The full change-range whitespace check reports one extra trailing blank line
in each of the three already-frozen protocol files. Their executed bytes and
original commits are preserved; these advisory findings are not reported as
a clean check, and no whitespace rule is disabled. The final unstaged/staged
report edits pass their own whitespace check.
