# Phase-domain initial powering

Optional `recover_sectioned_image` constructs initial mode constraints by
binary section powering in the phase quotient. It falls back to seeded
streaming when its phase-aware logical-slot preflight fails.

From this directory, generate source with `python3 generate.py`, then compile
`rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info driver.rs -o /tmp/initial-check`.
Run `/tmp/initial-check` for focused deterministic correctness controls, and
`/tmp/initial-check --inherited` for inherited regression. No timing panel.

DERIVATION.md states domain, identity and costs. Sources/generation are additive;
prior artifacts and the original cohort remain unchanged. Runtime accepts public
points and the existing character source. Scalar enumeration is fixture-only.
