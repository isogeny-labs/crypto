# Producer validation and limits

PCC-20261115-30dcbf pins raw outputs, check log and source digest list. Standing
is unverified: no independent signer, compiled Lean proof, isolated execution
receipt or performance certificate is supplied. [pcc:unverified]

`check.sh` passed formatting, four release tests, strict package Clippy and
byte-for-byte recomputation of toy.json and trace.json. Public fixtures have
subgroup orders 109, 823 and 4111 in fields 1009, 4093 and 16381 respectively.
Six scalars (0,1,2,17,37,43) and three seeds (7,19,20261115) give 18 paired
cases per field. Zero is an explicit edge case. [pcc:unverified]

The source and mixed destination recover/replay every expected scalar. Tests
cover corrupted representative multiplier, invalid source point, zero collision
denominator and zero search budget; the fixed panel exercises the destination
escape path. RustCrypto P-256 coordinates match the separate BigUint backend
on four fixed scalars. The actual-field trace checks 32 transitions per arm,
source scalar replay and destination image replay at every transition.
[pcc:unverified]

Trust limits: the Cantor/image routines are reused from this producer's earlier
implementation. No independent geometric proof is implied. The actual P-256
trace uses a public planted target 37G, not an unknown full-size logarithm.
Caller preconditions are prime subgroup order and G,Q in the stated common
cyclic subgroup; the public fixtures enforce these by construction. The API
is research code, not a parser or validator for arbitrary hostile Jacobian input.

The 16-state window detects short cycles; longer cycles can exhaust the
explicit budget. Escapes try at most 64 doublings then 32 independent jumps;
failed escape permits at most eight restarts. The 20,000 cap counts ordinary
walk additions; escape and setup additions are separate bounded counters.
This is not a proof of random-walk mixing or successful recovery for every seed.

Counters are development events, not a complete arithmetic price. Scalar
multiplications, inversion, coefficient arithmetic, hashing, allocations and
canonicalization costs are not interchangeable group additions. No step-count
ratio here is promoted to a cost or timing comparison.

Root-workspace checks are outside this standalone package; the inherited
sparse workspace is missing members. Dependencies compile through this package.
No prior pinned source or evidence file was modified.

The RunPod package was rebuilt from its extracted source archive and reproduced
the frozen toy output. The initial build failed because the archive omitted a
compile-time historical JSON fixture; both failed and amended build logs are
retained. `payload-sources.sha256` lists the full Rust/manifests/compile-time
fixture closure. This packaging replay is a producer check. [pcc:unverified]
