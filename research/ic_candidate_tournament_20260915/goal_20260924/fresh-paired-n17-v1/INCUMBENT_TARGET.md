# Card-bound one-use incumbent arm (fresh paired n17)

The [four-arm freeze](four-arm-freeze-v1/RESULT.md) pins arm order
`rho → f5 → sat → incumbent` with
`dispatch_authorization=not_granted_freeze_only`. It does not authorize an
incumbent run. The [target-free incumbent source publication](INCUMBENT_SOURCE.md)
archives the pairinv worker under seal
`36923bdb0110841fef8c481b941f134c87076244ed20440684bae3d385c6e6bf` but
supplies no card-bound one-use controller. IC1 stays null until an exact
pre-card factor-base inventory is sealed separately.

## Commands (tip `icprog`, not the frozen F5 capsule)

```bash
icprog incumbent-target-freeze \
  --source-publication .../incumbent-source-v1/result-v1/publication \
  --source-descriptor .../incumbent-source-v1/result-v1/source-descriptor.json \
  --card .../public-card-v1/card.json \
  --four-arm-freeze .../four-arm-freeze-v1/freeze.json \
  --capsule /Volumes/SSD990-2/tmp/ic-incumbent-target-capsule-... \
  --out /Volumes/SSD990-2/tmp/ic-incumbent-target-freeze-....json

icprog incumbent-target-publish \
  --capsule ... --registration-sha256 <seal> \
  --publication /Volumes/SSD990-2/tmp/ic-incumbent-target-publication-... \
  --out /Volumes/SSD990-2/tmp/ic-incumbent-target-publish-....json

icprog incumbent-target-replay \
  --publication ... --registration-sha256 <seal> \
  --out /Volumes/SSD990-2/tmp/ic-incumbent-target-replay-....json
```

Freeze/publish/replay never launch the archived worker. Capsule status is
`INCUMBENT_CARD_REGISTERED_NOT_DISPATCHED`.

## Execute (separately authorized only)

```bash
icprog incumbent-target-execute \
  --capsule ... --publication ... --execution ... \
  --registration-sha256 <seal> \
  --dispatch-authorization separately_authorized_incumbent_arm_v1
```

`--dispatch-authorization not_granted_freeze_only` is refused. Do not invent
that token. There is no in-repo seal, ledger entry, or automated issuer for
`separately_authorized_incumbent_arm_v1`: only an external human operator may
grant it by supplying the CLI flag when intentionally authorizing the sole
consume. Do not redispatch a consumed registration. Use busy lock,
`TMPDIR=/Volumes/SSD990/tmp`, and outs on SSD990-2.

## Pins

| item | value |
| --- | --- |
| card | `bd969a4092a329bf093688b9d4715c86d77850c11f115ff47852c9235d7c2379` |
| target | `[87633, 27]` |
| incumbent source | `36923bdb0110841fef8c481b941f134c87076244ed20440684bae3d385c6e6bf` |
| worker | `b3d8afacc746a34cf15562452dbec3d75deadbb9faf16051e97ad8fa07cbbd66` |
| freeze.json | `a94df27cd3ed73974787e4cf87d755bbb062f5431645704a61b8c4d45f761aa2` |
| method | pair-table, 3 summands, tiny_gauss, subgroup_orbits points=102 seed=43 |
