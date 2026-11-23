# Public interval-width plan

T-1884 adds an explicit optional constructor selecting min(ceil(sqrt(2m)),block cap) before first use. The stream identity remains (r,m,cap); doubled streams borrow the planned engine. The recovery wrapper chooses the policy only from public eligibility and the prescribed opposite-radius schedule. See DERIVATION.md for arithmetic, scope and accounting; RESULTS.md for raw and summarized controls. PCC-20261107-fb6d9b is an unsigned producer attestation. [pcc:unverified]

Replay into a fresh output directory:

```sh
python3 validate-recovery.py /tmp/planned-width-replay
```

Rust performs arithmetic; Python compiles, captures output and renders reports. The validator uses optimized Rust with overflow checks, native timeout 60 seconds and dependency hashes. generation.json pins predecessor sources; publication.json pins the delivered bundle. No cloud or timed workload.

The scalar-stream controls use (r,m)=(281,35),(1009,63),(2017,63),(2521,315), five block caps and the same e=0,1,2 streams in all arms. Snapshots after the first m stream are explicit no-rescue controls. Public E/F_257: y^2=x^3+x+7, P=(1,3), r=281 checks all targets at m=5 caps [8,10,16,20,32,64] and m=35 caps [64,128]. Radii [1,280,2,3], extra offsets 0..16 and call cap 128 are fixed. The original 4563-case cohort is unchanged.

validation01 passes on its first attempt. Historical full-Cargo fixture/deadline and ledger conflict remain as recorded. The width policy remains optional: batches, convolutions and query-tree calls have different costs, so these counts do not establish a runtime advantage.
