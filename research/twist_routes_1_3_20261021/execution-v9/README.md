# Executed: exact target-identifiability diagnostic, with failed GP attempt

[Report/graph](../reports/relation-rank-20261027/RESULTS.md) ·
[PDF](../reports/relation-rank-20261027/RESULTS.pdf) · [admission](ADMISSION.json).
All18 unchanged v8 matrices are checked:9 ordinary systems fix the target log;
9 twist-progression systems leave it ambiguous. RREF's full nullspace and
particular solution are retained. No factor-base logs, fixture scalar or new
point queries enter elimination. Same-base transport preserves each outcome.

The original frozen native run passes. The original GP attempt fails strict
verification despite process0/final marker: p was initialized inside a
function definition. Its stderr/receipt and faithful failed-row export remain.
Before the affected independent replay, [explicit amendment](PARI_AMENDMENT_V2.md),
[preload](PRELOAD_V2.gp), [new protocol](protocol-v2.json) and
[main/preload pins](PARI_V2_FREEZE.json) were committed/checkpointed. The second
GP run checks identical matrices with empty stderr and exact sentinel. Original
native output and main GP script are unchanged.

[Original export](experiment.original.export.json) and
[corrected-reference export](experiment.export.json) each bind to their
respective frozen protocol/files with zero findings. The same actual native
attempt appears in both; do not pool them as independent runs. Three actual
processes total:1native and2GP, first GP verificationfailed. Process completion
does not mean verified mathematical output. A rejected initial failed-row
serializer and a failed separate GP vector/matrix control are preserved.

Sourcea3da15bda17804c75b48bbe487e48cb4ae5cb560; native/input pins in
[FROZEN.json](FROZEN.json). Same L0 host as v8,CPU0,one thread,4GiB;
300s cap per stage, no admitted time/performance. The earlier60s prose was
corrected before execution to match the structured300s cap. No paid host,
NIST result, extra independent relation information or solving speedup.

Hydrate with `python3 tools/artifacts/artifacts.py pull
research/twist_routes_1_3_20261021/execution-v9`, check
`sha256sum -c runs.tar.xz.sha256` inside the directory, then `tar -xJf runs.tar.xz`.
The deterministic2,712-byte archive has11 files and SHA256
80052a20f9d52aa5bad20dc1f1295bd878f76b24c290f7bcee78310a96fdde34.
[Fresh remote member/repack verification](RAW_VERIFICATION.json) covers every
file, including both GP attempts. Older runs/protocols are not rewritten.

Rebuild `twist-relation-rank` at its frozen source using the standalone runner
manifest/Rust1.90. Call `twist-relation-rank V8_RESULTS FRESH_OUT`; compare the
generated RESULTS.json/verify.gp to the archive. For the valid reference,
`twist-pari-transport SCRIPT FRESH_RECEIPTS PRELOAD_V2.gp` uses the pinned local
GP layout. `twist-kernel-publication admit` strictly checks the actual successful
receipt and hashes; its source records publication, without changing the
frozen analysis. Native linear controls and independent GP controls cover both
deficient-but-identifiable and deficient-and-ambiguous cases.

Future experiments: alter a target-blind base under a new protocol to test
the rank defect, or price a verified quotient's full solving cost. Those are
different questions and neither is run here. All original campaign gates stay open.
