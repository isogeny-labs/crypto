# Native quadratic-tower F4 witness gate

Date: 2026-11-25. Parent: `PROTOCOL.md`. This amendment is frozen before
running the native F4 witness census or a qualifying performance row. It
replaces the expanded resultant backend rejected in
`RESULTANT_DIAGNOSTIC_20261125.md`; historical rows remain immutable.
The source of the focused census test is commit `d44ca651bc8` in
`src/cryptanalysis/ic_framework/pkm_semaev.rs`. A built test executable
and any benchmark binary must receive separate pre-call SHA-256 receipts.

## Map, equations, and witness algorithm

Use the `p-1` Kummer coset `V={g*zeta^j:0<=j<2^t}` on a prime field with
`2^t | p-1`. Retain the PKM low-degree presentation: for each of three
abscissae use `t` quadratic squaring rules and a terminal equation. The
system couples those rules to the `m=3` chain
`S3(x1,x2,u)=S3(u,x3,x(R))=0`. The native candidate is
`PkmSemaevOracle::new(3,t,g)` with `mode=chain,engine=f4,descent=1,trace=0,
guess=0,degree=12,max_nnz=1000000`. It runs native tower F4, then its
linear-RREF and quadratic-root basis descent, then exact point-sign lifts
and a group-law equality check. Every failed or undecided query is charged.

The oracle currently constructs the `O(B)` Kummer root list and rank map
during preparation. This amendment permits that list as **cold setup** and
charges its construction, memory, and point lifts. The candidate's target
decomposition must use basis descent; an `O(B^3)` grid fallback is prohibited
in its timed arm. The grid, on the identical factor base, is a separate
control with its own charged setup and target work. At `t=3`, the public
oracle's `B^3=512` crossover would choose the grid. The diagnostic unit
test therefore calls native F4 and the basis extractor directly. At `t>=4`
the existing public route selects basis descent. No performance inference
is drawn from the forced unit test's runtime.

## Correctness gate before timing

Run only `kummer_m3_basis_descent_complete_small_curve_census` first. Its
fixed curve is `E/F_97: y²=x³+3x+15`, with 101 points, generator `(4,24)`,
and Kummer height 3. For every nonidentity target it compares the F4
basis-descent existence verdict and exact lifted relation with independently
enumerated ordered three-point sums. It includes all targets whose
abscissa belongs to the base. The test's per-F4-call cap is 2 seconds,
degree cap 12, and maximum matrix nonzeros 1,000,000. An external runner
will cap the entire test process at 240 seconds and 2 GiB, capture both
wall and process CPU, peak RSS, exit status and binary hash. A timeout,
resource cap, or any census mismatch is a failed gate, not a pass.

## Scaling and full-cost falsifier

After a passing census, freeze the exact independent prime-order curve
roster, target scalars, controls, source commit, binary hashes and resource
runner before timing. Retain the parent's three curves at each `t=3,4,5`,
16 independent cold targets per curve, candidate/control/control/candidate
mirrors, 60 seconds and 2 GiB per target. Record every attempted query,
including a grid crossover, timeout, F4 degree/nonzero cap, incomplete
basis descent, failed point lift, and unsuccessful target. The paid query
cost is F4 matrix work plus basis extraction and sign verification; cold
single-target cost also includes map/factor-base setup, all failed relation
queries, full-rank collection, matrix solve, and target descent. Compare
with both equal-size enumerative controls and matched same-target rho on
the same curve and target. A reused factor base is a separately labelled
amortized result.

Let `B=2^t`, `T_F4(B)` be charged per-query native F4 work, and
`T_ext(B)` be charged basis-descent and verification work. The working
prediction is `(T_F4+T_ext)/B²` decreasing at each successive height on
each independent curve, with peak memory below 2 GiB and verified
witnesses. Nondecreasing ratios over `t=3,4,5`, censoring before `t=5`,
or a complete cold cost that does not improve on both controls and rho
falsifies this backend for the stated objective. No wide-field timing is
admitted until this gate passes. A failure localizes the bottleneck to
F4, extraction, rank, descent or memory using the recorded stage costs.

This protocol covers `p-1` Kummer only. The `p+1` norm-one/Dickson
construction and the auxiliary-isogeny map need separate correctness and
cost records. The current native field and tower widths do not implement
P-192; see `WIDE_M3_GATE.md`.

Sources checked (KB): `source:13c233fff93bab742d135ed6c12bb7b6`
(`crypto:research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md`)
for prior native tower F4 work, and
`source:362976dae6e255e22be8c13a8c7a2e2f`
(`crypto-autoresearcher:knowledge/literature/KN-LIT-10b0fa.md`)
for the paper construction. Primary: Petit, Kosters and Messeng, PKC
2016, §3.1 and §3.3,
https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf.
