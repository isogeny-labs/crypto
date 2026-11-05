# R111: exact x=0 Newton entry data for the marked-23 source

The pinned R51 degree-112 triangular donor has base fiber `f51(0,y)=y^4(y+1)^3`. Expanding all eight y coefficients gives the complete Newton support. At y=0, the lower hull vertices (z exponent, x exponent) are `(0,2),(1,1),(4,0),(7,0)`. The two positive edges have `v_x(z)=1` and `1/3`; their residual polynomial supports are `1+w` and `1+w^3`. Lean checks `1+w^3=(1+w)(1+w+w^2)` over F2. The factorization is useful for the next lift, but the residual roots alone do not certify distinct normalized places.

At y=1, with z=y+1, the lower hull is `(0,3),(3,0),(7,0)`. Its positive edge has `v_x(z)=1` and residual support `1+w+w^2+w^3=(1+w)^3` in F2. This is a repeated root; substitute `z=x(1+z1)` and continue the Newton process before asserting branch count or valuations.

For the displayed G51 lambda coefficients, termwise lower weights at y=0 are `{l^0:-160,l^1:-127,l^4:-97,l^8:-80,l^16:0}` on either first base edge; at y=1 they are `{-161,-128,-98,-81,0}`. These are lower bounds from individual rational terms, not exact coefficient valuations: cancellation and the lifted y series can raise them. The next calculation should use the normalized base series to evaluate the actual combined G coefficients, then its lambda branches and N51 valuations.

The source/protocol and a pre-run correction were committed before data. Fresh replay was byte-identical, a changed donor failed the SHA gate, an independent binomial expansion reproduced every f support point and verified each supporting line and G bound, and Lean 4.34.1 checked the two finite edge factorizations and x=0 fiber bits. There was no producer/replay failure. The pre-run correction fixed a mistaken hand-entered support assertion for y^4(y+1)^3; its original commit remains in Git history.

Result SHA-256: `e35e271cb6c86fc440676f1884709430eb091482f8531635294bc7dbdf49c4e5`. Raw archive SHA-256: `4eda8f1652c97d1140c4f5a76fb3a717a2d3e60ae287b05f65618fb178fcc52f` (1,000 bytes). Replay: `python3 research/koblitz_modular_ic_20261018/rounds/111/newton111.py <output-directory>`; check: `python3 research/koblitz_modular_ic_20261018/rounds/111/verify111.py`.

Sources: R51 donor, R97 x=0 base-order result, R105 parser, [Stacks integral closure](https://stacks.math.columbia.edu/tag/00GH), [Newton polygon background](https://www.math.columbia.edu/~gyujinoh/ANT_revised.pdf).
