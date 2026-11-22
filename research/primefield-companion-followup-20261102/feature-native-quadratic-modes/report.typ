#set page(paper:"a4",margin:16mm)
#set text(size:9.5pt)
= Apply the next mode before lifting reciprocal candidates
T-1864 · 2026-11-07 · Prime-field quadratic quotient algebra

The next sign-mode constraint now acts directly on compressed reciprocal candidates. Balanced streamed evaluation represents F₁(V), F₂(V) as pairs a(W)+b(W)V modulo Gᵤ(W) and V²−WV+1. No full mode vector or candidate-root enumeration is needed.

#image("flow.svg",width:100%)

*Exact orientation compatibility.* For P=a+bV and C=c+dV, a common quadratic root exists exactly when both norms a²+Wab+b² and c²+Wcd+d² and the determinant ad−bc vanish. Separate norms can select opposite roots. The proof includes zero leading coefficients and repeated quadratic fibers. After filtering W, a capped lift still enforces the phase and actual C₂; zero is checked separately.

#table(columns:(2.7fr,1fr),inset:4pt,
 [Native exact control],[Result],
 [Pair cases / norms-only false positives],[7,424 / 12 rejected],
 [Zero-leading / ramified controls],[3,248 / 1,536],
 [Mode configurations / exact mode values],[18 / 324],
 [Exact full dense intersections],[972],
 [Public targets / source calls],[281 / 2,881])

*Same-cap construction result.* All public scalars, routes and statistics match the isolated path. At cap7, the new path completes40of275attempts and falls back235times; the preserved predecessor completes10and falls back265. Among890processed branches, projection-degree sum falls from2370to1530, with410strict reductions. Early returns exclude unprocessed branches from these sums. No timing ratio follows.

*Charged work.* Three base polynomial products per pair multiplication; balanced streamed mode evaluation, phase powers, norms, determinant, squarefree normalization and gcds all count. Modes are reevaluated per branch. Pair-state space is conservatively O(d log(L+1)+B), plus context/kernel allocations. Charge Vec capacity and failed work before fallback. The lift cap is not a process byte ceiling.

*Evidence.* validation01 pins57transitive inputs and exact output; native overflow checks pass. Initial generator quoting error was fixed before native compilation; failed source/error retained. Producer evidence unsigned. No cloud/timing run or fullCargo rerun; original4,563-case cohort unchanged. See DERIVATION.md for complete proof and replay.

*Sources and next step.* Exact predecessor; Bhargava–Zieve reciprocal fibers (websites.umich.edu/~zieve/papers/manjul.pdf); official FLINT modular conventions. KB unavailable; no novelty claim. Next keep phase-tagged states across further radii/history constraints with exact common-orientation handling.
