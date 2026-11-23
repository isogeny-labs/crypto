# Native centered terminal witness

Implement the new symbolic construction in reviews/proof-bridge-20261030/reviewers/TASK-N83-BLIND/report.md, equations (13)–(22). This is an implementation and development-validation task (T-1002), preceding a separate frozen production experiment on EKS. No new production degree sweep is part of the unit tests.

For odd width ell=2s+1 over the stated odd prime, use y_j=z_j-w_j/2 and y_j^2=b_j^2, b_j=w_j/2. Set c=sum b_j and P=product(y_2a-y_2a+1). Compute A0=P, A1=NF((c+Y)P), A2=NF((c+Y)^2 P), then F=NF(P S(U,X,t)) as three sparse coefficient polynomials in U. Keep U unrestricted.

Compute the alternating projection by selecting monomials having exactly one variable from each pair. Their signed coefficients, divided by 2^s, give the coordinates in the basis P and y_last P. Verify these two U-polynomials against S(U,c,t)+b_last^2(U-t)^2 and 2c(U-t)^2-2((U+t)(Ut+2)+6). Generic target remainder degree is s+3; at t=c it is s+2. The degree-17 derivation bound at ell27 and nonmembership follow from the separately reviewed mathematical bridge, not from executing this instrument.

Development acceptance criteria, fixed before running tests:

1. Centered multiplication agrees with independent general quotient multiplication on small polynomials.
2. Fast projection agrees with literal pairwise (1-swap)/2 on every squarefree basis monomial for widths3,5,7.
3. Projection kills NF(X^k q) for k=0,1,2 and every squarefree q of degree at most s-1 at widths3,5,7.
4. At widths3,5, primes101/103, original and alternate nonzero weights, targets c,c+1,0, evaluate F=P*S at every selector assignment and U=0,1,2. Exact degrees and both projected U-polynomials must agree.
5. Reject even width, characteristic two, zero weights, noncanonical coefficients and noncanonical/altered polynomial bytes.
6. Small fixture generation/replay reproduces exact bytes; mutate one saved coefficient and require verification failure.

The development tests cover exhaustive finite small fixtures; no timing comparison or extrapolated numerical LFD is admissible. A production launcher will retain A0/A1/A2, all three F coefficient polynomials, projection coordinates, target/weights, hashes, exact source identity and terminal failures in a new run directory. The 83-bit field production targets will include c and c+1 for both original and spike40 weights, then source-bound ordinary targets. Changing the multiplier does not change the fixed factor base.
