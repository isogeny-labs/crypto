# Resume: crypto @ claude/index-calculus-ecdlp-symmetry-65jh09

## Goal
User request (2026-11-03): run a deep research loop on prime-field ECDLP index
calculus. Find a connection to a seemingly unrelated field, use it to find a
factor-base symmetry that lowers the degree of regularity, and make sure it is
new (not a repeat of the lab's tried lines). Treat the IC steps together.

## Lead / hypothesis
Two connections, both new to the lab (grep found no Elekes/Ritt/toric fiber
product/phylogenetic hits):
1. Barrier: Ritt's theorem on commuting maps (complex dynamics) and
   Elekes–Szabó (incidence combinatorics) imply a dichotomy. A factor-base
   structure either commutes with the group law (an isogeny coset, which
   degenerates to IC on E/K) or does not interact with S_{m+1} at all.
2. Constructive: phylogenetic group-based tree models / toric fiber products.
   Pair the summands into "cherries" of a balanced binary tree, encode each
   cherry symmetrically (s, q), and glue the cherries by S3. This quotients
   the Sylow-2 wreath symmetry (S2)^{m/2} while keeping every input at degree <= 4.
   Prediction (from the gluing analogy): the solving degree D is flatter in the
   arity m than the caterpillar chain's. Kill: tree D >= caterpillar D in every cell.

## Spec
research/ic_tree_gluing_20261103/gen.py builds the systems and checks each one
on a planted on-curve decomposition. run.sh runs msolve 0.6.5 (-g 2 -t 1 -v 2).
Cells: p=786433, m in {2,3,4}, t in {3..7}, 2 targets per cell. Arms: C
(caterpillar), T (tree), F (full symmetric S4(e), m=3 only).

## State (2026-11-03)
- gen.py written; every planted check passes for all cells (m=2 t>=3, m=3/4 t>=4).
- Next: run msolve, parse D / last productive degree / widest matrix, then the
  report, graph and PDF, then the PR.

## Resume command
python3 research/ic_tree_gluing_20261103/gen.py OUT && bash research/ic_tree_gluing_20261103/run.sh OUT
