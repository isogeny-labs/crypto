# Root-free affine descent with actual subgroup identities

The public-divisor routine now emits projected anchor decompositions using the actual prime subgroup orders on the six direct-profile targets. Production scans the rational catalogue and buckets both sheets by (y-v)/u. A bucket of g+1 points certifies complete regular splitting by degree; it does not require a norm polynomial or gcd for every parameter. The separate gate forms each admitted norm, verifies its original Jacobian principal identity, and checks every emitted decomposition after an independently certified multiplier. This is classical divisor arithmetic implemented and checked, not a novelty or speed claim. [pcc:unverified]

## Frozen boundary and outcomes

Scientific source79f304759cf7, checkpoint20261116T065252Z-79f304759cf7 before once-only execution. Source/lock/native-import/binary checks passed. DESCENT.exit=0, stderr empty. Fixed g2/F251/c11/ell61667, g3/F211/c12/ell4620611, g4/F61/c2/ell16790591; targets ell/7+19 and ell/11+29, seed2026111607/08. Full polynomials, source/target Mumford coordinates and expected scalars outside the API are in DESCENT-INPUTS.txt. This reuses the direct-profile panel, without a fresh-holdout claim.

Twenty-four policies contain five verified projected decompositions, fifteen no-component outcomes and four precondition failures. Every selected witness is a SINGLE fibre; the curve gate does not exercise a selected multi-fibre affine cancellation. The earlier exhaustive graph gate covers component elimination algebra, but is not substituted for that missing projected-group case. No failures were replaced or rerun.

| Genus / p | Prefix H | Targets | Projected decompositions | No component | Precondition failures | Restricted reference completed |
|---|---:|---:|---:|---:|---:|---:|
|2 / 251|8|2|0|2|0|2|
|2 / 251|13(size rule)|2|0|2|0|2|
|2 / 251|16|2|0|2|0|2|
|2 / 251|123(all)|2|2|0|0|2|
|3 / 211|8|2|0|1|1|0|
|3 / 211|16|2|1|0|1|1|
|3 / 211|36(size rule)|2|1|0|1|2|
|3 / 211|104(all)|2|1|0|1|2|
|4 / 61|8|2|0|2|0|1|
|4 / 61|16|2|0|2|0|2|
|4 / 61|22(size rule)|2|0|2|0|2|
|4 / 61|36(all)|2|0|2|0|2|

The H values marked size rule use Compact's genus-specific cardinality formula, but **not its target-seeded anchor membership**. This is a prefix-anchor panel. Only one of six targets decomposes at those sizes. Genus3 target1 has repeated input support: its u is (x+35)^2(x+37) over F211, violating this routine's squarefree-u hypothesis. The original unrestricted descent already splits that target without any addition. That input remains a precondition failure in all four policies.

## Actual projected identity and reference controls

The gate finds h coprime to ell with [h ell]P=0 for every catalogue point, verifies [ell]target=0, and checks [h]target=sum alpha_j[h]P_j. Every projected catalogue point is checked ell-torsion; branch points are individually checked zero. Certificate multipliers are h=1(g2),2(g3),1(g4). This is an explicit certificate for the tested catalogue/target, not a claimed point-count theorem or free production setup. Source selection and certificate work stay outside the candidate API. The candidate consumes no expected scalar. It does not perform source descent or a complete scalar solve.

Unrestricted existing point-add descent completes all24policy calls (six distinct inputs repeated four times). Per-target additions are g2:1,3; g3:5,0; g4:6,33. The same logic restricted to anchors completes20calls; four retain the2048-add cap. Successful reference rows reconstruct the supplied divisor in the original group. Unrestricted reference has a different output-support contract; restricted reference changes sampling. Neither count is a same-unit speed ratio.

A candidate invocation scans123/104/36catalogue entries on eligible g2/g3/g4inputs, with123/103/35–36field inversions respectively. Every policy restarts cold algorithm state; failed scans are retained. Counters cover selected operations, not a complete operation inventory: polynomial input checks, allocation and graph assembly are additional work. Catalogue discovery/table setup, multiplier certification and gate verification are not free production costs. There is no elapsed-time measurement here, no operation-to-seconds conversion, and no independent scientific validation.

## Conditional guard-and-fallback exclusion

Consider a guard that tries this exact fixed-prefix pencil once, then invokes the unchanged original point-add routine with the unchanged input/seed if it fails. On a failed pencil its additive charged work is

    W_hybrid = W_common + W_pencil + W_reference,
    W_baseline = W_common + W_reference.

All component costs are nonnegative and a scanned pencil has positive work. Therefore it cannot supply a tenfold arithmetic-work reduction on those inputs under this architecture. The recorded size-rule policies fail on both genus2targets and both genus4targets, and reject one genus3target before scanning. This is a conditional work identity, not a measured runtime regression or an executed complete hybrid solver. Reusing a different anchor membership, changing the function family, or exploiting previously absent intermediate state would be a different construction requiring its own cost case. It does not rule out all fibre methods or all possible implementation gains.

The degree-based selector and projected descent are exact building blocks, but the simple fixed-prefix guard is not an all-input tenfold mechanism. Generalizing repeated-support inputs alone does not repair the observed genus2/genus4 no-component cases. Further work needs a structural change that produces useful constraints at the target-seeded anchor scale, and must charge all source/target setup and failures against Compact.

## Evidence

[Protocol](DESCENT-PROTOCOL.md), [raw outcomes/counters](DESCENT.tsv), [native summary](DESCENT-SUMMARY.tsv), [input/certificate receipt](DESCENT-INPUTS.txt), [graph](DESCENT-COUNTS.svg), [two-page PDF](reports/run-20261116T0652Z/REPORT.pdf). Native Rust reduces recorded counters and renders the figure; no scientific Python. Every PDF page inspected. Original sealed runs and canonical performance graphs unchanged. No timing benchmark, benchmark-index row, independent VALIDATION.json or performance promotion. Prior primary-PDF access gap remains explicit; no priority claim. Shared daily log remains reserved; SESSION.md carries the handoff. PR remains unmerged; mathematical tenfold objective remains active.
