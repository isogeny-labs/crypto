#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Polynomial blocks for recurrence products
T-1855 · 2026-11-07 · Capped scalar-field interval arithmetic

The coefficient stream now evaluates long numerator/denominator products with a short cached polynomial and bounded multipoint batches. Short intervals retain direct multiplication. Symmetric binomial initialization also avoids long work for constant modes. Public-point source and recovery control flow are unchanged.

#image("flow.svg",width:100%)

*Identity.* For R(a,l)=∏(a+i), split l=t w+u. Each full block is P_w(a+jw), with P_w(X)=∏(X+i), i=0..w−1. Their product times the final u factors equals R exactly, including zero factors and modular wrap. Substituting these exact products preserves every signed-binomial ratio and the prior recovery proof. Interval values are never inverted by the product helper.

*Conditional cost.* Choose w=min(ceil(sqrt(m)),B). The cached polynomial costs O(M(w) log w). At most ceil(floor(l/w)/w) evaluation batches each cost O(M(w) log w), plus floor(l/w) value products and a tail shorter than w. If B admits square-root width and l=m, this becomes O(M(sqrt(m)) log m+sqrt(m)) per product including setup. With the native Karatsuba model this is O(m^0.79249 log m). This is a derived arithmetic bound; fixed small B does not have that scaling, and the threshold32 is not a measured crossover.

#table(columns:(2.5fr,1fr,2fr),inset:4pt,
 [Native exact check],[Count],[Outcome],
 [Interval comparisons],[4,820],[Equal direct multiplication],
 [Blocked helper calls],[1,808],[Width/batch bounds hold],
 [Zero / wrapped products],[2,485 / 1,305],[Exact boundary behavior],
 [Constant-mode initializers],[3],[No block polynomial built],
 [Additional public panel, m=56],[281],[14,421 calls; zero unresolved],
 [Inherited public panel],[617],[5,929 calls; zero unresolved])

*Storage and scope.* One polynomial of w+1 coefficients persists per stream; each temporary tree has O(w log w) coefficients, with kernel workspace separate. Mode-object storage remains32bytes. First C0 remains dense and mode-order validation costs O(m). The product cap applies even when the retained image degree is small. No end-to-end or observed timing claim follows from the per-product bound.

*Evidence.* validation01 pins46transitive sources and stdout; overflow checks enabled. Coefficient, residue and42,324state controls remain passing. PCC-20261107-8055fc is unsigned producer evidence. No native/report failures, cloud trial or full Cargo rerun. Original4,563-case cohort and prospective trace protocol unchanged.

*Sources and next construction.* Bostan-Gaudry-Schost2004 recurrence and multipoint-evaluation discussion, plus exact predecessor/native polynomial engine; KB unavailable. The implementation uses ordinary product/remainder trees, not the paper's specialized progression-shift algorithm. Next reduce repeated evaluation-tree construction or share evaluations across adjacent ratios. DERIVATION.md supplies proof, limits and replay; prior graphs remain unchanged.
