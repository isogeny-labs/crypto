#set page(paper:"a4",margin:17mm,numbering:"1")
#set text(font:"DejaVu Sans",size:10pt)
= Koblitz cover transfers — R35
*Exact construction observations; no IC performance claim*

The fixed degree-five padding point now supplies 154 complete coefficients,
a 770-dimensional algebra and exact section spaces of dimensions 68,196,324.
All 770 inverse-basis columns are recombined, with the coefficient embedding
tied to the actual source point.

#image("round35.svg",width:100%)

Run algebra35-01 verifies the coefficient lift and full forward/inverse basis.
Run jets35-01 checks all five source equations at every Newton stage, eight
units and the discriminant, all five coefficient conjugates, 68 literal
function-descent comparisons and every retained nine-term regression.

Run multiply35-01 checks all 52,650 unordered products of the 324 sections,
including all products after rank 708 is reached. Scalar field convolution
and binary bit-plane convolution agree on every product, and all 1,024 field
products are independently checked. Every product and pivot is recombined.

Run inflate35-01 constructs all ten 196-dimensional vanishing spaces in the
actual 324-dimensional ambient space, checking all 46,240 products in both
arithmetic implementations and all original/ambient/new-basis recombinations.
Completeness as L(3D0-D) uses the documented proper/local/pole bridge; exact
native rank is not itself a proof of global geometry or a target factor map.

Lean: 4 declarations in 1 accepted pinned runs.
Coefficient-lift identities use explicit characteristic-two, residue-field
and nilpotence assumptions. Native arrays, normalization and Riemann–Roch
are not imported as formal proofs. Every attempt is indexed in results.json.

Next: exact subspace division, flips, addition and equality, then the signed
class using certified deck order and the actual F2 factor / nonzero order-11
action. Full quotient saturation, polarized embeddings, theta, node187 and
larger arbitrary collector supports remain open. No timing comparison,
IC relation rank, scalar recovery or attack claim. Native 4GiB/1200s limits;
Lean4.34.1/mathlibd13f23b, -j1 -M4096/1200s. No independent external replay.

Reference: Khuri-Makdisi math/0409209v3, Representation B and large model;
R34 LOCAL_GEOMETRY, COMPLETE_DIVISORS and NEXT_CONSTRUCTION.
