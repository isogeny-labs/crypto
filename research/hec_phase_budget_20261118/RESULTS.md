# Arithmetic cost of the integrated collector by phase

All 48 answers verified. Every sixteen-entry counter vector for both fields and every stopping anchor matched the earlier uninstrumented arm. BUDGET.tsv contains totals per phase and fixture; percentages below divide the sum of a phase's REDCs by the sum of all REDCs for that genus. Large jobs consequently receive more weight.

| Genus | Largest aggregate costs | Largest phase by fixture | Minimum number of whole phases to erase to reach the original tenfold REDC budget |
|---|---|---|---|
| 2 | Slope scan 57.8%, linear insertion 15.6%, walk setup 8.4%, eligibility 8.0% | Scan 12/16, eligibility 4/16 | Three on 8 targets, four on 8 |
| 3 | Eligibility 25.5%, scan 22.6%, row assembly 21.2%, linear insertion 17.2% | Eligibility 9/16, scan 6/16, insertion 1/16 | Three on 9 targets, four on 7 |
| 4 | Eligibility 41.5%, scan 29.0%, linear insertion 10.7% | Eligibility 16/16 | One on 6 targets, two on 10 |

For genus two the scan deserves attention ahead of further singleton-row bookkeeping. Row assembly is 4.2% of aggregate counted arithmetic after deferral. Even jointly erasing the two most expensive phases on each genus-two or genus-three fixture would leave the original tenfold arithmetic budget unmet. A collector that reduces the number of anchor visits can affect scan, row construction and insertion together, and has a different cost case from merely speeding up one loop. Eligibility and its shared table represent a separate fixed per-input cost. For genus four, eliminating that phase alone would suffice in this optimistic counting model on only six of sixteen inputs.

These are exact counts under the stated equal-REDC convention; they do not price memory traffic, polynomial allocation, additions, native division, cache or scheduling. The phase-erasure calculation is a diagnostic, not an algorithm or CPU-time lower bound. No solver speedup is attributed to this run.

RUN.txt identifies the prospective commit. CONTROL.txt contains the per-phase, per-modulus vectors. summarize.rb checks exact equality against the earlier record and regenerates BUDGET.tsv.
