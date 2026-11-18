# Local square-root startup on matched targets

The local construction reproduced every attempted reduced divisor, including nonsplit supports, under the translated-coordinate audit. Both two-order arms had identical startup admissions, accepted target rows, anchor counts and answers on all 44 targets. Every arm verified the scalar by original-curve multiplication. The complete run captured 176 counter snapshots and 2,112 timed solves.

| Genus | Fixtures | Original / local wall | Deferred without startup / local wall | Generic startup / local wall | Generic startup / local REDC |
|---|---:|---:|---:|---:|---:|
| 2 | 24 | 1.367–2.457× | 0.945–1.165× | 1.070–1.162× | 1.072–1.179× |
| 3 | 12 | 1.533–2.539× | 0.819–0.899× | 1.095–1.159× | 1.099–1.115× |
| 4 | 8 | 2.220–4.908× | 0.842–1.087× | 1.116–1.178× | 1.070–1.106× |

Using the local formula improved complete median wall time and total field reductions over constructing the same startup rows generically on all 44 targets. Against omitting startup it won median wall time on 18/24 genus-two, 0/12 genus-three and 3/8 genus-four targets. Combined field reductions against omitting startup fell on only 4/44 targets. The most favorable comparison against the original reference was 4.908×. The tenfold gate remains open. These are individual same-host medians with no confidence intervals.

RUN.txt identifies the remote freeze. CONTROL.txt retains raw counters and times and summarize.rb regenerates COMPARISON.tsv.
