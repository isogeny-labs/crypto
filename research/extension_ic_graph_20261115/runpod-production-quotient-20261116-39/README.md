The optional Rust production basis-reuse path and both references recover
216251322113766629 on a fresh p1009seed367 fixture, L263807419864521289.
The copied qualifier passes actual affine/checkpoint certificates, target,
structural prefixes, kernel/projection/counter checks and exact primitive
phase conservation. PCC-20261116-c04e84 remains unverified. [pcc:unverified]

| Observed incomplete primitive bitops | Original IC | Structural graph | Basis reuse |
|---|---:|---:|---:|
| All recorded phases |6787702462546|7919959237308|7541914541330|
| Linear algebra |320975823394|764235601774|386190905796|
| Relations |3001722368526|3719258435526|3719258435526|
| Descent |3463709644428|3435088679132|3435088679132|

The paired graph arms have identical sources, descents, collection endpoint,
checkpoint matrices/ranks and final projection. All non-LA primitive phases
match exactly. Independent replay explains LA as removal of the original
full affine solver and conversion events, then addition of the two-column
solver, conversion, kernel and dot-projection events. No difference is
estimated by subtracting38from37. [pcc:unverified]

The graph full/small bases contain492/418points, with514original sources,
two accepted descents and3427residuals. The independent homogeneous/full
rank is417/419; the actual quotient rank is2. Affine rows used419→124.
Removed events:9159251modr products/8983271adds/419inverses/189285known words.
Added events:1121products/1532adds/2inverses/1016069known words. The retained
kernel, every actual projected row and its target are checked independently.
Explicit serialized rank/product scope refers to quotient coordinates,
not an invented full-matrix execution. [pcc:unverified]

Original IC's observed subtotal remains below both graph subtotals in this
cohort. Native container/PRNG/RREF/Euclidean/fixed-factor and other work
remains incomplete: all arms are unrankable, full costs/bands/ratios null.
This admits no fastest-IC, wall-speed, unknown-log T1 or rho-parity claim.
[pcc:unverified]

Sourceb2cf8aaf10aa667e20d4d512a92a6a9d24c4524d precedes the sole Runpod call.
One cold round, no warmups/retries/exclusions;3/3workers+copied qualifier0,
143.652s whole practicality only.4vCPU/8GiBcpu3c/0.12USDh/pinned Rust image/
actualCPU24/1800s bound. All317committed source blobs and portable pre/
preworker/post/qualified hashes pass. Collection precedes own-pod
DELETE204/GET40420:09:48UTC; no owned paid pod remains. [pcc:unverified]

[Final one-page report](report.pdf) was visually inspected. Exact PDF/SVG/
150dpiPNG/native figure copies are bound by `figure-copies.sha256`.
Exact546374Bbaseline graph raw/560194Breuse raw/1135238Bexport stay
untruncated under`/workspace/ic-runpod-runs/runpod-production-quotient-20261116-39`.
`local-custody.sha256` also binds immutable binaries/source, sealed bundle
and returned archive.31888Bsummary/7403BICraw/small metadata fit Git;
`presentation.json` is a separately tagged native field projection.
No original raw, qualifier or summary was rewritten. Native export claims
no unavailable canonical-schema or independent signed execution admission.
[pcc:unverified]

Source checks:34library tests pass(two existing ignored),3example tests,
54native qualifier tests, warnings-denied Clippy/build, actual three-arm
transport/first-arm-failure continuation and capture controls pass. The
workspace-interrupted initial local test compilation is retained; the
unchanged frozen-source resumption passes. No global parent-suite pass is
asserted. The initial packaging refusal was a stale local origin ref,
resolved by explicit fetch before any scientific dispatch. [pcc:unverified]
