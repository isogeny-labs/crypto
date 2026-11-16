# Exact source coefficient prerequisite

Frozen correctness gate only; no timing or complete-cost gain is measured.
Purpose: independently compute the Hasse-Witt residues required by the cost
screen's Weil uniqueness certificate. Existing cold records are unchanged.

Use odd prime p in {5,7,11,13,17,31,127,257,499}, 12 deterministic coefficient
vectors per p specified in main.rs. They test coefficient extraction, not a
cohort of smooth curves. Require equality of all coefficients through 3p-1
between the p-adic linear recurrence and independent repeated polynomial
multiplication modulo p. Then compute W_ij=c_(pi-j) for the public F127 curve
[31,89,73,48,91,41,123,1]. Require det(I-TW) residues (1,84,46).
The reference L-polynomial enters only the final assertion, never extraction.

Recurrence proof: for F=f^n, fF'=nf'F. Compare degree k-1 to get
k f0 c_k=sum_i((n+1)i-k)fi c_(k-i). Work initially modulo p^3.
For 1<=k<=3p-1 the only nonunits are k=p,2p, each with valuation one.
At these steps the exact numerator is divisible by p; dividing its canonical
residue and modulus by p preserves correctness. All prior coefficients have
at least current precision. At other steps divide by unit k*f0. Thus all
returned residues are correct modulo p. Require f0!=0; f0=0 unsupported.
Current bounded arithmetic p<=65537 ensures u128 products fit. Coefficient
extraction uses O(p) ring operations, O(p) storage; validation reference is
O(p^2), retained only in the correctness worker, not a production claim.
The executable accepts only the listed hardcoded prime tests; this is not a
general input validator or production point counter.

Literature reviewed: Harvey-Sutherland Hasse-Witt II, §1 and Remark2.2,
https://arxiv.org/html/1410.5222v1 ; established Hasse-Witt identity and
precision-loss concern. The recurrence follows elementary differentiation;
no novelty claim. Exact Weil lifting screen and primary source discussion:
../../research_notes/Genus%20three%20cost%20barrier%20screen/weil.md .

Commit source/protocol and save S3 checkpoint before one execution. Retain
stdout/stderr and exit status, including failure; do not rerun after failure
without a separately committed amendment. Successful residue recovery alone
does not test full polynomial uniqueness, subgroup proof, speed, generic
mapping, destination complexity or the 10x genus2+ goal. The next step after
this prerequisite is exact interval/CRT lifting with ambiguity retained,
then replacement admission in a complete scalable pipeline.
