# Source pins and geometric references

Scientific/operational parent63bc0895df3d2cbd38e22a3efe828a52a0fad731,
ready PR2215. R01–20 source/evidence versions remain historical. The ledger
pins every native implementation file, local Lean module, cryptolib
arithmetic dependency, proof-gate script/manifest, workflow and these
protocol/proof bridge documents. REPLAY.sh checks seven native outputs,
seven generated Lean modules, the native summary and two source-bound
identity records. Mathematical arithmetic, search, counts and hashes use
Rust; layout uses the existing report venv only.

Primary external references consulted for the written bridges:

- Stacks Project, specialization by a valuation ring:
  https://stacks.math.columbia.edu/tag/01J8
- Stacks Project, valuation rings and discrete valuations:
  https://stacks.math.columbia.edu/tag/00I8
- Fulton, Algebraic Curves, curve normalization/local valuation rings:
  https://www.math.lsa.umich.edu/~wfulton/CurveBook.pdf
- Moonen, book draft on abelian varieties, group-scheme and tangent theory:
  https://www.math.ru.nl/~bmoonen/research.html
- Pinned Mathlib primitive-root API used by the universal proof:
  Mathlib/RingTheory/RootsOfUnity/PrimitiveRoots.lean, especially
  IsPrimitiveRoot.eq_pow_of_pow_eq_one and pow_inj.
- Pinned Mathlib cyclotomic finite-field factorization, consulted for the
  order-of2 approach; the final proof instead proves the explicit trace
  identity and needs no irreducibility theorem:
  https://leanprover-community.github.io/mathlib4_docs/Mathlib/RingTheory/Polynomial/Cyclotomic/Factorization.html

The new obstruction and averaging construction are derived here; none of
these sources is cited as having already proved this source-specific
quotient statement. The proof map separates the actual Lean statements
from the written geometric, CM-reduction and native implementation bridges.

Identity source binding: geometric-series19 is IDC1h83eef58c80bbbbbf;
ordinary-infinity-chart is IDC1hf0e17a52634561b4. Both records refer to
KoblitzRound21.lean by SHA256 and are rechecked on replay. The failed
external Fulton PDF fetch returned502;its searched bibliographic
reference is retained,not called a fetched/read full proof. The source-
specific valuative/formal-group argument is written here independently.
