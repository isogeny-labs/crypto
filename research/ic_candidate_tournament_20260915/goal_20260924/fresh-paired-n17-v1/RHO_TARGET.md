# Card-bound one-use rho arm (fresh paired n17)

The [four-arm freeze](four-arm-freeze-v1/RESULT.md) pins arm order
`rho → f5 → sat → incumbent` with
`dispatch_authorization=not_granted_freeze_only`. It does not authorize a
rho run. The [target-free rho source publication](RHO_SOURCE.md) archives
`ecbench` under seal
`f74601e7be2ef1c7141fee5a997b38c090e0f19eadbe0d7e843577b59918cf59` but
supplies no card-bound one-use controller.

## Commands (tip `icprog`, not the frozen F5 capsule)

```bash
icprog rho-target-freeze \
  --source-publication .../rho-source-v1/result-v1/publication \
  --source-descriptor .../rho-source-v1/result-v1/source-descriptor.json \
  --card .../public-card-v1/card.json \
  --four-arm-freeze .../four-arm-freeze-v1/freeze.json \
  --capsule /Volumes/SSD990-2/tmp/ic-rho-target-capsule-... \
  --out /Volumes/SSD990-2/tmp/ic-rho-target-freeze-....json

icprog rho-target-publish \
  --capsule ... --registration-sha256 <seal> \
  --publication /Volumes/SSD990-2/tmp/ic-rho-target-publication-... \
  --out /Volumes/SSD990-2/tmp/ic-rho-target-publish-....json

icprog rho-target-replay \
  --publication ... --registration-sha256 <seal> \
  --out /Volumes/SSD990-2/tmp/ic-rho-target-replay-....json
```

Freeze/publish/replay never launch `ecbench`. Capsule status is
`RHO_CARD_REGISTERED_NOT_DISPATCHED`.

## Execute (separately authorized only)

```bash
icprog rho-target-execute \
  --capsule ... --publication ... --execution ... \
  --registration-sha256 <seal> \
  --dispatch-authorization separately_authorized_rho_arm_v1
```

`--dispatch-authorization not_granted_freeze_only` is refused. Do not invent
that token. There is no in-repo seal, ledger entry, or automated issuer for
`separately_authorized_rho_arm_v1`: only an external human operator may grant
it by supplying the CLI flag when intentionally authorizing the sole consume.
Do not redispatch a consumed registration. Use busy lock,
`TMPDIR=/Volumes/SSD990/tmp`, and outs on SSD990-2.

## Pins

| item | value |
| --- | --- |
| card | `bd969a4092a329bf093688b9d4715c86d77850c11f115ff47852c9235d7c2379` |
| target | `[87633, 27]` |
| rho source | `f74601e7be2ef1c7141fee5a997b38c090e0f19eadbe0d7e843577b59918cf59` |
| ecbench | `60fa3f7a042de090faabffcc09af28a59970c6d112320564b0f0621ecd413d47` |
| freeze.json | `a94df27cd3ed73974787e4cf87d755bbb062f5431645704a61b8c4d45f761aa2` |
| method | `rho.signed_frobenius_strong`, 32 lanes, `dp_bits=4`, `step_cap_factor=2000` |
