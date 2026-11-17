# Pre-data protocol amendment v3

The original v2 run was stopped on its first replay failure, cell 16. Its full 17-cell tree is retained unchanged and its verified 16-cell prefix is reported separately in `RUN-v2-failure.md`. No replacement cell was generated before this amendment.

`protocol-v3.json` is dated 2026-11-01T20:16:49Z and amends v2 file SHA-256 `0a51be43a049a6e7b8b046112150112a3ebd3b5c8d1a98e8a992b513b56267dc`. Its canonical body SHA-256 is `3e0d09a8f47442b908aea4d5e686eca1508ea1e7637874e72527c169ad404661`; file SHA-256 is `1dcdc25f99c444c7a695dfc856437d22c45c6b97345629df93968d1089044490`.

Source commit `1c869a18dc60d207393e50b4fda6f2c7bd399e35` gives m5..15 native event persistence half of the same bounded post-setup algebra loan and fails closed on any group event persistence failure. The m4 loan remains unchanged. `OBSERVER-BUDGET-v3.md` records the observed cap arithmetic and m15 initial callback bound. The 44-cell n47/n61 × m5..15 × both-twist matrix, seed, 50,000 total requested work/cell, 30 seconds/cell, 2048 MiB memory and 64 MiB trace remain unchanged. There is no comparative or statistical claim; `partial` retains null FFD/LFD labels.

A separate `plan-v3.json` will bind this amendment after this file is committed. The new output path is `/home/ec2-user/ecdlp/wide-prime-m5-15-smoke-v3-20261101`; it must be absent before the one replacement invocation. Neither the v1 nor v2 output paths are to be overwritten or retried.
