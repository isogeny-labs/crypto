# perfbench evidence: null-136x5

* base      `?` (binary `e0ffb2ebc0458014`)
* candidate `?` (binary `f518c68934d37e03`)
* threads 1, 5 planned rounds, A/A on, seed 20260926, schedule `a74b35928acc5791`
* performance index: 1.005x; qualified: false; valid: true

Warnings recorded with the run:
* 1 verdict(s) are not corroborated by the best-of ratio (marked †): ['fp_gb/f4_solve_quart_n3_p31']
* 38 kernel(s) had contended rounds (left out of their medians)
* 1 kernel(s) have no A/A spread (fewer than 3 A/A pairs)

Check that the numbers follow from the raw rows (this re-times nothing):

```bash
sha256sum -c SHA256SUMS
cargo run --release --example perfindex -- replay .
```

`raw.jsonl` holds every harness row exactly as the harness printed it, with the machine's
load and pressure sampled at each round's start and end.  Wall-time rows are evidence only
when uncontended (AGENTS.md section 10); `compare.json` says how many rounds were left out.
These are host-specific observations, not a claim about another host class.
