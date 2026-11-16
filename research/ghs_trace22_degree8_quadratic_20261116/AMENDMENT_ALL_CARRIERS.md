# Pre-run expansion to twelve trace-22 carrier models

16 November 2026. The first four cells in `search.sage` completed with zero
exact degree-eight `B=xH^2` hits on carrier masks 32280 and 35349, source
masks 68 and 84. The source and first-cell protocol remain frozen. The new
`search_all.sage` extends that same quadratic-square shape to all twelve
trace-22 monic odd-degree carrier masks from the independently audited census:

`32280, 35349, 36380, 51009, 67186, 106121, 112490, 113379, 127268,
168265, 175537, 175771`.

For each carrier, the unique rational branch polynomial is the monic linear
factor `L`, not necessarily `x`; the new code uses `B=L H^2` and takes ten
direct point-filter controls away from its zeros. It writes to distinct
`allcell_<carrier>_<source>.json` files, including a fresh replay of the four
original cells. The search algebra and exact-identity gate are unchanged.
There are 24 cells, each with 243 branch assignments and 59,049 quadratic
parameter specializations. The Frobenius conjugates of source masks 68 and
84 cover the other eight normal-form source models. This is a finite shape
screen, not a classification of all degree-eight covers.
