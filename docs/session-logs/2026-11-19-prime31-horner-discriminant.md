# Session log — 2026-11-19 — Prime31 Horner discriminant / rational chart

## What was done (commands, files, features)

- Read prior #4812 14-variable report, #4692 public-only extractor, #4822/#4816 parallel extractor work, #4809/#4826 rootfinder owners, and #4714/#4793/#4532 historical whole-cost controls. Checked root AGENTS.md. Source publication: Petit–Kosters–Messeng (PKC 2016; DOI 10.1007/978-3-662-49387-8_1).
- Derived exact universal identity `disc_r(E)=64*f(u)*f(z)*M(z)^2` for rationally eliminated F3 and identified full-factor-base triangular rational splitting and geometric reducedness under odd-prime-order curve assumptions. Contrasted with isolated-E torsion-root caveat.
- Locally ran independent Python modular tests: 10,000 deterministic uniform u,z pairs, 40,000 coefficient checks, 10,000 duplication identities, 961 original-curve base-point pairs, 1,922 signed sum roots, 0 identity exceptions on that restricted base-pair panel. Used the exact p=2147483647, a=847514846, b=2087444724 and first-rational-x base from existing Rust source.
- Committed frozen native protocol SHA `5aae81de213997ec1844585c1ec582fe494d8ac1`, extended the **existing** original-crate Rust example with one focused 8,192-pair discriminant/root-split test SHA `0a4a396763bf174d7d9ef203cd150e87ed6b907e`, and committed mathematical report SHA `5580bde824c15ab89cbe3426f72e365bd1449891`. These commits update existing PR #4812 branch, not a duplicate PR.
- Source-specific Rust workflow is external to the local container; `cargo` / `rustc` were unavailable locally. CI outcome still requires verification, and independent signed PCC is unverified.

## Decisions made (and why)

- Reuse PR #4812, original Prime31 arithmetic, existing test workflow, and existing #4692/#4822 RR9/F4 ownership. Do not claim that quadratic splitting itself is a non-generic extractor, or that planted controls solve ECDLP.
- Characterize the **full chart** rather than claiming that the isolated cleared-denominator E equation is free of extension-field solutions. All H leaf roots are ordinary Fp points and the bottom-up quadratic discriminant is a nonzero square for every reachable suffix.

## Open threads (unfinished, pick up next time)

- Read the source-specific CI run for SHA `0a4a396763bf174d7d9ef203cd150e87ed6b907e` or latest branch head, fix any compilation/test failure, and append exact artifact URL if successful.
- Hand this splitting theorem to #4692 public-only RR9/F4 owners; test 14 vs 21-variable Macaulay costs and denominator saturation on frozen blinded 46/18 B8 inputs.
- Once *public-only* unknown-index extraction works, integrate full 31-rank/1,024-Q Rust solver and compare fully charged same-host cost versus existing index calculus, shared BSGS and rho.

## Errors (what went wrong, how it was handled)

- Local runtime has no Rust toolchain and cannot reach GitHub with shell git/network DNS. Used authenticated GitHub connector for repository reads/writes, independent local Python for mathematics, and the preexisting repository GitHub Actions gate for native compilation. Did not claim native test success prematurely.

## 2026-11-19 19:29 UTC — source-specific verification closeout

### What was done (commands, files, features)

- Verified [Actions #44598103355](https://github.com/isogeny-labs/crypto/actions/runs/44598103355) **completed success** on exactly the new original-crate Rust test SHA `0a4a396763bf174d7d9ef203cd150e87ed6b907e`. Job `original-rust-x-y-gate` passed all steps and ran **40/40 original library tests, 2/2 x/y tests and 4/4 x-only tests**, including `eliminated_horner_discriminant_and_split_fibres`. Complete existing 1,024 planted-control and independent Python replay stages also passed. [Artifact #14567650828](https://github.com/isogeny-labs/crypto/actions/runs/44598103355/artifacts/14567650828), sha256 digest `b4248835df12f59cf62bc135d9b559541c4c1a30a77a2ca0043e32fcc5717c09`.
- Independently performed universal symbolic expansions using SymPy in indeterminates u,z,a,b; verified all leading-coefficient, F3 discriminant, and duplication numerator identities exactly (not probabilistic tests). Documented that full-chart denominator saturation is algebraically redundant.
- Updated report and existing #4812 PR summary; posted concise handoff to extractor owner [#4692, comment #7019894251](https://github.com/isogeny-labs/crypto/pull/4692#issuecomment-7019894251).

### Decisions made (and why)

- Treat the full-chart extension-root ambiguity as closed mathematically and native-source checked, *not* a performance gain. Keep signed PCC unverified.

### Open threads (unfinished, pick up next time)

- Actually solve the frozen blinded public-only 46-positive/18-negative B8 instances in existing F4/RR9 extractor; then measure full 31-rank/1,024-Q non-generic solver against optimized same-host IC, BSGS and rho.

### Errors (what went wrong, how it was handled)

- Initial workflow observation was in-progress; fetched the completed workflow job and raw artifact before labeling native tests passed. There were unrelated repository-wide workflow failures/cancellations on the same commit, so the claim is specifically the successful source-bound Horner gate.
