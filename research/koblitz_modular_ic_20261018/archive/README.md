# First certificate encoding

The entry `archive/identity-n131-horner.json.gz` in `../evidence.tar.gz`
preserves the first successful large-target
certificate (IDC1hfd0bc625e8307959), before replacing the deeply nested
pretty-printed Horner expression with a balanced, compact encoding for
portable replay. It asserts the same polynomial identity. It is historical
issuance evidence; use `../identity-n131-balanced.json` for the final replay.

Uncompressed bytes: 92,391,155.
Uncompressed SHA-256:
`7705ab1aebdd90ffff921dfa58cb7e1101587f53a0bec47c4d2be88e366f4e14`.
Compressed SHA-256:
`60f522899e1fe7ef30a56d3081c4f210eff3211cea2f094e11d6a8522805dec1`.

```sh
tar -xzf ../evidence.tar.gz -C /tmp archive/identity-n131-horner.json.gz
gzip -dc /tmp/archive/identity-n131-horner.json.gz > /tmp/koblitz-original-identity.json
sha256sum /tmp/koblitz-original-identity.json
```

The old JSON's size was indentation overhead, not a large computation.
Its initial command and successful issuer/extra-point check are retained
in `../large-identity.log`. No failed experiment or certificate was deleted.
