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
