# Independent read-only review of the exact Hermite unit

2026-10-30 UTC. Reviewer `/root/modular_curve_sources/unit_review`.
Reviewed the frozen protocol, native source and retained summary/control
records against AppendixD.4.3-D.4.4, LemmasD.4-D.5. No source edits,
scientific timings, contraction implementation or theorem certification.

Initial source snapshot:

    49f0b8c7ea88dfe7b385cdf9f93fe5a652193bd08a5824241a60d0471b07b9bf  hermite_unit.cpp
    f5999206fc10110ed6076beb50414fdb4bd497a3816590f6252178c3e4a75c8c  PROTOCOL.md

## Verdict

The positive fixed-case reconstruction and local-integrality checks match
the stated rational-coordinate lemma contract. No positive-case arithmetic
or mathematical defect was found in the reviewed implementation.

One negative-control coverage mismatch needs correction or explicit
qualification before claiming the entire frozen protocol was executed:
the initial source's low-precision control calculates the expected bad
weight directly, rather than injecting the bad quotient coefficient and
running the actual reconstruction path.

## Checks supported by source inspection

- The grid, alpha, h, active nodes, tapered multiplicities and p_t agree
  with the source. The complete grid is used for chart hypotheses,
  including t=0; reconstruction uses only nonzero positive-multiplicity
  nodes.
- Every polynomial coordinate has its degree checked, its F(0) checked
  against the supplied known constant, and every coefficient of every
  full local chart checked for2-integrality. The Cauchy family actually
  has rational ordinary coefficients; it does not silently replace the
  locally integral examples by integral ordinary polynomials.
- All actual Cauchy-kernel differences are included. W_delta(0), equal
  derivative valuations across the grid, chart integrality, and the
  extra factor of2 off the distinguished chart are checked.
- E_t(0) is computed by its direct rational product. Its exact valuation
  is compared against D.10 with the required j/2^j factor. The source
  literal `-6L` is C++'s long integer literal negative6, so the tested
  lower bound is -6D_t h, as required; it is not a multiplication by
  the grid's L.
- The inverse series is obtained by the ordinary recurrence and checked
  by multiplication modulo z^q_t. Its integrality is checked before
  any precision reductions.
- Canonical rational reduction modulo2^p_t inverts only odd
  denominators. Both perturbed-input and perturbed-quotient variants
  add prescribed2^p_t multiples, and the lift precision is explicitly
  checked. The reconstruction uses the ordinary truncated series,
  not divided-power or derivative coefficients.
- Every node contribution is independently evaluated by Horner and
  by a weighted coefficient sum. Exact reconstruction is compared as
  a rational identity to the supplied F(0); all variants' final
  integral residues and exact coordinatewise error valuations are
  checked. Zero-error valuation is consistently represented as inf.

## Negative-control mismatch in the initial snapshot

The first three controls actually call the hypothesis checker and reject
nonintegral F(0), a nonintegral odd chart, and excess degree respectively.

The fourth control computes

    error=(1/8)*(1/4)^2=1/128, v2(error)=-7,

and checks that this hand-specified rational fails the precision and
integrality predicates. The calculation is correct for node t=-1,j=2
when a=u=M=1. However, it does not obtain E_t/chart/quotient from the
Hermite code, alter a quotient coefficient, or reconstruct the altered
sum. Thus it is an analytic weight/predicate control, rather than the
actual injected-corruption reconstruction control frozen in the protocol.

Recommended correction: construct the a=u=M=p=1 instance using the
same E_t, inverse, quotient and evaluation arithmetic as the positive
path, perturb only coefficient2 at node-1 by1, retain the actual
correct/perturbed sums and verify their difference is1/128. Preserve
this initial finding and the old records if a new source/run supersedes
them. The alternative is to qualify the existing control and mark the
injection obligation open; it would be inaccurate to call it executed.

## Scope of the four coordinates

The protocol chooses four rational coefficient coordinates described as
1,U,V,UV in a synthetic degree-two unramified coefficient ring with
V^2=0. Every Hermite multiplier is rational, so interpolation acts
coordinatewise. This is a valid isolated rational-coordinate unit of
the lemma; no extension-ring multiplication is needed for that check.

Do not describe it as enforcing the entire recursion's e=4lambda or
other global size parameters. The code does not implement such a
coefficient field, packet tensor or recursive call. The same scalar
coordinate identities can mathematically be embedded in four basis
coordinates of an allowed larger ring, but that observation is not
additional executed code or a full-ring implementation.

## Record observations and limits

The retained initial summary reports72 polynomial cases,308 kernel
charts,552 active-node instances,8832 node-error records and1152
final-coordinate records. This review inspected the source and summary;
it did not independently replay every arithmetic record. The reported
minimum error margin above p is a finite-panel observation and provides
no warrant for lowering the theorem's precisions or inferring a
uniform improved bound.

The source opens the main TSV streams and checks them initially. A
minor robustness improvement would check final stream/summary write
states as well, so a disk-full failure cannot leave a successful exit
with truncated output. This is not a mathematical defect in the
reviewed fixed records.

## Resolution after the corrected negative-control run

Subsequent reviewed source:

    975880b7e4fd4ed12c2b07b8615903f4781622df5898845c2cd98aa278cec6a9  hermite_unit.cpp

The initial source and complete records were preserved separately as
`hermite_unit.initial.cpp` and `attempt1-records/`. In the corrected
`negative_controls()` the valid polynomial F=1 at a=u=M=p=1 is
reconstructed using both active nodes-1,+1. The code forms the actual
E_t chart, its inverse, F's jet and quotient with the same exact
polynomial/series routines. It changes coefficient2 of only the node-1
quotient by1, evaluates both ordinary reconstructions, checks the
uncorrupted sum is exactly1, and compares the actual difference to the
independently specified1/128.

This closes the initial protocol/coverage mismatch. The observed second
run log reports the full positive panel passing, and the new retained
negative record explicitly identifies `actual_D9_injection` with the
correct error and rejection. This is a source-and-record review of
that correction, not an independent execution or a theorem certificate.
The final scoped verdict has no remaining blocking correctness or
protocol mismatch found in the examined fixed unit. The coefficient-ring
and global-recursion limitations above remain material.
