---
title: "Triage and attack plan: the open problems imported from math-street/math-st"
date: 2026-11-19
---


**Status: attacks done (2026-11-19).** All three "attack now" items and the
P3.3 counting check have run; each ended at its preregistered kill. Every
upstream claim quoted here is the upstream authors' unverified assertion
[pcc:unverified]; our results below are measurements or written proofs in the
files named, not pcc-certified claims.

| item | outcome | file |
|---|---|---|
| A1, P2.3 | Kill. $R(n,d)\le L(n,d)+10$ on all 1,263,300 pairs, $n<2^{16}$; generic cost $\Theta(\sqrt{n/L})$ is already met by Cheon and its base-changed form. | `a1_trinomial_roots/REPORT.pdf` |
| A2, P1.2/Q004 | Kill. Coppersmith lattices reach $\alpha_{\max}=0.14$ for $S_3$ (wall at 0.16, matching the determinant limit $1/6$) and $<0.02$ for $S_4$, against kill lines 0.30 and 0.20. | `a2_semaev_small_roots/REPORT.pdf` |
| A3, P5.2/Q025 | Kill. (A)–(C) proved on paper; Q025 is closed in the generic model with an endomorphism oracle. No Lean proof (no toolchain). | `A3_P5.2_Q025.pdf` |
| P3.3 | Upstream's "pure-power penalty" is the counting bound ($N_{\ell^e}\approx p/12$); only Q099 (KLPT vs. optimum) stays open. Medians read from upstream's CSV, not regenerated. | `P3.3_counting_check.md` |

None of these is an index-calculus improvement; each closes one non-generic
route that upstream left open.

## Scope

The public record `math-street/math-st` (upstream commit
`f631bccdb19945a90e92bab6fc11c00c4ec841f5`, 2026-08-16) was imported verbatim
into `isogeny-labs/knowledge` under `imports/math-st/` (knowledge PR #8). By its
own status lines, 16 of its 21 problems and 23 of its 31 open-question entries
are open. This directory triages all 39 open items (`triage.json`; the set was
checked to equal the import's open set exactly) and fixes the order and the
kill criteria for attacking them.

Triage was done by reading each item's `PROBLEM.md`, `STATE.md`,
`HANDOFF.md` and question entry, and selected upstream attempts, against the
lab goal: speeding up index calculus (ECC2K-130 and other Koblitz binary
curves; prime-field Semaev) and finding non-generic ECDLP algorithms.

**Sources checked.** The `research-kb` MCP server was not available in the
session that wrote this plan, so the "Knowledge base first" receipt is
**pending**: each attack below starts with the research-kb queries named in
its row, and records them, before any novelty claim. Literature named in this
plan is cited from memory and is to be checked at that step.

![Triage of the 39 open items: tractability against relevance. Data: `triage.json`; layout: `plot_triage.py`.](triage.svg)

## Result of the triage

Three items get bounded attacks now, three later, and 33 are recorded with a
reason and not attacked. None of the 39 promises a direct index-calculus
speed-up; the three attacks each settle one non-generic route.

### Attack now (in this order)

| # | item | what is actually open | attack | preregistered kill criterion | tooling |
|---|---|---|---|---|---|
| 1 | P2.3 (Cheon generalization) | Given $g, g^x, g^{x^d}$ with $d \nmid n\pm1$, is there a better-than-$\sqrt n$ algorithm, and what is the tight generic bound? In the generic group model a collision is a root of a trinomial $a+bX+cX^d$, so the bound is governed by $R(n,d)$, the largest root count of such a trinomial in $\mathbb F_n$. | Exact $R(n,d)$ for all $d$ over toy primes $n \le 2^{20}$, compared with $\gcd(d,n-1)$, $\gcd(d,n+1)$ and cyclotomic factors of $n$. | Kill: $R(n,d) = O(\gcd(d,n-1))$ off the known divisor cases, so the generic cost is pinned at $\sqrt{n/\gcd}$ (then state and formalize that bound). Lead: a family of $d$ with $R \gg \gcd$, which is a candidate speed-up to implement and verify on recovered secrets. | Rust; Lean for the bound |
| 2 | P1.2/Q004 (coordinate-aware prime-field factor base) | Can small-$x$ Semaev decompositions over $\mathbb F_p$ be found faster than search, i.e. small roots of $S_3$, $S_4$ modulo $p$? Upstream's only coordinate-aware test was a 5-second Gröbner timeout. | Coppersmith/Howgrave-Graham lattices for $S_3$ (2 unknowns) and $S_4$ (3 unknowns) with $\lvert x_i\rvert \le p^{\alpha}$; find the largest $\alpha$ recovered, $p$ of 30–60 bits, dimension up to about 150. | Kill: $\alpha_{\max} < 0.6\times$ the counting threshold ($1/2$ for $S_3$, $1/3$ for $S_4$) as dimension grows; this closes small-$x$ prime-field Semaev as a lattice target. Lead: $\alpha_{\max}$ reaches the threshold. | Rust (LLL) |
| 3 | P5.2/Q025 (nonlinear orbit collisions on small-CM curves) | Upstream's own data has the eigenvalue subgroup $H$ of index 2 or 3, so an "orbit collision" is the event that $\log P$ and $\log Q$ share a quadratic or cubic residue character. Q025 is equivalent to computing that character of an unknown logarithm. | Pen-and-paper: (A) if $\omega$ is a non-automorphism endomorphism of degree $d$ acting by $\lambda$ of order $m$ mod $r$, then $r \mid \deg(\omega^m-1) \le (d^{m/2}+1)^2$, so $m \ge \log r/\log d - O(1)$; (B) a generic-group lower bound for the $H$-coset of $\log P$ with an endomorphism oracle. Optional Rust toy with a simulated character oracle. | Kill (closes Q025): (B) gives $\Omega(\sqrt{\varepsilon r})$ for advantage $\varepsilon$. Lead: a counterexample to (B), or a character oracle cheaper than $r^{1/6}$, which would then beat $\sqrt r$ through Legendre-PRF-style recovery. | pen-and-paper; Lean for (A) |

### Attack later

| item | attack | kill criterion |
|---|---|---|
| P1.6/Q018 | Reproduce the JKSST p = 17 Experiment A count under 2–3 explicit samplers, classifying dependence with a certified height-regulator test (2-descent is needed only for the curve's rank); reuse the sibling Xedni lifter. | Rates spread more than 3× across samplers: the published 317/100,000 is sampler-specific and Q018 closes as unanswerable. |
| P3.3 and P3.3/Q099 | Record the restatement first. Unconstrained $\sqrt p$-norm equivalent ideals are a 4-dimensional shortest-vector problem (easy). For $\ell$-power norms $\sqrt p$ is impossible by counting ($\ell^e$ ideals against about $p/12$ classes). The real gap is $\approx p$ against KLPT's $p^{3}$–$p^{7/2}$. Then fit $\min \ell^e/p$ against size, and compare KLPT with the exact optimum. | $\min \ell^e/p$ roughly constant across sizes: upstream's "structural gap" is counting. |

### Recorded, not attacked (33)

Each reason is in `triage.json`. In summary:

- **Mis-posed or trivial as stated:** P1.1 (the free-coordinate model is vacuous on prime fields), P1.6 (the literal height bound fails by choosing $a_6$ through the point), P1.2/Q002 (circular: decomposing over reduced lifts is a multi-dimensional ECDLP in $E(\mathbb F_p)$), P1.2/Q003 (a specification choice), P3.2/Q012, P3.4 and Q011 (no attack model fixed), P4.2/Q010 ($\rho<2$ holds automatically for prime-order cycles by Hasse), P5.4/Q021.
- **Known results:** P1.4 and Q003 (Menezes–Qu, Hess, Menezes–Teske–Weng; Gaudry, Enge–Gaudry, Diem), P5.2 main (Duursma–Gaudry–Morain), P3.2 and Q013 (published simulator tables), P3.1 (check Herlédan Le Merdy–Wesolowski 2025 before anything else).
- **Breakthrough-level, no bounded sub-target:** P1.5 and Q004 (transfer to a finite field of another characteristic; its attempts A033–A049 are useful as a list of dead routes), P2.1 and Q005 (uniform smooth auxiliary groups; the sufficient CM condition is heuristically false asymptotically), P4.3, Q016, Q017 (rigorous $L(1/3)$), P5.1 and Q026 (the Koblitz conjecture on prime orders, about curves over $\mathbb Q$, unrelated to Koblitz binary curves).
- **Off goal:** P2.2 and Q003, P4.2, P4.1/Q027, P5.3/Q014, P5.4, P5.4/Q012 (the possible RFC 9380 `sqrt_ratio` fault needs rational 2-torsion; checking the errata list and filing are the owner's call), P1.4/Q002, P1.6/Q008.

**One checked fact for ECC2K-130.** The multiplicative order of 2 modulo 131
is 130 (checked by direct computation), so $x^{131}-1 = (x+1)\,f(x)$ with $f$
irreducible of degree 130 over $\mathbb F_2$. GHS magic numbers over
$\mathbb F_{2^{131}}$ are therefore 1, 130 or 131, and the Koblitz coefficient
$b = 1$ gives $m = 1$, the degenerate genus-one case. GHS Weil descent does
not apply to ECC2K-130. This agrees with the lab's existing treatment; it is
recorded here because P1.4 frames GHS as open for binary curves in general.

## Rules every attack follows

- Native code (Rust) for every computed result; Python only renders recorded
  values (AGENTS.md, "Implementation language: no Python").
- A preregistered protocol in this directory before each run: hypothesis,
  frozen inputs, success and kill criteria, as in the tables above.
- Lean 4 for each mathematical result where feasible, with `#print axioms`.
- A report, a graph and a PDF per attack, negative and inconclusive results
  included.

## Files

- `triage.json`: one row per open item: tractability (1–3), relevance (0–3), verdict, attack, kill criterion or note.
- `plot_triage.py`: renders `triage.svg` from `triage.json`; it computes nothing.
- `report.pdf`: this file rendered with the figure (Pandoc to Typst).
