# Preregistered restricted-control predictions

2026-11-13. Proposed exact values, not yet validated results. Preserve this
prediction and any failed attempt; compare against the actual definitions in
the next Lean module before promotion [pcc:unverified].

Use the completed two-line F17 control F=A, H=X+8, V=X^2+6X+4 and
mu(P)=coeff2((PV) mod F). The predicted moments mu(X^j), j0..6, are

    (1,15,8,12,10,5,5).

At target(0,6), choose five coordinates(s1,s2,s3,t0,t1) and actual polynomials

    A_w=s1 X+s2 X^2+s3 X^3+6t0,
    B_w=t0+t1 X.

This enforces the branch identically. The predicted actual restricted moment
matrix is

    S = [[8,12,10,5,0],
         [12,10,5,14,0],
         [10,5,5,4,0],
         [5,14,4,9,12],
         [0,0,0,12,6]].

The original witness coordinate vector is w=(12,8,1,14,6). Predicted exact
checks: S*w=0, rank S=4, adj(S)=2*w*w^T. These must be tied to the actual
quotient functional and branch polynomials, not a supplied rank assumption
or an unrelated literal matrix [pcc:unverified].

Proposed rank certificate route: upper-left3-by-3 determinant3. The S minor
with zero-based rows(0,1,2,4), columns(0,1,2,3) has predicted determinant2.
For an upper rank bound, factor S as its first four columns times the4-by-5
matrix whose first four columns are identity and whose last column is
(15,10,14,9). Prove these statements exactly; do not infer rank from floating
arithmetic [pcc:unverified].

Normalize the last adjugate column by its last entry (predicted4). Expected
coordinate vector is(2,7,3,8,1), giving A_new=3F, B_new=H and actual norm9G.
Predicted coefficient-pencil parameters are the first six moments
(1,15,8,12,10,5). Derive all actual coefficient-quadratic cofactor pairings and
instantiate guarded recovery with every nonzero/corank guard supplied
[pcc:unverified].

This validates a known-answer nonconstant chart, not a public-input finder.
Any mismatch is a real mathematical/implementation diagnostic; retain it
and inspect coefficient order, field normalization, branch substitution,
remainder identity and guard hypotheses. Exact equalities do not depend on
host CPU contention. No runtime/scaling experiment is proposed here.
