# Exact code and affine-width contract

Concatenate the14coefficient observations into C*theta+b. Each exact table
has a bijective affine-hull coordinate map a_i+B_i*u_i on its finite domain.
Quotient by image(C). Then sum_i q(B_i*u_i)=q(b+sum_i a_i) is necessary and
sufficient for a tuple of local table values to come from some theta. A right
inverse recovers one theta; an explicit basis of kernel(C) recovers every
alias. Original q/leading guards are applied only after original recovery.
No untracked independent local lift is introduced.

For a prefix A, an eligible syndrome lies in U_A intersect (target+U_B).
If nonempty, this affine intersection has dimension
rank(U_A)+rank(U_B)-rank(U_A+U_B). It contains2^w affine candidates. The actual
finite-domain reachable set can be smaller. Width is a dense-allocation
forecast, not a lower bound on nonlinear state counts, runtime or original
root count. Empty table domains remain explicitly empty even though their
affine placeholder is0. An affine-feasible code is not a finite-table solution.

The oracle enumerates every original tiny theta and every finite-domain tuple,
compares complete recovered root sets including noninjective kernels, and
independently enumerates all hull partial sums to check intersections.

At most4096rank evaluations, one core4096MiB,90-second outer supervision;
worker finalizes by84s and demotes at78s. No dense frontier is allocated.
Promotion requires the PEAK across an entire ordering <=20 and at least6bit
improvement against natural order, not a trivial small endpoint cut. Large
affine width rejects only this dense representation. Linear-code trellises,+variable elimination and MDDs are established ideas; novelty is unestablished.
Only one necessary binary norm family is represented, not original339-variable
root completeness, prime intervals, or qualified F4/F5/F6 comparisons.
