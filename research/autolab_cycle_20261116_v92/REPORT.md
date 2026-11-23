# C92 research checkpoint

Three frozen experiments passed within one-core, 4 GiB worker limits. Earlier sources, inputs, manifests and evidence remain unchanged. The 50× goal and the frozen release gate remain unpassed.

## Complete conditional root sets

A disjoint cell cover now returns every ordered pair (x,u) satisfying one original binary S3 equation for fixed y and x in a prefix domain. Independent exhaustive tests covered 48 GF(8)/GF(16) cases and 306 root pairs. The proof accounts for every pruned cell and every surviving leaf.

Fifty-four cold runs compared this solver with scanning all u and with the stronger alternative of enumerating x and solving the quadratic for u. All complete root sets matched. On the two GF(2^11) cases, the cell cover took 0.0534/0.0554 s, versus 0.0254/0.0257 s for reverse enumeration. Despite pruning hundreds of cells, the tree is not the preferred conditional enumerator on this ladder. Observed peak process memory was 12.4–13.5 MiB. These are small conditional calibrations, not Gröbner comparisons.

## Larger cells and an exact rank bound

In GF(2^83), the certificate rejected 24 of 32 sampled cells at each tested width of 4, 8, 12 and 18 free bits. Every planted root survived. The cells overlap across widths, so their volumes cannot be added into a global pruning claim.

The observed rank pattern led to an exact product-space identity: kd mixed columns can be replaced by d+k−1 images under the linearized map t ↦ t²+yt. The full certificate rank is at most 2d+2k−1. For d=k=18, 360 columns become 71, preserving the exact span. See RANK_NOTE.md for the derivation.

A third experiment passed 240 exhaustive small span checks and 160 actual-field certificate comparisons. At the new width k=23, 23 of 32 sampled cells were certified empty; each contains 8,388,608 separator values. No contents enumeration was needed: each rejection has a directly checked dual witness. Passing cells remain unresolved.

Six additional cold runs measured the full and compressed 160-certificate panels at median 0.1952 and 0.1063 s: a 1.84× internal certificate gain. This is not an end-to-end rootfinding or F4/F5/F6 speedup. All 60 cold runs retain time, CPU, memory, outputs and diagnostics.

## What changes next

Use the certificate as a conditional filter, not a whole-field enumerator. Cache its separator-independent span and require an explicit mapping from actual outer-chart states to its cells. A 23-bit cell still leaves 2^60 high-bit positions for fixed y.

A simple fully free-source extension is provably blind for binary B=1: the unit triple monomial already spans the constant in the independent-monomial relaxation. The next variant must keep source guards or product consistency. Prime fields need a separate backend; a one-dimensional Fp span test becomes vacuous once it has any nonzero column.

The next queue includes guarded coefficient spans, complete tiny guard covers, cached mixed spans, prime interval/lift methods, the genuine baseline interfaces and the certified GF(2^83) residue search. No actual five-source hard solve, 20-instance qualification, novelty clearance or comprehensive cross-field speed claim has been established.
