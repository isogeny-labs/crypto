#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Paired weighted-symmetry transport
2026-11-02 · Derived identical-generator update · Numerical trials: zero

*Identity.* For u=eᵢ−eⱼ and P=I−uuᵀ, compose the existing row then column
transport. The exact generator columns for PAP are:
#block(inset:6pt,fill:rgb("edf3f6"))[
G′ = [PG, u, −Zₐu, PAu, −PAZᵦu] K′ = [PK, PAᵀZₐᵀu, PAᵀu, Zᵦᵀu, u]
]

#image("pipeline.svg",width:100%)

*Reuse.* With cᵢ=Aeᵢ and cⱼ=Aeⱼ, Au=cᵢ−cⱼ and
Aᵀu=D(dᵢ⁻¹cᵢ−dⱼ⁻¹cⱼ). The selected pivot column is always cached.
Only the other column may be missing. After swapping, cache key t becomes
P(t), and its vector becomes Pcₜ. This preserves provenance across pair pivots.

*Derived per nontrivial swap.* Let h be missing columns (0 or 1) and w input
width. Identity swaps remain zero-query controls.
#table(columns:(1.8fr,1.5fr,1.5fr),inset:4pt,
 [Quantity],[Sequential reference],[Paired transport],
 [Source applications],[4],[2+h],
 [Query widths],[w,w,w+2,w+2],[w throughout],
 [Transpose / forward],[2 / 2],[1 / (1+h)],
 [Output width],[w+4],[w+4],
 [Extra field products],[4],[3n+4],
 [Extra field additions],[2n+2],[4n+1])

*Native-product accounting.* The preserved fused kernel records 4w+8 native
products per successful application. Reference total: 16w+48. Paired total:
(2+h)(4w+8). Savings are 4w+24 for h=1 or 8w+32 for h=0, alongside 3n
additional field products and 2n−1 additions. These units are distinct.
Bit lengths, cache copies and runtime are unmeasured. For a second swap use
its actual input width w+4.

*Integration and ownership.* Only the complete driver's transport import
changes. Final G′/K′ columns match the reference in ordering and signs, under
the stated identities. Cache starts with one/two selected columns and grows
by at most one per swap (at most four retained). Selection is one-use and
source-bound; unchanged generators and internal metric/cache provenance are
required. Query/event paths and counters intentionally differ.

*Checks / next.* Full driver normalized-AST equality and dependency hashes;
all Python files syntax checked. No numerical, mutation, heap or performance
trial. Validate exact generator arrays, all swap positions, both cache branches,
pair reuse, nonuniform metrics, corruptions, ledger widths and full root
solution/witness equations under the unchanged upstream cohort. Next derive
metric provenance views and safe zero-prefix reuse; neither saving is counted.

*Sources.* paired.py, driver\_paired.py, check\_preparation.py, DERIVATION.md,
and preserved row/column transport modules. Bostan et al. (2017), equation (2)
and Section 1, DOI 10.1137/16M1062855, provides established shift/displacement
context; no new displacement theory or measured speedup is claimed.
