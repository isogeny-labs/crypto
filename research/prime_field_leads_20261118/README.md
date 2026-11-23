# Prime-field lead experiments

This round follows [PR #4624](https://github.com/isogeny-labs/crypto/pull/4624) and uses base commit `9ec88b7637aac4ce816739155f12f518b393da37`. Each lane has a bounded protocol, a native runner and a replay check.

| Lead | Result | Scope |
| --- | --- | --- |
| [APS label weights](coverage/README.md) | All 12,416 fixed-slot sign patterns checked had nonzero denominators. The compact law certified fewer unrestricted tuples in the mixed-slot audit. | Two public prime-order toy curves, coupled draw streams, the repository's sampler and recovery bodies, and complete Split4 equation evaluation. |
| [Split4 support reuse](split4/RESULT.md) | At B = 256, the packed x-bucket layout used 589,828 bytes for the left table and took 406.4 ms median per 24-target pass. The x hash used 3.19 MB and took 598.4 ms. | Local count-only layout comparison. The retained support is quadratic and does not carry witness indices. |
| [RR9 triangular elimination](rr9/README.md) | On two fixed negatives, the reduced certificate used 2,691 multiplier terms instead of 20,672. Median certificate time was 159 ms versus 342 ms for the full system. | All exact identities and reconstructed positives replayed. The basis-only timings overlapped. |

The results suggest different next tests. APS needs a workload that preserves one point per slot if it uses the compact law. Split4 needs a witness-preserving comparison to the existing Rust buckets before a production change. RR9 needs a wider frozen residual panel before a scaling claim. The [scaling note](SCALING.md) gives a coverage bound for fixed explicit support and records the size threshold of the separate tensor-contraction schedule.

These are local research checks. Timing is descriptive. No production solver or benchmark ledger changed. [pcc:unverified]
