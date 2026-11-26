# Source-pinned degree-32 replay result — 2026-11-24

The frozen size gate stopped the replay before timing. The rebuilt binary from
commit `a5503f4bb66ac3aa4eac3125fba01534d2574096` reproduced the 13-bit roster
instance `icv1-fp13-t19-0d05ae31` and the interval arm's 16 lifting
abscissae, but the degree-32 isogeny arm contained 18 lifting abscissae rather
than the 16 recorded by the evidence binary. Both preflights emitted all 16
rows and exited 1 because their deliberately truncated one-trial subtraction
queries did not complete; their JSON was valid and sufficient for the size
gate.

The append-only store is `degree32_source_replay.jsonl`, four records,
SHA-256 `513dfc016f98021f2804cf0f14efafbb230badd78c135fa993629f5da8aeb6bc`.
Its terminal verdict is `no-size-matched-instance`. No timed candidate/control
record was produced.

Sources checked (KB revision `750797a540fb4c38837c67b12df84d5b`, refreshed
2026-11-25 03:19 UTC): `source:91b6bbe9fb73267bbeb5d7c97f493b26`
(`docs/ecbench/README.md`) supplied the charged auxiliary-curve construction;
`source:81a446d8a0f88023cfe934a2fcb608bb` (`docs/ic/SWEEPER.md`) supplied the
matched algebraic-sweep contract; and
`source:c61ed5eb3ba377e55889ea94a3975ebc`
(`research/notes/index-calculus/RESEARCH_INDEX_CALCULUS_FACTOR_BASE.md`)
supplied the structured-factor-base scope and solver limitation. The lexical
index was current; no semantic index was active.

This result closes the original source-replay protocol as a reproducibility
mismatch. Matching the three files listed in `SOURCE_BUILD_RECEIPT_20261124.md`
was insufficient to reproduce factor-base construction. The evidence binary
did not embed a source commit, and its recorded path was later overwritten, so
its remaining transitive source state cannot be reconstructed from the binary
receipt. A new performance panel must start from a source-pinned commit,
reselect an equal-size interval or explicit control, freeze the resulting
binary hash, and retain the executable or a digest-pinned build recipe.
