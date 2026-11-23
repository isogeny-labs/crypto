# H116 protocol freeze receipt

Frozen: 2026-11-18T00:59:36Z, before structural cohort selection, candidate
moment construction, predicate scoring or timing.

Draft commit:

```text
0dc2972ca1cf278106a191a2d2ffc667214a96a2
```

Canonical SHA-256 is defined by the v1 schema as sorted compact JSON with the
entire `freeze` block removed:

```text
jq -cS 'del(.freeze)' protocol.json | sha256sum
e1638152e26e31c5274828b9ed268d55c1cf2a7a489d0c98e43f69823539803b  -
```

Validation:

```text
python3 -m jsonschema -i protocol.json \
  ../cm-marked-scalar/protocol/experiment-protocol.v1.schema.json
exit 0
```

The canonical upstream experiment-protocol executable was inaccessible because
the sibling checkout exposed it as `VLT1` encrypted data. The tracked schema in
this repository was used directly. This receipt does not substitute for an
instrument binding, independent verifier, checkpoint, EKS receipt or result.
