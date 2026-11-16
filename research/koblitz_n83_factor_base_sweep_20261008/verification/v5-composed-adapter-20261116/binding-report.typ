#set page(paper: "a4", margin: 18mm)
#set text(font: "Helvetica", size: 10pt)
#set par(justify: true)

= Retained K600 V5 composed-adapter admission

*Result:* the exact read-only binding check passed at source
`cdc078c51511d97a1320d11a9be210cd816064ee`. The native receipt reports
`PASS_EXACT_ADAPTER_BINDING` for ordinal `26960859236`. This admits one
implementation route. It does not measure relation yield, rank, target work,
or a cold index-calculus runtime.

The run used shared Darwin arm64 and Rust 1.99.1. Its endpoints were
2026-11-16 13:14:43 and 13:14:57 UTC; the one-second-resolution interval is
14 seconds and the exit was 0. The interval includes warm compilation and
admission overhead. It is not an IC cost comparison.

#image("binding-flow.svg", width: 100%)

The V5 decoder checked the pinned V2, V3, and V4 parent bytes and the saved V5
design. The exporter matched the primary `a=0` K600 `public_x_hash` base,
seed `2026100801`, mask 18, six summands, zero split bits, SAT, binary
enumeration, zero large primes, one thread, and modular BigUint linear
algebra. It matched the existing astronomy-store compressed object SHA-256
`3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf`
(3,025,798 bytes). The exporter example suite passed 30/30 and the V5
example suite passed 2/2; the adjacent ordinal was rejected.

*Evidence:* `PROTOCOL.md`, `run-binding.sh`, `binding.stdout`,
`binding.stderr`, `binding.start.utc`, `binding.end.utc`, `binding.exit.txt`,
and `binding.source-commit.txt` in this directory. Standard output was 960
bytes, SHA-256
`53bc33c93ea59c4bef28199135a3162077101736fb2a6eab24a7a6f23c08baa0`.
Standard error was 13,477 bytes, SHA-256
`a0377145f394b82d4df5f3eff5ea75588851c0887922a5ad853498301cff0a98`.

The check made no S3 request and did not invoke the solver. A cold run needs
its own frozen protocol and host admission. The selected best total runtime
remains null. Canonical IC graphs were left unchanged because there is no
algorithm timing or complete comparison to plot.
