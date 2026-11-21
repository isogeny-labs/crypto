# Affine-pencil route collides with existing two-place addition

The suggested two-parameter challenge-conditioned interpolation is an exact
form of ordinary two-place composition/reduction. This is a mathematical
pruning result for the10xsearch, not a general impossibility or speed result.
The proof was committed in992262265ebefore the native gate.

For degree-gD=[u,v], distinct points P_i,P_j with nonzerouvalues, interpolate
w=lambda*x+mu through z=(v-y)/u. Then t=v-u*w agrees with vonDand y_i,y_jon
the two points. The monic quotient

    R = monic((t²-f)/(u*(x-x_i)*(x-x_j)))

has degreeg for nonzero lambda and g-1for lambda0. Its reduced oriented
divisor [R,t modR] equals -(D+P_i+P_j). Thus the apparent genus2quadratic
advantage is the usual reduced degree2polynomial after two place additions.
Splitting it tests the smoothness of the same shifted divisor already
computed by the factor-base walk. The method itself supplies no additional
per-input relations beyond that equality. Different pair reuse/scheduling
can change correlations and costs, which would need a separate analysis.

This derivation uses the existing jac.rsMumford reduction and is consistent
with Bernatska--Kopeliovich Section3.2, reduction of degreeg+2divisors:
https://arxiv.org/html/1912.13277. The point sieve's known one-parameter
construction is recorded separately in the preceding study. The broader
Nagao literature review is incomplete; no novelty claim is based on it.

## Native finite gate

|Genus|Exact class comparisons|Split principal checks|Non-split Dinputs|
|---:|---:|---:|---:|
|2|9611|4991|25|
|3|11202|1899|76|
|4|9946|429|85|

All30759class comparisons agree exactly with two existing add_placecalls.
The native splitting oracle matches exhaustive root division on every
residual, including nonsplit rejections. All7319split cases pass the full
principal identity with oriented multiplicities. Coverage includes1190degree
drops,781repeated-root and347ramified split residuals,25low-degreeDskips and
614point skips from u(x)=0. These are gate counts, not a yield benchmark:
small fields, correlated inputs and pair reuse invalidate an independent
sampling interpretation. No ratio is a speedup or confidence estimate.

The original gplace-input cohort alone often retained split Dsupport. Its
13577case output is preserved in GATE-SPLIT-INPUT.txt. The explicit pre-code
amendment in SESSION.md adds a2g+3place cohort so reduction occurs before
testing. It supplies nonsplit Dcoverage per genus. The algebraic proof itself
does not require split u, and the gate is finite implementation evidence.

## Decision and next work

Close this algebraic mechanism as relabelling of established composition;
do not reopen it because it leaves a quadratic in genus2or directly includes
the public challenge. Arithmetic batching remains an engineering possibility,
priced against optimized two-place addition, with no promised10xfactor.

Return to a complete stopping-time cost comparison of the known point sieve
and both forest variants. Carry the observed g4p127rank tail, all cold setup,
public jumps, maps, forest expansion and algebra into that protocol. The
fixed128batch correctness gate cannot substitute for it. A genuinely distinct
mathematical gain needs a mechanism that survives this exact group-law
equivalence, a useful-relation cost case and prior-art review.

Run gate.sh with stable Rust/native jq, HEC_AFFINE_OUTa scratch directory and
a separate CARGO_TARGET_DIR. All scientific execution is native; no timing
campaign, scientific Python, cloud worker, closed-record change, default or
benchmark admission is involved. Full10xobjective remains active.
