# 2026-11-19 — Prime31 weighted slot-root resultant exactness (continuation of #4812)

## What was done (commands, files, features)

- Read AGENTS.md, existing #4812 PR and original Rust source, primary #4692/#4816/#4822 owners, #4793/#4728/#4532 cost controls, and related #4807/#4808 weighted grading reports. Consulted prime-field index-calculus literature: Petit–Kosters–Messeng PKC 2016, Amadori–Pintore–Sala FFA 2018, Kudo–Yokota–Takahashi–Yasuda CANS 2018. No live shared research-KB tool exposed in this runtime, so no exhaustive literature/novelty claim.
- Read current shared 2026-11-19 session log and existing ACTIVE #4812 lines; added current task line in docs/ACTIVE.md first (commit 1f652619). Continued on open #4812 branch instead of duplicating the F4 extractor PR.
- Froze slot-resultant protocol in commit a3ebe34, before producer code. Added an explicitly registered Jacobian amendment before its implementation (3eb6cc2).
- Derived eight slot-specific, public transformed-root supports T_i, each with 8 distinct x values; for these four frozen original-curve B8 windows all 64 x values/window are distinct. Proved Res(H_i,M)=0 for all 8 slots is equivalent to M being the monic product of exactly one root per slot, with no saturation. Proved the eight-resultant membership ideal has exactly 8^8=16,777,216 geometrically split simple solutions and nonzero Vandermonde-scaled Jacobian; no target-conditioned Q predicate is present.
- Independently executed Python bigint original Prime31 affine arithmetic on the frozen 512 positive / 512 negative controls. Obtained 4096 positive zero resultants, 512 negative cases failing exactly slot 0, and 512/512 nonzero Jacobian determinants. First-case determinants: 293475591, 155765304, 235689639, 1686373975. This is local Python preflight, not new complete ECDLP or Rust CI.
- Added original-crate Rust example `prime31_weighted_slot_resultants.rs` (reuses Arithmetic, PRIME31), independent Python verifier `verify_prime31_weighted_slot_resultants.py`, strict source-bound existing GitHub Actions workflow stages and scientific report `WEIGHTED_SLOT_RESULTANTS_RESULT_20261119.md`.
- GitHub Actions run for source SHA 688a3fca: https://github.com/isogeny-labs/crypto/actions/runs/44619747239 launched; status pending at initial observation. No native PASS claim until job and artifacts are inspected.

## Decisions made (and why)

- Keep all new work on the existing #4812 weighted Horner/algebraic-candidate lane. #4692 owns public-target-only F4/remainder extraction, #4816 owns fixed-M8 unweighted quotient, #4822 owns parallel Wiedemann. Do not duplicate engines.
- Use eight resultants as an exact **unsigned slot membership** equation system; do not confuse it with the complete sign/group-sum relation conditioned on Q. Its 16.8M geometric solution dimension is a strong reason to couple Q *before* expensive elimination, not to precompute the entire quotient.
- Keep the fastest historical shared-BSGS 170.113ms/1024Q as a separate-run frontier sensitivity; full same-runner IC comparison still required. No automated paid cloud compute, no merge.

## Open threads (unfinished, pick up next time)

- Inspect exact-source GitHub CI and artifact for the new Rust/independent all-cohort gate; fix test or compilation issues and update report with exact run SHA and final state. Verify original 40 library tests + focused example.
- Post result to public-only #4692 owner. The decisive next experiment is joint eight-slot-resultant plus RR9 public-Q constraints on the frozen 46-positive/18-negative B8 corpus, versus 14/21-variable Horner charts, with no generic sum tables. Then 1,024 original Prime31 public logs and fully charged same-machine frontier comparisons if public-only extraction works.
- Signed PCC remains unverified.

## Errors (what went wrong, how it was handled)

- Local container lacks network access to GitHub and cargo/rustc. Used authenticated GitHub connector for reads/writes, independent local Python for algebra, and source-specific GitHub Actions for native compilation. No native validation result inferred from preflight.
- Initial Rust source used a nested mutable RHS/sqrt expression; separated the method calls before CI.

## 2026-11-19 21:35 UTC — exact-source verification closeout

### What was done (commands, files, features)

- Fetched GitHub Actions run [#44619747239](https://github.com/isogeny-labs/crypto/actions/runs/44619747239), completed job 135897900043, job logs and artifact [#14576493274](https://github.com/isogeny-labs/crypto/actions/runs/44619747239/artifacts/14576493274). All new source-specific steps passed.
- Verified original Rust library 40/40; existing x/y 2/2; existing x-only 4/4; existing fixed/weighted family 1/1; new slot-root resultants 1/1. Original Rust release JSON passed registered gates: 64 distinct slot roots in each of 4 windows, 512 positives, 512 negatives, 4096 zero positive resultants, 512 nonzero negative R0, 512 nonsingular Jacobians. Separately authored Python bigint replay all 512 matched and checked generator order.
- Recorded exact Rust JSON digest `690f24656bf2a7587ba10d8dcb23ad017a43c5737216db233cad505e94242fa4` and source digests in the report. Signed PCC remains unverified.

### Decisions made (and why)

- Mark this **exact membership characterization/test** complete; retain #4692 public-only target-conditioned extractor as unresolved. Full membership variety has 16.8M roots, so the only plausible next use couples Q at the start rather than a 16.8M-point pre-enumeration.

### Open threads (unfinished, pick up next time)

- #4692 owner to compare joint slot-resultant + RR9 target constraints against 14-var and 21-var Horner systems on 46/18 frozen B8 public-only corpus. If successful, full rank31/1024Q same-runner cost.
- Independent signed PCC, exceptional other-curve slot collisions, scalable asymptotic analysis.

### Errors (what went wrong, how it was handled)

- None in the final CI run. Earlier corrected a Rust nested mutable borrow before exact-source test.
