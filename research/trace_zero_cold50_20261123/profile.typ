#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 9pt)
= Remaining cold-query costs before the 50% attempt
Two identical fresh profiles of the frozen exact-half-trace incumbent, fixed ECC2K-130 d12 uniform input. Both execute34,987,635 simulated instructions and retain valid group-checked query results. [pcc:unverified]
#image("profiles/baseline-cold/hotspots.svg", width: 92%)
Inclusive input conversion costs10,410,863 instructions, including10,328,908 under from_hex. The full oracle stack costs18,088,853. Inclusive values include descendants and are not summed with these self-cost bars. This diagnostic chooses optimization sites; it is not a gain comparison. [pcc:unverified]
Sources: unchanged retained numerical/harness inventory; profile.protocol.json published before data; two complete framework manifests, raw Callgrind files and annotations. CPU0,2GiB; no retries or paid resources. Elapsed profiler time is not native-runtime evidence.

Next hypotheses: parse ordinary ASCII hex directly into field words while preserving the legacy syntax fallback; unroll the two/three-limb carry-less products using established Karatsuba identities. Preserve the original44,023,404Ir denominator for the comparative goal;50% is unmeasured.
