#set page(paper: "a4", margin: 18mm)
#set text(font: "Helvetica", size: 10pt)
#set par(justify: true)

= N83 V5 retained K600 second binding admission

The revised source `804b1389c2fd624943a7cfc8b5034461f75c9ccf` returned
`PASS_EXACT_ADAPTER_BINDING` for V5 ordinal `26960859236`. The frozen
amendment added a one-thread environment gate to the future cold route. This
check ran with `RAYON_NUM_THREADS=1`. Its unit test passed for value `1` and
rejected absent and `4` values. The earlier binding receipt is preserved.

On shared Darwin arm64 with Rust 1.99.1, the check ran from
2026-11-16 13:37:11 to 13:37:26 UTC and exited 0. The endpoint interval is
15 seconds at one-second resolution; it includes warm compilation and is not
a cold index-calculus timing. The exporter suite passed 30/30 and the
standalone V5 suite passed 2/2.

#image("binding-flow.svg", width: 100%)

The decoded case is the primary K600 `public_x_hash` base, seed
`2026100801`, mask 18, six summands, zero split bits, SAT backend, binary
enumeration, zero large primes, one thread, and modular BigUint linear
algebra. The retained compressed object is 3,025,798 bytes, SHA-256
`3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf`.

The raw `stdout` is 960 bytes, SHA-256
`53bc33c93ea59c4bef28199135a3162077101736fb2a6eab24a7a6f23c08baa0`;
`stderr` is 13,477 bytes, SHA-256
`a701809280338f45ce76c3a5c4f4ed00d445a91abdcdbb3e671d38b07716a387`.
The check made no S3 request and did no solver work. Future host thread count,
cgroup, relation, rank, target, and cold total remain open. Canonical IC
graphs were left unchanged.
