# Active-column and single-line controls

Frozen before inspecting the six-arm census outcomes or running any
image-base polynomial collector. The original six arms remain in the
experiment, with their original fixed-column stopping rule.

## A modeling issue visible from the formulas

The `phi-l0` arm uses only original L0 factors and phi(L0). Its terms
therefore have zero coefficients on every original L2 logarithm column.
Keeping those unused columns prevents full rank, regardless of how many
relations it finds. This is an algebraic support restriction of that
particular encoding, not a failure of the proposed factor base or of
target recovery. Retain it as a control and add the natural encoding
that drops those unused columns.

Also include the strongest directly matched single-line peeling control:
three original L0 factors plus a fourth original L0 factor. It uses the
same S4 engine and number of peel points as phi(L0), and must enforce
four distinct physical factors and deduplicate the same four-point tuple
found under different choices of peeled factor. Comparing only with an
unpeeled S5 baseline would miss this available S4 implementation.

## Three additional arms on all 21 frozen instances

| Arm | Peeled points | Unknown columns |
| --- | --- | --- |
| union-l0 | Original L2 union phi(L0) | Original full folded columns |
| phi-l0-active | phi(L0) | Only original L0 folded columns, with the same anchor and target unknown |
| l0-peel | Original L0 | Only original L0 folded columns, with the same anchor and target unknown |

Candidate factor validation is scoped to its usable physical factors,
L0 union its peeled set. It may use original orbit representatives as
auxiliary log variables with verified scalar coefficients. Unused L2
factor logs are not claimed recovered by either active-column arm.

For L=|L0| and uniformly sampled residuals, unique signed four-distinct-L0
tuples have expectation `16*C(L,4)/n`. If phi(L0) is disjoint from L0
up to sign and is injective, unique three-L0/one-image tuples have
expectation `16*C(L,3)*L/n`. Their ratio is `4*L/(L-3)`, while both
explicit peeling implementations attempt 2L signed peels per residual
apart from infinity exceptions. This is a counting identity, not a
rank, elapsed-time, cold-cost or asymptotic advantage. A reference census
must establish the hypotheses on each actual base. The full image base
has 2L physical representatives and retains only the L0 logarithm
columns; its construction and all extra validation remain charged or
explicitly unpriced.

All nine arms receive the same complete-stream reference census and
native F4 collection through rank/cap, preserving the initial six-arm
run and failures. Reference sets can be projected or unioned only where
that operation gives exactly the newly specified physical tuple set;
the L0 peeling arm requires its own complete enumeration. Candidate
collection remains blind to reference sets and known factor logs.

For the polynomial stage, arms may be separate native process shards,
each with the original 8 GiB/1,200 s cap. This ensures that a censored
control arm does not prevent execution of the other arms. A/A equality,
independent signed-set/rank checks, negative controls and all complete-cost
and external-solver gates in the original protocol still apply.
