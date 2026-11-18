#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Fuse coefficient denominator inversions
T-1875 · 2026-11-07 · Prime-field compressed recovery

Stride and adjacent denominators are known before advancing coefficients. The new reader concatenates them, performs one prefix-product inversion pass, emits both modes, and preserves the exact anchor value/index needed at the next chunk boundary.

#image("flow.svg",width:100%)

*Exact local accounting.* For C output pairs and S=min(C,remaining−1)transitions, the fused chunk uses one inversion and5(S+C)+1multiplications outside inverse exponentiation. The predecessor adds one inversion and one verification multiplication whenever S is positive. A final singleton chunk has no saving. Initialization and interval-product work are unchanged and remain charged.

#table(columns:(3fr,1fr),inset:4pt,
 [Native control],[Observed],
 [Sequences / coefficient-and-state equalities],[460 / 6,420],
 [Chunk boundaries],[2,415],
 [Reference / fused chunk inversions],[4,635 / 2,415],
 [Reference / fused outside-exponentiation products],[66,535 / 64,315],
 [Individual inverse identities / invalid inputs],[544 / 4],
 [Branch-mode / canonical-state equalities],[300 / 2,516])

*Public panels.* For281targets each, paired-stage inversions fall2,200→1,100atcap64and4,280→2,140atcap16. Source calls remain2,881/3,152, routes match, zero unresolved. Polynomial product counters agree.

*Memory and scope.* The list contains at most2Bdenominators:128in the sequence panel,14in public panels. Prefixes, inverse vectors, interval results, requests, old/output buffers and polynomial caches remain charged with their actual capacities. Local counts exclude inverse exponentiation internals and unchanged source/setup work; no end-to-end timing claim.

*Evidence.* validation01passes with85pins; inherited controls pass. No new compiler/native/render failure or cloud/timing run. Producer assertion; independent signed verdict pending. Original4,563cohort unchanged. Details/replay: DERIVATION.md, RESULTS.md, README.md.

*Sources and next.* Exact neighboring-reader predecessor and NIST DLMF26.3, dlmf.nist.gov/26.3. KB unavailable. Next address retained evaluator state or direct constraints that avoid initial-support construction, keeping source costs explicit.
