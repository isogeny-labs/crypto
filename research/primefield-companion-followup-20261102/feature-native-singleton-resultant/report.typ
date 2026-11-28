#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= Streamed singleton resultants
T-1832 · 2026-11-07 · Fused public source tail

The public sign source now computes its singleton tail resultant during the existing baby-point loop. It no longer collects a separate prefix-coordinate vector, constructs its coefficient polynomial or traverses that vector again. A general singleton specialization also serves either orientation of the root-resultant API.

#image("flow.svg", width: 100%)

*Identity and convention.* For monic root products f and g, the source uses Res(g,f), the product of point-minus-root differences. With one point y this is the product of y−x over roots; with one root x it is the product of y−x over points. Multiplicities and zero factors are preserved. Empty inputs still return the existing error. Noncanonical u64 inputs are reduced before subtraction.

*Derived cost.* For N nonempty factors, initialize from the first difference and multiply N−1 remaining factors: exactly N−1 field products, zero inversions, constant auxiliary scalar storage. The API builds no coefficient polynomials on this path. Source fusion applies the same rule to s tail entries, removing the prefix temporary. Input slices and other source/workspace arrays remain charged separately. Nonsingleton main resultants retain the general algorithm and trace-dependent costs.

#table(columns: (2fr, 1fr, 2fr), inset: 4pt,
 [Exact software check], [Count], [Coverage],
 [Singleton cases], [8,320], [Both orientations; lengths 1–65; 8 primes],
 [Zero / orientation controls], [3,852 / 2,276], [Common roots; wrong sign detectable],
 [General-route / empty controls], [128 / 24], [Fallback equality; preserved errors],
 [Large-prime boundary control], [1], [Prime 2^61−1; noncanonical u64 inputs],
 [Source points / windows], [1,108 / 24], [Direct products, covariance, signs],
 [Additional recoveries], [617], [5,929 calls; zero unresolved],
 [Inherited public comparisons], [63], [511 calls])

*Evidence.* Every singleton case asserts N−1 products, zero inversions and zero root-polynomial calls. The unchanged general coefficient resultant and direct field product provide controls. The pre-fusion API checks and fused-source checks both passed; earlier source snapshots and receipts are retained. The inherited 160,660 division equalities, 4,100 lifetime plans and eight preparation controls pass. Final normal build pins seven sources with audit cfg disabled.

*Scope and custody.* Original 4,563-case cohort unchanged. No timing comparison, external benchmark, cloud run or full Cargo rerun. Prime/coprime source and word-size assumptions remain. PCC-20261107-8783a9 is unsigned producer evidence. See DERIVATION.md, validation-02, recovery-02 and normal-build-02 for formulas, commands and input hashes.

*Sources and next work.* The standard root-resultant identity is documented by FLINT; prior source/accounting code fixes orientation and integration. KB unavailable; no novelty claim. Next inspect the nonsingleton remainder trace and data lifetimes to target larger coefficient intermediates. Historical reports and canonical graphs remain unchanged.
