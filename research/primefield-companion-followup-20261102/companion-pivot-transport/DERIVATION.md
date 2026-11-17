# Charged row pivots without changing the source field

2026-11-01. Derived identities and syntax-checked source; zero numerical trials.
This extends the explicit inverse construction's rank-profile interface. It
preserves prime-field elliptic inputs and the exclusions on index calculus,
GHS, generic elliptic collision search and extension-field substitution.

## Sources and scope

Read companion-recursive-inverse/DERIVATION.md and recursive.py, plus
companion-schur/schur.py. Revisited Bostan et al. (2017), Section 6, especially
the preconditioning and leading-principal inversion discussion. That work
uses structured preconditioning and a declared rank-profile condition. The
present exact transposition identity is elementary; it is not a new general
structured-inversion theorem and imports no randomized success bound.
https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf

## Row-swap displacement identity

Let Z_a A - A Z_b = G K^t. Swap distinct rows i,j with
u=e_i-e_j and P=I-u u^t. Over our odd prime field, P is exactly the row
transposition matrix, P^t=P, P^2=I. Set B=PA. Then

    Z_a B-B Z_b
      = P G K^t + (Z_a P-P Z_a) A
      = P G K^t + u (A^t Z_a^t u)^t - (Z_a u)(A^t u)^t.

Therefore a generator pair is

    GB=[P G | u | -Z_a u],
    KB=[K   | A^t Z_a^t u | A^t u].

The constructed width increases by two. Both additional right columns come
from charged transpose applications of the existing source operator. Source
wraps a,b and the base prime do not change. An identity swap returns the
original representation and makes no queries.

For Z_a the cyclic down-shift, Z_a u=[a*u[-1],u[:-1]] and
Z_a^t u=[u[1:],a*u[0]]. The implementation deliberately charges two field
products even when their operands are zero, and N+1 field subtractions:
one to form -1 and N to negate Z_a u. These exclude the complete source
kernel ledgers. The new generator arrays contain 2N(s+2) field slots;
input copies, query temporaries and accumulated audit histories also count
towards memory and must not be hidden behind this retained-size statement.

## First-column pivot, or a genuine right-null witness

Query c=A e0 once. Scan in ascending row order, retaining the exact number
of inspected entries as a separate native counter. If c is identically zero,
return the nonzero vector e0 with its zero source product. This proves A is
singular assuming the source operator is correct. It does not imply that an
arbitrary RHS is inconsistent, and it is not a left-null exclusion witness.

Otherwise choose the first j with c[j]!=0, and swap rows 0,j. Then
(PA)[0,0]=c[j]!=0. This takes one source query if j=0, or one forward plus
two transpose source queries if j!=0. Every nonsingular A has a nonzero first
column, so this step always finds a scalar pivot on nonsingular inputs over
the original prime field. No probabilistic field-size premise is needed.

The standard invertible swap matrix [[0,1],[1,0]] is an immediate algebraic
control: its first column selects row 1, and P A=I. No numerical experiment
was performed for this observation.

## Coordinate and certificate transport

For B=PA, the system Ax=b is equivalent to Bx=P b. Right solution and
right-null coordinates are unchanged. If w^t B=0, the original left-null
vector is P^t w=P w. Pairings are preserved:

    (P^t w)^t b = w^t(P b).

Thus normalized exclusion witnesses transfer only when the reduced pairing
is actually one. Source feature-nullness remains a separate check; matrix
nullness alone does not establish it. pivot.py provides permutation helpers
for the RHS and left witness. It does not manufacture an exclusion witness
from its first-column right-null certificate.

## What a complete pivot policy would cost

This primitive guarantees the first SCALAR pivot on nonsingular A. It does
not ensure that a prescribed balanced leading block is invertible. Repeated
scalar pivots followed by Schur reduction provide a deterministic route for
nonsingular matrices: a nonzero pivot and invertibility of PA imply that its
Schur complement is invertible. But each swapped level adds two generator
columns and its Schur step adds two more. After d such levels, constructed
width can reach s+4d, and scalar elimination has N-1 levels. The balanced
recursive inverse currently assumes a successful fixed pivot tree; this
primitive is not silently installed as an uncharged fallback.

Next investigate rank-revealing block selection or generator compression
that keeps this width growth controlled. A complete singular-system path
also needs rank-profile progress when the first column is zero, not simply
an inverse failure. Those are separate constructions, with complete costs.

## Required checks before execution

The new module has only been syntax checked. Freeze a full deterministic
cohort containing every i,j transposition for declared dimensions, both source
application directions, boundary/interior and identity swaps, all allowed
wrap patterns, odd dimensions and scalar cases. Include zero first columns,
nonzero top pivots and later pivots, plus invertible matrices with singular
leading blocks. Independently compare generated B with explicitly permuted A;
check displacement identities, RHS equivalence, left/right null transport,
normalized pairings, source-query counts, scalar work and scan lengths.
Corrupt the commutator sign, wrap, row index and witness permutation and
require rejection. Preserve all original source contexts and charge every
failed pivot and obstruction. No selected-instance trial precedes the frozen
contract or bypasses existing full-kernel and integration prerequisites.
