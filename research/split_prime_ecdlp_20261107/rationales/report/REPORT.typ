#set page(paper: "a4", margin: 2cm, numbering: "1")
#set text(size: 10pt)
#set heading(numbering: "1.")
#show link: underline

#align(center)[
  #text(size: 16pt, weight: "bold")[Three attack rationales for a uniform split-prime weighting: class-group relations, rational-isogeny lifting, modular-curve embedding]
  #v(4pt)
  #text(size: 10pt)[research/split_prime_ecdlp_20261107/rationales · 2026-11-08 · branch `claude/class-group-lifting-attacks-pw7y65`]
]

#block(inset: 8pt, stroke: 0.5pt, radius: 3pt)[
  *Status:* derived analysis of three candidate rationales, a native re-weighting of the lab's 21-curve census, and a PARI class-group computation for the smaller Frobenius fields; no algorithm claim. *Result class:* boundary study, follow-up to the parent split-prime study (§3 obstruction, §5 expander degree) and to the skipped-curve census (`nist_skip_criterion_20261025/strict_elkies_20261029`). `S` and end-to-end cost are unset; no leaderboard, benchmark or rho comparison is touched. Every measured number here is a single-producer value [pcc:unverified] except the 21 strict counts at 691, which match the strict study's table (third producer).
]

= Question and answer

*Request* (user, 2026-11-08). Assess three stories under which a curve generator of 1997–98 would have counted split primes $ell <= 691$ with uniform weight per prime, and whether any is an attack: (1) class-group relation attacks (each split $ell = frak(l) overline(frak(l))$ gives a generator $[frak(l)]$ of $"Cl"(cal(O)_K)$ with a cheap Elkies realisation; Hafner–McCurley / Bisson–Sutherland relations need many small split primes of roughly uniform value); (2) lifting attacks needing rational isogenies (xedni 1998, anomalous 1997–98; a curve with many rational $ell$-isogenies is "closer" to reductions of curves on $X_0(ell)$; weakness: rational $ell$-isogenies over $bb(Q)$ exist only for $ell <= 163$); (3) modular-curve embedding (split $ell$ puts $j(E)$ on $X_0(ell)(bb(F)_p)$ with an explicit Frobenius eigenvalue on $E[ell]$; a CRT-combining attack would cost proportionally to the number of usable $ell$; missing step: how per-$ell$ data on $E[ell]$ reach the subgroup of prime order $n >> ell$).

*Answer.* None of the three is an attack, and none predicts the weighting the census shows. (i) The class-group route gives only scalars on $E(bb(F)_p)$ (parent theorem; prior form Theorem 15.1 of `RESEARCH_ISOGENY_FIBERS_ECDLP.md`), and its scale is wrong: a Hafner–McCurley factor base for a 194-bit discriminant reaches norms near $10^8$ and the GRH generating bound $6 ln^2 |Delta|$ is near $10^5$, so ten split primes below 691 change no class-group cost measurably; the one documented class-group defence in a standard (Brainpool, RFC 5639 §2.1 item 3) is a class-number floor $h > 10^7$ motivated by lifting and JMV reachability, not a split count. The class groups computed here (§4) do not separate published from skipped curves. (ii) The lifting route can use only Mazur's primes ${2, 3, 5, 7, 11, 13, 17, 19, 37, 43, 67, 163}$, at most two odd ones at once; the weight $[ell in M]$ separates at *no* cutoff, and at $B = 691$ the published P-256 curve has more Mazur-split primes (6) than its skipped sibling (5); the conductor claim is not a mechanism since isogenous curves over $bb(Q)$ share their conductor. (iii) The modular-curve route CRT-combines Frobenius data mod $ell$ into $t$, hence $n$, which is public; $E[ell]$ and $E(bb(F)_p)$ have coprime orders, so every per-$ell$ datum is point-independent. Its weight $ln ell$ separates at 5 of 302 cutoffs against 4 for the uniform count: the census cannot tell them apart, and both are post-hoc fits. The census does locate the deficit: published-minus-skipped split fraction $-0.107$ at $ell <= 50$, $-0.082$ at 51–163, $-0.056$ at 164–400, $-0.025$ at 401–691, $-0.003$ beyond 691 (descriptive two-proportion $z$ $-1.8, -1.7, -1.5, -0.7, -0.2$): spread over $ell lt.tilde 400$, absent above the cutoff, no single band carrying it. Nothing here bears on the historical cause or reduces any ECDLP cost.

*Scope.* The lab's census of the four seed streams with both published and skipped curves (P-192: 3 + 2; P-224: 1 + 3; P-239: 3 + 5; P-256: 1 + 1) plus the published P-384 and P-521 singletons: 21 curves, $p$ and $n$ from `nist_skip_criterion_20261025/inputs.tsv`. Objects: $t = p + 1 - n$, $Delta = t^2 - 4p = f^2 D_K$, $K = bb(Q)(sqrt(Delta))$, $chi(ell) = (Delta slash ell)$, $"Cl"(cal(O)_K)$.

*Sources checked.* KB unavailable (no `research-kb` server; `scripts/research-kb` finds no executable). Repository: parent README §3, §5, §6 and `ideas/idea-synthesist.md`; `followups/README.md` (P3 null); `strict_elkies_20261029/REPORT.md` and `results/at691.tsv`; `sea_replay_20261026/REPORT.md` (SEA coverage proxy separates in neither direction); `RESEARCH_ISOGENY_FIBERS_ECDLP.md` §15.1; `problems/split-prime-pattern-scalar-only.md`. Literature: Mazur's prime list checked against two expository sources (arXiv 2209.03153, 2206.08891); RFC 5639 §2.1 read directly; Hafner–McCurley 1989, Bisson–Sutherland 2011, Jacobson–Koblitz–Silverman–Stein–Teske 2000 (xedni presented at ECC '98, 14–16 September 1998) and the X9.62 timeline (developed 1998, approved January 1999) confirmed by web search; Kenku's composite list, Bach's bound, conductor invariance, Lenstra 1996, JMV 2005, Shoup 1997 and SEA from memory.

= What each rationale needs, and the weight it implies

Let $M = {3, 5, 7, 11, 13, 17, 19, 37, 43, 67, 163}$ and $W_B (E) = sum_(ell <= B "odd", chi(ell) = +1) w(ell)$.

*Story 1, class-group relations.* $"Cl"(cal(O))$ acts simply transitively on the class; a split $ell$ gives $[frak(l)]$, $[frak(l)]^(-1)$ and two horizontal isogenies at $tilde(O)(ell)$ to $O(ell^2)$ field operations each. Hafner–McCurley collect relations over all split primes of norm $<= B_"HM" = L_(|Delta|)(1 slash 2, c)$, $c approx 1 slash sqrt(2)$; for $|Delta| approx 2^194$ this is $B_"HM" approx e^(18.2) approx 7.6 times 10^7$, a factor base of about $2.3 times 10^6$ primes, so ten split primes below 691 are a relative change of order $10^(-5)$. Bach's GRH generating bound is $6 ln^2 |Delta| approx 1.1 times 10^5$ (P-192) and $1.9 times 10^5$ (P-256). Neither scale is 691; a class-group rationale with a cutoff near 691 would have to be structural (is $"Cl"(cal(O)_K)$ generated by the first few dozen split primes, is $h$ smooth?), which §4 measures. Implied weight: $w = 1$ (one generator, one column per prime) or $w approx 2 slash ell$ (smoothness yield). DLP: every class acts through an isomorphism commuting with scalars and every endomorphism is $alpha mod (pi - 1)$ (parent theorem); knowing $"Cl"(cal(O)_K)$ completely adds no generic operation. Brainpool's $h > 10^7$ is a defence against lifting with End preserved ($[L : bb(Q)] >= h$) and JMV/HR reachability, not against relations.

*Story 2, lifting with rational isogenies.* A rational $ell$-isogeny of a lift $tilde(E) slash bb(Q)$ reduces to one of $E$, so split or ramified $ell$ is necessary; the anomalous attacks need $t = 1$ and no isogeny; xedni and Silverman–Suzuki lift points and need Mordell–Weil dependence. Over $bb(Q)$ cyclic isogeny degrees lie in Kenku's list (primes in $M union {2}$; two distinct odd primes $>= 5$ only via degrees 15 and 21), so a split $ell > 163$ is invisible to any lift. Isogenous curves share their conductor, so "extra isogeny structure lowers the conductor" is not a mechanism. Implied weight $[ell in M]$ or $[ell <= 163]$; the complement $[ell > 163]$ should carry nothing. DLP: the reduced isogeny is a horizontal edge, a scalar action; the lift's own content is the standing xedni obstruction.

*Story 3, modular-curve embedding.* $chi(ell) >= 0$ iff Frobenius has an eigenvalue in $bb(F)_ell$ iff $E$ has a rational $ell$-isogeny iff $(j(E), C) in X_0(ell)(bb(F)_p)$; the Elkies kernel polynomial gives $lambda$ and $t equiv lambda + p slash lambda$; CRT gives $t$. That is SEA, and every curve sits on $X_0(ell)$ for about half of all $ell$. Implied weight $ln ell$ (information); the SEA-cost variant (product of usable $ell$ past the Hasse width) was replayed by the lab and separates in neither direction (published P-256 191 vs skipped 211; published P-224 257 vs skipped 139, 179, 233). DLP: the data CRT combines are functions of $(E, ell)$; $E[ell]$ and $E(bb(F)_p)$ have coprime orders, so no non-zero equivariant $bb(Z)$-linear map exists between them and every $ell$-pairing with a point of order $n$ is 1; what is reconstructed is $n$.

*A fourth reading (speculation).* A generator fearing transport of the instance to a weaker isogenous curve (Frey's Weil-descent talk at the same ECC '98; Galbraith 1999; GHS 2002 for extension fields) would minimise cheap rational isogenies: one edge pair per split $ell$ up to an affordable Elkies degree, i.e. the strict count with a hard cutoff. Indistinguishable from story 1 in the census; it predicts no dependence on $h$. On prime fields the moved-to curve is equally hard (JMV, parent theorem). No source found.

= Census re-weighting (measured, native)

*Method.* `tools/weightings.rs` (dependency-free Rust) computes $chi(ell)$ for every odd prime $ell <= 2000$ (302 cutoffs, the strict study's family) for the 21 curves, forms $W_B$ for eight weights, and records the cutoffs at which every published $W_B$ is below every skipped $W_B$ in each paired field ("lower"), the reverse ("upper"), and the margins at 691. Control: the 21 strict counts at 691 equal `at691.tsv` row for row.

#table(
  columns: 5, align: (left, left, left, center, left), stroke: 0.4pt, inset: 4pt,
  [*$w(ell)$*], [*rationale*], [*lower-separating cutoffs (of 302)*], [*upper*], [*margins at 691, published max → skipped min, P-192 / 224 / 239 / 256*],
  [1], [story 1 generation; §2 mobility], [*4*: 691, 701, 709, 769], [0], [60→61 / 50→53 / 57→61 / 57→58],
  [$ln ell$], [story 3 information], [*5*: 37, 151, 173, 179, 601], [0], [334.0→331.5 ✗ / 281.9→281.5 ✗ / 316.4→329.0 / 306.4→315.9],
  [$1 slash ell$], [story 1 smoothness], [0], [0], [0.584→0.572 ✗ / 0.309→0.469 / 0.441→0.490 / 0.586→0.526 ✗],
  [$1 slash sqrt(ell)$], [intermediate], [0], [0], [4.69→4.72 / 3.33→4.26 / 4.26→4.69 / 4.65→4.50 ✗],
  [$[ell in M]$], [story 2 (Mazur set)], [*0*], [0], [4→4 ✗ / 2→3 / 4→4 ✗ / 6→5 ✗],
  [$[ell <= 163]$], [story 2 (bound)], [0], [0], [16→17 / 11→12 / 17→18 / 18→16 ✗],
  [$[ell > 163]$], [story 2's blind range], [0], [0], [47→43 ✗ / 39→33 ✗ / 42→42 ✗ / 39→42],
  [1 on $chi >= 0$], [inclusive], [0], [0], [63→61 ✗ / 53→57 / 59→65 / 59→60],
)

The uniform row reproduces the strict study's four cutoffs (positive control). The Mazur weight fails in three of four fields at 691 and at every cutoff; the published P-256 curve has six Mazur-split odd primes (11, 13, 17, 37, 43, 163) against five for its skipped sibling. $[ell > 163]$ alone fails everywhere: the separation needs both ranges, which a uniform count supplies. $ln ell$ separates at five cutoffs, none of them 691; with 302 correlated post-hoc cutoffs, four against five favours neither weighting.

#figure(image("../figures/weightings.svg", width: 100%), caption: [$W_691 (E)$ per field for each weight; circles published, crosses skipped; titles give the number of separating cutoffs of 302. Source `figures/plot.py`, data `data/weightings.json` (native, `tools/weightings.rs`).])

*Where the deficit sits.* Pooled over the four paired fields (8 published, 11 skipped curves):

#table(
  columns: 5, align: (left, right, right, right, right), stroke: 0.4pt, inset: 4pt,
  [*band of odd $ell$*], [*published split / cells*], [*skipped split / cells*], [*difference*], [*two-proportion $z$ (descriptive)*],
  [3–50], [36 / 112 = 0.321], [66 / 154 = 0.429], [−0.107], [−1.77],
  [51–163], [81 / 184 = 0.440], [132 / 253 = 0.522], [−0.082], [−1.68],
  [164–400], [137 / 320 = 0.428], [213 / 440 = 0.484], [−0.056], [−1.53],
  [401–691], [184 / 376 = 0.489], [266 / 517 = 0.514], [−0.025], [−0.74],
  [692–2000], [702 / 1424 = 0.493], [972 / 1958 = 0.496], [−0.003], [−0.20],
)

The $z$ values treat cells as independent trials although the cells of one curve share its discriminant: a yardstick, not a calibrated test. The deficit is spread over $ell lt.tilde 400$, weak per band, and absent above 691. A Mazur-only rule would leave 164–400 untouched; an information rule would put most of the deficit in the upper bands. The profile is compatible with a uniform count under a hard cutoff and with a weight decreasing slowly in $ell$; 19 curves cannot choose.

#figure(image("../figures/band_profile.svg", width: 85%), caption: [Pooled split fraction by $ell$-band, published against skipped, with difference, descriptive $z$ and cell counts.])

= Class groups of the Frobenius fields (measured, PARI/GP 2.15.4)

*The class number is the split pattern, to first order (derived).* $h(cal(O)_K) = (w slash 2 pi) sqrt(|D_K|) L(1, chi)$ with $L(1, chi) = product_ell (1 - chi(ell) slash ell)^(-1)$, so $log h = 1/2 log |D_K| + sum_ell chi(ell) slash ell + O(1)$: a deficit of split primes *is* a class-number deficit. A defender who wanted a large class group (Brainpool's $h > 10^7$; the reading of "smooth, well-generated $"Cl"$ is dangerous" as "small $"Cl"$ is dangerous") would have selected for *more* split primes, the opposite of the observed direction. The observed direction is what a defender worried about availability of cheap edges or relations (§2) would pick, and §2 showed 691 is not a scale at which either matters.

*Exact facts for all 21 curves* (`tools/classgroup.gp`, stage `facts`: full factorisation of $Delta$, fundamental discriminant, conductor, partial Euler product for $L(1, chi)$ over $ell <= 10^6$; `data/cl/facts.jsonl`; 3 minutes, longest factorisation 360 s for P-384). The conductor is exact. Eighteen Frobenius orders $bb(Z)[pi]$ are maximal; the published P-224 has $f = 3$, prime239v2 (p239-2320, published) and the skipped p239-2682 have $f = 5$, so for those three $"End"(E) in {bb(Z)[pi], cal(O)_K}$ is undetermined here. The $h$ estimate is $(w slash 2 pi) sqrt(|D_K|)$ times the partial product (heuristic tail): a size, not a class number.

#table(
  columns: 7, align: (left, left, right, right, right, right, right), stroke: 0.4pt, inset: 3.5pt,
  [*label*], [*role*], [*bits*], [*$f$*], [*largest prime of $Delta$ (bits)*], [*$L(1, chi)$ partial, $ell <= 10^6$*], [*$log_2 h$ estimate*],
  [p192-139 (P-192)], [published], [192], [1], [184], [0.411], [94.0],
  [p192-2198], [skipped], [192], [1], [157], [0.498], [93.6],
  [p192-5228 (prime192v3)], [published], [192], [1], [84], [0.352], [93.8],
  [p192-5405], [skipped], [192], [1], [119], [0.434], [94.0],
  [p192-6013 (prime192v2)], [published], [192], [1], [179], [0.529], [93.6],
  [p224-777], [skipped], [224], [1], [97], [0.607], [110.5],
  [p224-1485], [skipped], [224], [1], [211], [0.495], [110.3],
  [p224-5736], [skipped], [224], [1], [112], [0.540], [110.4],
  [p224-7026 (P-224)], [published], [224], [*3*], [149], [0.359], [108.1],
  [p239-36], [skipped], [239], [1], [205], [0.436], [117.1],
  [p239-1562], [skipped], [239], [1], [160], [0.590], [118.1],
  [p239-2185 (prime239v1)], [published], [239], [1], [217], [0.246], [116.8],
  [p239-2320 (prime239v2)], [published], [239], [*5*], [132], [0.476], [115.2],
  [p239-2491], [skipped], [239], [1], [177], [0.415], [117.6],
  [p239-2682], [skipped], [239], [*5*], [81], [0.558], [115.7],
  [p239-2863], [skipped], [239], [1], [149], [0.505], [117.7],
  [p239-3048 (prime239v3)], [published], [239], [1], [239], [0.410], [117.5],
  [p256-876], [skipped], [256], [1], [134], [0.530], [126.4],
  [p256-1842 (P-256)], [published], [256], [1], [136], [0.723], [126.9],
  [p384-2688 (P-384)], [published], [384], [1], [265], [0.276], [189.5],
  [p521-1136 (P-521)], [published], [521], [1], [277], [0.579], [259.0],
)

*Does class-number size separate the roles?* Per field, $L(1, chi)$ partial (the factor carrying the split pattern): P-192 published 0.411, 0.352, 0.529 vs skipped 0.498, 0.434 (ratio of means 0.92; not separated, 0.529 > 0.434); P-224 0.359 vs 0.607, 0.495, 0.540 (0.66; published below all skipped); P-239 0.246, 0.476, 0.410 vs 0.436, 0.590, 0.415, 0.558, 0.505 (0.75; not separated, 0.476 > 0.415); P-256 0.723 vs 0.530 (1.36; *reversed*: the published curve has the larger class group). Class-number size separates in neither direction. The strict count separates where $L(1, chi)$ does not because $L(1, chi)$ weights $ell$ by $1 slash ell$ (the `inv` row of §3, zero separating cutoffs) and is dominated by $ell = 3, 5, 7$, while the deficit is spread to $ell approx 400$. Whatever selected the published curves was not selecting on the class number.

#figure(image("../figures/classgroups.svg", width: 100%), caption: [Left: partial $L(1, chi)$ per field, published against skipped (annotated where $f > 1$). Right: $log_2$ of the class-number estimate against field size; squares mark exact values where `quadclassunit` has finished. Data `data/cl/facts.jsonl`, `data/cl/cl_*.jsonl`.])

*Exact class groups and generation by the small split primes (P-192 field; pending at the time of writing).* `quadclassunit` on this PARI build took 27 s for a 40-digit discriminant and did not finish a 45-digit one in 70 s or the 59-digit P-192 discriminant in 25 minutes. Exact $h(cal(O)_K)$, its factorisation and the orders of the prime forms above the split $ell$ (stage `orders`: index of the subgroup generated by split $ell <= B$ for $B in {50, 163, 691, 2000}$ as $h slash "lcm"$ of orders, exact when $"Cl"$ is cyclic; the Mazur subset; the smallest $ell$ whose form has full order) are running for the four other P-192 curves and are not attempted for the larger fields (P-224 to P-256 would be days). Whatever they return bears only on the structural reading of story 1 and cannot reverse §3 or the direction argument above; the GRH bound already gives generation by primes below $6 ln^2 |Delta| approx 1.1 times 10^5$, and the heuristic expectation is generation by the first handful of split primes for every curve. Outcome (2026-11-08, 09:25 UTC): two attempts produced no result (one set of four runs lost to a container restart after 16 minutes; a second set of four parallel `quadclassunit` processes ran for the two-hour cap and none finished), so the exact P-192 class groups exceed two hours per curve on this PARI build and remain *pending*; nothing in §3 or §4 depends on them.

= Where each rationale meets the obstruction (derived)

#figure(image("../figures/rationales_diagram.svg", width: 100%), caption: [The three rationales and the point at which each loses contact with $E(bb(F)_p)$. Derived; dashed arrows carry no information about the discrete logarithm.])

#table(
  columns: 4, align: (left, left, left, left), stroke: 0.4pt, inset: 4pt,
  [*rationale*], [*object it reads*], [*how it reaches $E(bb(F)_p)$*], [*status*],
  [1 class-group relations], [$"Cl"(cal(O)_K)$, $[frak(l)]$ for split $ell$, relation lattice], [isomorphisms $phi$ with $phi([k]P) = [k]phi(P)$; endomorphisms as $alpha mod (pi - 1)$], [scalar-only (parent §3 (1), (2)); knowing $"Cl"(cal(O)_K)$ adds no generic operation],
  [2 lifting with isogenies], [$tilde(E) slash bb(Q)$ with a rational $ell$-isogeny, $ell in M$], [reduction is a horizontal edge of $E$], [scalar-only; the lift's DLP content is the standing xedni obstruction],
  [3 modular-curve embedding], [$(j(E), C) in X_0(ell)(bb(F)_p)$, $lambda in bb(F)_ell$], [CRT over $ell$ gives $t$, hence $n$], [point-independent (parent §3 (3)); reconstructs a public parameter],
  [4 mobility (speculative)], [horizontal degree $2 dot \#{"split" ell <= B}$], [transport cost of JMV walks], [generic-model effect only (parent §5)],
)

Nothing here exhibits the non-scalar map that `problems/split-prime-pattern-scalar-only.md` asks for; its status is unchanged.

= Status of every statement

#table(
  columns: 2, align: (left, left), stroke: 0.4pt, inset: 4pt,
  [*statement*], [*status*],
  [separating-cutoff counts and margins, eight weights, 21 curves], [measured natively; uniform row equals the strict study's four cutoffs; strict counts equal `at691.tsv` (third producer) [pcc:unverified]],
  [band profile of the deficit], [measured natively; $z$ descriptive [pcc:unverified]],
  [class numbers, structures, conductors, generation indices (§4)], [PARI/GP under GRH; no certification; single producer [pcc:unverified]],
  [scale arguments for story 1], [derived from $L$-notation and Bach's bound (the latter from memory)],
  [Mazur prime list], [checked against two expository sources; Kenku's composite list from memory],
  [conductor invariance under isogeny], [standard, from memory],
  [RFC 5639 requirement and motive], [read directly],
  [obstruction statements], [derived in the parent study; Lean obligations in aburan28/crypto-lean\#3],
  [fourth reading], [speculation, labelled; no source found],
  [historical cause], [the strict study's conclusion stands: no authenticated acceptance trace],
)

*Next steps.* (1) Pre-registered out-of-sample prediction: under a uniform strict count with a hard cutoff in [691, 769], any further published curve of these streams has $W_691$ at or below its field's published envelope (60, 50, 57, 57) and a larger skipped set has a flat deficit profile up to the cutoff; no such curves exist, and Brainpool's documented procedure is not a test of these streams. (2) P-384 and P-521 class groups need a dedicated run; nothing in §4 suggests it would change the reading. (3) No new Lean obligation.
