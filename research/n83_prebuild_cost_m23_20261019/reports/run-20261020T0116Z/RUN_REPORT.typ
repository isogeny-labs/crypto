#set page(margin: 18mm)
#set text(size: 9pt)
#set par(leading: 0.62em)
= XOR-cache v2 bounded work-counter run
Run ID: `xor-cache-v2-20261020T011634Z` · Complete bounded · 2026-10-20 02:00:10 UTC

Worker revision `1de47a9d4186bff08abc84c224a111183e820106`; driver revision `f1667c313910a1021daed455c9e48cd03524f42a`; policy `small-m4-xor-cache/v1`.

The existing authorized SSH card ran 16 fresh natural attempts across K2048/D12 and public targets 0/1. Each cell ran A1/B1/B2/A2. Every attempt reached 10,000 conflicts and ended `UNKNOWN_search_cap`; no natural relation returned. Four constructed controls and four independent general-field replays passed. The shared Spot host used CPU 31, one CPU quota, 4 GiB, zero swap, read-only inputs and no worker network (L0).

#image("work-counters.svg", width: 100%)

Each cell has two identical repetitions per setting. Decisions, conflicts, restarts, XOR passes, row operations and row scans match between cache off and on. Cache-on repivot visits plus skipped cached rows equal cache-off repivot visits. These are recorded work counters; wall times are not compared.

Natural yield/rank, complete IC cost and base selection remain unresolved. The card is censored by its conflict cap and does not establish a timing improvement.

Archive SHA-256: `cf3310c7cb597c144b30a427683ed41d37cf4e2ec2425945d259a1173e776abd`. Manifest SHA-256: `55a87e7e0ca6561c48d1ad558ec8b3d8ba385e4cb4c26a6b6188688732eea4ee`.
