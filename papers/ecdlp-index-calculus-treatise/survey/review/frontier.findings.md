# Review findings: group `frontier`

Sections: rho_side, summer, prog_frontier, unconventional, distance, audit, corrections.
Primary sources were read with `git show origin/main:<path>` in /home/user/crypto (HEAD aaa12fefa8, 5 Nov) and /home/user/crypto-autoresearcher.

Numbers re-read from source and found correct (32): rule v8 4.67 [4.34,5.07]…18.07 [15.98,20.58], slope 0.1679 [0.128,0.208], gap change 1.30–1.58, canonical-step model 6.45–23.32, 408/408 rows (rule/v8/README.md); n83 K1107 BL arms 40.9/4.10/33.1 with CIs, cold 155.5 s vs 36.96 s (n83_k1107_bl_reproduction_20261028/RESULT.md); K1107 saving 1.195 s [0.957,1.433] (n83_cold_base_confirm_20261021/RESULT.md); K1120 55.4 (= 119.826/2.164); k-sum 1.9×/7.2×, 83.7→11.6, α 0.620 [0.586,0.650] → 0.521 [0.506,0.538], 5.3×/13.1× (prime_ksum_table_20261028/README.md); base tuning 0.818 [0.773,0.866], 7.75 [6.92,8.70], 9.47 (leaderboard.json prime_base_size_component); Table B prime 5.47–13.46, binary 10.62–38.21, Table A 3.43–11.51, Table C 3.62–14.90, 19 of 28 validated (leaderboard.json); campaign table values and exponents 0.620 [0.557,0.663] / 0.435 (ic_frontier_campaign_20261015/README.md); genus-3 0.697/0.710/0.696, slope 0.0339 [−0.0465,0.1050], c_ℓ=9 range 2.66–12.42 (ghs3_vs_rho_20261023/TABLES.md); JV 1.59–1.98, slope −2.05, p*≈1330, pooled 1.344 (jv_cover_p_scaling_20261024/README.md); JV sieve 0.00647 [0.00545,0.00799], ×155 [125,184], VAL-20261103-c31c76 exists; quartic DL 0.005 [0.004,0.008], q* 555 [389,752], 501 units / 2,442 rho runs, exponents 0.983/1.543 (quartic_n4v2_20261020/README.md); summer counts 74/364/964 = 1,402, 561/267/124/157, 672 (ecdlp_jul_sep_index_20261029/SUMMARY.json, REPORT.md); DEC-20260727-008 2287; DEC-20260801-005 0.0472 / 0.0311; EV-ICPERF-784b25 +6.3 / +40.8; EV-FROB-d336b0 57.0–96.0, 72.7–121.7; calibration table (ecbench_calibration_20261002/README.md, all 11 rows); Cheon table (cheon_ecbench_20261022/REPORT.md, all rows); FRONTIER.md 0.529, 0.398, 0.431, 0.451, 1.089, 2.203, 15.558; rho_assurance 34 theorems, 41 configs, 0.994/0.922/0.943, 1,196→938; bounds challenges 0.8250, 15.79, 0.2991; E27 1.453–1.463, 1.448–1.456, 6.39e3–4.06e5, VALIDATION.json present; curve25519 parity 1.517/1.889, 1.82/1.86/1.94/1.81; exotic dispositions 26/2/2/1/1 (32 YAMLs); LEADS.md §9 errata (9–30×, 3,364, 11.9k, 1.4× vs 3.42×, c≥1, 256c²); byte-counter 83–99 %, 17–58× vs 2.6–5×, 2 of 8 vs 6 of 8; torsion 0.975 [0.641,1.420] (bootstrap_recheck.txt), 13.3 [10.3,17.5]; Semaev t=2 154/46 of 200; f4 harvest +7.0 %/−0.3 %/−4.1 %, 7–1,250×; accounting v2 0.182/0.223/0.678 and 83.3→39.9 %; statistics v2 6.981→648.2 (92.9×), 60.4×; cofactor clear 6.180 [5.915,6.456], 1.149.

---

## F1 [severity: blocker] audit.tex:211
"\textbf{No validated row is below its reference.}" The treatise reports one validated measurement below a matched rho. prog_transfer.tex:282 says "It is the programme's one validated measurement of index calculus below a matched rho". corrections.tex:45 says the same, and abstract.tex:63–64 says "$0.0065\times$ a matched negation rho with the sieve route (validated, $p=1511$)".
Evidence: research/jv_sieve_negrho_20261102/VALIDATION.json exists, and review/validator/REPORT.md records VAL-20261103-c31c76. README: "the sieve route is **below** rho in every cell … The order (sieve below rho) holds across the band in all four cells."
Replace with: "No validated leaderboard row is below its reference. The one validated measurement below a matched rho is the Joux–Vitse sieve route on the `jv_cover` weak class ($0.0065\times$, $p=1511$, \texttt{VAL-20261103-c31c76}; \S\ref{sec:transfer}). It is not a leaderboard row and never has prime order."

## F2 [severity: blocker] audit.tex:7–9
"Every correction moved IC's position against rho \emph{down}, and several moved it from apparently ahead of rho to behind it." The treatise's own corrections appendix contradicts this:
- corrections.tex:133–148 and 156: under byte-counter option B the residual arms went from "res4/rho $41$--$58\times$" to "$3.6$--$4.8\times$". IC moved up.
- corrections.tex:45: the JV sieve went from "comparisons withheld" to "below rho in every cell … validated".
Evidence: research/ic_byte_counter_pricing_20261101/README.md:19 says "their w > 0 distances to the matched rho are 17-58x rather than 2.6-5x".
Replace with: "Most corrections in this chapter moved IC's position against rho down, and several moved it from apparently ahead of rho to behind it. Appendix~\ref{ch:corrections} records two that moved it up: the byte-counter pricing rule and the matched-rho re-measurement of the Joux–Vitse sieve."

## F3 [severity: blocker] prog_frontier.tex:756–758
"is $53$--$221\times$ slower online than preprocessing rho given equal or less advice". The same chapter (line 368), audit.tex:43 and the source all give 53–317.
Evidence: benchmarks/README.md:225 says "audit: BL with equal or smaller advice is 53–317× faster online than IC".
Replace "$53$--$221\times$" with "$53$--$317\times$".

## F4 [severity: blocker] prog_frontier.tex:309–311
"…the only one where measured IC beats a rho arm: the item-7 rho, which walks at $1.75\sqrt\ell$ steps, at $q=2^{15}$ ($0.69$ $[0.55,0.83]$)."
Two problems:
- "beats" is used against a rho that is not the strongest (1.75√ℓ against an ideal of about 0.89√ℓ), and no band is stated, so the §5a rule is not met.
- "the only one" is stale. The same chapter later reports the JV sieve below a matched rho across the band (lines 695–698), and distance.tex reports the plane quartic at 0.005.
Replace with: "…and the first where measured IC cost fell below a rho arm, the item-7 rho (which walks at $1.75\sqrt\ell$ steps, about twice the ideal), at $q=2^{15}$ ($0.69$ $[0.55,0.83]$; no band pricing; not the strongest rho). Against the strongest rho see \S\ref{sec:transfer} (genus-3 round 4, plane quartic, and the validated Joux–Vitse sieve)."

## F5 [severity: major] distance.tex:201–205, 222–224, 245–267, 269–271
This is the chapter that "states the position as it stands at the end of the programme", but it omits the programme's only validated below-rho result:
- Line 204: "The genus-3 hyperelliptic row is the programme's only preregistered decision of IC below the strongest rho". The JV sieve study also ran under a frozen protocol (jv_sieve_negrho_20261102, "frozen 2026-11-02") and is validated.
- Table tab:distance-weak lists Joux–Vitse only as "$1.59$--$1.98$ … lower bound".
- Table tab:nearest (Smallest measured distances) also lists Joux–Vitse only at "$1.59$--$1.98$". It should list $0.0065$.
The abstract (lines 61–64), prog_transfer.tex:258–284 and prog_frontier.tex:695–698 all carry the sieve figure.
Action:
- Add a row to tab:distance-weak: "Joux–Vitse cover, sieve route | $p=1511$ | $0.00647$ $[0.00545,0.00799]$ (U1); $0.0156$ (U2); $0.107$--$0.123$ at $w=0.1,1$ with unemitted work upper-bounded | matched negation rho (142 pooled runs) | | validated (\texttt{VAL-20261103-c31c76}); weak class only; replay sampled; local run".
- Put the same figure at the top of tab:nearest.
- Change line 204 to: "The genus-3 hyperelliptic row and the Joux–Vitse sieve row are the programme's preregistered decisions of IC below the strongest rho, and the sieve row is the only one validated."
- Keep the Nagao arm's 1.59–1.98 as a second line labelled "plain Nagao arm".

## F6 [severity: major] distance.tex:245–267 (tab:nearest) and 114–167 (prime)
Two families' smallest distances are stale or missing.
- Prime: the improvement log carries "Residual collector res2 vs rho.negation … best cell icv1-fp36 (r = 2^36.0) | 2.59× | matched | S" (docs/ic/LEADERBOARD.md:33). prime_ext.tex:97 and 382 cite it ($2.6$, "improvement-log row, no validator"), and corrections.tex:146 says it is unchanged. distance.tex gives prime minima of 5.3 (k-sum) and 5.47 (Table B) and never mentions 2.59.
- Koblitz: "fully priced orbit (best cell) $1.566$ … never below 1" (lines 81–84 and 257). A later fully priced arm on the same cell (icv1-f2m29, $r=2^{15.4}$) is closer. leaderboard.json lazy_members_component, a1-n29, has lazy_over_rho_frob 1.272 [0.770, 1.803]. LEADERBOARD.md lists "Lazy orbit-member index, fully priced … 1.27×" and "Two orbit columns … 1.29×". The interval includes values below 1, so "never below 1" holds only for the point estimate. Neither row appears anywhere in the treatise (grep "1.27", "lazy orbit").
Action:
- Add the res2 row: "Prime, residual collector res2, 36 bits | $2.59$ | $w=0$; improvement-log row; no validator".
- Replace the Koblitz best cell with "lazy orbit-member index, $1.27$ $[0.77,1.80]$ ($n=29$, $r=2^{15.4}$; component, not a board row)".
- Change "the ratio is never below 1" to "no point estimate is below 1; at $r=2^{15.4}$ the intervals include 1".

## F7 [severity: major] distance.tex:49–55 and audit.tex:187–188, 201–203
The v8 row's status reads only "inside pre-declared ranges". research/ic_tool_program/rule/v8/ carries its own VALIDATION.json, replay-certificate.json and validator-report.json (order_holds_across_band: True, ic_over_rho 18.065). The other chapters disagree with each other about v8:
- prog_koblitz.tex:363 calls v8 "archived as incomplete".
- audit.tex:201–203 says the rule v8 row "is from Chapter distance", and table tab:audit-validation lists only v7 as validated.
Action: in distance Table tab:distance-koblitz, set v8 Status to "VALIDATION.json present (replay + validator; \art{research/ic_tool_program/rule/v8/VALIDATION.json}); not on the board, which still carries v7". In audit.tex, add "(rule v8 also carries a VALIDATION.json; the board has not been regenerated on it)". Reconcile prog_koblitz.tex:363 "archived as incomplete" with the validation file, or explain what is incomplete.

## F8 [severity: major] distance.tex (all "validated" rows: 35, 50, 146, 154, 175–176, 258–261) and audit.tex:173–209
Sabotage rule: "Cite a result by its pcc claim id and standing, or label it an unverified assertion." Every Table A and Table B row has a pcc measurement claim on main, for example PCC-20261104-24b9af (Table A, f2m59), PCC-20261104-f4bc7f (Table B, fp12, which is the 5.47 row quoted in tab:nearest) and PCC-20261104-a93a81 (Table B, fp20). Neither chapter cites them. Rows that carry no claim (k-sum, campaign, n83 BL, quartic) are not labelled "unverified assertion".
Action: add a column or footnote naming the claim id and its `pcc.py` standing for each board row. For example tab:nearest line 260: "Prime, Table B, 12 bits | 5.47 | validated; \texttt{PCC-20261104-f4bc7f} (standing: …)". Label every other row "producer-reported (unverified assertion)".

## F9 [severity: major] rho_side.tex:3–6 and 18–21
The chapter says "no pcc claim (pcc/claims/) covers any result in these chapters" and "the claims on main are leaderboard rows and Curve25519 autolab rounds". Both are false.
- pcc/claims holds 159 claims. They include tensor audits (PCC-20261104-1df881, 458dda, cited by unconventional.tex:25), Lean-style lemma claims (for example PCC-20261104-4da33e) and APS tool reports.
- This part relies on at least two claimed results. Table tab:gf-collision's measured column 1.82–1.94× is R4-I1, PCC-20261104-c97d81, and the re-baseline, PCC-20261104-d91a51, both of standing unverified (curve25519_autolab_20261104/README.md:139–146).
Replace with: "Two results in this part carry proof-carrying claims, both of standing unverified: the \texttt{wwalk} seed-averaged distances (\texttt{PCC-20261104-c97d81}, \texttt{PCC-20261104-d91a51}). Every other measured figure is the producer's reported value." Cite the ids in the caption of tab:gf-collision.

## F10 [severity: major] rho_side.tex:260–262 vs prog_prime.tex:381–383
rho_side opens §gf-cheon by repeating prog_prime's sentence "Cheon's attack beats rho from 24 bits up". Its own table then shows:
- "the $r-1$ comb is cheaper than rho with an interval excluding 1 at 36 and 44--56 bits and dearer at 32 and 40";
- "No exponent separation is established";
- every Cheon/rho reading on the P-224 shape was withdrawn.
"Beats rho from 24 bits up" comes from the September study (torsion_auxiliary_inputs_20260918), which was not priced across the band, so it violates the §5a wording rule and contradicts the ecbench result.
Action: in prog_prime.tex:381, replace it with "Cheon's attack with auxiliary inputs is cheaper than matched negation rho at most sizes from 36 bits in ecbench's counted unit (dearer at 32 and 40 bits; no exponent separation; \S\ref{sec:gf-cheon}), and only for the DLP with auxiliary inputs". Delete the quoted claim in rho_side.tex:260 or mark it as superseded.

## F11 [severity: major] prog_frontier.tex:483–489, 749–750, 796–798 (stale k=4 crossover)
The chapter still states "an extrapolated crossover near $n\approx2^{68}$", "closing as $n^{-1/4}$" in the cold-frontier paragraph, and as next-step (vi) "where the fitted trend predicts parity near $n\approx2^{68}$". extension.tex:110–129 reports the red-team verdict: "not survived (unsupported), not refuted", with $m=15$ corrected to $5.2\times$ ($2.4$--$15.0$) and "The corrected crossover is at or above the edge of what was measured". distance.tex:226 and abstract.tex:65 already say "unsupported".
Action: at line 489 add "A later red-team review found this crossover unsupported (Chapter~\ref{ch:extension}; $m=15$: $5.2\times$ after corrections)". At line 750 replace "($4.3\times$ a measured rho at $n\approx2^{56}$, closing as $n^{-1/4}$)" with "($4.3\times$ a measured rho at $n\approx2^{56}$ on one target, $5.2\times$ at $m=15$ after corrections; crossover unsupported)". In (vi), replace "predicts parity near $n\approx2^{68}$" with "was read as predicting parity near $2^{68}$; the red team found that unsupported".

## F12 [severity: major] prog_frontier.tex:745–752 (cold frontier) vs 695–698
The cold-frontier paragraph lists the Joux–Vitse cover only as "plain Nagao arm; $1.59$--$1.98\times$ … a lower-bound comparison". It says "three families are within a single-digit factor", but 45 lines earlier the same chapter reports the validated sieve at $0.0065\times$ with the order holding across the band. Next-step (ix) at line 804 ("measure rho on the Joux–Vitse cover family at $p\ge503$") is partly done: the sieve at $p=1511$ has a matched rho.
Action: add "; its sieve arm is $0.0065\times$ a matched negation rho at $p=1511$, validated (\S\ref{sec:transfer})". In (ix), add "(done for the sieve arm at $p=1511$; the Nagao arm still lacks it)".

## F13 [severity: major] prog_frontier.tex:536–542; also prog_method.tex:216–218 (stale BMI2 diagnosis)
prog_frontier says F4 counts "are therefore comparable across instruction sets only when the path is stated, and until the method identity records it the root-finder rows rest on the recorded receipt". audit.tex:118–130 gives the 26 Oct addendum: the cause was eager versus lazy row specialisation, "Since \#2043 every host is eager and the two eager paths charge identical counts. Sessions now record the version of the F4 accounting". The same story is told three times, and two of the versions are stale.
Evidence: research/validation_sweep_20261019/D-known-issues/ADDENDUM-20261026-item1-accounting.md, cited by audit.tex:126.
Action: keep the full account in audit.tex §4. In prog_frontier.tex (ii) and prog_method.tex item 6, replace the last sentence with "The 26 October addendum states the cause precisely and closes it (Chapter~\ref{ch:audit}, item 4)".

## F14 [severity: major] audit.tex:141–146 (stale plane-quartic item)
"rest on a degree-3 reduction … that merges distinct classes (reported in an open pull request) … relation counts and $S$ are unverified." prog_method.tex:255–258 says "Canonical $\mathrm{Pic}^0$ keys later separated those classes, and a re-run found that the earlier toy logarithms came from enumerating the subgroup". The N4/N4v2 rows used in distance.tex (0.005, "all verified") run on the canonical-key branch (quartic_n4v2_20261020/README.md: "cut from origin/claude/quartic-pic-canonical-20261018"). A reader of audit.tex alone would conclude that the treatise's headline $0.005$ is unverified.
Action: append "Canonical $\mathrm{Pic}^0$ keys later fixed the merge (\art{research/quartic_pic_canonical_20261018/}). The N4 and N4~v2 measurements of Chapter~\ref{ch:distance} use the fixed code. The withdrawn values are the earlier toys only."

## F15 [severity: major] distance.tex:86–92, 262 vs prog_koblitz.tex:306–311
distance says "The last cold IC/rho ratio is the K1120 holdout's $55.4$, from five points" and lists it in tab:nearest as "cold, five points". prog_koblitz.tex:309–311 says "No matched cold rho was run, so no cold ratio is admitted; the hosted single-target benchmark's $133.7\times$ is the nearest cold figure". 55.4 is cold IC over same-point rho *online* wall (LEADERBOARD.md:27: "Tier 1 cloud-instance cold IC / same-point rho online wall"). Plain rho has no set-up, so the reading is defensible, but the two chapters contradict each other and distance does not state the unit.
Action: in distance, write "cold IC over same-point plain-rho wall ($55.4$, five points, K1120 holdout; rho has no set-up, so its online wall is its cold cost)". Reconcile prog_koblitz.tex:309–311 with this, or drop 55.4 from tab:nearest.

## F16 [severity: major] distance.tex:96–112 (tab:distance-n83): missing standing
The table gives no host or standing. The source says: "It describes this L0 host only. It is not an improvement, a §8a gate result, a validation-gate rho reference or a leaderboard row … One run. No replay certificate or validator report" (RESULT.md). It also reports that IC online on this host was 115 ms against about 63 ms on the Tier 1 host.
Action: add to the caption: "L0 shared VM, one run, no replay certificate or validator report; public reproduction panel, not a holdout; producer-reported."

## F17 [severity: major] corrections.tex:3–9 and §§corr-engine, corr-rho (duplication, with an incorrect premise)
The appendix says it collects corrections "that the main text does not already carry". Most rows of tab:corr-engine and tab:corr-rho are told at greater length in rho_side.tex:
- calibration round 1 → rho_side §mr-calibration (full table);
- independence → §mr-stats ¶Independence;
- frontier v1→v2 → §mr-frontier (full table);
- statistics v2 / censoring → §mr-stats;
- $n=29$ panel → §gf-bsgs;
- automorphism rho erratum → §rv-aut;
- negation map → §mr-negaudit;
- Cheon P-224 → §gf-cheon;
- curve25519 \texttt{wwalk} → §gf-collision;
- hyperelliptic rho → §rv-jacobian;
- E27 → §rv-aut;
- multi-target Koblitz → §rv-batch;
- accounting v2 → rho_side §mr-bias (partly).
In unconventional.tex: ETH-route leads → §tn-corrections; learned WDSat → §ln-components; hashed-ECDL hang → tab:tn-native.
Action: keep the full accounts in rho_side.tex and unconventional.tex. Cut these rows from corrections.tex to one-line pointers in tab:corr-xref, or rewrite the intro to "collects every correction; where a chapter tells it at length, the row points there". Extend the "Main text" column to all these rows.

## F18 [severity: major] corrections.tex:35 (tab:corr-xref, xedni row) vs summer.tex:71–73
The cross-reference table says the JKSST correction ("conditional (Lean audit); overbroad readings refuted") appears in Chapters obstructions and summer. obstructions.tex:88–99 carries it. summer.tex does not: it says only "Xedni-style lifting was closed in exact form … This is the result restated in Chapter~\ref{ch:obstructions}."
Action: append to summer.tex:73: "The exact count closes one family and slot shape only. A 1 November Lean audit found the classic JKSST obstruction conditional (Chapter~\ref{ch:obstructions}; \art{research/jksst_xedni_audit_20261101/})." Alternatively, drop "\ref{ch:summer}" from the xref row.

## F19 [severity: major] corrections.tex:43 (tab:corr-xref, $n=83$ refutations) vs prog_binary.tex
The table says the FieldStructure correction is in \S\ref{sec:n83} and \S\ref{sec:binary}. prog_koblitz.tex:326–332 (sec:n83) carries it. prog_binary.tex (sec:binary) does not mention it. That chapter still reports pre-fix $n=83$ engine results without a caveat: line 203–208, "1{,}237 of 1{,}800 positive-dimensional cells were certified empty" (17–19 Oct, before the 24 Oct fix).
Action: either drop \S\ref{sec:binary} from the xref row, or add a sentence to prog_binary.tex:208. Check whether the norm-predicate certificates used the truncated field table (research/n83_msweep_20261024/README.md). If they did, mark the 1,237 cells void.

## F20 [severity: major] corrections.tex:457–461
"As in Chapter~\ref{ch:audit}, no correction listed here moved a generic or leaderboard index-calculus row below a matched rho. The one row below rho is the Joux--Vitse sieve route". The second sentence reads as if the sieve were the only IC measurement below rho. The treatise also reports the plane quartic ($0.005$, distance.tex:219), genus-3 round 4 ($0.70$) and the $n=29$ panel ($0.366$, corrections.tex:215).
Replace with: "The one row in this appendix that ends below rho is the Joux–Vitse sieve route, …".

## F21 [severity: major] unconventional.tex:433–767 (Ch.~tensor) vs prime_ext.tex:1209–1300 and 1357–1460
The same material is told at length twice:
- the tensor preprint and D.1's conditional status;
- the 34 release tests and the Rust port;
- the finite-size review;
- the counts-only protocol v2;
- the L09 census ($-16.1$ to $-16.3$ bits, $37$--$41{,}000\times$);
- the L06 width bound (15--51 against 2--10.7);
- the twelve-lead table (tab:pa-eth against tab:tn-refuted plus prose);
- the errata on $10^5b$ edges and $c\ge1$/$256c^2$.
corrections.tex:373 repeats the errata a third time.
The two tables also give L05 different standings: prime_ext.tex:1258 says "refuted as closed", while unconventional.tex:635 and 642 say "refuted as stated". The source (LEADS.md §9 / SKEPTIC_DIGEST) should decide.
Action: keep the full account in unconventional.tex Ch.~tensor. Shrink prime_ext §pt-tensor and the twelve-lead paragraph of §pa-sat to a summary paragraph with a pointer. Keep tab:pa-eth only if it is the single standing table and delete tab:tn-refuted's duplicate headlines, or the reverse. Align L05's standing with the skeptic digest.

## F22 [severity: major] prog_frontier.tex:550
"The $(3,7)$, $\ell=8$ and $m=4$, $N=20$ cells remain." The same chapter, lines 651–666, reports that the $(3,7)$ cell refutes with $k^*\le2$ guesses on 10 of 10 draws, and that the $(22,8)$ $m=3$ runs completed.
Replace with: "The $m=4$, $N=20$ cell remains; the $(3,7)$ and $\ell=8$ cells were since run with guessed variables (\S\ref{sec:ladder3})."

## F23 [severity: minor] distance.tex:148, 151
In the prime table, the "Campaign pair table, $w=0.1$" row gives exponent $0.620$ $[0.557,0.663]$, which is the $w=0$ fit (campaign README:45, "at w = 0"). The $k$-sum row's $5.3$ is at $w=0$ and sits beside the pair table at $w=0.1$, so the two rows are not comparable. The text (line 131) quotes the same exponent as $0.620$ $[0.586,0.650]$, the k-sum study's refit, so the same quantity has two intervals within 20 lines.
Action: label the exponent column "($w=0$)". Add the campaign's $w=0$ ratios (9.8 and 95.2) or the k-sum study's $w=0.1$ ratios so both rows use one weight. Note that 0.586–0.650 is the k-sum study's refit of the same arm.

## F24 [severity: minor] distance.tex:154
"New 20-bit baseline … $7.75$ … validated; was $9.47$". The board's 20-bit Table B row (icv1-fp20-t1293-3c2999ba) reads $8.30$, not $9.47$: 9.47 is the incumbent measured inside the tuning study's own session. A reader comparing with "Table B prime rows … $5.47$--$13.5$" will look for 9.47 on the board and not find it.
Replace with: "validated component (\art{…/VALIDATION.json}); incumbent $9.47$ in the same session (board row: $8.30$)".

## F25 [severity: minor] distance.tex:156, 169
- "Planner cells & & $12.6$--$25.8$" has no bits, no weight and no source in the caption.
- "The Kummer tower is $28$--$159\times$ rho": the campaign gives $30.3$--$141.3$ at $w=0$ and the 30-bit row $158.9$ at $w=0$. I could not find the lower end, 28, in research/ic_frontier_campaign_20261015/README.md or select/SELECTION.md.
Action: give a source for both. State the weight, or use "$30$--$159\times$ ($w=0$)".

## F26 [severity: minor] audit.tex:173, 184–193
The text says "19 of the 28 leaderboard rows", but the table's rows sum to 13+6+9+2 = 30. The "components outside A–C" line counts 2 validated components, while leaderboard.json has more than 30 `*_component` sections, several of them validated (for example lazy_members_component and generator_orbit_component). Validated components are not board rows.
Action: drop the component row, or caption it "not counted in the 28; two of several validated components".

## F27 [severity: minor] corrections.tex:227–228
"Table~\ref{tab:audit-rho} gives the rho-side changes that moved leaderboard rows." Two of its five rows (better of $P\pm R$; signed $\sigma$) are, per audit.tex:103–104, "rho studies, neither yet in a campaign arm", so they moved no row.
Replace with: "…gives the rho-side changes listed with the leaderboard corrections".

## F28 [severity: minor] rho_side.tex:711–713
"Chapter~\ref{sec:frontier} records that the strong signed-Frobenius rho fell from 1{,}196 to 679.7 instructions per step with no established native gain." prog_frontier records 1,196→718 over three rounds, then a fourth round of 707.2→679.7 "with no established native-time gain". The no-gain statement applies to the fourth round only: the canon round measured $1.40$--$1.61\times$ native time (tab:mr-rounds), and Q4 measured 1.21× at $n=61$.
Replace with "…fell from 1{,}196 to 718 instructions per step in three rounds, and a fourth round took \texttt{main} from 707.2 to 679.7 with no established native gain".

## F29 [severity: minor] rho_side.tex:1070–1074
"On every sealed domain \texttt{kangaroo.vow} is dominated". On the Koblitz toy domain, FRONTIER.md:57–59 says signed-Frobenius and negation rho are better "at the charged cost but not across the pricing band … not dominating" (flips at 0.290 and 0.417), and rho_side's own §mr-band says so too. The Koblitz-medium figure 15.558 is a three-size bound that "a frontier entry needs at least 4" sizes to enter (FRONTIER.md:95).
Replace with: "\texttt{kangaroo.vow} is never an operations leader: dominated on the prime toy tier, unresolved inside the band on the Koblitz toy tier, and unranked (three sizes) on the Koblitz medium tier at $15.6\times$ its floor."

## F30 [severity: minor] rho_side.tex:240–245
The end-to-end census's second exception, "the GHS-weak plane-quartic family is below matched rho in its own comparison", carries no standing. Distance's own caveats are: L0 under heavy load, no validation file, and frozen-baseline records that do not replay.
Add: "(L0, no validation file)".

## F31 [severity: minor] summer.tex:275, 285–289
The table row "boolean $d_{\rm reg}$, $n=21$ | \texttt{DEC-20260815-6b806c} | supersede" cites a decision the text never mentions. The text cites EV-DREG-4500cd and DEC-20260828-653923, and the caption says "Every row is discussed above". In crypto-autoresearcher, DEC-20260815-6b806c exists only under ledger/corrections/schema-supersessions/20260815/.
Action: cite it in the text at line 146, or replace it in the table with the decision the text uses.

## F32 [severity: minor] corrections.tex:22–24
"Seven corrections are being written into the main text" is a drafting-state sentence.
Replace with "Seven corrections are carried in the main text (Table~\ref{tab:corr-xref}) and listed here with a pointer only." This also depends on fixing F18 and F19.

## F33 [severity: minor] distance.tex:269–271
"On every prime-order curve of cryptographic shape that the programme measured, rho is cheaper." True, but the programme's prime-order curves are toy-sized (10–40 bits), and the statement does not say that.
Add "(at the toy and medium sizes measured, $\le2^{47}$)".

## Gaps (sources not in the treatise)
- G1: research/ecbench_lazy_orbit_members_20261018/ and research/ecbench_lazy_columns2_20261018/: fully priced Koblitz arms at 1.27× and 1.29× rho-frob ($r=2^{15.4}$), listed on LEADERBOARD.md. They are missing everywhere (see F6).
- G2: rule v8's VALIDATION.json and the fact that the board has not been regenerated on v8 (see F7).
- G3: pcc claim ids for Table A and B rows (see F8).
