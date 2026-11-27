# Amendment 1: bind the archived m=7 curve identity

Date frozen: 2026-11-27 UTC. Draft PR: #5267. Standing:
`[pcc:unverified]`.

This amendment is frozen after a development unit assertion detected a curve
cardinality mismatch and before any source-custody commit, checkpointed
census, target classification, or result row.

## Correction

Replace only the `Fixed public source` curve block in `PROTOCOL.md`. The
authoritative archived m=7 manifest is
`research/norm_one_torus_m7_single_target_20261126/public-precommit/deferred-y-manifest.json`.
It pins registered curve `icv1-fp31-tm3989-ae9db972`:

```text
p = 2147483647
E: y^2 = x^3 + 2147483644 x + 21
r = 2147487637, cofactor 1
G = (1391443431, 177979044)
```

The matching registry source is
`docs/curves/sources/fp-p31-subexp-20261121.json`. The archived certificate
requires 34 lifting abscissae and 68 signed points at Dickson depth six.

The original block named the different curve
`icv1-fp31-t36817-76528ee1`, copied from the oriented-accounting control. Its
depth-six fibre produced 33 lifting abscissae in the development assertion and
therefore cannot replay the archived m=7 boundary.

## Unchanged protocol

All Dickson parameters (`depth=1..6`, `lo=0`, `c_top=0`), branch convention,
signed lift mask, semantic transducer, target rules, state/rank/circuit fields,
counted-work boundary, independent replay, subquadratic gate, 1.6-million cold
ceiling, and no-cross-target rule remain unchanged. No target or result was
available when this correction was frozen. [pcc:unverified]
