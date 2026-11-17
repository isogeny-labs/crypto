# Review findings: group `front`

Sections: abstract, intro, prog_background, model, prog_theory, prog_method, cover, falldegree, hardness, obstructions, prog_openproblems, conclusion, artefacts.
Sources read with `git show origin/main:` in /home/user/crypto (C:) and /home/user/crypto-autoresearcher (CA:). Line numbers are those of the section file.

Numbers re-read from primary sources (31): 4,848 executions; 3.8–15x and 9.8→95.2 / 10.5→101.8 (campaign); 0.00647 [0.00545,0.00799], x155 [125,184], x64 [52,76], band 0.107–0.123 (JV sieve); 3.43–11.5x (Table A v7); 4.67–18.07 (v8); 19 validated rows; 113.755/47.166/66.486/0.099 s; setup 98.5–99.7% / descent 0.3–1.4% (MISMATCH, F5); 1.089 [0.927,1.261], alpha 0.451 [0.399,0.512] (historical bound, F22); 124.85/2,864.1/1,189.7; 3,021–5,802x; C = 0.25/0.27/1.2–1.5; S = 16.7/421.4; 120/168, 72/168, 168/168; 301,961 vs 300,531 (via audit); 0.5347, 0.2403, 0.309; 1,196→679.7; 84.9x; 5.2x, m* 17.6 (16.3–19.2); 0.005 [0.004,0.008] at q = 2,097,091; T33a cert sha cf9dcae8…22cf, codim 4888, dims (0,0,64,2968,48068), 38.8 s, 4 min 40 s; P45a dims (0,1,224,8950,166705), 224 = 5n−1; certified codims 5287/10066/12742/9253/6817; sheriff orders 2/24/192/1920/23040 and 2.1 M fibres; lead-atlas tally (MISMATCH, F11).

---

## F1 [severity: blocker] obstructions.tex:49
**What is wrong.** "§Oracle-augmented index calculus stays generic": "a jet or $p$-adic lift; an endomorphism; an incidence or halving test; an elliptic-net evaluation. The programme proved, at derivation tier, that each of these is simulable in the generic group model, so it cannot beat $\sqrt r$ (KN-FIND-002, KN-FIND-b7e091, KN-FIND-194294; halving is equivalent to an $x$-coordinate oracle)." The 4 November split says the opposite for field-valued jet, field-valued net and $x$-valued halving. The cited KN-FIND-194294 itself says the $x$-oracle is NON-SIMULABLE. So "halving ≡ x-oracle" argues against simulability. The jet "C=1" verdict was withdrawn in July. The endomorphism argument's premise End = Z is false.
**Evidence.** CA:knowledge/leads/ecdlp/exotic/LEAD-EXO-coordinate-revealing-oracles.yaml: "The GGM gives no bound for this class. Two x-queries plus free local work give log_P Q up to sign on any curve, so x, y, full point, x-valued halving and field-valued jet and net have simulation overhead Theta(sqrt N'')", disposition `open_next_step`; evidence note "KN-FIND-194294.md: x-oracle NON-SIMULABLE". LEAD-EXO-augmented-oracle-ggm.yaml: "the End_{F_p}(E) = Z premise of KN-FIND-b7e091 is false"; "Derivation tier … not yet ratified". The treatise already carries the correction elsewhere: unconventional.tex:231–270 (§sec:um-oracles) and corrections.tex:369 (jet: "withdrawn, no derivation exists; closure countermanded; simulability open").
**Action.** Replace lines 51–61 with: "Several leads proposed giving index calculus an extra oracle on the group. Since the 4 November split, label-valued oracles are Shoup-simulable with overhead $\Theta(\log N)$ or $O(m)$: endomorphism image (by an eigenvalue argument, since End$=\ZZ$ is false), label-valued halving, incidence check over a label base and label-valued net. They stay closed (\texttt{LEAD-EXO-augmented-oracle-ggm}, derivation tier, not ratified). Coordinate-revealing oracles ($x$, $y$, $x$-valued halving, field-valued jet and net, $C_t$) are not simulable, and the generic group model gives no bound for them (\texttt{LEAD-EXO-coordinate-revealing-oracles}, open; \S\ref{sec:um-oracles})." Also drop "so it cannot beat $\sqrt r$" for the coordinate class.

## F2 [severity: blocker] abstract.tex:52–56 (and intro.tex:100–103)
**What is wrong.** "The distance at a matched reference is $4.7$--$18\times$ on Koblitz curves of 37--47 bits and $10.5$--$102\times$ on prime curves of 20--40 bits." Both ranges are given with no standing and are not the validated rows.
- 4.67–18.07 is rule v8. The board carries v7, 3.43–11.5x, as the validated row.
- 10.5→101.8 is the 15 October campaign pair table at w = 0.1 (L0, pre-gate). On the same 20–40-bit ladder the later k-sum table is 5.3–13.1x (L0, no validation), and the validated 20-bit baseline is 7.75x.

So the abstract and intro quote a stale, superseded prime figure as "the distance". The 102x figure overstates the current distance by about 8x at 40 bits.
**Evidence.** C:docs/ic/LEADERBOARD.md Table A rows: "**3.43×** … **11.5×** … validated". C:research/ic_frontier_campaign_20261015/README.md: "prime 20–40 bit | pair table | 9.8 → 95.2 | 10.5 → 101.8 | 13.5 → 131.4". distance.tex:144–150: "$k$-sum table (28 October) & 20 & $5.3$ … 40 & $13.1$ … L0; no validation"; "New 20-bit baseline & 20 & $7.75$ … validated".
**Action.** Replace with: "At a matched reference rho is cheaper on every prime-order and prime-degree binary curve measured. On Koblitz curves of 37–47 bits the gap is $3.4$–$11.5\times$ on the validated board (v7), and $4.7$–$18\times$ against the matched rho of rule v8 (not on the board). On prime curves of 20–40 bits it is $5.3$–$13\times$ for the best generic arm ($k$-sum table, L0, not validated) and $5.5$–$13.5\times$ on the validated Table B rows of 10–24 bits." Make the same change in intro.tex:100–103 and conclusion.tex:40–41.

## F3 [severity: blocker] conclusion.tex:44–49
**What is wrong.** "Nineteen leaderboard rows passed a validation gate … None of them is below its reference. Every online win disappeared …" The count is right: leaderboard.json `validation_gate.status_counts.validated: 19`. But the conclusion leaves out the programme's one validated measurement of index calculus below a matched rho, the Joux–Vitse sieve at $p=1511$. Read with "Where index calculus wins" (lines 50–56, which cite only the unvalidated, L0 plane-quartic 0.005x), the conclusion understates the validated result and overstates the unvalidated one.
**Evidence.** C:research/jv_sieve_negrho_20261102/VALIDATION.json: "Validator VAL-20261103-c31c76 … Accounting, not an improvement. Primary cell: S/rho U1 0.00647", "order_holds_across_band": true. README: "No leaderboard or `benchmarks/` row." prog_transfer.tex:283: "It is the programme's one validated measurement of index calculus below a matched rho." distance.tex Table tab:distance-weak: plane quartic "L0; no validation file".
**Action.** After line 46 insert: "Off the board, one validated measurement is below a matched rho: the Joux–Vitse sieve route on the weak class at $p=1511$. It is $0.0065\times$ a matched negation rho in group additions and $0.016\times$ in $\FF_p$ multiplications, and the order holds across the band (\texttt{VAL-20261103-c31c76}). The validator classes it as accounting: a local run, a sampled replay, and F4/descent/linear-algebra additions bounded rather than emitted." In "Where index calculus wins", label the plane-quartic row "L0, no validation file".

## F4 [severity: blocker] abstract.tex:21–24; intro.tex:64–66
**What is wrong.** "we prove … the first fall degree is 2 for every $n$, including Semaev's chained systems". The corollary is conditional. It is proved only under a hypothesis checked on computed instances, and the Petit–Quisquater reading also needs $\ell_0\ne0$.
**Evidence.** falldegree.tex:192–197: "Suppose every descended equation has degree exactly 2; this held on every instance computed. Then: $d_{\rm ff}(F)=2$ … if moreover $\ell_0\ne0$, the Petit--Quisquater first fall degree is also 2."
**Action.** Replace with: "the first fall degree (Caminata–Gorla) is 2 for every $n$ whenever every descended equation has degree 2, as on every instance computed, including Semaev's chained systems, where 4 was stated".

## F5 [severity: major] model.tex:74–76
**What is wrong.** "reusable set-up is $98.5$--$99.7\%$ of cold $S$, and descent is $0.3$--$1.4\%$ (docs/ic/LEADERBOARD.md, phase table)". The cited table gives other values.
**Evidence.** C:docs/ic/LEADERBOARD.md "Phases, as a share of S", Table A: 99.0, 99.1, 98.7, 99.6, 99.6, 99.6% set-up and 1.0, 0.9, 1.3, 0.4, 0.4, 0.4% descent.
**Action.** "is $98.7$--$99.6\%$ of cold $S$, and descent is $0.4$--$1.3\%$".

## F6 [severity: major] abstract.tex:4–5; intro.tex:16–17; conclusion.tex:3
**What is wrong.** "a four-month research programme (July--October 2026)", "between July and October 2026", "Four months of work". These are stale: the treatise now reports work from November. That includes the validated JV sieve row (2–3 November), the whole hardness chapter ("between 2 and 4 November 2026", hardness.tex:12–13), the collision floor (4 November, rho_side.tex:73) and the oracle split (4 November).
**Action.** Use "July–early November 2026" (abstract, intro) and "Four months and more" or "Eighteen weeks" (conclusion).

## F7 [severity: major] abstract.tex:1–70; intro.tex:41–106; conclusion.tex
**What is wrong.** Three new chapters have no summary sentence in the front or back matter:
- the hardness chapter (ch:hardness);
- the generic floor and collision floor (ch:genericfloor);
- the matched-rho chapter.

Missing results include:
- Petit–Quisquater Assumption 2 refuted in its literal reading on all 119 census instances (hardness.tex §hd-pq2);
- the kernel-checked left-kernel theorem: a single target needs no rank condition (§hd-rank);
- PKM's "$\Delta=10$ suffices" contradicted (`rank_deficit`);
- Nagao's Lemma 4 resolved in four reading/placement cells;
- the collision floor: any generic single-target IC with $F$ unknown base logarithms costs at least $1.5$, $1.875$, $2.188\times$ rho for $F=1,2,3$ (rho_side.tex Prop. collisionfloor), which explains the 1.8x generic incumbent.

"The argument in brief" lists six steps and none of these.
**Action.** Add an abstract sentence: "A generic collision floor shows that any single-target index calculus whose base logarithms are unknown costs at least $1.5\times$ rho ($1.875\times$ at two base elements, matching the measured generic incumbent). A hardness audit refutes the literal Petit–Quisquater Assumption 2 on 119 instances, and the kernel-checked left-kernel theorem shows that one target needs no rank heuristic. Both are census or derivation results, unverified under the proof-carrying-claims rule except where Lean-checked." Add intro paragraphs "7. What hardness rests on (Chapter ch:hardness)" and "8. The rho side (Part rho)".

## F8 [severity: major] abstract.tex:5–7; intro.tex:119–127 (Conventions)
**What is wrong.** "Every number is drawn from committed evidence and carries its boundary". This gives no standing convention. Under the owner's sabotage rule (2026-11-04), every result needs a pcc claim id or the label "unverified assertion". The new chapters do this (rho_side.tex:17–24; hardness.tex:28–43; corrections.tex:14–19). The abstract and the Conventions list do not, and the abstract labels only one row "validated", leaving the 0.005x, genus-3 and all distance figures unlabelled.
**Action.** Add to Conventions: "\emph{Standing.} No pcc claim covers a result in this treatise unless one is named. A measured figure is the producer's reported value (an unverified assertion) unless a \texttt{VALIDATION.json} is named, and `validated' means only that. A theorem is `kernel-checked' only where a Lean kernel accepted it." In the abstract add "(no validation file)" to the plane-quartic and genus-3 items.

## F9 [severity: major] conclusion.tex:50–56; abstract.tex:62; intro.tex:93; prog_openproblems.tex:135–138
**What is wrong.** The Joux–Vitse Nagao figure "$1.6$--$2.0\times$" (pooled, unmatched rho constant, p = 1009, lower bound), with "the line meeting rho near $p\approx1330$ by extrapolation", is stale. The November studies measured the plain Nagao arm against a matched negation rho at $p=1301$, 1451 and 1601. They found it above rho at 1301 and 1451 in additions and above at every prime in multiplications. With the factor-base line search it is below rho in additions ($0.548$–$0.791$) but above in multiplications, so it is unranked across the band. The modelled crossovers are $p^*\approx1185$ (additions) and $1799$ (multiplications).
**Evidence.** transfer_ext.tex:1523–1565 and Table tab:jac-nagao; C:research/jv_cover_negrho_rescore_20261101/, research/jv_cover_fbline_20261102/README.md.
**Action.** Abstract: "Joux–Vitse covers: the Nagao arm within $0.55$–$1.4\times$ a matched negation rho at $p=1301$–$1601$ in group additions, but above it in $\FF_p$ multiplications (unranked)". Open problem 11: replace the $p\approx1330$ extrapolation with these matched figures and the modelled $p^*$ values.

## F10 [severity: major] falldegree.tex:373–379 (Cor. cor:assump) and hardness.tex:330–333
**What is wrong.** "Theorem~\ref{thm:lfd5} contradicts it in three cells:" followed by four cells, including $(21,3,3,7)$. Theorem lfd5 certifies only C19a/b. C21a/b rest on implementation A alone, so the $(21,3,3,7)$ cell is not certified. hardness.tex:330–333 repeats all four cells as refuted.
**Evidence.** falldegree.tex:357–358: "C21a and C21b (the chain at $n=21$) have $1\notin W_4$ by implementation~A alone". CA:analysis/semaev-m2-last-fall-degree/results/certificates.txt lists C19a/C19b and no C21.
**Action.** "Theorem lfd5 contradicts it in three cells: $(35,2,2,18)$ with random subspaces; $(45,2,2,23)$ with his own default subspace; $(19,3,3,7)$ inside his tables' range. Implementation A alone also gives $1\notin W_4$ at $(21,3,3,7)$, with no checked certificate." Edit hardness.tex:332 the same way.

## F11 [severity: major] obstructions.tex:3–11
**What is wrong.** "The programme kept 148 leads … (reviewed 2026-10-24). 73 closed, 51 open, 16 blocked, 8 unexamined." This is stale.
**Evidence.** CA:knowledge/leads/ecdlp/REVIEW_STATE.json: "reviewed_at": "2026-10-30", "leads": 153; tally closed_dominated 5 + closed_known 18 + closed_obstructed 45 + exhausted_at_scale 6 (74 closed), open_blocked 17, open_next_step 53, open_unexamined 9. origin/main now holds 173 LEAD-*.yaml files, including the 4 November split.
**Action.** Re-run `lead_atlas.py --brief` and quote the current tally with its review date, or write "153 leads at the 30 October review (74 closed or exhausted, 53 open with a next step, 17 blocked, 9 unexamined); later rounds added more (Chapter ch:unconvmap)".

## F12 [severity: major] prog_theory.tex:85–89
**What is wrong.** "Our own measurements on toy fields (\S\ref{sec:binary-descent}) find first and last fall degrees of 3--4 at every reachable size …" This contradicts Chapter ch:falldegree:
- the first fall degree of the binary $\Sem3$ descent is 2 (Cor. ffd2);
- the last fall degree is exactly 5 on R35a and certified $\ge5$ at $n=33$–45;
- the $(m,\ell)=(3,7)$ draws are $\ge5$ (falldegree.tex Table tab:fd-other).
**Action.** Replace with: "Early toy measurements (\S\ref{sec:binary-descent}) found last fall degrees of 3–4 at small $\ell$. Chapter ch:falldegree proves a first fall degree of 2 for the $m=2$ descent, finds a last fall degree of exactly 5 at $n=35$, and certifies $\ge5$ on named instances at $n=33$–45. Neither supports or refutes a subexponential claim."

## F13 [severity: major] intro.tex:102–103; model.tex:31–35
**What is wrong.** "with the gap widening as $r^{1/6}$" / "two-summand prime-field index calculus loses ground to rho as $r^{1/6}$". This is the classical $m=2$ balance. On the prime curves the treatise now reports, the best arm (the $k$-sum table) fits $0.521$ $[0.506,0.538]$ against rho's $\approx0.5$, so the gap widens as about $r^{0.02}$. The campaign pair table's 0.620 against 0.435 gives about $r^{0.19}$.
**Evidence.** distance.tex:145–146 "$k$-sum … $0.521$ $[0.506,0.538]$"; campaign README "prime pair table 0.620 [0.557, 0.663] vs rho 0.435".
**Action.** intro: "with the gap of the two-summand pair table widening as about $r^{1/6}$; the generic $k$-sum table narrows that to $r^{0.02}$ but stays above BSGS". model.tex: add "(the $k$-sum oracle, a generic table of 4-sums, moves the exponent to $k/(2k-1)$, Chapter~\ref{ch:distance})".

## F14 [severity: major] intro.tex:105–106
**What is wrong.** "Chapter~\ref{ch:audit} lists every such correction." This is stale: ch:corrections now holds the corrections not in ch:audit (corrections.tex:4–9).
**Action.** "Chapter~\ref{ch:audit} lists the corrections that moved the leaderboard, and Chapter~\ref{ch:corrections} the remaining corrections, retractions and withdrawn readings."

## F15 [severity: major] artefacts.tex:3–6 and intro.tex:35–37
**What is wrong.** "Every number in this treatise is copied from a committed artefact." This is false for the Kummer-tower last fall degrees, which are session-reported and never committed (falldegree.tex:565–570, "the $d_F$ column is session-reported, not re-derivable"). The same values feed intro "6 against 4" (intro.tex:73) and falldegree.tex:648. The asymptotic $\sqrt F$ in rho_side.tex:107 is also "ours, not the source's". In addition, artefacts.tex:6 says paths are relative "unless prefixed crypto-autoresearcher:", while intro.tex:124 also allows `cryptanalysis:`.
**Action.** "Every number is copied from a committed artefact, except the Kummer-tower last fall degrees of \S\ref{sec:fd-pkm}, which are session-reported (raw data not committed)." Add "or \texttt{cryptanalysis:}" to line 6. In intro.tex:73 write "(6 against 4, session-reported)".

## F16 [severity: major] artefacts.tex:25–62
**What is wrong.** The artefact list is pre-expansion. It has no entries for the rho side, the hardness chapter, the corrections chapter, the machinery chapters, binary_ext, prime_ext, transfer_ext or unconventional. It omits the source directories of headline claims: research/jv_sieve_negrho_20261102/, research/curve25519_rho_parity_bound_20261104/, research/ic_weak_assumptions_20261102/, research/ecdlp_weakness_census_20261104/, research/ecdlp_weakness_lean_20261104/, docs/ecdlp-hardness-corpus/ and research/jksst_xedni_audit_20261101/. "Late measurements (Chapters ch:extension–ch:audit)" no longer describes the part structure.
**Action.** Add a paragraph per new part, naming at least the directories above. Change the heading to name the parts.

## F17 [severity: major] cover.tex:49–61 vs abstract.tex:9–11 and intro.tex:48–49
**What is wrong.** The abstract and intro state the Galois group "for every curve, field and characteristic" with no qualification. The chapter itself says that characteristic 2 needs an expert check of the separability lemma. The theorem is a draft paper's, upheld by one sheriff case, with no Lean or external review.
**Evidence.** cover.tex:213–217: "Two items must be settled before any of this is submitted: an expert check of the separability lemma in characteristic 2, where $E[2]$ is non-reduced; a full literature search." CA:research/sheriff/SHF-20261023-7b9227/REPORT.md C1: "upheld_scoped … primitivity needs m≥4 stated"; C2: "three derivations agree" overclaimed.
**Action.** Abstract: "We give, in a draft proof upheld by an integrity audit (characteristic 2 awaiting expert check), the Galois group …". cover.tex:77: "It is primitive if and only if $m-1$ is odd ($m\ge4$)".

## F18 [severity: major] prog_openproblems.tex:36–44 (item 4)
**What is wrong.** It asks for "a sub-$N^2$ structure for $\Sem4$ membership over a prime-field base" that might change "the $r^{2/3}$ cost". This is stale twice over. First, the $k$-sum table already moved the generic exponent to 0.521. Second, ch:genericfloor shows that any oracle simulable from group operations stays inside the $\sqrt r$ floor, and that a single-target generic IC costs at least $1.5\times$ rho. A sub-$N^2$ table structure built from group operations therefore cannot beat rho.
**Action.** Scope the question to non-generic (coordinate-algebraic) membership tests, and cite rho_side.tex §gf-shoup and Prop. collisionfloor and distance.tex's $k$-sum row.

## F19 [severity: major] prog_openproblems.tex:51–55 (item 6)
**What is wrong.** "move a modelled handover over $\FF_{p^4}$ from about $2^{150}$ to $2^{119}$--$2^{137}$". This contradicts extension.tex:87–88 ("from $2^{150}$ to about $2^{103}$--$2^{119}$, depending on the frame") and prog_frontier.tex:479 ("from $2^{135}$ to $2^{103}$").
**Action.** Use the extension chapter's latest figure: "from about $2^{150}$ to $2^{103}$–$2^{119}$ depending on frame (an extrapolation through a measured constant)".

## F20 [severity: major] conclusion.tex:66–70 (item 3) and prog_openproblems.tex:91–95 (item 8)
**What is wrong.** Item 3 calls the $k=4$ replication "the cheapest experiment that could put a measured crossover on record", and item 8 says a crossover at $q\ge2^{22}$ "would be the programme's first \cls{advance} against rho". Both are written as if nothing below rho is on record. The validated JV sieve row and the preregistered genus-3 decision at $q=2^{16}$ (distance.tex:204–206) both are. The sentence should distinguish a size crossover, an advance class, from a below-rho measurement, accounting class.
**Action.** Conclusion: "the cheapest experiment that could put a measured size crossover (an \cls{advance}) on record; the one validated below-rho measurement (JV sieve, $p=1511$) is accounting, below rho at every size measured". Add the JV sieve counter-emission experiment (open problem 11) as an item, since it is the cheapest way to turn a validated row into a priced advance.

## F21 [severity: major] abstract.tex:46–47
**What is wrong.** "double-large-prime genus-3 index calculus … from $q^{1.91}$ to $q^{1.36}$". This compares a fitted global exponent (1.910) with a local exponent at $q=2^{22}$ (1.356). The fitted exponent after round four is 1.368 over $2^{16}$–$2^{22}$, and double large primes alone gave 1.385.
**Evidence.** prog_frontier.tex:305 "from $q^{1.910}$ to $q^{1.385}$"; prog_frontier.tex:457–458 "local exponent $1.356$ (fitted $1.368$ over $2^{16}$--$2^{22}$)".
**Action.** "from a fitted $q^{1.91}$ to $q^{1.37}$ (local $1.356$ at $q=2^{22}$, still above $4/3$)".

## F22 [severity: minor] prog_method.tex:91–95
**What is wrong.** The incumbent is cited by bound ECBND1h05a2ea918f3c with alpha [0.399,0.512]. FRONTIER.md marks that bound "historical: fit version 1". The current ranked bound is ECBND1hcf69c27b4978 with [0.399,0.503], which is what rho_side.tex:59–62 quotes. The two chapters therefore give different intervals for the same fit.
**Evidence.** C:docs/bounds/FRONTIER.md:73 "0.451 | [0.399, 0.503] … `ECBND1hcf69c27b4978`"; :82 "`ECBND1h05a2ea918f3c` | historical: fit version 1".
**Action.** "(bound \texttt{ECBND1hcf69c27b4978}, fit version 2): … $\alpha=0.451$ $[0.399,0.503]$".

## F23 [severity: minor] prog_method.tex:250–253
**What is wrong.** "against which IC is $84.9\times$ rho". The source scopes this to one crater and $w=0$.
**Evidence.** C:papers/ecdlp-ic-improvement-ledger/sections/gate.tex:16 "at $w=0$ … and $84.9$ on the third volcano's $j=0$ crater".
**Action.** "against which IC is $84.9\times$ rho at $w=0$ on the third volcano's $j=0$ crater".

## F24 [severity: major] prog_method.tex:168–257 (sec:corrections) duplicates audit.tex and corrections.tex
**What is wrong.** Items 5 (unpriced word XORs, 16.7→421.4), 6 (cross-host drift, 120/168, 72/168, BMI2), 8 (lopsided pricing, 0.93/1.53/5.7) and 10 (rule v8, volcano crater, plane-quartic merges) are told again at length in audit.tex:55–151, with the same numbers. corrections.tex:5–9 then points to both.
**Action.** Keep the full account in ch:audit. In prog_method §sec:corrections keep only the pre-ecbench items 1–4, 7 and 9, plus a one-line pointer: "Corrections that moved leaderboard rows (word-XOR pricing, cross-ISA drift, lopsided pricing, rule v8, the volcano and quartic withdrawals) are in Chapter ch:audit; the remaining ones are in Chapter ch:corrections."

## F25 [severity: minor] prog_openproblems.tex:7–17 (item 1), 153–180 (items 12–13); conclusion.tex:61–67
**What is wrong.** Item 1 ("none is proved and the experimental evidence is contested") predates ch:falldegree and ch:hardness. Items 12 and 13 restate the same problem with the new evidence, and the conclusion's items 1–2 do it a third time.
**Action.** Merge item 1 into item 12. Change its first sentence to "No growing lower bound is proved; Chapter ch:falldegree certifies $d_F\ge5$ at $n=33$–45 and Chapter ch:hardness's census gives $\ge4$ at $n=17$–24", and keep the grid experiment as the measurement half.

## F26 [severity: minor] prog_openproblems.tex:113–115 (item 9)
**What is wrong.** "N4~v2 extends this against a negation-map rho to $q\approx2^{22}$, at exponent $0.983$ and $0.005\times$ rho". The 0.005 is at $q=2{,}097{,}091\approx2^{21}$, which is what abstract.tex:61 and distance.tex give. $2^{22}$ is only the top of the ladder.
**Evidence.** C:research/quartic_n4v2_20261020/README.md:48 "At the top sizes DL / rho-neg = 0.005 [0.004, 0.008] (q = 2097091)".
**Action.** "… over $q=2^8$–$2^{22}$, at exponent $0.983$, and $0.005\times$ rho at $q\approx2^{21}$ (L0, no validation file)".

## F27 [severity: minor] prog_openproblems.tex:80–82 (item 8)
**What is wrong.** "before rounds three and four cut a further $0.44\times$". The wording is garbled: the source says the whole solve after rounds three and four is about $0.44\times$ the second round's (derived).
**Action.** "before rounds three and four brought the whole solve to about $0.44\times$ the second round's at $q=2^{22}$ (derived, in the inversion-charging unit)".

## F28 [severity: minor] model.tex:84–85 vs intro.tex:80–81, abstract.tex:40–41, conclusion.tex:28–29
**What is wrong.** model.tex has "Oracle speed-ups of three to five orders of magnitude"; the other three have "two to five orders". The comparable F4-baseline oracle ratios are 383–1,535x (2.6–3.2 orders) and 3.5–4.7·10⁴x against `groebner_f4`. Five orders needs the PS-0 wall ratio (115,160x), and the 3,021–5,802x figures are against Buchberger (prog_method item 4).
**Action.** Use one phrase everywhere: "between about $400\times$ and $5\cdot10^4\times$ per decomposition against F4 (up to $10^5$ in wall time against the PDP baseline)".

## F29 [severity: minor] conclusion.tex:32–35
**What is wrong.** "None of this changed an exponent on a prime field … The prime-field $k$-sum oracle moved its exponent from $0.62$ to $0.52$". The two sentences contradict each other as written.
**Action.** "None of this moved an exponent below rho's on a prime or prime-degree binary field. The generic $k$-sum oracle moved its own exponent from $0.62$ to $0.52$, but its limit is baby-step giant-step, and the collision floor (Chapter ch:genericfloor) keeps any generic single-target arm at least $1.5\times$ rho."

## F30 [severity: minor] conclusion.tex:21 and intro.tex:73, falldegree.tex:648
**What is wrong.** "measured for prime-field Kummer towers (6 against 4 at $t=10$)". These values are session-reported, never committed (falldegree.tex:565–570; open problem 14), so they are not a measurement on record.
**Action.** "session-reported (raw data not committed) for prime-field Kummer towers".

## F31 [severity: minor] abstract.tex:63–64
**What is wrong.** "$0.0065\times$ a matched negation rho with the sieve route (validated, $p=1511$)" quotes only the most favourable unit. The same validated row is $0.016\times$ in $\FF_p$ multiplications and up to $0.12\times$ at $w=1$ under the upper-bound layer. The validator also classes it as accounting.
**Evidence.** C:research/jv_sieve_negrho_20261102/README.md band table: layer A U1 "0.0065 … 0.107 … 0.121"; VALIDATION.json "Accounting, not an improvement".
**Action.** "$0.0065\times$ a matched negation rho in group additions ($\le0.12\times$ across the band; validated as accounting, $p=1511$)".

## F32 [severity: minor] prog_theory.tex:114–115, 124–127
**What is wrong.** Two results have no citation and no standing. "a 2024 analysis again concludes that no subexponential prime-field method is known" has no citation. "A 2026 structured-generic-group bound shows … $\Omega(\min(\sqrt q,1/\delta))$" is uncited and stated as shown. The lead atlas records that its body was not read (KN-LIT-7606, Corrigan-Gibbs–Henzinger–Wu, ePrint 2026/384), and whether it covers coordinate access is an open lead.
**Evidence.** CA:knowledge/leads/ecdlp/exotic/LEAD-EXO-coordinate-revealing-oracles.yaml: "read the body of KN-LIT-7606 … the record states the body was not read".
**Action.** Cite both. Write "a 2026 structured-generic-group bound (Corrigan-Gibbs, Henzinger and Wu, ePrint 2026/384; abstract-level, body not read) states …".

## F33 [severity: minor] prog_openproblems.tex:1–4
**What is wrong.** "% Survey section (draft 2026-10-15)" is a stale comment. The items now cite November results.
**Action.** Update the comment date, or drop it.

## F34 [severity: minor] intro.tex:87–96 (paragraph 5) and abstract.tex:58–66
**What is wrong.** The intro lists "the Joux--Vitse cover" and "possibly $k=4$" with no number and no standing. The abstract gives the $k=4$ item as "unsupported beyond $m=15$" but not its distance ($5.2\times$ after corrections, crossover extrapolated near $m^*\approx17.6$). Abstract item "genus 3 via GHS: below the strongest rho at one size, unresolved at the others" omits "above it under the heavier sensitivity pricing", which open problem 8 and distance.tex both state.
**Evidence.** C:research/gaudry_k4_char2_redteam_20261026/REPORT.md:37,56 "**5.2** … m* is about 17.6".
**Action.** Abstract: "$k=4$ in characteristic two: $5.2\times$ rho at $m=15$ after corrections; the crossover near $m\approx17.6$ is an extrapolation"; and "genus 3 via GHS: below the strongest rho at $q=2^{16}$ under the registered pricing, unresolved at other sizes, above under the sensitivity pricing".

## F35 [severity: minor] hardness.tex:28–43, §hd-lean
**What is wrong.** Theorems are labelled "kernel-checked" from the producer's local `lake env lean -o` compile. The chapter admits that no proof-gate run is recorded for ecdlp_weakness_lean_20261104 and that pcc claims are "pending CI signing". The theorem headers (e.g. Theorem hd-subspace "kernel-checked") do not carry that qualifier.
**Action.** Write "kernel-checked (producer's compile; no gate run recorded)" in each theorem header of that project, or define the term once with that scope in the Standing paragraph.

## F36 [severity: minor] model.tex:42–64 (Table tab:levers)
**What is wrong.** The lever table predates the newer chapters. It omits:
- the $k$-sum table (prime $C_{\rm dec}$/$N$);
- the Hilbert early stop and post-Gröbner linear stage (validated components);
- the JV sieve route;
- the collision floor as a fixed-cost bound.
**Action.** Add these to the matching rows, with section references.
