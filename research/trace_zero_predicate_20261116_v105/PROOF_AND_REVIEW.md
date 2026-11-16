# Trace-zero canonical predicate, exact coverage and cost boundaries

The coefficient formulas and inverse (including the inverse t^5 constant)
are derived in C104 TRACE_NORMALIZATION_NEXT.md. Both are affine over F2 for
fixed t. The oracle checks every constant/basis vector for every t in each
canonical field against independent Horner polynomial translation and inverse;
linearity then covers every coefficient tuple. Root translation preserves
all multiplicities. Each local-to-canonical field map is explicitly checked
on every addition and multiplication pair, including the degree-one T modulus.

Trace-zero templates are derived from all allowed multisets and certified
by complete finite rejection unions. Their zero sets equal the split table,
except the already proved squared-irreducible-quadratic family in GF32;
the full guard removes exactly that family. Small fields use exact cubic
templates. Thus field isomorphism and root translation transfer an exact
predicate to every inherited residue without adding independent selectors.
All coefficient wires still use the SAME76global bits and SAME3wrap q bits.

Odd normalization is checked separately through generic Horner substitution
H(X-h4/5), with5 interpreted in the prime subfield. GF3/GF9 and all five NIST
prime presets are exercised on repeated/zero root products. Characteristic5
is explicitly rejected for this normalization and requires a separate branch;
no binary trace-guard claim transfers to odd fields. These are algebraic
controls, not original-root solves or prime-field timing coverage.

Template generation, its exact finite certificates and field-map setup are
separately measured. A subsequent hard search must include fresh instance
wire construction, native formulation, model recovery, support verification
and reporting. Source/template/binary hashes must be frozen before that run.
Existing precomputed templates cannot be counted as free instance-dependent
work in a qualified end-to-end comparison. Worker lifetime has78-second
demotion,90-second outer limit,4GiB and one core; exact original counts remain
unset. Novelty and qualified performance remain separate unpassed assessments.
