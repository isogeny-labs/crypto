#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Joint neighboring-mode coefficients
T-1874 · 2026-11-07 · Prime-field compressed recovery

F2 coefficients are now derived from the existing F1 stream through the adjacent signed-binomial ratio. A bounded batch inverse supplies the denominators. Both modes are evaluated together across the surviving phase branches.

#image("flow.svg",width:100%)

*Alignment.* For m≥3 dividing r−1, h mod m is0or m/2, never1. F1 and F2 therefore have equal lengths. At paired positions N=jm+1, N+1≤h is strictly below r, so every denominator is nonzero. Singleton and top-h endpoints are included.

#table(columns:(3fr,1fr),inset:4pt,
 [Native control],[Observed],
 [Sequences / coefficient-pair equalities],[460 / 6,420],
 [Singleton / maximal-order / top-h sequences],[100 / 50 / 5],
 [Branch-mode / canonical-state equalities],[300 / 2,516],
 [Removed second-stream transitions, cap64 / cap16],[7,425 / 14,445],
 [Added adjacent batch inversions, cap64 / cap16],[1,100 / 2,140],
 [Added multiplications outside inversions, cap64 / cap16],[39,600 / 77,040])

Both281-target public panels preserve routes and2,881/3,152source calls, with zero unresolved. Per-branch product counters agree. The pair buffer reaches7pairs in those panels.

*Costs.* One initialization and L−1long-stride transitions replace two initializations and2(L−1)transitions. Adjacent conversion adds one inversion per chunk and5Lplus the chunk count in modular multiplications outside inverse exponentiation. Buffer refill overlap, simultaneous mode values, caches and all source/setup work remain charged. Counts cover this stage only and do not establish a timing advantage.

*Evidence.* validation01passes with80pins; inherited controls pass. No new compile/native/render failure or cloud/timing run. Independent signed verdict pending. Original4,563cohort unchanged. Full proof, counts and replay: DERIVATION.md, RESULTS.md, README.md.

*Sources and next.* Exact shared-branch predecessor and NIST DLMF26.3.E1/E6, dlmf.nist.gov/26.3. KB unavailable. Next combine adjacent and anchor-stride denominators in one inversion batch, preserving all prefix-product and storage costs.
