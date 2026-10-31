# Prospective optional local mode-split regression addendum

Frozen2026-10-30 before implementing/executing this addendum. Run only after
the corepacket unit is implemented; this is a tiny local coefficient-enumeration
identity, not fullcontraction,3-adicHermite recovery or a runtime/scaling test.
It does not use the derived recursive callprecision p_t; p=1,2,3 below is an
arbitrary local modulus chosen for the fixed structural regression.

Fixed r5,a1,L3 class gamma=(R1,R2,C1,C2)=(0,0,1,0), component1.
Rowgroups are[0,4,1], colgroups[1,0,4]. Use integer gridS={-3,...,5},
C(T)=prod_{s inS}(T-s), C'(0)=-720, T=10=1+9.
Compute all nine exactinteger Cauchy entries
W_xy=C(10)/(C'(0)*(10-(y-x))) and verify each division exact.
Expected column0 is[-36036,-32760,-30030].

For all64 pairs of pinned unary assignments X,Y in{0,1}^3 and p=1,2,3:
1.Compute direct product prod_x l_Yx(sum_y W_xy X_y) exactly, then reduce mod3^p.
2.Split every sourcecolumn's crossing mode by its distinct targetrowgroup.
 Each row has its local affine constant plus its crossing mode terms.
 Enumerate ordinary polynomial terms by one term choice perrow; keep total
 modeweight<p, multiply source evaluations X_y, and compare mod3^p.
 Cross coefficients in this fixed matrix havev3>=1; verify this before
 applying the weight truncation. Require all192 split/direct residuesequal.
3.Preserve the intentionally incorrect unsplit result: one variable per
 sourcecolumn, a common exponent beta_y imposed at both crossed targetgroups.
 Enumerate beta>=0 withsum beta<p; multiply eachrow's coefficient for its
 targeted beta-subvector and the source X_y^beta_y. Record all192 results
 including coincidences and mismatches; do not treat coincidence at lower
 precision as a valid unsplit identity.

Explicit required witness:X=(1,0,0),Y=(1,0,0),p3. Direct
W00*(1-W10)*(1-W20) is9mod27. Correctsplit mustgive9;
incorrectunsplit mustgive0. This is an actual coefficient-enumeration
regression for the assumption that every mode has one targetgroup.
It does not claim that a genuinely shared globalvariable evaluated before
contraction is invalid; the badoperation is forcing its one exponent at
both targetgroup coefficients.

Retain all192 case rows, exactmatrix, split/bad mismatchcounts byprecision,
strictbuild/run/repeatlogs, hashes and explicit witness. No Python research
computation. Exactbounded int64 arithmetic with checked bounds is sufficient.
