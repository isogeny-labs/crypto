# Exact signed-pair support screen for held-out X1(2,8) maps

Frozen before the six-curve screen. One preliminary p=2081 extractor run
confirmed the library API and is excluded from this result. Use the six
curves and all 16 target points per curve in `z2xz8_holdout/manifest.json`,
SHA-256 `99140281d9a94c288b8993b503cf91e4226879052e25c63dfdad0fccd20c8bc8`.
The source-pinned map library is commit
`a1c7d2699bd01690416274300da4b1ace3fa92e6` and clean release `ic`
build receipt `z2xz8_selected_field_20261128/BUILD.json`. Its release
`libcrypto_lib` SHA-256 is
`00b3f89920eb9877070584c0016e0c6626f4e978926823d1795d4fdedab83544`.
Extractor source SHA-256 is
`4524579faca7f6960d54450df70ce7d80fbfb62692402b7a5c292a0259676d1e`;
standalone binary SHA-256 is
`0809632efd249cb6539c91d8dfc23cfa9fc108abc7af880508ddb1bb40bb7191`.
The extractor calls the actual `z2xz8_equal_size_tower` and prints sorted
target-lifting abscissae from its selected coset and from `x=0..31`.

Independently verify every extracted x lifts on the target curve, each
selected x belongs to the recorded chosen coset, both bases have the same
number of abscissae, and the map's selected `m`, coset count and search group
operations match the archived held-out preflight. Construct both signed
points for every abscissa using a separate modular-square-root table.
Enumerate unordered pairs with repetition from all signed points. Verify
each computed sum is on the curve. For every frozen target, record the number
of distinct signed-pair representations, binary hit, and whether at least
two distinct coefficient rows occur. Over the whole prime-order group,
record empty, single-row and multi-row target counts, total pair rows and
maximum multiplicity. The sum of multiplicities must equal the enumerated
pair count. The same group and all 16 targets are used for map and interval.

**Falsifier:** if the selected map does not give a reproducible increase in
frozen-target multi-row support across all three curves at each size, then
this particular map has no evidence of a multi-row query mechanism at these
sizes. A favorable component screen still cannot establish a cold solve or
beat matched rho. The full relation runner, failed queries, rank, linear
algebra, setup, memory and verification remain in the primary cold panel.
Keep all six curves and both signs even when a map underperforms.
