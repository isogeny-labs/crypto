# Review findings: group `algos`

Sections: prog_rootfinding, prog_groebner, machinery, prog_koblitz, prog_binary, binary_ext.
Sources read with `git show origin/main:<path>` in /home/user/crypto and /home/user/crypto-autoresearcher.

Numbers re-read from primary sources: 46. They matched except where a finding below says otherwise.
Matched: generator-orbit n=29 (0.546/0.611/1.196 vs 1.569, 0.366 [0.241,0.523], like-for-like 1.503/1.58, 0.413/0.478/1.063); accounting v2 (10.6–11.0x, 34–49x, 504/504, 1.7–8.8%, 0.720 [0.591,0.867]); statistics v2 (1.351→1.429, 1.892→2.486, 6.98→648, 20.26 vs 0.627, 71/71); allocator (16.85/17.28/21.16 vs 4.00/4.14/5.36, 127.1→119.1, 18.4–38.8x); child overhead (4.26M, 169.47M→9.86M, 37.20→2.92 ms); msweep (17.3 s, 2^75.3, 188.5/175.2 vs 1.509 s); PMULL/sign-fold; tab:kc-orbit (all 8 rows); tab:kc-lp top half and tuning half; tab:kc-routing (all 6 rows, first study); confirm16 table; chain-SAT order/meet; E31 (2.62–2.69x, 2.50–2.56x, best 4.42e3); E32 (−0.243, 2^85.5 [79.2,94.3], 91.2, 97.6); E33 row; f4_fp cover table; trace-replay e2e (0.2837, 1.567, 27.9/1.83e4/1.13e5); tab:bo-sat; tab:bo-harvest; F6 ladder (54.75, 2,091, 1.105, 1.582); cross-family (0.6465, 0.6414, 198.6, 222.1, 64.6–68.8, 1,303–1,353); rootfinding oracle (383x, 1,535x, 3.1 µs/482 µs, 3,007–115,160x, 197 vs 8.99e13, 707/707); pdp ecbench (350x, 2.9–25,116x); Koblitz loop (38.82, 28.38, 0.828, 23.22, 22.97, 0.0545, 14.82); free-relation census table; 0.960 [0.946,0.976]; torsion n41/n37; Semaev 154/46; degree-5 C1 (607,530/760,099, 31,632 s, 52.6 GB); guess ratios 78.2/5.09/1.30/0.66; symlp table; LA (10.40x, 1.93x, 1.206x, 5.72x); explicit relations (1,053.9 s, 11.05%); hashed ECDL; n83 index-capacity figures; top-form bounds; learned cost model (+1.83/+0.80/+0.80, +0.96/+2.47/+2.47, 1.8–17.9 bits).

---

## F1 [severity: blocker] prog_koblitz.tex:266
**What is wrong.** "They kept kernel gains (queries up to $2.998\times$ faster, resident memory $-52\%$)". 2.998x is the *smallest* exact-query gain of the PMULL study. The largest is 6.868x. machinery.tex:842–844 gives the correct range (6.11–6.87x).
**Evidence.** research/f6_n83_pmull_xonly_20261005/RESULT.md:28–32: "| 258 | Exact query ... | 6.110× |", "| 1,048 | Exact query ... | 6.868× |", "| 4,054 | Exact query, two-run median | ... | 2.998× |".
**Replacement.** "They kept kernel gains (exact queries $3.00$--$6.87\times$ faster with ARM64 PMULL, resident memory $-52\%$)".

## F2 [severity: blocker] machinery.tex:1686–1689
**What is wrong.** "The largest *verified* end-to-end engine change in this part is a factor of about 5". The factor of 5 is the f4_fp cover study (\S fe-e2e). That study has no VALIDATION.json, and the chapter itself calls its S/rho a lower-bound comparison. tab:in-summary marks only validated rows as validated, and this row is not marked. The sentence therefore overstates its standing.
**Evidence.** research/f4_fp_ic_effect_20261018/README.md:18: "Class engineering." There is no VALIDATION.json in the directory. The validated end-to-end rows are E31 (benchmarks/20261024-e31-chain-solve/VALIDATION.json: S falls 2.50–2.56x at 0 GAE and 1.034–1.053x at 1 GAE) and generator-orbit (1.26–3.20x).
**Replacement.** "The largest end-to-end engine change in this part is a factor of about 5 (the \texttt{f4\_fp} cover pipeline; producer-reported, lower-bound comparison). The largest validated ones are the generator-orbit base ($1.26$--$3.20\times$) and E31 ($2.50$--$2.56\times$ at 0~GAE, $1.03$--$1.05\times$ at 1~GAE)."

## F3 [severity: blocker] prog_rootfinding.tex:204–211
**What is wrong.** The passage reads: "A compact-orbit base ... reaches $S=15.55$, below the then-unpriced F4 ($16.75$) ... and it is $1.08\times$ rho. Amortised over 64 targets that arm reaches $S\approx11.2$, below rho". Both comparisons divide one lower bound by another, and no band is given. In the source the strong rho is itself a lower bound ("rho-strong | 14.822 | yes"), and so are the compact arm and the pre-#1774 F4. The "below rho" wording breaks the band rule. The 1.08x also mixes the v2 arm value (15.552) with the v3 rho (14.82). The source's v3 figure is about 1.06x.
**Evidence.** research/ecbench_rootfind_faster_20261014/README.md: "| ic-c2-frob-counted-m2 | 15.552 | yes |", "| ic-f4-m3-d5 | 16.749 | yes |", "| rho-strong | 14.822 | yes |". Also: "Best LB compact-frob is ~1.06× rho but remains a lower bound ... Amortized secondary amS@64 for counted-c2 ≈ 11.2 (beats rho) — not a cold claim".
**Replacement.** "A compact-orbit base with a counted Frobenius pair table reads $S=15.55$ against F4's then-unpriced $16.75$ and about $1.06\times$ the strong rho. All three figures are lower bounds (factor-base construction natives, F4's word XORs and rho's table work unpriced), so neither order is ranked. Amortised over 64 targets the same arm reads $S\approx11.2$. That is a ratio of lower bounds with no band, and the ledger counts it as a secondary, multi-target diagnostic, not a result below rho."

## F4 [severity: blocker] machinery.tex:257–261
**What is wrong.** The text quotes "$3.43\times$ on the admitted Koblitz ladder" as one of the closest markers to rho. 3.43x is the lower end of Table A rule v7. prog_koblitz.tex:362 and note (a) say that rule's rho reference was "$0.57$--$0.85$ of strong walk ... Per step, so the distance was understated". prog_koblitz.tex:402–403 also says that Table A understates its gap by 1.7–2.0x and that the 24 October gate did not check it. Calling the figure "admitted" and listing it as a closest marker contradicts the treatise's own correction.
**Evidence.** research/ecbench_e2e_frontier_20261031/REPORT.md:43–45 ("3.43× on the admitted Koblitz ladder") is the report's wording. research/ic_tool_program/rule/v4/README.md:10 gives the canonical-step gap as "1.76 to 1.94 times the as-built figure".
**Replacement.** "... and $3.43\times$ on Table~A's rule-v7 Koblitz ladder, whose rho reference was $0.57$--$0.85$ of the strong walk per step, so that figure understates the gap by about $1.7$--$2.0\times$ (\S\ref{sec:tablea}, \S\ref{sec:kc-rho}); the report states that these are different studies and units".

## F5 [severity: major] machinery.tex:690–725 and binary_ext.tex:1353–1384 (duplication)
**What is wrong.** The 5 November few-candidate F6-IC closure is told twice at length, almost word for word. Both copies give CPU 2.67–10.29x, worst pairing 1.66x, word XORs 1.84–10.13x, 204–1,049→5–16 reductions, inherited-F4 control 0.87–1.05x, unchanged pair lookups and group additions, the "is the mitm oracle, unverified" inference, Four-Russians batching at 32 row ops per pivot against 1,500 XORs, and the parallel split at 1.59–2.88x and 0.94–1.11x. The two cross-reference sentences do not prevent the repetition. Only binary_ext says the closure is now on by default. Only machinery gives the counted-work cost of the parallel split.
**Action.** Keep the full account in machinery §bo-f6, Step 3, and add "now on by default (KIC_F6_FEW_CANDIDATES=64)" there. Cut binary_ext §bd-f6 to about three sentences: the identity "with the pair index, F6-IC is the pair-index MITM scan" plus a pointer to \S\ref{sec:bo-f6}. Move its final paragraph (see F17) elsewhere.

## F6 [severity: major] machinery.tex:891–917 (tab:bo-chain) and binary_ext.tex:880–925 (tab:n83l-sat) (duplication)
**What is wrong.** The two tables cover the same six chained-$\Sem3$ SAT studies (order, meet, rebuild, indexed, parallel, parbuild) with the same numbers: 12.8→6.2 s, 0.68–958.8 s, 0.91–81.5 s, 78.4→39.7 s, 41.7→11.4 s, 57.0→13.7 s, 1,006.8→287.7 s, 3.50x. The follow-on paragraph is also repeated: "by the last step no decided target makes a SAT call ... generic meet ... matched strong rho about 1.5 s".
**Action.** Keep tab:n83l-sat in binary_ext, which is more complete (it has the order-study medians, the lookup-domain sizes and the 167-point base). In machinery, replace tab:bo-chain and its paragraph with two sentences: "Six follow-ups extended chained SAT to $m=9$ by putting root-finder tables and a $|B|^d$ meet-in-the-middle lookup in front of it; by the last step no decided target makes a SAT call (Table~\ref{tab:n83l-sat}). The engine became a generic meet in the middle."

## F7 [severity: major] machinery.tex:1298–1393, binary_ext.tex:1095–1184, prog_frontier.tex:650–671 (triplication)
**What is wrong.** The $\ell=8$ guessed-variable closures are told three times:
- the four-guess (7,15,23,24) result, 64/64 branches, 10.6 GB, 124,314 columns;
- the three-guess failure with closure ranks 128,213–129,893 of 136,698;
- the (22,7) null;
- the invariant-guess series with p=0.031;
- the second-link guess with PCC-20261106-607061;
- the ECC2K-130 ratio $78.2/5.09/1.30/0.66$ with $k\approx105$–108;
- the blocked high-arity v4 launch.

The 17 October specialisation null (dreg_closure_specialise) also appears twice (machinery 1386–1393, binary_ext 1179–1183).
**Action.** Keep the full account in binary_ext §bd-guess, which has tab:bd-guess. In machinery §in-closure keep only "The instrument" and "A null control that failed" (the controls lesson), and replace the rest with a pointer to Table~\ref{tab:bd-guess}. Delete the specialisation paragraph from binary_ext (machinery uses it as the instrument lesson). Cut prog_frontier's $\ell=8$ sentences to a pointer.

## F8 [severity: major] machinery.tex:1395–1420 and binary_ext.tex:1625–1650 (duplication)
**What is wrong.** The linearization-excess rule $D=2+[e<-3.5]+[e<-8.5]$ is stated twice: 656 bases, MAE 0.06/0.08, the $\dim V^{(2)}\ge2\ell-1$ progression result, 15–40x cheaper, the homogeneous regularity $\ell+1$, and the $m=3$ formulation point.
**Action.** Keep binary_ext §bt-fbselect "A structural law for $m=2$", which has more detail (up to 100x, 500–1,500x at $m=3$). Reduce machinery §in-excess to its atlas-lead sentence (open, unexamined) and a pointer.

## F9 [severity: major] machinery.tex:818–829 and binary_ext.tex:1319–1350 (duplication)
**What is wrong.** The ML-guided SAT ceiling study (pdp_ml_sat_ceiling_20261027) is told twice: 0.963/0.998 conflicts per triple, within 4.1%, 5.2–9.0x prize, AUC near 0.5, positive control 0.72–0.79, closed by rule.
**Action.** Keep binary_ext §bd-sat. In machinery §bo-sat keep the atlas identity (the CNF-XOR model is a complete enumerator) and end with "measured directly in \S\ref{sec:bd-sat}: about one conflict per sorted triple, and no learnable signal".

## F10 [severity: major] prog_koblitz.tex:327–337, machinery.tex:865–890, binary_ext.tex:652–655 and 876–881 (triplication)
**What is wrong.** The 24 October field-truncation correction is told in full in prog_koblitz and in machinery: the engines listed, "proved UNSAT" on planted relations, the 64-bit word, 17.3 s per query, $2^{75.3}\times$ rho, 175–189 s against 1.509 s, 116–125x. It is restated again at the start of two binary_ext sections and in corrections tab:corr-xref.
**Action.** Keep the full account in machinery §bo-n83. In prog_koblitz cut it to one sentence and a pointer: "On 24 October every Weil-descent engine's $n=83$ refutation was voided by a 64-bit field-table truncation (\S\ref{sec:bo-n83}); after the fix the $m=4$ root finder solves in 175--189\,s, $116$--$125\times$ strong rho."

## F11 [severity: major] prog_binary.tex:65–70 (stale; contradicts prog_binary.tex:116–117 and newer chapters)
**What is wrong.** "the hybrid gives $+8.43$ bits if the number of guesses stays flat in $\ell$ and $+58.27$ if it grows linearly, which only a run at $\ell=8$ can separate". The $\ell=8$ runs have been made: prog_binary.tex:116–117 says so, as do prog_frontier:661 and binary_ext:1170–1172. binary_ext concludes that they do *not* separate the rules, because $k$ at $m=4$ is unmeasured. machinery.tex:1300–1309 quotes the stale sentence and adds "Chapter corrections lists the staleness" instead of the text being fixed.
**Evidence.** research/dreg_closure_guess_20261022/RESULTS.md:74 ("Both registered rules grow linearly in ℓ and give k ≈ 105–108 there") and research/dreg_closure_guess3_20261023/RESULTS.md.
**Replacement (prog_binary 68–70).** "... and $+58.27$ if it grows linearly. The $m=3$, $\ell=8$ runs since made bound the count at 4 but do not measure it at $m=4$, so the range stands (\S\ref{sec:bd-guess})." Then drop machinery's quotation and its "lists the staleness" sentence (1300–1309) and replace them with a pointer.

## F12 [severity: major] machinery.tex:640–643, 935–937, tab:bo-harvest last column, tab:in-summary harvest row; prog_rootfinding.tex:190–191, 203–205 (missing reference version)
**What is wrong.** Several distances to strong rho do not say which strong reference they use: "inherited F4 is about $22$--$33\times$ the strong rho at $n=13$", "$22$--$33\times$ ... to $10^2$--$10^4\times$ rho", "$7$ to $1{,}250$ times the strong rho", "2.9–25,116x the strong rho", "still $1.55\times$". All are against the frozen, set-up-inclusive strong reference. machinery §em-accounting (line 92–100) shows that accounting v2 makes that reference about 5x cheaper at $n=13$ (2.0–2.3x → 10.6–11.0x for the best IC arm) and that the corrected gap is larger. Unlabelled, these figures read as current distances and understate the gap.
**Evidence.** research/ecbench_accounting_v2_20261023/REPORT.typ:141: "10.6 to 11.0 times strong v2 at n = 13 (2.0 to 2.3 times the frozen strong) and 34 to 49 times at n = 19 (10 to 11 times the frozen strong)".
**Action.** Write "frozen strong rho" (or "strong rho, frozen accounting") at each occurrence, and add once in §bo-summary: "against the v2 reference of \S\ref{sec:em-accounting} these distances are roughly $3$--$5\times$ larger at $n=13$--19."

## F13 [severity: major] prog_koblitz.tex:30–34 (missing standing label; stale)
**What is wrong.** "In the ecbench factor-base suite it is the best measured Koblitz construction: $S=9.114$ at $n=17$ ..., $9.985\times$ matched signed-Frobenius rho". The source labels this cell a lower bound from an L0 operation smoke. It is also superseded: binary_ext tab:kc-orbit (14 October, fully priced) has the orbit pipeline at $1.719$ [0.810,2.980] of signed-Frobenius rho at $n=17$, and the generator-orbit base (machinery §em-frontier) is lower still.
**Evidence.** research/ecbench_frontier_20261013/README.md:27 ("operation smoke, L0; not a bound") and :87 ("best measured arm in the matched cell; lower bound").
**Replacement.** "In the 13 October ecbench factor-base suite it was the best measured Koblitz construction: $S=9.114$ at $n=17$ ..., $9.985\times$ matched signed-Frobenius rho (L0 operation smoke, a lower bound). The fully priced orbit ladder of 14 October and the generator-orbit base of 22 October are later and cheaper (\S\ref{sec:kc-closure}, \S\ref{sec:em-frontier})."

## F14 [severity: major] binary_ext.tex:1717–1720 (below-rho wording on a model)
**What is wrong.** "Composite $n$ is the one binary setting where index calculus can be priced below rho on time alone." The basis is a zero-experiment cost model whose verdict depends on an unmeasured $c_{\text{trial}}$, and the same model is worse than rho under time×memory on every row. The sentence also calls the c2pnb lift-dense base "$+0.50$ bits, modelled". The first clause reads as a measured or ranked result.
**Evidence.** crypto-autoresearcher:analysis/ecdlp2m-iteration-20261009/c1-gaudry-n4-c2pnb/README.md (cited at 1727) is a "zero-experiment derivation". Text at 1747–1750: "The verdict turns on $c_{\text{trial}}$, never measured".
**Replacement.** "Composite $n$ is the one binary setting where a cost model puts index calculus below rho on time alone, and only for an unmeasured per-decomposition cost; under time×memory the same model is worse on every row."

## F15 [severity: major] machinery.tex:1040–1046, 1159–1166, 1677–1680; binary_ext.tex:401–408; machinery.tex:281–285 (pcc standing)
**What is wrong.** Rows called "validated" or "verified improvements" (E31, E39, E40, generator-orbit, the Koblitz two-large-prime component) are cited by VALIDATION.json only, with no pcc claim id or pcc standing. The 4 November rule (CLAUDE.md: "Cite a result by its pcc claim id and standing, or label it an unverified assertion") requires one or the other. binary_ext does this correctly for PCC-20261106-607061; machinery's "verified improvements of the index-calculus baseline" (1159–1161) does not.
**Action.** For each row, add the pcc claim id and its standing if one exists. Otherwise append "(validation-gate pass; no pcc claim, so an unverified assertion under the 4 November rule)". At least change "verified improvements" to "validation-gate improvements".

## F16 [severity: major] binary_ext.tex:626–650 (gap)
**What is wrong.** The batch section omits the one study that answers its question directly: research/ic_many_target_crossover_20261030. It is not cited anywhere in the treatise.
**Evidence.** README: "Amortising relation collection over k targets never makes IC cheaper than batch rho at any k ... G = 3.0 to 4.1 at n = 41..61 ... Section 5a count band ... G stays above 1 at n = 41, 43, 53, 61 (3.1 to 4.8 at w = 0, still 1.6 to 2.4 at w = 1) ... At n <= 45 and at n = 47 ... unresolved by counts ... n = 83 ... G = 4.8 to 5.1 for K3300". It is classed as accounting/audit.
**Action.** Add a paragraph to §kc-batch with these figures and their class (accounting/audit, not an improvement), and a row to tab:kc-summary.

## F17 [severity: minor] binary_ext.tex:1386–1393 (clarity, misplaced)
**What is wrong.** "The same conclusion holds from the cost side. A desk evaluation of 28 candidate point-decomposition strategies ..." does not follow from the F6-IC section. "On the ladder rows the walk is 61--82\% of $S$" names no ladder. The "ceiling of $1.00\times$" is for algebraic oracles in general, not F6.
**Action.** Move the paragraph to the start of ch:binarydegree or to §bt-fbselect, and name the ladder and source of the 61–82% figure.

## F18 [severity: minor] binary_ext.tex:1717ff., prog_binary.tex:65–70, prog_koblitz.tex:436–437 (notation)
**What is wrong.** "$+45.71$ at $w=2$" uses $w$ for an exponent or touch price. Everywhere else $w\in\{0,0.1,1\}$ is the §5a band weight. prog_frontier:657–658 explains it as "the exponent the measured closure tracks". A reader will take $w=2$ for an out-of-band weight.
**Action.** Rename it, e.g. "$+45.71$ bits with the closure exponent $\omega_c=2$", in all three tables and texts.

## F19 [severity: minor] prog_groebner.tex:305–318, machinery.tex:672–682 (notation)
**What is wrong.** "$m=13$", "$m=19$", "$m=31$" denote the field degree $n$. In the same chapters (and in tab:prog_groebner-engineering, where both uses appear) $m$ is the number of summands. prog_groebner's caption defines "$m$ the number of summands" and then uses $m=19$ for a field degree.
**Action.** Write $n=13$, $n=19$, $n=31$ for field degrees, or add a note that the source labels field degree $m$.

## F20 [severity: minor] machinery.tex:1378–1380 (internal inconsistency)
**What is wrong.** "Even a bounded $k$ only recovers the degree-4 row, $2^{+7.62}$ over rho". With $k\approx4$ guesses the hybrid pays $2^k$ and the row is "$+8.43$ bits if the number of guesses stays flat" (prog_binary:68, prog_frontier:661, binary_ext:1172). +7.62 is the $k=0$ floor.
**Replacement.** "Even a bounded $k$ gives at best the degree-4 floor, $2^{+7.62}$ over rho, and $+8.43$ bits at the measured $k$".

## F21 [severity: minor] machinery.tex:1036–1039 (attribution)
**What is wrong.** "B, with its selection pass priced, falls behind E25 at 0.1 and 1 GAE and is unresolved (\art{research/e31\_chain\_solve\_20261024/}; validated ...)". E31 did not price B's selection. Its README says "E31 is below E25 on every row at every weight", including B at 1.051–1.071x. The priced statement comes from E33.
**Evidence.** research/e31_chain_solve_20261024/README.md:15–17. research/e33_solver_b_pivot_count_20261031/REPORT.typ:34: "B, B′, B″ against E25: unresolved (each is behind E25 at 1 GAE)".
**Replacement.** "B is ahead of E25 at every weight as E31 counted it, but with its selection pass priced (E33, below) it falls behind E25 at 0.1 and 1~GAE and is unresolved."

## F22 [severity: minor] binary_ext.tex:314–315 vs prog_koblitz.tex:358–364 (consistency)
**What is wrong.** "Table~A prices the ledger \S23 one-target Koblitz pipeline at $4.7$--$11.8\times$ rho". No rule in tab:prog_koblitz-tablea has that range: v3 is 5.23–21, v7 3.43–11.51, v8 4.67–18.07. Separately, prog_koblitz gives v4 as "$9.9$--$43\times$" (the point column, 43.46) while binary_ext:272–274 gives 9.89–42.12 (the bootstrap column).
**Evidence.** research/ic_koblitz_descent_routing_20261026/README.md:14 (4.7–11.8x, the routing study's quotation of Table A). research/ic_tool_program/rule/v4/README.md:94 ("**43.46** | **42.12 [37.09, 49.28]**").
**Action.** Say which Table A revision the 4.7–11.8 refers to, as quoted by the routing study. Use one v4 column in both chapters.

## F23 [severity: minor] prog_groebner.tex:290 (wrong cross-reference)
**What is wrong.** "found one soundness bug (\S\ref{sec:corrections}, item 6)". Item 6 of prog_method §corrections is "Cross-host counter drift". The soundness bug is item 7. machinery:647 correctly uses item 6 for the drift.
**Replacement.** "item 7".

## F24 [severity: minor] machinery.tex:523; binary_ext.tex:705 (dead artefact paths)
**What is wrong.** \art{research/f5_support_cut19_final_recheck_20261003/README.md} does not exist on main; the file is RESULT.md, and its numbers match. \art{docs/IC_BENCHMARK_TIERS.md} does not exist on origin/main.
**Action.** Change the first to .../RESULT.md. Find the tier definition's real path (or the n83 study that defines "Tier 1") and cite that.

## F25 [severity: minor] machinery.tex:663, 765, 835, 1299; binary_ext.tex:877, 1096, 1320, 1354 (clarity)
**What is wrong.** Each of these sections opens with a bare cross-reference sentence placed directly under \section and followed by a double blank line, for example "Its results for binary decomposition are in \S\ref{sec:bd-f6}." These read as editing seams.
**Action.** Fold each pointer into the section's first paragraph, or into a footnote. Once F5–F9 are applied, most of them become unnecessary.

## F26 [severity: minor] prog_rootfinding.tex:189–191 (precision)
**What is wrong.** "stays $2.9$--$25{,}116\times$ the strong rho in $S$ on binary curves". The ladder is Koblitz ($n=13$–37, strong signed-Frobenius rho), not generic binary. The source also attributes the jump at $n=31$ mainly to the 1,492 cofactor.
**Evidence.** research/pdp_root_finder_ecbench_20261013/README.md:32–35.
**Replacement.** "... on Koblitz curves $n=13$--$37$ (the jump at $n=31$ is mostly the curve's cofactor) ...". Also label the reference "frozen strong" (F12).

## F27 [severity: minor] prog_rootfinding.tex:128 (wording)
**What is wrong.** "Validated in host emulation only (no GPU available)." Elsewhere in the treatise "validated" means a validation-gate pass.
**Replacement.** "Checked in host emulation only (no GPU available); no validation file."

## F28 [severity: minor] binary_ext.tex:404–416 / ic_m2 cycle family (gap)
**What is wrong.** §kc-torsion reports the $m=2$ cycle-stop family but omits three companion studies on the same $a=0$, $n=37$ curve: research/ic_m3_cycle_n37_20261101 ("relations with three summands cost 26 to 43 times more decomposer calls than the two-summand relations ... All 18 pairs"), research/ic_mix_cycle_n37_20261103 (no mixture fraction beats $q=0$, prediction confirmed) and research/ic_s4_direct_n37_20261101 (a direct $\Sem4$ solve cannot run at $n=37$; analysis only).
**Action.** Add one exploratory paragraph with those three results and their standing (local, no replay certificate).

## F29 [severity: minor] binary_ext.tex:1700–1702 vs prog_koblitz.tex:411 (rounding)
**What is wrong.** "rho's $2^{60.9}$" (QSP) against "$2^{60.81}$" everywhere else. The source uses 60.9, so this is rounding, but the two figures sit two pages apart.
**Action.** Write "$2^{60.81}$ ($2^{60.9}$ in the source)", or use 60.81 throughout.
