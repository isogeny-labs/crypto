# Signed order certificates on the integrated solver panel

All 48 scalars verified against the public divisor and the fixture scalar. Every stopping anchor and every phase vector outside eligibility matched the prior solver. Each input used fewer modular reductions. Ratios compare full-solver counts under the equal-price REDC model.

| Genus | Previous reductions | Signed certificate | Decrease across 16 inputs | Range of original / signed | Inputs reaching 10× |
|---|---:|---:|---:|---:|---:|
| 2 | 532090 | 526880 | 0.98% | 1.39–3.42 | 0 |
| 3 | 507838 | 493232 | 2.88% | 2.44–3.29 | 0 |
| 4 | 555520 | 523552 | 5.75% | 4.21–8.57 | 0 |

For a signed expansion with positive leading term, move that term to one side of the equality and negate the remaining sum. Equality in the Jacobian is equivalent to the original order check. The scalar-order and divisor validity checks still apply. Any extra doubling is charged and discarded before collection. The digit selection prices a doubling like an addition and may miss a cheaper expansion. This experiment measures arithmetic only.

Run ruby summarize.rb from this directory to regenerate BUDGET.tsv from both raw records.
