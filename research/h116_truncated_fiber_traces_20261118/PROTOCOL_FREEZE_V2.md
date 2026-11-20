# H116 protocol v2 freeze receipt

Frozen: 2026-11-18T01:19:53Z, after structural feasibility and before cohort v2
emission, candidate/reference moment construction, predicate scoring or timing.

Draft commit:

```text
189053138811e03e2fae58cb1d8ad414c89b0cab
```

Canonical SHA-256 is defined by the v1 experiment-protocol schema as sorted
compact JSON with the entire `freeze` block removed:

```text
jq -cS 'del(.freeze)' protocol.json | sha256sum
42b510bd0f67d23ad49e5827d947510de5e1c68bf82d29c78fa35e4861fe4bf3  -
```

Schema validation:

```text
python3 -m jsonschema -i protocol.json \
  ../cm-marked-scalar/protocol/experiment-protocol.v1.schema.json
exit 0
```

This receipt binds the sole cap amendment selected by the committed feasibility
ladder. It does not substitute for cohort verification, an instrument binding,
independent verifier, checkpoint, EKS receipt or scientific result.
