# Bounded curve-transfer sweeps

`curve-transfer-sweep` enumerates the explicit same-field cover shapes that
the repository implements for one exact elliptic model. It is a construction
and evidence-inventory tool, not a discrete-log solver.

The executable supports two families:

- `binary_cubic_pullback_v1`, the fixed genus-three degree-three cover in
  characteristic two;
- `prime_quadratic_pullback_v1`, the genus-two degree-two cover
  `x = u^2 + c`, swept over an explicitly bounded interval of prime-subfield
  shifts `c`. This includes direct short-Weierstrass targets and the existing
  checked Montgomery/Edwards normalization route.

It does not search arbitrary rational functions, unimplemented Xarles
families, isogenies, correspondences, non-hyperelliptic curves, or all curves
of a genus. “Exhaustive” in an output means only every parameter point in the
declared finite family/shift bounds was checked. Zero candidates means no
candidate passed the family checker in that search space; it is not evidence
that no cover exists.

## Sweep

Build or install the standalone executable, then provide a strict request:

```json
{
  "schema": "curve-transfer-sweep-request/v1",
  "selector": {
    "kind": "icv1",
    "value": "icv1-f2m7-t13-616700dd"
  },
  "families": ["binary_cubic_pullback_v1"],
  "bounds": {
    "prime_shift_start": 0,
    "prime_shift_end": 3,
    "max_attempts": 4,
    "max_field_bits": 256,
    "max_genus": 5,
    "max_degree": 5
  }
}
```

```sh
curve-transfer-sweep sweep \
  --request request.json \
  --registry docs/curves/registry.json \
  --output sweep.json
```

Selectors are a full ICV1 identity or ICV1 slug (`kind: icv1`), an exact EC1
alias (`kind: ec1`), or an exact model object (`kind: model`). EC1 and ICV1
must resolve to exactly one registry row. An exact model may be unregistered;
the result then says so and does not invent EC1/ICV1 links.

Every candidate retains:

- exact target and source coefficients;
- field of definition, map formula, degree, HC1/CV1 content identities, and
  the replayable construction certificate;
- separate statuses for construction, geometry, Jacobian arithmetic,
  arbitrary-class pushforward, subgroup transfer, solver evidence, and DLP
  advantage;
- the exact EC1 subgroup obligations when registry representations exist.

The current odd-characteristic cover has two rational points at infinity. The
separate balanced backend handles monic split sextics over deterministically
checked prime fields of at most 64 bits, but this sweeper does not replay that
arithmetic. Odd-characteristic induced maps, extension fields, and larger
prime fields remain explicitly unsupported. A geometric cover candidate is
not promoted to subgroup transfer or solver evidence.

The command rejects a search whose parameter count exceeds `max_attempts`;
it never returns a silently truncated prefix. Incompatible families and
invalid parameter points remain in `attempts` with structured dispositions.

## Replay

Verification uses the saved request and supplied registry to regenerate the
complete result and require exact JSON-value equality:

```sh
curve-transfer-sweep verify \
  --input sweep.json \
  --registry docs/curves/registry.json \
  --output verification.json
```

A changed registry, coefficient, bound, attempt disposition, certificate, or
status makes replay fail. A matching replay reports the SHA-256 of the compact
JSON result value and its candidate count; it does not add an independent
arithmetic implementation or establish a computational advantage.

Exit code `0` is a successful sweep or replay. Invalid input, an unresolved or
ambiguous identity, an over-budget finite search, and replay disagreement use
exit code `3`. Output-write failure uses exit code `1`.
