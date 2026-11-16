# Frozen five-rational-node sextic construction screen

Source `search.py`, SHA-256 `bb55f548a990e9611d48bd9359df4575a68df49ca1d83813189dccb4f0686383`, frozen before the counted full coefficient run. The exploratory 1,000- and 100,000-vector preflights are retained as development artifacts, not counted evidence.

Fix five F3-rational intended nodes in P² at `(0:0:1),(0:1:0),(1:0:0),(1:1:0),(0:1:1)`, the first five-rational-node configuration in Kudo–Harashita, Section 4.1, https://arxiv.org/pdf/2102.07270. Construct the 13-dimensional F3-kernel of the 15 independent constant/first-derivative conditions on 28 homogeneous sextic monomials. Enumerate every nonzero 13-vector modulo scalar by keeping the first nonzero base-three coordinate equal to one: `(3^13-1)/2 = 797161` projective equations.

Filter in order: all five prescribed tangent cones nondegenerate; normalization F3 count 4; normalization F9 count 10; then counts 28, 82, 314 over F27,F81,F243. Plane points are counted by evaluating the sextic at every P²(F3^r) point, in batches using independently precomputed field-coordinate evaluation matrices. For five ordinary F3 nodes, the normalization correction is +5 at even r and `2s-5` at odd r, where s is the number of split tangent cones over F3. A first ordinary-node candidate is directly evaluated over F3 and F9 as a control of the matrix filter.

A full-count match is only a candidate. Verify irreducibility, all geometric singularities, five ordinary nodes and no extra singularities, geometric genus five, and an independent L-polynomial computation before calling it a new carrier. Then construct an explicit E-cover, selected-subgroup map and full GHS/rho cost. A zero result excludes this fixed five-node linear system only, not other node configurations or nonhyperelliptic genus-five curves.

Counted command: `sage -python search.py > FULL.json 2> FULL.stderr`. Preserve source hash, all stage counts, and any hits. Do not edit the frozen source; create a new version for corrections.
