#set page(paper: "a4", margin: 2cm, numbering: "1")
#set text(size: 10pt)
#set heading(numbering: "1.")
#align(center)[#text(size: 15pt, weight: "bold")[P2: twist-pure coset selection for 2-power factor bases on prime-order curves]
#v(3pt) research/split_prime_ecdlp_20261107/followups/p2_twist_coset · run-20261107T214118Z · 2026-11-07]

#block(inset: 8pt, stroke: 0.5pt, radius: 3pt)[*Outcome (frozen rule 4):* toy-scale gain confirmed at stage level, smaller than first written [pcc:unverified]. F4 cost on planted targets is coset-independent; exact coverage follows $c_E^m$. The frozen statistic $(c_"sel" slash c_"rand")^m >= 2.25$ at $t = 4$; after the red-team correction the gain that matters for an index calculus is $(c_max slash c_"mean")^(m-1)$, about 1.9 ($m = 2$) and 3.5 ($m = 3$) at $t = 4$, decaying toward 1 as $t$ grows. A transfer of the lab's frame-choice lever to prime-order curves, not a new mechanism. `S`, end-to-end cost and any rho comparison are unset. Card `CARD-p2-twist-coset-20261107-1`, director assessment: executed as specified. Host: this container, L0; only exact counts decide.]

= Question, instances, method
A 2-power base $V_c$ (Kummer $x^(2^t) = c$; Dickson torus $D_(2^t)(x) = c$) has about half its $2^t$ elements on $E$, half on the twist. Protocol (frozen 535d9da826, Addendum A, Clarification A.1, merged in \#3657) asks whether F4 cost depends on $c$, whether exact coverage scales as $c_E^m$, and whether picking the coset with the largest $c_E$ among $C$ candidates pays. Primes: Kummer 1,049,089 and 16,777,729; Dickson 1,049,599 and 16,780,031; three prime-order curves each; $t in {4, 6, 8}$; $C = min(10^4, "cosets")$; msolve 0.6.5 with 1.5 GiB and 1,200 s per job. Instrument `p2coset` (Rust); analysis `p2analyze` (Rust) applies the frozen rules to the raw JSONL.

= Results
#table(columns: 3, stroke: 0.4pt, inset: 4pt, align: left,
  [*rule*], [*test*], [*result*],
  [1 lemma application], [nontrivial E-target relations with twist part $!= O$; exhaustive $S_3$ at 5 primes $<= 10^4$, $S_4$ at 101, 151, 199; 3 curves each], [0 violations in 24 runs],
  [2 twist columns inert], [F4 work, D, widest: selected vs random coset, 12 groups × 4 targets], [max rel. diff 0.39 % (A/A 0.45 %); D and widest identical; kill line 10 %],
  [3 coverage law], [coverage ratio vs $(c_"sel" slash c_"rand")^2$, 36 cells], [max deviation 0.93 %; 0 cells beyond 20 %],
  [4 gain], [$(c_"sel" slash c_"rand")^m$ at $t = 4$, curve 0, 8 groups], [2.25 to 125],
)
M1: observed $c_max slash c_"mean"$ within 10 % of the exact binomial order statistic in all 36 cells; means 1.878 / 1.458 / 1.225 (exact 1.886 / 1.471 / 1.230) at $t$ = 4 / 6 / 8. Expected gain against a typical coset, $(c_max slash c_"mean")^m$: about 3.5 and 6.6 at $t = 4$, 2.1 and 3.1 at $t = 6$, 1.5 and 1.8 at $t = 8$ ($m = 2, 3$). The rule-4 maximum (125) comes from one seed-drawn coset with only 3 E-columns of 16.

#figure(image("../figures/p2_summary.svg", width: 100%), caption: [(a) order statistic vs $t$; (b) coverage law, 36 cells; (c) F4 work per arm, mean of 4 targets. Values derived by `p2analyze` from run-20261107T214118Z.])

= Red-team corrections
(1) Exponent $m - 1$: an index calculus needs about $c_E$ relations, so the net gain is $(c_"sel" slash c_"ref")^(m-1)$; against the frozen random arm 1.5 to 25, against a typical coset about 1.9 / 3.5 ($t = 4$), 1.46 / 2.1 ($t = 6$), 1.22 / 1.5 ($t = 8$). (2) A single seed-drawn random coset is a high-variance reference; $c_"mean"$ is used instead. (3) F4 cost was measured on planted targets only; unplanted targets are untested. (4) The T\* values for $m = 3$ in the derived table use $m = 2$ coverage and are wrong. Full review: `followups/red-team-p1-p2.md`.

= Scope and caveats
Stage improvement of 2-power-base relation search on prime-order curves at $2^20$ and $2^24$ [pcc:unverified]; it changes no exponent ($c_max slash c_"mean" -> 1$ as $2^t$ grows) and moves no leaderboard row (validation gate not run). Selection cost ($C dot 2^t$ Legendre symbols, about $3.8 dot 10^6$ field multiplications at $t = 4$) is repaid before the first relation at these sizes (derived; mixes field multiplications with F4 mul-adds). The card cannot bind the binary it builds, so a post-run lint flags the binary-calling commands; the run recorded the binary hash. The relay added only the `BENCHREC=` prefix required by the repository hook. Purity is formalized abstractly as `twist_part_sum_eq_zero` (aburan28/crypto-lean\#3).

= Round 2: F4 cost on unplanted targets
Card `CARD-p2-twist-coset-20261108-r2` (executed as specified). 144 msolve cells on unplanted targets ($R = [k]G$), same primes, cosets and arms as round 1's M3. *Outcome (PROTOCOL-R2 rule 1): coset-independence holds exactly* [pcc:unverified]: all 144 systems inconsistent (no target decomposed); F4 work, D and widest identical across arms, targets, primes and tower kinds at each $(m, t)$ — 258,271 (m2 t4), 10,759,232 (m2 t6), 662,045,497 (m3 t4) — at 90 to 98 % of the planted cost. On a failed attempt F4 does the same work whatever the coset, so coset selection changes only the success rate, and round 1's gain $(c_max slash c_"mean")^(m-1)$ holds for the relation search as a whole.
