# Changed full-quintic ordering: e2/e3 coordinates, then the e4 border

UNEXECUTED. No frozen source/raw record is changed. The coordinator reports
v9 passed exact family equivalence, but the104-coordinate compiler was SLOWER:
paid medians166baseline0.358818555s versus104block0.382526923s (ratio0.9380),
with compiler34.524ms versus52.846ms. AS/sqrt setup outweighed smaller Gaussian
work. These are reported component observations, not author reruns or qualified
GB comparisons. The new hypothesis removes the identified69-element W4 sqrt
cache and uses a different exact elimination order; it is not the same retry.

## Exact any-fixed-k map

Use the existing coupled normal form, pinned literal norm and SAME h,k target:

    e2=C2+r²+hr,
    e3=C3+zr²+kr+h w,
    e4=C4+w²+kw,
    e5=C5+z w²,
    w=s+zr, e1=b=h²+h+z.

Constants are those already independently proved in
research/coupled_line_cells_20261029_v1/PROOF.md and the104-plan. Keep h!=0,
admitted b inW1, and the fixed field/domain/target. k may be ANY fixed element;
it is never chosen independently for different equations. No h0 formula is
asserted; such a branch remains unsupported/separate, not empty. For the actual
z outsideV and admittedh, h0/1 are excluded by e1 itself.

First enforce e2 inW2 exactly by the existing AS trace preimage. With basis
v_i of W2, require sum t_i Tr(v_i/h²)=Tr(C2/h²). Let actual rankrho be0or1.
rho0/RHS1 gives an exact empty coefficient family. Otherwise construct every
r by one particular, dim(W2)-rho halftrace/linearized directions and the AS
kernel directionh. Thus actual r has36-rho coordinates:35or36. In even binary
extensions use a complete linearized AS solver, not odd-degree halftrace.

Now choose v3=e3 DIRECTLY as a free independent W3 element (52 coordinates),
and DEFINE

    w=h^-1*(v3+C3+zr²+kr), s=w+zr.

This is a global F2-affine bijection between(r,v3) and all original(r,s)
satisfying e2/e3 spans: originalr and e3 recover both coordinates, and the
formula enforces the same e3. It imposes no collision/multiplicity/rank limit.
The compressed variable count is87 ifrho1,88 ifrho0consistent. There is NO
W4 square-root basis, no new AS equation for w, and no k0 restriction.

Only e4 inW4 remains on the actual low18 domain:14 binary affine constraints
because w²+kw is F2-linear in these coordinates. e5 has no constraint because
actualW5=K; e1 was admitted before construction. In tiny domains whereW5 is
notfull retain the additional e5 constraint exactly. Nonprefix spans must be
actual basis-product spans. Constant or inconsistent branches are retained.

Analytic direction columns avoid full norm evaluation during compilation:
for an r direction dr, dw=h^-1*(z*dr²+k*dr); for a v3 direction dv,
dw=h^-1*dv. Then deltae4=dw²+k*dw, deltae5=z*dw². Original r/s directions
are(dr,dw+z*dr). Pay h inverse, all trace/AS work and each field multiply;
these transformations are not free. A full literal-norm replay of the affine
particular and every mapped kernel remains part of the charged verification.

At the known actualk0 input, the old full family has73 dimensions. The proved
bijection gives87/88 coordinates followed by atmost14 rows. rho0consistent
would leave at least74 dimensions, contradicting that existing certificate.
Therefore that input must have rho1 and border rank14, if implementation and
constraint accounting are correct. RECOMPUTE these quantities; do not assume
other h/k cells have this rank. For otherfixedk preserve measured consistency,
rho, border rank and complete mapped family, even if empty or larger.

The final actual family still has73 bits. This is only a compiler change,
not supported-root dimension reduction, enumeration of2^73, original root
recovery, outer h/k traversal or an8x release result. Five-distinct/rank4
candidates are preserved but no rank4 supported solution is asserted.

## Frozen next test and acceptance

Create new source and independent review before source/input hash freeze.
Independently exhaust GF4/GF8/GF16 allr/s on source-defined fixedh/k cells,
includingk0ANDnonzerok, rho0consistent/empty/rho1, h0refusal, repeated roots,
nonprefix spans and extrae5 constraints. Compare literal norms, EVERY original
coefficient membership and the mapped complete affine sets to the unchanged
166 solver; do not compare different coordinate-basis arrays directly.
Synthetic trace controls must be labeled separately from actual norm cases.

Then compare THREE arms on the SAME exposed actual h,k0 input: strongest166
baseline, frozen104 compiler and NEW87 compiler. Do not select104 as an easier
baseline just because it lost. Include3 alternating SAMEPROCESS paid reps,
fieldparse/spans, inverse/AS/cache construction, rowformation, elimination,
full mappedfamily/literal verification and serialization, plus processRSS.
Retain both coordinate and mapped particular/kernel/rank, constants, trace
row/proof, border rows and exact mutualcoset/span verification. Rawcompact
aliasing is allowed only for identical complete repeated records.

Initial falsifiable prediction: removal of69square roots and fewer direction
columns lowers PAID total below the strongest166 median, not merely below104
or in isolated elimination time. If not, record that loss and revise the
normal-form construction/input-processing or the higher-level support strategy,
not the same timeout. Fix budgets90s/4GiB/onecore and compactraw caps before
execution; any late/incomplete output clears complete-family/ratio claims.

All algebra here is characteristic-two. Odd primes/extensions require their
own proof; square/AS linearity is not transferred. Classical normal forms,
AS inversion and bordered elimination have no novelty clearance. Full support
search, original auxiliary/ghost/exceptional coverage and genuine matched
F4/F5/F6 hard all-roots timings remain the program's unmet goals.
