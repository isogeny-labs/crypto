# Bounded curve-transfer sweeps

`curve-transfer-sweep` enumerates the explicit same-field cover shapes that
the repository implements for one exact elliptic model. It is a construction
and evidence-inventory tool, not a discrete-log solver.

The request schema supports two integrated families:

- `binary_cubic_pullback_v1`, the fixed genus-three degree-three cover in
  characteristic two;
- `prime_quadratic_pullback_v1`, the genus-two degree-two cover
  `x = u^2 + c`, swept over an explicitly bounded interval of prime-subfield
  shifts `c`. Geometry construction also retains a target-model normalization
  when the catalog supplies one, but the induced-map backend currently accepts
  only direct short-Weierstrass targets.

It does not search arbitrary rational functions, unimplemented Xarles
families, isogenies, correspondences, non-hyperelliptic curves, or all curves
of a genus. “Exhaustive” in an output means only every parameter point in the
declared finite family/shift bounds was checked. Zero candidates means no
candidate passed the family checker in that search space; it is not evidence
that no cover exists.

## Capability inventory

The machine-readable inventory distinguishes integrated sweep requests from
other bounded constructors and searches in the repository:

```sh
curve-transfer-sweep capabilities --output capabilities.json
```

The `curve-transfer-sweep-capabilities/v1` result contains six rows. Only
`binary_cubic_pullback_v1` and `prime_quadratic_pullback_v1` are accepted by
the `sweep` request schema. The other four rows are routing records:

- the bounded genus-two bielliptic atlas and chained-transfer backend;
- the selected genus-three hyperelliptic constructor;
- the genus-three non-hyperelliptic plane-quartic diagnostic; and
- the bounded small-`F_2` characteristic-polynomial factor search.

The quartic row explicitly reports an elliptic norm projection without a full
quartic Jacobian group. The small-`F_2` row is a candidate filter and constructs
no induced map. Neither is promoted to an implemented transfer or solver. The
inventory also records that an empty bounded search is not a nonexistence
proof.

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
proved prime fields in the complete `u64` domain and below the strict
Sorenson--Webster `psi_13` bound, but this sweeper does not replay group
operations. For direct short-Weierstrass and Montgomery-normalized candidates
in that boundary it now instantiates the runtime transfer object and compares
its source sextic with the family certificate; arbitrary-class operations and
composition identities remain implemented-but-not-replayed by the sweep. The
Montgomery path uses an everywhere-defined projective isomorphism and never
reinterprets original target coordinates as short-Weierstrass coordinates.
Odd extension fields, Edwards exceptional-point charts, and primes without a
deterministic proof remain explicitly unsupported by the induced-map backend.
A geometric cover candidate is not promoted to subgroup transfer or solver
evidence. For a Montgomery target outside the proof boundary, the primary
disposition is `unsupported_without_primality_certificate`; an Edwards target
retains `unsupported_normalized_target_transfer_backend`. Source balanced
arithmetic alone never promotes either case.

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

Catalog-wide odd-map disposition is recorded separately in the canonical
`odd-transfer-catalog` corpus, not inferred from a sweep result. That tracked
17,879,352-byte artifact (SHA-256
`7c60fc122c73abbe7b0df1413d24abbb064a9842bb35c2da8248d30c87f65670`)
retains all 22,243 odd maps and 940 named bindings, including its 675 supported
records and every typed unsupported or failed record. Its source-bound replay
does not turn this bounded family sweeper into a complete cover search,
independent Jacobian oracle, relation solver, or advantage result.
[pcc:unverified]

Exit code `0` is a successful sweep or replay. Invalid input, an unresolved or
ambiguous identity, an over-budget finite search, and replay disagreement use
exit code `3`. Output-write failure uses exit code `1`.
