#set page(paper: "a4", margin: 2cm, numbering: "1")
#set text(size: 10pt)
#set heading(numbering: "1.")
#align(center)[#text(size: 15pt, weight: "bold")[P1: Riemann–Roch system with a Dickson-torus tower on NIST-shape primes]
#v(3pt) research/split_prime_ecdlp_20261107/followups/p1_rr_dickson · run-20261107T214621Z · 2026-11-07]

#block(inset: 8pt, stroke: 0.5pt, radius: 3pt)[*Outcome under the frozen rules: rule 5, stage-level positive at $m = 3$; rule 6 undecided* [pcc:unverified]. On T24 = 14,680,063 and T31 = 2,046,820,351 ($p equiv 3 mod 4$, $2^t | p + 1$), the separator-free Riemann–Roch system (RR) carries the Dickson-torus tower with exactly its Kummer-tower solving degree and beats the cherry-tree chain (T) on degree and widest matrix at m3 t3 and m3 t4 on both primes. A transfer of round7_rr's Kummer result to the NIST field shape, not a new mechanism; a stage diagnostic (`S`, end-to-end cost, rho comparison unset). Card `CARD-p1-run-01`: executed as specified (director). Host: this container, L0.]

= Question and method
round7_rr found D_RR = 4 < D_T = 6 at m3 t4 with a Kummer tower [pcc:unverified]; NIST-shape primes admit only the Dickson torus tower. Protocol frozen 535d9da826 (Addendum A before any claim cell, merged in \#3657). Instrument `p1tool` (Rust; emits round 7's systems byte for byte, reproduces round7_rr's D and widths exactly). msolve 0.6.5, 1.5 GiB, 1,200 s, one job at a time; 10 cells × 2 seeds × 2 systems = 40 jobs; analysis `p1analyze` (Rust).

= Results (max over 2 seeds; "≥" = capped run, a lower bound)
#table(columns: 6, stroke: 0.4pt, inset: 4pt, align: (left, right, right, right, right, left),
  [*cell*], [*D RR*], [*D T*], [*widest RR*], [*widest T*], [*status*],
  [T24 m3 t3], [4], [5], [632], [883], [both completed],
  [T24 m3 t4], [4], [6], [2,801], [7,129], [both completed],
  [T24 m3 t5], [5], [≥ 6], [31,911], [≥ 23,852], [T capped],
  [T24 m5 t3], [5], [≥ 8], [2,878], [≥ 39,898], [T capped],
  [T31 m3 t3], [4], [5], [632], [883], [both completed],
  [T31 m3 t4], [4], [6], [2,801], [7,129], [both completed],
  [T31 m3 t5], [5], [≥ 6], [31,911], [≥ 23,852], [T capped],
  [p0 Kummer m3 t3], [4], [5], [632], [883], [both completed],
  [p0 Kummer m3 t4], [4], [6], [2,801], [7,077], [both completed],
  [p0 Kummer m3 t5], [5], [≥ 6], [31,870], [≥ 23,720], [T capped],
)
Frozen rules: 1 control met (RR-Kummer m3 t4 D = 4; every completed job finds the planted relation); 2 not killed; 3 transfer holds (D_RR Dickson = Kummer at every shared cell; widths within 0.2 %); 4 no trade in completed cells; *5 yes* (m3 t3, m3 t4, both primes); *6 undecided* (T capped at m5 t3; the frozen text does not decide a one-arm cap). A post-data reading of capped values as lower bounds (sound) would give "met"; it is labelled and not reported as the outcome.

#figure(image("../figures/p1_cells.svg", width: 100%), caption: [D and widest matrix per cell; hatched bars are capped runs (lower bounds). From run-20261107T214621Z via `p1analyze`.])

= Red-team findings that bound the reading
(1) At m5 t3 the RR system has no tower: with $t = 3$ the membership is one division $U Q = D_8(X) - c$; the base holds 8 abscissae and the relation uses 5. It meets the letter of the "$m >= 5$ cell" criterion without showing RR beating T in a cell with a tower; round 7's m5 t4 had RR 25 to 60 times wider than T. (2) At m3 t5 RR's widest (31,911) exceeds T's lower bound (23,852), and round 7's completed T had 23,720: the width advantage most likely reverses at $t = 5$. (3) T is not the lab's strongest $m = 3$ system: round 7's arm F has widest 1,224 (m3 t4) and 10,493 (m3 t5); against F, RR is a trade (different curves and memory, so not combined into a claim). (4) Round 7's T ran at 6 GiB; with sound bounds a cap can only leave a cell undecided. Full review: `followups/red-team-p1-p2.md`.

= Next steps
Rerun T at m5 t3 with at least 6 GiB, then RR and T at m5 t4 on a host with at least 16 GiB; add round 7's arm F at m3 t3–t5 on the same instances. The Dickson remainder tower's faithfulness is Lean-checked as `dickson_tower_faithful` (aburan28/crypto-lean\#3) [pcc:unverified].

= Round 2, card A: capped cells completed and arm F
Protocol `PROTOCOL-R2.md` (frozen before data, \#3693); card `CARD-p1-r2-a` (executed as specified); 26 jobs at 6 GiB, the two T jobs at m5 t3 capped again. Analysis `p1r2analyze` (Rust) with the bound reading frozen before data. [pcc:unverified]
#table(columns: 6, stroke: 0.4pt, inset: 3pt, align: left,
  [*cell*], [*RR*], [*T*], [*F*], [*RR vs T*], [*RR vs F*],
  [m3 t3], [D 4, 632], [D 5, 883], [D 5, 231], [RR dominates], [trade],
  [m3 t4], [D 4, 2,801], [D 6, 7,129], [D 5, 1,224], [RR dominates], [trade],
  [m3 t5], [D 5, 31,911], [D 6, 23,978], [D 6, 10,493], [trade], [trade],
  [m5 t3 (no tower)], [D 5, 2,878], [D ≥ 8, ≥ 82,927], [n/a], [RR below (bounds)], [n/a],
)
Control met exactly (T on the Kummer curve at m3 t5: D 6, widest 23,720, round 7's values). Against the lab's strongest $m = 3$ system F, RR is a trade in all 9 cells: one degree lower, 2.3 to 3.0 times wider. Against T, RR dominates at t3 and t4 and is a trade at t5. The m = 5 cell with a tower (card B, T24 m5 t4, 13 GiB) decides rule 3.
#figure(image("../figures/p1_r2_cells.svg", width: 100%), caption: [RR, T and F per cell, rounds 1 and 2 merged; hatched bars are capped runs (lower bounds).])

= Round 2, card B: the first m = 5 cell with a tower
Card `CARD-p1-r2-b`, T24 m5 t4, seed 2, 13 GiB and 10,800 s, alone on the host. RR timed out after 10,800 s at degree ≥ 6 (working in degree 7), widest ≥ 146,277; T hit the 13 GiB cap after 2,619 s at degree ≥ 9, widest ≥ 662,876. *Rule 3: undecided under the frozen text* (RR did not complete) [pcc:unverified]. Descriptively RR was at a lower degree with 4.5 times narrower matrices when stopped, limited by time not memory. Deciding it needs RR run to completion on a larger host with about a day's time limit.
