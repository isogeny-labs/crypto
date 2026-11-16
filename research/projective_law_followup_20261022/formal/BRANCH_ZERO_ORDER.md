# Simple-branch zero order for the common 2+2 model

This packet records a conditional formal lemma, not a new target experiment or cost comparison. The [diagram](branch-zero-order.svg), [gate table](branch-zero-order.csv), [Lean source](Common22BranchZeroOrder.lean), [compiler output](branch-zero-order-lean.log), and [PDF](BRANCH_ZERO_ORDER.pdf) preserve the exact scope for review.

At a rational simple root `a` of the cubic `f`, the existing local-stalk lemmas establish `y² = (x-a)q`, where `q` is a unit, and `ord(y)=1`. The new theorem `actual_branch_x_factor_zero_order` states: if the local numerator `N` has the explicit factorization `N=(x-a)t` and `B` is a unit, then `ord(N-By)=1`. Its proof substitutes `x-a=y²q⁻¹` and factors the expression as `y(yq⁻¹t-B)`. The second factor is a unit in the local ring, so the zero has order one. The more general `branch_zero_order` and its actual-stalk specialization are also checked.

The theorem does not assert that every numerator from the compiled genus-2 addition law has that factorization, that all finite zeros are covered, or that the associated rational function has the required divisor in `Pic⁰`. The global divisor-class equation and the Rust source-semantics bridge remain open. No previously frozen target row, operation count, or matched-rho ratio is changed by this proof.

Prior mathematics checked: [Stacks Project, Lemma 10.119.7](https://stacks.math.columbia.edu/tag/00PD) characterizes the discrete valuation ring and its unit-times-uniformizer valuation; this packet uses the actual-stalk DVR theorem already checked in `Common22BranchDomain.lean`, then proves the new conditional order statement in Lean. The Stacks lemma does not establish the genus-2 divisor-class equation.

Verification: from this directory, `lake env lean -o .lake/build/lib/lean/Common22BranchZeroOrder.olean Common22BranchZeroOrder.lean` under Lean 4.34.1 and Mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612`. Source SHA-256: `5e89be1013b4234938e336634a3a20455d3605e287abcf2c6ecb3e321729f5d9`. The three imported local-branch source hashes are in the CSV. The saved compiler output records axiom audits; each theorem uses only `propext`, `Classical.choice`, and `Quot.sound`, with no `sorryAx`.

Repository validation after materializing the tracked sparse-checkout test fixtures: `cargo test --release --lib -- --test-threads=8` exited zero with 4,954 passed, 0 failed, and 122 ignored. `scripts/validation_gate.py evidence` returned `valid`; the commit's artifact and large-file checks passed.
