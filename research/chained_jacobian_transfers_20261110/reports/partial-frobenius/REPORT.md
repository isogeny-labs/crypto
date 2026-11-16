# Actual partial-Frobenius transfers over F101

The complete native suite passes **35 tests**, with no failed, ignored or
filtered tests and all **623** source/fixture/engine hashes unchanged.
Applying Frobenius to just E0 in the actual T0 gluing graph gives classes
A24/A12 and two explicit two-glue models over F101. All rational factor
points stay fixed. Full02 is PCC-20261112-f04008; selected01/6bbfba is one
diagnostic with 34 filtered. Scientific standing remains unsigned.
[pcc:unverified]

| Recorded value | Cpi1 | Cpi2 |
| --- | --- | --- |
| Equation y²= | 84x⁶+77x⁴+25x²+52 | 93x⁶+14x⁴+85x²+72 |
| Intrinsic factors | E0=(1,46), F0=(44,21) | E0, F0 |
| Actual two-graph / four-graph sign class | Gamma24 / A24 | Gamma12 / A12 |
| Finite rational points / rational infinities | 118 / 2 | 120 / 0 |
| Leading / constant F101 square roots | ±36 / ±31 | neither exists |
| N1 / N2 / Jacobian order | 120 / 10436 / 12317 | 120 / 10436 / 12317 |

No retwist or monic normalization is used. The native HLP and short-cover
adapter checks coefficient identities, every rational curve point, extension
infinity/x=0 points, all six branch roots and the exact four-point graph.
Direct/reciprocal cover parameters (destination,a,b/3,s) are
(F0,84,93,9)/(E0,52,42,9) for Cpi1 and
(F0,93,72,37)/(E0,72,62,37) for Cpi2. The old universal cover identities and
seven exact algebra certificates remain unchanged. [pcc:unverified]

## Actual graph and rational checks

The source Q0 is extracted from actual elliptic four-torsion Cantor
pullbacks into T0. For powers k=0,1,2,3, the checker verifies all 16 graph
points, full projections, 256 sums and product Weil pairs, and Frobenius
stability. It separately checks Frobenius covariance on all E0[4] pairs.
Q1/Q2 match independently reconstructed balanced states24/12. The first
three sign classes and their two-graphs are pairwise distinct; Q3 equals
the first-factor negative of Q0 exactly. Treating Frobenius as identity on
geometric four-torsion misses 15/16 graph points for both k1/k2.
[pcc:unverified]

Frobenius fixes all 109 E0 and 113 F0 rational points. Both k1/k2 fix every
12317 rational product pair; the signed three-step product route matches
CRT6103, residues108/1, on all those pairs. The alternative sign gives6214.
These are product-point checks, not norms from arbitrary new Mumford inputs.
The three-step two-graph loop already closes with rational identity;
applying the additional elliptic involution gives6103. The four-graph loop
needs the relative-sign identification. [pcc:unverified]

## Global proof and the computational comparison

[PARTIAL_FROBENIUS_PROOF.md](../../PARTIAL_FROBENIUS_PROOF.md),
PCC-20261112-d67ccf, uses actual stable quotient kernels and standard Picard
and group-scheme theory. It gives purely inseparable degree101^k maps with
connected kernel scheme of length101^k and exactly one geometric kernel
point. Their adjoint norms descend diag(101^k,1). Degree coprimality
preserves both prime DLP subgroups; all constructions descend to F101.
Primitive-four-glue target equations and whole Jacobian evaluation remain
separate obligations. [pcc:unverified]

For canonical product pull/norm P_i,H_i, with norm scalar n=2 or4, the
cleared global identity is [n]F_k=P_k phi_k H0. On rational points phi_k is
identity, hence F_k=[n]^-1 P_k H0. The existing direct bridge P_k H0 already
has separable degree16 or256 and preserves the same odd-prime logarithm.
Both have an identity factor stage, and the common output multiplier n can
be retained for DLP transfer. This is a **negative factor-stage comparison**:
partial Frobenius supplies different global inseparable maps and actual
targets but establishes no further evaluation saving over that bridge.
Construction, target pullbacks, solver and recovery remain unmeasured.
[pcc:unverified]

## Formal map, failures and custody

| Lean declaration in PartialFrobenius | Claim / exact formal scope |
| --- | --- |
| frobenius_preserves_form | e9174e; all finite four-module symplectic pairs |
| partial_preserves_product_form | e19537; one-factor action preserves product form |
| finite_sign_orbit | 106caa; F³=-I, F⁶=I, no early ±I |
| public_moduli_and_norms | 72739d; signed CRT and norm/inverse constants |
| public_inverse_halves_quarters | 4b1b04; inverse2/4 in an IntModule killed by12317 |
| rational_bridge_agreement | f99da7; normalized direct-route equality from explicit fixed-image, cleared-map and inverse hypotheses |

Gate02 passes six declarations, no trusted native. All six exact bindings
pass existing PCC locally with the corrected PATH; the project has **177
written declarations in35 modules**. New bulk compiled203 is expected,
not yet observed in its own CI. Actual curves, torsion bases, Weil forms,
AV/Picard/group-scheme and global degree bridges remain written/native.
The old five plus two cover certificates are unchanged; no new polynomial
identity certificate is inferred from finite module checks. [pcc:unverified]

Static01/336266, Lean parser01/21c177 and binding PATH02/a31d7b remain intact,
with a separate [failure report](../partial-frobenius-failures/REPORT.md).
The first e9174e claim is reused; its initial not_covered verdict and new03
local pass are both retained. No mathematical prediction failed in these
attempts. [pcc:unverified]

Raw full02 output is 89,109 bytes, whole standard XZ10,120, with raw/archive
SHA256, exact roundtrip, source snapshots, 623 before/after guards and
start/finish/status in [custody](../../evidence/partial-full-02/custody.json).
Rust1.99.1, Lean4.35.0. Reproduce the focused workflow commands: locked/offline
release tests under15 minutes, old reference/legacy guards, formatting and
strict Clippy; proof-gate the new module and inspect all axiom outputs.
The existing algebra job replays23 tests and seven exact outputs.
[pcc:unverified]

Sources checked before implementation: Galbraith9.6.14/15/24 and26.2.3;
Milne AV I7/II1; Bruin–Doerksen0902.3480 Lemmas2.4/2.6 and Theorem3.2;
the prior CHAIN_COMPRESSION_PROOF.md and actual cycle census. Exact external
CT43cc5d5541 already supplies the abstract anti-orbit and is credited.
KB MCP absent, wrapper unavailable, atlas encrypted; protocol e60790 retains
queries/fallback. No novelty inference follows from the search gap.
[pcc:unverified]

Next implement the generic nonmonic chart and cover norms from actual input
Mumford divisors, then check all12317 classes, dual/recovery and exceptional
fibres for these and C10/C20. T20 and new primitive-four targets remain open.
The fixed72-class catalog and its earlier figures retain their scope;
Cpi1/Cpi2 add explicit presentations within it. Editable DOT/SVG, recorded
counts, renderer and inspected PDF accompany this report. No benchmark or
paid experiment was launched. [pcc:unverified]
