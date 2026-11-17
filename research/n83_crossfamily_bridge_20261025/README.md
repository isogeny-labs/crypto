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

`nominate.py` freezes the candidate rule before either panel result is
available: require complete admission and a statistically selected online
and cold candidate within each family, then compare the deduplicated union
of those selections and fixed current-source K2304, K4300, and K4405
controls (five to seven exact candidate IDs) on these new points. It recomputes the
nearby-panel selection from its individual rows and the wide-panel selection
from its validated rows. An inconclusive panel leaves nomination unknown.
The raw panel archives must be independently replayed before their aggregate
JSON is passed to this nominator; the nominator's output is not a bridge
dispatch or a timing result.

## Adaptive nomination after the nearby confirmation

The independently replayed 16-point nearby confirmation admitted all seven
candidate bases but selected no unique online or cold winner under its frozen
paired sign-flip rule. `nominate_v2.py` is a separate, versioned response to
that result. Its rule was written after the nearby confirmation and before any
timing on the sealed bridge points. It leaves `nominate.py` and its original
selection requirement intact.

The v2 rule recomputes both panel selectors from admitted rows. From the
nearby panel it carries the three lowest descriptive online scores, the three
lowest descriptive cold scores, and fixed K1107. From the current-source
K3300/K3450/K3500/K3596 panel it carries a uniquely selected candidate when
one exists; when a metric remains unresolved, it carries the two lowest
descriptive means for that metric. It adds the fixed K2304, K4300, and K4405
controls. All candidate manifest hashes and index policies are included in
the output, with at most fourteen exact candidates. A failed admission,
source mismatch, changed aggregate, or missing manifest leaves the nomination
`BOUNDED_UNKNOWN`.

The adaptive shortlist is a dispatch input, not an estimate of bridge
performance. The 16 bridge targets remain unused; the capacity, resource
envelope, host allocation, ordering, and final paired analysis still need a
separate sealed dispatch before a Tier 1 comparison. The high-K panel must
first complete and independently replay.

The K2304 control has 382,464 usable points and 2,304 folded columns. Its
[current-source prefix receipt](../n83_k2304_current_source_prefix_20261025/README.md)
replays the exact historical base from the current K3596 dump. The earlier
five-finalist comparison left K2304 and K3300 close in online time on each
host, so a bridge omitting K2304 would leave that candidate unresolved.

The [K4300/K4405 prefix replay](../n83_k4300_k4405_current_source_prefix_20261025/README.md)
pins 713,800 and 731,230 usable points, respectively, to the independently
replayed current-source K5500 dump. The earlier 192-point online screen put
both near the large-base frontier without selecting a unique winner. They
remain fixed contenders in the fresh bridge, subject to its memory and
runtime caps. Neither has a current-source bridge timing row yet.

The admitted arms share the same IC producer source but use different root
index policies: nearby-grid arms use dynamic five-byte slots at 75% load
without a Bloom filter; K3300/K3450 use fixed five-byte slots with the wide3
Bloom profile; K3500/K3596 use fixed four-byte slots with the standard Bloom
profile; K2304 uses dynamic five-byte slots at 75% load without a Bloom
filter; K4300 and K4405 use fixed five-byte slots and wide3 Bloom filters.
The bridge therefore compares complete candidate pipelines while
factor-base size and index policy both vary. A claim about the factor-base
effect alone needs a matched-index comparison on the same points and hosts.

Regenerate and check the committed artifacts from the repository root:

```sh
python3 -m research.n83_crossfamily_bridge_20261025.generate_fixture
```
