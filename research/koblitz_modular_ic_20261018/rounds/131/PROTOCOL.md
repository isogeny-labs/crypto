# R131: N51 residue screening on the twelve degree-7 places

Exact source-bound local calculation. `source-data/N51.sing` is the verbatim
N51 assignment excerpt from the SHA-pinned R51 native donor. Parse all 105
rational terms with the pinned R102 parser. At the degree-7 x-residue `a`,
verify that every displayed denominator is a unit. For each of the three
R130 base residue fields, evaluate the 105-term N51 coordinate as a polynomial
H(lambda) of degree below 16. Reduce H modulo every R130 distinct-degree
factor product of G51 and compute exact gcds. A coprime group gives N51
valuation zero at each place in that group. A nontrivial gcd gives the exact
number and residue degree of zero-residue places but requires a local lift
before assigning positive valuation. Check all 12 places are accounted for.

Controls: source/excerpt and prior-result SHA-256, independent rational
evaluation and gcd, changed-input rejection, byte replay, and finite Lean
arithmetic for any place counts. Do not infer an N51 valuation from a zero
residue alone. Local deterministic computation with no cloud cost; commit
source/protocol before producing data.

Prior work: R51 native donor, R102 coefficient parser, R130 normalized
degree-7 lambda fiber. [Stacks' DVR criterion](https://stacks.math.columbia.edu/tag/0B8Y)
supplies the valuation interpretation.
