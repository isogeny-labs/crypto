# Held-out selected-map signed-pair support

This exact component screen uses the six frozen 11-bit and 14-bit curves,
their 16 independently selected target points each, and equal-size
`x=0..31` interval controls. The source-pinned extractor calls the actual
X1(2,8) selector. Independent Python checks compare its selected `m`, coset
count, base size and group ledger with archived preflight, then enumerate all
unordered signed-point pairs. A second replay enumerates every `kG` and
computes pair sums by adding scalar labels modulo the verified group order.
It reproduced every map and interval metric on all six curves. The result
SHA-256 is `ecb23834f01012dd879b493c969f268b66e3301d4ccaf4719d7019f3999ee22a`;
verification receipt SHA-256 is
`b4323000884ca7867e6e9a76bf173eee687b838d9c483aa6adb452635c318ad4`.
The original first-case ledger assertion failed; its raw bytes and the
pre-rerun amendment are preserved in `FIRST_ATTEMPT.json` and
`AMENDMENT_1.md`.

| Curve | p | Columns each | Frozen-target hits, map / interval | Distinct rows on frozen targets, map / interval | Frozen targets with ≥2 rows, map / interval | All-group points with ≥2 rows, map / interval |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| b11_c0 | 2081 | 17 | 5 / 8 | 7 / 11 | 2 / 3 | 50 / 64 |
| b11_c1 | 2113 | 16 | 2 / 5 | 4 / 6 | 1 / 1 | 74 / 30 |
| b11_c2 | 2273 | 17 | 2 / 3 | 2 / 3 | 0 / 0 | 68 / 56 |
| b14_c0 | 16417 | 14 | 0 / 0 | 0 / 0 | 0 / 0 | 4 / 0 |
| b14_c1 | 16481 | 20 | 1 / 0 | 1 / 0 | 0 / 0 | 18 / 16 |
| b14_c2 | 16673 | 17 | 3 / 1 | 3 / 1 | 0 / 0 | 12 / 26 |

Across the three 11-bit curves, the selected map hits 9 of 48 frozen targets
versus 16 for the intervals, with 3 versus 4 targets having at least two
distinct coefficient rows. Across the three 14-bit curves it hits 4 versus
1, but neither base has a multi-row frozen target. The paired hit discordance
is 8 map-only and 15 interval-only at 11 bits, then 4 and 1 at 14 bits.
These directions differ by rung and curve. The all-group multi-row counts
also mix favorable and unfavorable curves. No repeatable multi-row advantage
survives the prespecified two-rung screen for this exact selected map.

The expected average raw pair multiplicity for a uniformly selected group
target is fixed by the equal base size; the map can only redistribute the
pair sums. This screen identifies direct-pair coverage and concentration,
not F4 solver efficiency or rank collection from a random walk. The selected
map still incurs auxiliary search and construction, which this table does
not time. The frozen cold panel remains the authority for any complete-cost
comparison against interval, existing PKM and same-target rho controls.
