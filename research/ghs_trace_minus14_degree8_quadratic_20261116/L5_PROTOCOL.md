# Frozen degree-eight L-fifth linear-square screen

16 November 2026. Same eight representative source/carrier cells and field as `PROTOCOL.md`; this is a third independent denominator shape, not a revision of either preceding search. Source `search_L5linear.sage` SHA256: `ee529a87e23a7cb1a39bf8d51563ab27a237eecc3bb6f2f5f568497a8a25fbfc`.

Set `B=L⁵(x-c)²`, with c ranging over K except the root of the carrier's unique rational branch factor L. Its degree is seven and the branch pole has odd multiplicity five. For each of 243 compatible two-torsion assignments and each of 242 c values, reconstruct `A=(Bh) mod (Q1...Q5)`. Check degree eight, coprimality, necessary point-square conditions, and finally `f C²=B(A³+A²B+bB³)` exactly. A zero-hit receipt excludes this shape only; it does not classify all degree-eight morphisms. A hit must be independently replayed and transported nontrivially to the subfield Jacobian on the selected prime subgroup before an attack claim.

Run `sage search_L5linear.sage <fmask> <bmask> > l5cell_<fmask>_<bmask>.stdout 2> l5cell_<fmask>_<bmask>.stderr`. The script writes immutable `l5cell_<fmask>_<bmask>.json`. Preserve all raw outputs.
