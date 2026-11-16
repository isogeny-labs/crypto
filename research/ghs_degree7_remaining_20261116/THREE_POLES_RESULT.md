# Three quadratic branch poles: fixed trace +22 carrier

The frozen source at `652290a2d155` completed the final branch-only
denominator class `B=Q_iQ_jQ_k` for both trace-`+22` source representatives.

| target mask | denominators | assignments | logical candidates | point-filter survivors | coprime and divisible | exact squares | bitset controls | CPU s |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 68 | 10 | 90 | 643,043,610 | 14,151 | 14,149 | 0 | 1,440 | 22.407 |
| 84 | 10 | 90 | 643,043,610 | 13,980 | 13,980 | 0 | 1,440 | 19.561 |

The [b=68](three_poles_68.json) and [b=84](three_poles_84.json)
receipts bind to source SHA256
`d91a18684c229c3056ff8ec7f5c2e19388fdf73f21ca134c7cf7570d15e19391`.
The bitsets represent every value of the third numerator parameter; they do
not sample it. All survivors received exact quotient and square tests.
No full identity occurred in this denominator class on the fixed carrier.

This does not close degree seven by itself: the prior sparse and repeated
linear-square shapes have not yet been run on trace `+22`. The single Sage
implementation and its direct-evaluation bitset controls are local evidence,
not an independent algebra-system replay. [pcc:unverified]
