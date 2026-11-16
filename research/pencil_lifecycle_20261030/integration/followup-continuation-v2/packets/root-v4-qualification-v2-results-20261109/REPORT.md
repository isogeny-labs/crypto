# Root v4 permit qualification v2: fixed software cases

One released invocation recorded 24 passing cases and exited 0. Native elapsed time was 0.011666838 seconds; caller elapsed 0.064985912 seconds. The process was reaped. Full 60-second charge brings cumulative campaign charges to 16380 seconds. The preceding failed fixture invocation remains recorded separately with its 60-second charge.

The qualification extracts the actual pinned worker read/check functions and launcher show/permit functions without importing their top-level launch code. Cases cover fresh/expired permits, one bounded reread, malformed/oversize/nonfinite expiry, identity changes, final freshness, interruption/deadline, publication ordering, lease margin, cleanup/terminal guards and ownership/cgroup changes. The heartbeat ordering case is a source-structure check. All 24 case names and results are preserved in cases.json and cases.csv.

Worker SHA256 b3de879c3d030edc33441300cbdfb5558b17fcc423bd894feadf8968c0928e95; launcher SHA256 32d752703c87cbdf369bd0799c7183d5aa3f0d6af21cc23c2336733bc916238a. These match the v4 reviewed source pins. Qualification source 3c60a3453ac39cafe69483c1c7eb1aa6b0c80f3ade8dbfc96fe66fff4edc8a44 was published at 9c30e081236d842a7ee90701e88e7a5a2a1da6ca before execution.

No compiler, native research payload, network or real Conductor call was started. These fixed software fixtures do not measure live scheduler, filesystem latency or daemon correctness. The cache snapshot was not launched as part of this invocation.

The five-file archive includes the native result and complete caller publication/terminal/stdout/stderr receipts. Raw hashes were checked before and after archival. Limits were 512MiB address space, 1 MiB per file, 50-second internal alarm and 55-second outer TERM plus 5-second kill allowance.
