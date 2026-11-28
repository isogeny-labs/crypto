# Recorded-data audit, 2026-11-15

Standing: **[pcc:unverified]**. This is a local native audit of saved data,
not a solver execution replay, signed PCC verdict, or performance measurement.

## Checked scope

Source snapshot: `d5af3fb28a0548b63e5d7c6091390387408a85a7`,
study `research/prime31_ic_20261115/`. The audit uses a separate C++ curve
implementation and checks the source and raw-data SHA-256 pins in
`RESULTS.json`. It checks both primes by trial division, the nonsingular
curve, nonzero generator, unreduced `[n]G=O`, and Hasse uniqueness. It checks
all nine recorded public scalar answers, covering three distinct public
targets; the panel must contain each expected size/seed pair exactly once.
It also checks recorded rank and relation-count consistency and 102 summary
fields (34 per size). The rank check checks the recorded assertion against
the expected base size; it does not reconstruct the relation matrix.
All these observations are producer-local [pcc:unverified].

Public-answer claim `PCC-20261115-63ae2a` pins three
`pcc.solve-certificate/v1` files in `certificates/`. Its standing is
unverified with no signed verdict. Its scope is only the three public
equations, excluding execution provenance and performance.

The saved fixture is `p=2147483647`,
`E: y²=x³+847514846x+2087444724`, prime order `2147446831`,
`G=(625037715,2100627251)`. The audit fixes this domain before doing
word-sized field arithmetic. It is not a verifier for arbitrary-size curves.

## Aggregation detail

The recorded ratio-of-mean values for B=384 and B=640 divide the IC mean
after rounding it to three decimal places. The exact ratio of integer sums
differs by at most `5.508e-9`. The audit admits only the error implied by
that rounding; the exact rational ratios are retained in
`audit_recorded.results.json`. Original measurements are preserved.
This arithmetic detail does not change any reported three-decimal result.
[pcc:unverified]

## Reproduce

Dependencies: C++17, OpenSSL development headers/libcrypto, nlohmann-json
3.11.3. If the JSON header is not installed, put its single-header release
outside the repository and use `-I` for that directory:

```sh
curl -fsSL https://raw.githubusercontent.com/nlohmann/json/v3.11.3/single_include/nlohmann/json.hpp -o /tmp/json.hpp
```

The header SHA-256 is
`9bea4c8066ef4a1c206b2be5a36302f8926f7fdc6087af5d20b417d0cf103ea6`.
Its include path must be `nlohmann/json.hpp`, for example
`/tmp/audit-include/nlohmann/json.hpp` with `-I/tmp/audit-include`.

```sh
g++ -std=c++17 -O2 -Wall -Wextra -Werror research/prime31_ic_20261115/audit_recorded.cpp -lcrypto -o /tmp/audit_recorded
/tmp/audit_recorded research/prime31_ic_20261115
```

Run from the repository root, adjusting the source path to this directory.
The local native run passed. Three deliberately corrupted fixtures were
rejected: an altered public target (with updated raw digest), an altered
recorded rank (with updated raw digest), and an altered aggregate mean.
These are development correctness checks, not benchmark trials.
[pcc:unverified]

The original C++ solver also passed `g++ -std=c++17 -Wall -Wextra
-fsyntax-only`, with four pre-existing misleading-indentation warnings.
No solver measurement was rerun. No scalar recovery provenance, relation
matrix, field-operation count, memory peak, or wall time is certified by this
audit. A correct saved answer alone cannot establish how it was obtained.

## CI status correction

At the pinned head, the GitHub connector's commit-workflow query returned
an empty list because it selects `pull_request` events. A direct paginated
Actions query found two successful `pull_request_target` runs:

- [Free LLM pull-request review, 43681781801](https://github.com/isogeny-labs/crypto/actions/runs/43681781801).
- [artifact-autofix, 43681781785](https://github.com/isogeny-labs/crypto/actions/runs/43681781785).

Their job steps do not execute the solver or PCC verifier. Thus “no workflow
runs” was too broad; “no solver replay/PCC run was reported” describes the
observed gap. Dispatched the existing `pcc-gate.yml` on the pinned branch:
[43683296044](https://github.com/isogeny-labs/crypto/actions/runs/43683296044).
Its gate was running at the initial status check; its signing job was skipped as designed
because it signs only on a push to main. No signed standing is claimed.

## Sources and next step

Sources checked: study README, original C++ source, raw JSONL and aggregate
JSON; repository AGENTS.md, latest session log, `docs/CONDUCTOR.md`, PCC gate
workflow and PCC certificate contract. The existing study cites PKM PKC 2016,
Semaev and Shoup; this audit introduces no algorithm or novelty claim.
KB unavailable: no KB tools were loaded; wrapper availability is recorded in
the session log. Repository source/data served as the fallback.

Environment checks: `conductor check` and `cargo test --release --lib`
could not execute because their binaries are absent. The repository-wide
`benchrec.py gate` reported 609 historical record/artifact issues after
materializing all 3,568 tracked artifact manifests; this change adds no
benchmark records. The remaining absent historical artifacts were not
downloaded or rewritten as part of this saved-data audit.

The remaining validation task is an execution replay with independently
retained relations, costs and signed PCC standing. The development audit
does not discharge that task. No benchmark/leaderboard graph is changed:
the saved measurements and their standing are unchanged. The accompanying
diagram distinguishes checked artifacts from open execution obligations.
