# Constructor attribution before wider-field search changes

2026-10-25; freeze source and this protocol before output. The u32/u128
type port compiles, but its inherited quartic constructor still scans
all c3=0 candidates before c3=1. Its own source notes that the affine
linearized family can be entirely reducible. Do not infer an observed
cost at m=16--18 from that source inspection alone.

Use examples/ic_quartic_core_probe.rs: native `both 6` checks the old/new
field model and 4,096 base-field products; native `wide 6` is the fixed
constructor-only profile workload. Record frozen binary/source hashes and
use tools/profile.sh cpu --mode callgrind on the existing shared host.
Retain all raw instructions, tool/host manifests, output, statuses and
hotspot graph/report. It is an instruction attribution diagnostic at L0,
not native timing, hardware throughput or fastest-constructor evidence.

Before changing search order or excluding candidates, establish the exact
algebraic exclusion and check it on complete small fields. Then preserve
this baseline and repeat the same input with model/output parity; any
instruction change remains separate from full IC performance. Original
narrow backend and registered method identities remain frozen separately.
