# Twist routes 1–3: designs and bounded execution

[2026-10-28 target-blind rank repair](reports/rank-repair-20261028/RESULTS.md)
· [frozen panel and exact independent replay](execution-v14/README.md).
42 fixed configurations include29 filled/13 underfilled bases. All174 analyzed
matrix cells replay;114 fix the target and60 leave it ambiguous. Known degree17
endomorphism equations newly fix14 cases, including ordinary controls. This is
one public toy target, with no twist-specific or complete-cost gain established.
[Concrete next gates](execution-v14/NEXT_STEPS.md) preserve the full cold-cost
and larger held-out requirements.

[2026-10-27 generator-order results](reports/generators-20261027/RESULTS.md)
· [exact native/CAS replay](execution-v7/README.md)
· [preserved failed full-order candidate](execution-v6/README.md).
The maximal-order rule fills 39/40 nonzero-shift bases versus 35/40 for the
first-point rule. One twist is C7 x C119, and its original point had order7;
the maximal point has order119. All120 complete constructions replay. Discovery
work remains unpriced, and no IC speedup or scalar transfer is established.

[2026-10-27 all-shift/genus-two follow-up](reports/overlap-20261027/RESULTS.md)
· [frozen source, exact replay and artifact reproduction](execution-v5/README.md).
All 27,140 shifts agree independently. On exactly shifts 1 and 3, no source or
twist elliptic factor appears in the 20 overlap models, while all ten known
cover controls pass. Three conditional Lean theorems check the finite counting
identity; full scalar-transfer and cost claims remain unestablished.

[2026-10-27 twist-derived progression bases and companion costs](reports/progression-20261027/RESULTS.md)
· [frozen plan, independent replay and artifacts](execution-v4/README.md).
The new 880-cell panel has 751 accepted outputs, 105 cooperative caps and
24 underfilled bases. Shifted twist coordinates supply valid source bases;
ordinary shifts change coverage too. Group enumeration is cheaper in every
admitted same-base comparison, while companion equations cost more in all 38
admitted pairs. The structured protocol binding gap is explicit; no full-IC
or twist-specific gain is promoted.

[2026-10-26 continued experiments and inspected reports](reports/continue-20261026/RESULTS.md) · [protocols, exact audits and reproduction](execution-v3/README.md).

The stronger ordinary control changes the interpretation of the polynomial savings: group meet-in-the-middle is cheaper on every tested completed comparison. Given-map genus-two BSGS loses on all 24 public targets. The 600-box frontier has 57 full-mask and 190 reduced-U degree reductions; actual completed cost pairs for both box encodings exceed direct product membership and group MITM. Explicit signs and an opt-in linear-root shortcut preserve exact rows without reversing the ranking. No calibrated whole-IC, novelty or NIST-size gain is established. The dated reports below remain historical evidence.

[Earlier follow-up results and remaining requirements](reports/followup-20261023/RESULTS.md),
[PDF](reports/followup-20261023/RESULTS.pdf), and
[execution/raw publication status](execution-v2/README.md).
[New E26 encoding and instruction diagnostics](reports/block-20261025/RESULTS.md) and
[inspected PDF](reports/block-20261025/RESULTS.pdf).
[Original execution report](reports/EXECUTION_RESULTS.md).
[Original experiment designs](EXPERIMENTS.md), [machine-readable status](experiments.json),
[dependency diagram](PROGRAM.svg), [Lean proof map](lean/README.md).

[Follow-up proposals E18–E29](proposals-20261022/README.md),
[proposal PDF](proposals-20261022/REPORT.pdf) and
[machine-readable designs](proposals-20261022/experiments.json) add twelve
blocked, unfunded experiments. They cover actual 3-gluing, coupled restrictions,
partial-relation elimination, independent divisor replay and solver/cost gaps.
That directory is the frozen design snapshot. Bounded local follow-up phases
have now run under separately committed protocols; the full twelve-experiment
campaign remains partial. A separate E30 incidence pilot is also recorded.
[Proposal validation and check logs](proposals-20261022/CHECKS.md) retain the
sandbox-blocked test attempt and its passing socket-enabled replay.

The 2026-10-21–22 native panels recorded 1,920 combined IC/rho executions:
1,704 verified logarithms, all identically replayed, and 216 empty-base cells.
Trace/cover/class-map controls, three bounded solver attempts, rational-height
searches and known positive pairing/isogeny controls are preserved. All three
eligible isogenies now have checked explicit duals. No calibrated IC speedup,
novel algorithm or NIST-size benefit is established.

Actual 3-torsion and explicit BHLS degree-three cover maps now pass native
and PARI checks on the three positive toy sources and public P-521. The toy
divisor transfers verify 24 distinct destination-subgroup recoveries back on
their sources; no NIST DLP is attempted. Polarized identification with the
certified torsion graphs and full costs remain open.

Five actual higher-prime toy torsion cases also replay. Coupled and helper
experiments retain identical projected rows to their matched rational
controls. E30's 270 finite-incidence cells produce no maximum-degree
reduction in the fixed grid. The 17-theorem Lean follow-up is a partial
concrete bridge. None of these results establishes a calibrated speedup.

The eighteen-experiment campaign remains partial: the result table retains
polarized gluing identification, higher-degree factorization, broader solver
grids, full accounting/calibration, formal map bridges and independent-host
replay as open requirements. The backlog cards remain blocked and unfunded.
E26 now has a bounded four-candidate pilot with independent affine-basis checks,
matched ordinary controls and two fixed-case instruction diagnostics. Full
E26 costs, E27 accounting and E28 final selection remain open. The earlier
follow-up raw bundle is published in
`setec-astronomy-artifacts`: a fresh download and all original member hashes
verify. The earlier denials targeted the wrong repository-default bucket;
the proposed policy for that bucket is superseded. See the
[corrected publication record](reports/publication-20261025-setec/REPORT.md)
and [study store configuration and reproduction](store/README.md).
No continuing
background computation is implied by these records.

## Reproduction

Rust 1.90.0 builds the native runner in [runner/](runner/). Each versioned
protocol freezes its exact source, fixture inputs, argv, seeds, resource bounds
and stop rules. Use the pinned revision for historical replay. Fresh output
directories are required. The solver records include invalid and resource-limited
attempts, deterministic split raw archives and extraction/checksum instructions.

The native ecbench session and its full same-host audit are linked from the
result report. Native aggregation and equal 0/0.1/1 GAE sensitivity are in
twist-route-summary; charts read recorded data only.

The native `twist-browser-refresh` projects the frozen pre-study browser plus
the registered models and sealed new session into the current site index. Its
checks retain every historical yield and existing metadata value. Reproduce
its base with `git show ac09baf:docs/browser/data.json`; the source/hash receipts
are in `execution-v1/checks/catalog-browser-refresh-v2.json`. The legacy
`generated_by` label remains for the canonical site schema. Existing Node
catalog checks additionally replay declared-field cover identities; no Python
was used for the experiments, verification or catalog refresh.

```sh
cargo test --release --lib --locked
cargo test --release --locked --manifest-path research/twist_routes_1_3_20261021/runner/Cargo.toml
cargo build --release --locked --bin ecbench
cargo build --release --locked --manifest-path research/twist_routes_1_3_20261021/runner/Cargo.toml
target/release/ecbench verify --dir research/twist_routes_1_3_20261021/execution-v1/runs/ecbench-20261022T0012Z --replay-all --out FRESH_AUDIT.json --exit-code
bash research/twist_routes_1_3_20261021/build-current-results.sh
```

The earlier [E00 report](reports/RESULTS.md) and its frozen receipt retain their
original preflight scope. Current execution status is the linked result report.
