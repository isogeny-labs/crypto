# Prime31 signed cancellation support — 2026-11-19

## What was done (commands, files, features)
- Opened draft [PR #4803](https://github.com/isogeny-labs/crypto/pull/4803) after registering the task in `docs/ACTIVE.md`. Reviewed existing leads #4716, #4717, #4548, #4547, #4801, #4728, #4692, #4532, #4793; PKM 2016 and APS 2018 publisher sources. Shared KB unavailable via this connector runtime.
- Derived exact formula for admissible eight-leaf signed net coefficient vectors `||k||_1 <= 8`, parity even: shell counts 1, 128, 2816, 27008, 157184 = **187137**; fixed-eight distinct-x signed support <=256.
- Independently authored a Python BigInt elliptic-curve grading census over the original `p=2147483647`, `a=847514846`, `b=2087444724`, `n=2147446831` fixture; got **187137 distinct sums** from 187137 admissible vectors and **256 distinct squarefree sums**.
- Registered protocol before implementation. Created the original-Rust example `native/examples/prime31_rr9_signed_coverage.rs` and pinned workflow `.github/workflows/prime31-rr9-signed-coverage.yml`, with grading explicitly marked generic.
- Source-specific [Actions run #44506856237](https://github.com/isogeny-labs/crypto/actions/runs/44506856237) PASS: native 40/40 original + 2/2 focused tests; release executable exact JSON asserts, 223480 grading EC additions, 43.509259ms grading-only candidate runtime, artifact #14519549353. Source/workflow commit `806c13399d93043b354e6eefb90cf162c83d4dbf`.
- Added verified result report and permanent JSON summary to PR. [pcc:unverified] independently signed review outstanding.

## Decisions made (and why)
- The earlier `M=H_B` squarefree Macaulay predicate has only 256 reachable public residuals, and a more generous unweighted signed/repeated B8 predicate has at most 187137. Therefore do not substitute those polynomial systems as the oracle for the *position-weighted* W8 rank31/1024-target solver. Exact supports are finite uniform-R yield fractions, not empirical times or absolute impossibility for weighted maps.
- The genuine weighted unknown-index F4/remainder-tower implementation remains owned by PR #4692. No duplicate extractor or generic table delivered as a falsely novel solver.
- The 43.5 ms measures **generic group-sum grading** only, not any unknown-target algebraic or full ECDLP speed.

## Open threads (unfinished, pick up next time)
- PR #4692: exact weighted slot-specific polynomial ideal, same frozen 46-positive/18-negative B8 public corpus, field operations/failure accounting, independent root/group replay.
- Only after P0/P1, replace generic W8 grader in existing Rust 31-rank/1024-log pipeline; compare same-source whole solver vs published PKM/APS algebraic frontier and matched optimized BSGS/rho.
- Source-specific CI passed; global repository checks for other areas failed on inherited artifact/curve registry drift. Independently signed PCC review is outstanding.

## Errors (what went wrong, how it was handled)
- No Rust toolchain in the connector-local container. Ran exact-source Rust 1.100.0 in GitHub Actions and recorded both GitHub source pin and raw artifact hashes.
- Global workflow parse and curve-names checks were observed failing on pre-existing missing artifact-store records / curve-registry discrepancies, separate from the passing source-specific certificate.
