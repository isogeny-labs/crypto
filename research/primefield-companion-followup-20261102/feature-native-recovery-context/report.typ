#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Reuse interval setup within one recovery
T-1860 · 2026-11-07 · Prime-field sign recovery

Each recovery now owns one lazy interval evaluator. Coefficient, residue and phase streams borrow that evaluator while retaining independent coefficient states. The public entry creates a fresh context for every target.

#image("flow.svg",width:100%)

*Correctness.* The prepared polynomial and progression weights/samples depend only on the fixed field, mode and block cap. Reusing them leaves interval arguments and outputs unchanged. Generated arithmetic matches the isolated control. Exact key checks reject incompatible contexts; short fallible borrows reject conflicts without a panic.

#table(columns:(2.8fr,1fr,1fr),inset:5pt,
 [Deterministic control],[Isolated],[Shared],
 [Block-polynomial setups],[36],[4],
 [Progression-cache setups],[36],[4],
 [Exact coefficient values],[420],[420])

The setup control covers 45 sequences at r=1009, m=56, across caps 1,2,7,16,64. The cap-one context uses direct products. These are software check counts, not workload rates or timing ratios.

*Public recovery.* All 281 targets on F257, curve y²=x³+x+7, base (1,3), order281, mode56 agree with isolated recovery in scalar, route and statistics: 14,421 source calls, zero unresolved. Each starts cold: 224 construct one polynomial/cache; 57 construct neither. The inherited617-target panel retains5,929 calls and zero unresolved. Key rejection, borrow conflict, interleaving and prior arithmetic controls pass.

*Memory and costs.* B caps chunk/request elements, not process bytes. The context retains at most three vectors of w+1 live values, w≤B; charge Vec capacity, metadata and allocator overhead. Temporary trees remain O(w log w). Setup is charged once if used. Query arithmetic/inversions, first C0, source grid, order checks and final verification remain charged. No state is shared between targets.

*Evidence and scope.* validation01 contains compiler/exit receipt, source hashes and exact output. Producer evidence remains unsigned. No cloud/timing experiment or fullCargo rerun; original4,563-case cohort unchanged. See DERIVATION.md for replay and the historical check limitations.

*Sources.* Exact cached-shift predecessor; Bostan–Gaudry–Schost progression evaluation, mathexp.eu/bostan/publications/BoGaSc04.pdf; official Rust RefCell documentation. KB unavailable. No novelty claim. Next address query inversion or first-constraint costs under cold-target accounting.
