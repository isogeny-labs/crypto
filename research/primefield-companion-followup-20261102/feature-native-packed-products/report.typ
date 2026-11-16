#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Share evaluation batches across recurrence products
T-1856 · 2026-11-07 · Bounded request packing in prime-field modes

The coefficient stream now submits numerator and denominator intervals together. Their block points fill one bounded buffer across request boundaries, carrying destination indices. A flush evaluates the cached polynomial and multiplies each value into its designated request. Short products and tails retain exact semantics.

#image("flow.svg",width:100%)

*Routing invariant.* For each request, its accumulator contains precisely its direct tail and already-flushed block factors; pending point/owner pairs identify the rest. Each pair is consumed once. Repeated points, zero factors and wrap are admitted because monic remainder-tree evaluation does not need distinct-point inverses. All requests are validated before helper work.

*Exact tree count.* If blocked request i contributes t_i points, separate scheduling needs sum_i ceil(t_i/w) trees. Packing needs ceil(sum_i t_i/w), never more. It changes neither the number of block values nor the product factors. For seven transitions at r=1009, m=56, B=7, fourteen requests each contribute eight points:28separate trees become16packed trees. Exact coefficient equality is asserted.

#table(columns:(2.5fr,1fr,2fr),inset:4pt,
 [Deterministic native check],[Count],[Outcome],
 [Request-list panels],[375],[3,195 exact request products],
 [Fewer / equal tree panels],[123 / 252],[Both count formulas hold],
 [Real coefficient chunk],[16 vs28],[Same emitted coefficients],
 [Inherited interval checks],[4,820],[Zero/wrap controls pass],
 [Public m56 panel],[281],[14,421 calls; zero unresolved],
 [Inherited public panel],[617],[5,929 calls; zero unresolved])

*Costs and limits.* At most2Brequests/results, wlive points and owners, one cached degree-w polynomial and O(wlogw) tree coefficients; allocator/kernel workspace is separate. No complete block-point grid is stored. Fewer trees improves the conservative per-batch upper bound, but exact kernel arithmetic depends on tree sizes. No field-operation total or timing dominance is asserted. Existing source, first-C0, order-validation and inversion costs remain.

*Evidence.* validation01 pins47transitive inputs and stdout; native overflow checks enabled. Coefficient/residue/state controls also pass. PCC-20261107-720c23 is unsigned producer evidence. No native/report failure, cloud trial or fullCargo rerun. Original4,563-case cohort and prospective trace protocol unchanged.

*Sources and next step.* Exact predecessor and Bostan-Gaudry-Schost2004 standard multipoint evaluation; KB unavailable. This is request scheduling, not specialized progression interpolation. Next investigate structured progression evaluation or repeated block-value reuse with explicit caps and preconditions. DERIVATION.md gives proof, exact replay and limits. Historical/canonical graphs remain unchanged.
