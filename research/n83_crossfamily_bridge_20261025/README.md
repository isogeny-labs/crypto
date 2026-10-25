# N83 cross-family bridge fixture

This fixture seals 16 new public points on
`EC1N83Ckb1a1haacb69ee9f37` before the nearby-grid 16-point
confirmation or the K3300/K3450/K3500/K3596 current-source panel has
selected a factor base. Each point has a separate canonical one-target
workload ID and a scalar sidecar for independent replay outside the timed
interval. The generator verifies subgroup membership and checks the points
against the historical public-point inventory, the nearby-grid screen,
the nearby-grid confirmation, and the high-K panel.

These points are reserved for a direct, same-point Tier 1 comparison of the
admitted small-base and wide-base selections. Candidate IDs, hosts, source
hashes, execution limits, order rotation, and the statistical rule must be
frozen in a separate bridge dispatch before any timing begins. This fixture
contains no comparison timings or selected factor base.

Regenerate and check the committed artifacts from the repository root:

```sh
python3 -m research.n83_crossfamily_bridge_20261025.generate_fixture
```
