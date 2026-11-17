# Shared interval engine for conditional rescue

T-1880 adds an explicit, separately keyed doubled-stride wrapper around one Rc-owned interval-product engine. shared-rescue.rs checks field/stride/cap compatibility; shared-rescue-phase.rs and shared-rescue-adapter.rs use it in a separate recovery path. Original fresh-engine recovery remains available as the reference. See DERIVATION.md for cache identity, ownership and costs; RESULTS.md for three-arm controls and retained misses. PCC-20261107-2184b1 is an unsigned producer attestation. [pcc:unverified]

Replay into a fresh directory:

```sh
python3 validate-recovery.py /tmp/shared-rescue-replay
```

The validator compiles optimized Rust with overflow checks, captures compiler/stdout/stderr, hashes native dependencies and limits native execution to 60 seconds. Python performs orchestration/reporting only. generation.json pins the predecessor sources; publication.json records hashes and byte counts. The binary is excluded from Git and rebuilt from pinned source/compiler.

Scalar-stream controls use (r,m)=(281,35),(1009,63),(2017,63),(2521,315), block caps 1,2,7,16,64 and all three coefficient modes. Public controls retain E/F_257: y^2=x^3+x+7, P=(1,3), r=281, m=5, all 281 scalar fixtures, radii [1,280,2,3], degree caps [8,10,16,20,32,64], extra offsets 0..16 and call cap 128. Original 4563-case cohort unchanged.

validation01 passed on its first attempt. No cloud or timed workload. Historical full-Cargo fixture/deadline and ledger divergence remain as documented. This is an explicit optional adapter; setup reductions do not by themselves justify making it the default.
