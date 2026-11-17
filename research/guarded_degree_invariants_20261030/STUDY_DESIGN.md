# Controlled exhaustive study — design, not a frozen or executed campaign

The user's requested deliverable is a rigorously certified exhaustive study
of last fall degree, degree of regularity and solving degree on one prime
and one binary curve, with isolated controls. This document retains that
requirement. The present PR proves structural bounds; it does not substitute
those proofs for the unperformed finite campaign.

## Fixed public curve choices

- Prime: `F_7`, `y^2=x^3+1`. The discriminant is nonzero modulo 7.
- Binary: `F_8=F_2[t]/(t^3+t+1)`, polynomial-bit coefficient encoding,
  `y^2+xy=x^3+x^2+1`. Here `a2=a6=1`, so the ordinary model is nonsingular.
  Integer arithmetic modulo 8 is never a substitute for this field.

All affine target points and all 3-element subsets of rational affine
abscissas are to be enumerated. No curve/base/target search after observing
degree results is permitted. Both ordinate signs and repeated-root/two-torsion
cases stay in scope. Canonical polynomial inputs may share target abscissas;
retain a complete point-to-input map and multiplicities so deduplication does
not remove a target. Growing bases (4 and above), larger curves and Boolean
descent are separate follow-up panels, not claimed by this initial census.

## Matched presentations and metrics

Cross only the presence of the five explicit field equations. Keep curve,
field basis, base set, target and inverse guard fixed within a paired row.
Do not compare a raw five-variable system with a Boolean-descended system as
though only field size changed. Prime versus binary is two separate panels,
not a one-factor causal comparison.

For each input, independently check field validity, nonsingularity, the complete
rational point list, base lifts and every polynomial generator. Enumerate
field solutions independently of the closure producer and retain SAT/UNSAT.
A field-solution check alone is not an algebraic-closure UNSAT certificate
for the raw presentation. Preserve the rational-lift bridge or check an
actual unit-ideal certificate. Never condition the published population on
finding UNSAT inputs.

- `d_F`: ordinary total-degree closure with all degree falls. For UNSAT
  unit ideals, require a replayed upper refutation and an independently
  verified complete closed separating space one degree lower. For SAT cases,
  do not equate first non-contradiction/stabilization with last fall: require
  a verified Gröbner basis and certified discovery degrees, including all
  later degree-fall obligations under Caminata–Gorla Theorem 2.9.
- `sd_drl`: least degree whose closed Macaulay row space contains a verified
  Gröbner basis for an explicitly fixed graded reverse lexicographic order.
  Bind order, variable ordering and generator normalization; certify basis
  membership, leading-monomial coverage and S-pair reductions independently.
- `d_reg_BFS`: first full homogeneous graded piece. Raw degree-three-base
  presentations have no finite value by the new universal Lean proofs;
  report this exact status, not a finite rank cutoff. For the field-equation
  arms compute and independently verify Hilbert ranks through fullness and
  the preceding non-full piece. The proved caps are 31 for `q=7` and 36 for
  `q=8`; these are upper bounds, not observed degrees.
- Castelnuovo–Mumford regularity and implementation-specific F4/F5 peak
  degree are different quantities. Neither is silently substituted for the
  preceding metrics. Any later inclusion needs its own algorithm/definition.

## Controls and certification

1. Positive: a known unit input through the same parser and closure path,
   with a checked degree-zero refutation; a known finite-degree relation
   with a strictly lower-degree separating certificate.
2. Negative: planted SAT inputs with an actual full assignment. A false
   contradiction must fail independent polynomial evaluation/replay.
3. Representation: raw versus explicit field equations. The present Lean
   proofs verify that the raw surviving `u^d` witness stops separating in
   the augmented presentation and give the finite graded-fullness bound.
4. A/A: identical canonical input under generator permutation, and a variable
   permutation with the monomial order transported as well. Counted degree
   invariants must agree exactly. No timing or noise-floor claim is made.
5. Corruption: changed field modulus, coefficient, input hash, claimed closure
   flag, omitted multiplying consequence, overflowing DAG step and changed
   target/base lift. Rehash tampered payloads where appropriate; rejection
   must follow from semantics rather than only stale outer hashes.
6. Independent engines: dense producer versus separately implemented sparse
   replay/lower-space checker, with recorded source and binary identities.
   Existing binary certificate code is a candidate component, not an
   independently certified exhaustive result or an already complete prime
   engine.

## Freeze, execution and stopping requirements

Before any scientific data: finish the native instrument and independent
checks; produce and validate the canonical `experiment.protocol/v1` JSON,
freeze it and its input population/source/analysis hashes in Git, upload the
required resume checkpoint, and bind a tested source image. This design is
not a canonical protocol-validator pass or a frozen analysis implementation.
No RNG sampling or p-values are needed for the census; execution order is
fixed canonically and every timeout/resource failure stays in the population.
The full run's resource ceiling and original deadline must be frozen before
submission, not adjusted after observing a difficult instance.

Use the repository's EKS Spot route. No scientific campaign was launched.
The actual local access probe, after installing an isolated AWS CLI, returned
STS exit 255: `Unable to locate credentials`. Runtime reports no configured
secrets, runtime variables or outbound identities and no EKS MCP is installed.
The reviewed GitHub route accepts an ecbench spec and builds/pins ecbench;
it does not build this as-yet-unimplemented degree-study instrument. An
existing authorized role plus an appropriate immutable custom image/build
route is the concrete missing execution prerequisite. Do not invent an image
identity, dispatch the integration fixture, broaden IAM or switch to a local
scientific sweep without an explicit backend instruction.

After execution, retain every attempt, actual flow counts, all certificates,
source/binary/image hashes and control outcomes in a unified
`experiment.export/v1`, checked independently. A complete census requires
all rows resolved or explicitly unresolved; resource caps yield bounds/status,
not fabricated exact degrees. Leave performance/security/asymptotic claims
out of this study. A negative result remains reportable.
