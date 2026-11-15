# Review findings: group `fields`

Sections: prog_prime, prime_ext, extension, prog_transfer, transfer_ext (branch claude/treatise-expansion-8-20261106, worktree wt3).
Reviewer did not edit the repository. Primary sources read with `git -C /home/user/crypto show origin/main:<path>`.

## Numbers re-read from primary sources (all match unless a finding below says otherwise)

1. JV sieve vs negation rho, p=1511: S/rho U1 0.00647 [0.00545,0.00799]; U2 0.0156 [0.0131,0.0192]; x155 [125,184]; x64 [52,76]; layer-A band 0.107/0.117 (w=0.1), 0.121/0.123 (w=1); 16+126=142 pooled runs (research/jv_sieve_negrho_20261102/README.md; VALIDATION.json names VAL-20261103-c31c76, primary cell U1).
2. JV sieve line-vs-rho table: P 0.0244, H 0.0154, U 0.0141 (U2); S+ 0.0624/0.0344/0.0324; U1 0.0101/0.0064/0.0059; U by prime 0.031/0.0115/0.0081; run-level interval [0.0114,0.0182] (research/jv_sieve_line_vs_rho_20261103/README.md:33-58).
3. JV Nagao crossover 0.793/0.775/0.587, strong 1.349/1.063/0.910 (jv_cover_crossover_20261101:15-17).
4. JV re-scoring 1.405/1.160/0.870 (U1), 3.376/2.787/2.090 (U2) (jv_cover_negrho_rescore_20261101:23-28).
5. JV line search 1.71x [1.54,1.89], 0.587; 0.791/0.659/0.548; 1.900/1.585/1.318; p* 1185 / 1799 (jv_cover_fbline_20261102:36-58).
6. JV replication A4 0.792 [0.572,1.073]; 0.647 at p=503, 0.00404 at p=1823 (jv_sieve_strong_rho_v2_20261020).
7. bitop/v1 row 0.0098 [0.0069,0.0144] (ic_acct1_cover_bitop_20261021:18,39).
8. JV p-scaling: slope -2.05, p* about 1330 [1170,1540], 1.59-1.98 at p=1009, GM 620 at p=53, pooled rho 1.344 (jv_cover_p_scaling_20261024).
9. Hilbert stop 1.848-1.875; baseline S/rho 1361.1 ... 7.18 (benchmarks/20261029-jv-cover-hilbert-stop/validator-report.json).
10. Curve25519 miniatures table (p, A, r for k=20..32) and distances 7.8/10.6/18.1/24.7 (curve25519_ic_baseline_20261102/README.md).
11. Curve25519 autolab: R4-I1 1.82x (k=32), 1.86x (k=28); seed=run 1.94/1.81/1.82; multicurve 1.57/1.78/1.80/1.91; fixed-table 1.43/1.49/1.55 (curve25519_autolab_20261104/README.md:137-141).
12. Parity bound 1.5 (F=1), 1.875 (F=2), MC 1.889 (curve25519_rho_parity_bound_20261104/README.md).
13. T12 winner 0.0578 [0.0466,0.0716]; T13 0.2837 [0.2413,0.3335], 1.567x, 27.9x / 1.83e4 / 1.13e5 (t10_winner, t4_candidate READMEs; both validator-report.json `pass_with_errata`, disposition validated).
14. Curve25519 algebraic k=20 18.2 / 9.55e5 / 5.85e6 (curve25519_algebraic_ecbench_20261102:34).
15. Curve25519 m=3 generic 0.758 / 0.596; T0 1.944-2.016, 3.33-4.00; ratchet 0.996, 0.977 (respective READMEs).
16. Table B prime rows 6.34, 5.47, 6.43, 6.71, 5.82, 8.30, 10.50, 13.46 (ic_validation_gate_20261024/README.md); docs/ic/runs/VALIDATION.json is installed on main.
17. Residual collectors: res1eq 1.32/1.72/1.39 (w=0), 2.27/2.99/2.42 (w=1); res1dp 2.15/2.27/1.92; 863 verified; p36/p37 1.54/1.75/2.66 and 1.64/1.86/2.83 (ic_residual_scale_20261101:5-7, ic_residual_dp_eq_20261101:10).
18. k=5 sums: 8.51 vs 11.56, 0.547/0.565, 9.7x and 11.0x rho, 7.6x/10.6x BSGS; pooled 0.700/0.446/0.324/0.217, 0.496 vs 0.635, 6.0/5.8/7.3/7.3x (prime_ksum_k5_20261028, prime_ksum_pooled_20261029).
19. m=4 cost cells: IC/rho 35.6 at 28 bits w=0 (benchmarks/20261018-blindspots-prime-m4-cells/VALIDATION.json gae_band: 568572.9/15953.4).
20. Kummer tower ecbench 34.0, 41.9, 29.4, 96.6, 140.6, 896 runs, 0.26+-0.28 (pkm_tower_ecbench_20261013).
21. EcGFp5 BW finish: CPU 31004/16912 etc., speed-ups 1.83/1.97/1.72, log2 c 46.30/45.43 ... (recomputed; consistent with c = CPU-s x 2.8e9); D = 2^16 = 8^5*2.
22. N4 v2: DL/rho-neg 0.005 [0.004,0.008], q*=555 [389,752], 12.3 vs 616 CPU-s, DL/B0 0.042, D/B0 0.081 (quartic_n4v2_20261020).
23. Genus-3 vs rho at q=2^16: 0.697/0.710/0.70 ideal, 0.639/0.382 at w=0.1/1, c_l=9 3.27-3.32 and 12.4 at 2^17 (ghs3_vs_rho_20261023/TABLES.md).
24. GLV note E24 handover 2^116.7 -> 2^90.1 (2^91.7); E32 beta -0.243 [-0.279,-0.205], 2^85.5 [79.2,94.3] (RESEARCH_GLV_INVARIANT_FACTOR_BASES.md:2471-2494, 3706).
25. Weak-class probability 0.5908 [0.5754,0.6059] (p=11), 0.6320 [0.6169,0.6468] (p=47) (iso1_weak_classes_20261007/REPORT.md:111,117).
26. Tian route 26 of 26; 180-340 vs 5,000-300,000 group additions; q<=8191 (tian_n3_quartic_route_20261024/REPORT.md:22-32).
27. 31 Curve25519 study directories on main (`git ls-tree research/ | grep curve25519 | wc -l` = 31).

No numeric mismatch was found among these 27 groups. The findings below are about standing, staleness, contradiction, duplication, clarity and gaps.

---

## F1 [severity: blocker] prog_prime.tex:8-9
What is wrong: "No prime-field arm comes within a single-digit factor of rho except on the smallest Table~B rows, and the gap grows with size." This is false against the treatise's own newer chapters. Every Table B prime row is single-digit only up to 18 bits, and several later arms are much closer: the residual collectors are 1.3-1.7x at 36-42 bits with no growth (prime_ext.tex:25, 95, 542), the Curve25519 `wwalk` incumbents are 1.57-1.91x (prog_prime.tex:268-270 in the same chapter), and the k=5 sum tables are 5.4-7.3x at 20-32 bits (prime_ext.tex Table pg-ksum).
Evidence: research/ic_residual_scale_20261101/README.md:7 "res1eq 1.32/2.27 -> 1.72/2.99 -> 1.39/2.42 ... No growth with r over this range"; research/curve25519_autolab_20261104/README.md:141 "mean S / rho 1.57 / 1.78 / 1.80 / 1.91".
Replacement: "Every prime-field arm is behind rho with negation. The closest are generic collision searches at rho's exponent: the residual collectors at 1.3--1.7$\times$ (36--42 bits, Chapter~\ref{ch:primegeneric}) and the Curve25519 walk tables at about 1.8$\times$ (\S\ref{sec:c25519}). Every arm with a better-than-generic oracle is further away, and its gap grows with size."

## F2 [severity: blocker] abstract.tex:55-56 (consistency with prime_ext/prog_prime)
What is wrong: "The distance at a matched reference is ... $10.5$--$102\times$ on prime curves of 20--40 bits." The abstract presents the frontier campaign's two-summand arm as the prime-field distance. The treatise's own prime chapters put the closest prime arms at 1.3-1.7x (residual, validator report at 36-37 bits) and 1.6-1.9x (Curve25519 wwalk, unverified). A reader of the abstract gets a gap roughly six times too large.
Evidence: prime_ext.tex:95 (Table pg-summary row res1eq 1.32--1.72, "validator report (36--37 bits)"); benchmarks/20261101-ic-residual-dp-eq/validator-report.json.
Replacement: "... and $10.5$--$102\times$ for the campaign's two-summand prime arm at 20--40 bits; the closest prime-field arms, generic collision searches at rho's exponent, are $1.3$--$1.9\times$ (Chapters~\ref{ch:primegeneric} and \S\ref{sec:c25519})."

## F3 [severity: blocker] prog_frontier.tex:692-695 (Joux-Vitse; stale against prog_transfer/transfer_ext)
What is wrong: "rho cannot be run at $p\ge503$ (subgroups of $2^{52}$--$2^{58}$), so those ratios use the pooled rho constant". Matched negation rho has since been run at p=1009, 1301, 1451, 1511, 1601 and 1823 (142 runs at p=1511 alone). The same paragraph's extrapolated crossing "near $p\approx1330$ ($[1170,1540]$)" was tested and does not hold against negation rho: at p=1301 and 1451 the plain Nagao arm is above rho in group additions.
Evidence: research/jv_cover_negrho_rescore_20261101/README.md:11 "the cover costs **more** than rho in group additions at p = 1301 (S/rho 1.405) and p = 1451 (1.160 ...)"; research/jv_sieve_negrho_20261102/README.md "the pooled 142 runs".
Replacement: replace the sentence with "Rho was later run on the same instances at $p=1301$--$1823$ (\S\ref{sec:jac-jv}): against a matched negation rho the plain arm is above rho at 1301 and 1451 and unresolved at 1601, so the extrapolated crossing near 1330 does not hold. A factor-base line search moves the modelled crossing to $p^*\approx1185$ in additions and 1799 in multiplications, without a validation file." Also update open problem (ix) at prog_frontier.tex:803-806: the rho measurement it asks for exists.

## F4 [severity: major] abstract.tex:62-64 and distance.tex:223, 255 (Joux-Vitse)
What is wrong: The abstract still gives the Nagao arm as "$1.6$--$2.0\times$" (pooled constant, lower bound, p=1009) as the family's headline, and both weak-family tables in the distance chapter (tab:distance-weak and the summary table) list only the Nagao row "$1.59$--$1.98$; pooled rho constant; lower bound". They leave out the validated sieve row, 0.00647x matched negation rho at p=1511. That row is the treatise's only validated below-rho row (prog_transfer.tex:283). The November Nagao results (above negation rho at 1301/1451; 0.548-0.791 with line search, unvalidated) are also missing.
Evidence: research/jv_sieve_negrho_20261102/VALIDATION.json "Primary cell: S/rho U1 0.00647"; transfer_ext.tex Table jac-nagao.
Action: in distance.tex add a row "Joux--Vitse sieve route, $p=1511$ & $0.00647$ $[0.00545,0.00799]$ & matched negation rho (U1); $0.0156$ in $\FF_p$ mults & validated (\texttt{VAL-20261103-c31c76}), weak class, accounting". Relabel the Nagao row "plain Nagao arm, pooled constant (superseded at $p\ge1301$ by matched negation rho, \S\ref{sec:jac-jv})". In the abstract, write "Joux--Vitse covers: the sieve route at $0.0065\times$ a matched negation rho in group additions ($0.016\times$ in $\FF_p$ multiplications; validated, $p=1511$); the Nagao arm is still above matched rho at $p\le1451$."

## F5 [severity: major] prog_openproblems.tex:135-150 (Joux-Vitse, stale)
What is wrong: Open problem 11 says the Nagao cover "is $1.6$--$2.0\times$ ... with the line meeting rho near $p\approx1330$" and asks for an experiment that would "run both arms and rho at $p\approx1300$--$2000$". The Nagao arm and matched rho were run at 1301/1451/1601 (crossover, re-scoring and line search), and the sieve was run at 1009/1511/1823 with three arms (jv_sieve_line_vs_rho_20261103). The 1330 crossing failed against negation rho (F3). The question "where the Nagao arm itself crosses rho" now has a partial answer: below in additions with the line search, above in multiplications, unvalidated.
Evidence: as F3; transfer_ext.tex:1562-1575.
Replacement: rewrite item 11 to say what was run in November (pointing to \S\ref{sec:jac-jv}). Keep as open: (a) emit and price the sieve's F4, descent and linear-algebra additions; (b) validate the Nagao line-search row in both units; (c) the 30x arm gap; (d) a run on the isolated lab.

## F6 [severity: major] prog_transfer.tex:79-81 and 246-249 (stale "now")
What is wrong: "The Joux--Vitse genus-3 cover of $E(\FF_{p^6})$ is now within $1.6$--$2.0\times$ of a pooled rho constant" and "it is a lower-bound comparison and current". As of November it is not current: the matched negation-rho re-scoring at p=1301-1601 reversed the dp-rho reading (transfer_ext.tex:1529-1534).
Replacement: at line 80, "was, on 24 October, within ... (a lower bound against a pooled constant; superseded in November by matched negation rho, \S\ref{sec:jac-jv})". At line 249, drop "and current".

## F7 [severity: major] prog_transfer.tex:254-256 (stale)
What is wrong: "a replication with fixed thresholds failed its acceptance rule and is in an open pull request, and a prospective re-run is registered and has not run." The replication is merged on main (research/jv_sieve_strong_rho_v2_20261020/README.md, cited at transfer_ext.tex:1633-1634). The matched re-measurement that replaced the prospective re-run has run and is validated (the next paragraph of the same chapter). research/jv_sieve_prospective_20261023/ holds only PROTOCOL.md, cards and run.sh.
Replacement: "a replication with fixed thresholds failed its rho A/A rule A4 ($0.792$ $[0.572,1.073]$), so its decisions are withheld (\art{research/jv_sieve_strong_rho_v2_20261020/}); the registered prospective re-run (\art{research/jv_sieve_prospective_20261023/}) never ran and was superseded by the matched study below."

## F8 [severity: major] prog_transfer.tex:270-283; transfer_ext.tex:1515, 1684-1686 (standing under the pcc rule)
What is wrong: The sieve row is called "validated" and "the programme's one validated measurement of index calculus below a matched rho" with only a validation-gate id. Under the 4 November rule (CLAUDE.md "Sabotage is assumed"), a result is cited by its pcc claim id and standing, or labelled an unverified assertion. No pcc claim exists for this row: none of the 159 files under pcc/claims/ on main mentions jv_, Joux, 1511 or VAL-20261103-c31c76. The treatise applies the pcc rule to Curve25519 (prog_prime.tex:271-272) and prime_ext (prime_ext.tex:32-46), but not here.
Evidence: `git ls-tree origin/main pcc/claims/` (159 claims; grep for jv/joux/c31c76 empty).
Replacement: after "(\texttt{VAL-20261103-c31c76})" add "; it has no \texttt{pcc} claim record, so under the proof-carrying-claims rule it is a validation-gate result whose statement is an unverified assertion". Use the same wording in abstract.tex:64, corrections.tex:45 and prog_openproblems.tex:140.

## F9 [severity: major] transfer_ext.tex:1684-1686
What is wrong: "and $155\times$ in group additions at $p=1511$; only that last cell is validated." The validator report and VALIDATION.json cover all four cells: S and S+, in U1 and U2. "The order holds in all four." The 64x cell in F_p multiplications is validated too, so the sentence understates the scope and misleads.
Evidence: research/jv_sieve_negrho_20261102/VALIDATION.json note: "layer B and unit U2 are in the validator report and recompute.json, and the order holds in all four."
Replacement: "... and $155\times$ in group additions ($64\times$ in $\FF_p$ multiplications) at $p=1511$; only the $p=1511$ comparison is validated."

## F10 [severity: major] transfer_ext.tex:1635-1637
What is wrong: "Re-priced in \texttt{bitop/v1} with both arms executed, the weak-class row at $p=1511$ ... stays below rho at $0.0098$". The source row is not a weak-class-only measurement. It is the scoreboard row "Whole method from a curve that is not weak (section 18)": an isogeny walk from a non-weak challenge curve, transport and the cover, against strong rho R3. It also has no validator report ("validator report: missing"). The bullet hides both facts and sits beside the 0.00647 row as if it were a different pricing of the same quantity.
Evidence: research/ic_acct1_cover_bitop_20261021/README.md:11 "the scoreboard row \"Whole method from a curve that is not weak (section 18)\""; :166 "validator report: missing".
Replacement: "Re-priced in \texttt{bitop/v1} with both arms executed, the whole-route row (an isogeny walk from a non-weak challenge curve to the weak class, then the cover) at $p=1511$ ... reads $0.0098$ $[0.0069,0.0144]$ of strong rho R3 (largest band upper bound $0.075$); no validator report."

## F11 [severity: major] extension.tex:161-164 (stale against prog_transfer)
What is wrong: "This is the cheapest experiment in the programme that could yield a measured crossover of index calculus below rho on an elliptic curve." The programme already has measured below-rho results in weak extension families: the JV sieve (validated, p=1511), N4 v2 plane quartic (0.005x negation rho) and genus 3 at q=2^16.
Replacement: "It is the cheapest experiment that could put the $k=4$ characteristic-two family below rho; three other weak families already have measured below-rho rows (Chapter~\ref{sec:transfer})."

## F12 [severity: major] extension.tex:86-89 vs prog_prime.tex:428-440 vs transfer_ext.tex:2005-2009
What is wrong: Three chapters give three incompatible odd-characteristic k=4 statements:
- extension.tex:87-88: "$1.39\cdot10^6\times$ rho ... handover moves from $2^{150}$ to about $2^{103}$--$2^{119}$".
- prog_prime.tex:430-431 (sec:gaudry): "$2^{149.4}$--$2^{152.7}$ to $2^{135.1}$--$2^{135.8}$", plus $2^{156.1}$/$2^{146.0}$ against negation rho.
- transfer_ext.tex:2007: says "\S\ref{sec:gaudry} and Chapter~\ref{ch:extension} quote" $2^{103}$--$2^{119}$, which §gaudry does not.
The measured distance "$1.39\cdot10^6$" is also stale: E32 measured $1.5\cdot10^3$ at $n=2^{42.4}$ (validated, benchmarks/20261031-e32-handover-ladder/).
Evidence: RESEARCH_GLV_INVARIANT_FACTOR_BASES.md:3657 "E32 measured: the distance to rho over 2^{22.6}–2^{42.4} ... fitted crossover order 2^{85.5}".
Replacement: extension.tex:86-89 → "Over $\FF_{p^4}$ the best line (GLS twists with full rational 2-torsion) measured $1.5\cdot10^3\times$ a matched negation rho at $n=2^{42.4}$ (E32, validated), falling as $n^{-0.24}$, with a fitted crossover at $2^{85.5}$ $[2^{79.2},2^{94.3}]$ (Chapter~\ref{ch:glvprogramme}). The earlier single-point models in \S\ref{sec:gaudry} are superseded." In transfer_ext.tex:2007, attribute the $2^{103}$--$2^{119}$ figure to Chapter~\ref{ch:extension} only.

## F13 [severity: major] prog_prime.tex:433-436
What is wrong: "This is the one place in the programme where a crossover moves". Stale: the GLV programme moved the modelled k=4 handover from $2^{116.7}$ (E19) to $2^{90.1}$ (E24) and $2^{88.0}$ (E25), all validated (transfer_ext.tex Table glv-ladder). The JV line search moved the Nagao p* from 1528 to 1185.
Replacement: "This was the first place in the programme where a modelled crossover moved; the GLV-invariant line moved it further (Chapter~\ref{ch:glvprogramme})."

## F14 [severity: major] prime_ext.tex:1714-1716 (contradiction with prog_prime sec:c25519 and prime_ext:770-773)
What is wrong: "The algebraic track has no priced prime baseline beyond the Kummer-tower rows". The Curve25519 algebraic F4 arms are priced end to end in ecbench (18.2 / 9.55e5 / 5.85e6 x rho at k=20). Two of them have validator reports with disposition validated (T12 winner, T13 t=4 trace). prime_ext's own chapter intro (lines 770-773) lists the Curve25519 F4 rows among the priced algebraic costs.
Evidence: research/curve25519_t10_winner_ecbench_20261104/validator-report.json "disposition": "validated"; research/curve25519_t4_candidate_ecbench_20261104/validator-report.json same.
Replacement: "The algebraic track's priced prime rows are the Kummer-tower rows and the Curve25519 F4 arms; the best of the latter, validated, is still $1.83\cdot10^4\times$ rho at $w=0.1$ and scales as $r^{1.03}$ (\S\ref{sec:c25519})."

## F15 [severity: major] prime_ext.tex:541-550 vs prog_prime.tex:274-281 (parity bound vs measured residual collectors)
What is wrong: prime_ext says the parity bound of §c25519 applies "analogously" to the residual collectors. That bound says a collision-based IC with F>=1 unknown base logarithms costs at least 1.5x rho. Yet the same chapter reports res1eq at 1.32x (p36) and 1.39x (p42) matched rho at w=0. Either res1eq is outside the bound's hypotheses (for example, its base logarithms are known, or the measured rho is above the ideal $\sqrt{\pi r/4}$ constant), or one of the numbers is wrong. The reader is not told which.
Evidence: curve25519_rho_parity_bound_20261104/README.md table "1 | 2 | 1.500"; ic_residual_scale_20261101/README.md:7 "res1eq 1.32".
Action: add a sentence stating res1eq's F and the measured rho-neg constant against $\sqrt{\pi r/4}$, and explain why 1.32 < 1.5 does not contradict the bound. If no explanation is available, drop "as the parity bound ... shows" and flag the discrepancy as open.

## F16 [severity: major] prog_prime.tex:250-252 vs 291-306 (Curve25519: tower depth t)
What is wrong: The generic-track paragraph says "the Kummer tower is blocked, because $p\equiv5\pmod8$ forces $t\le2$". The algebraic paragraph then speaks of "the tower depth $t$" with t=4, 5, 6, 10 and does not say these are isogeny-tower bases (`pkm-iso-*` arms), which do not need $2^t\mid p-1$. As written the two paragraphs contradict each other.
Evidence: curve25519_ic_ratchet_20261102/README.md:16 "`pkm-semaev` needs `2^t ∣ p − 1` ... t ≤ 2"; curve25519_t10_winner_ecbench_20261104/README.md arms `pkm-iso-t6-g4`, `pkm-iso-t5-g4-s1307`.
Replacement at line 291: "The later gains came from an isogeny-tower base (the \texttt{pkm-iso} arms, a $2^t$-point base built by a chain of 2-isogenies, which lifts the $t\le2$ limit of the $\mu_{2^t}$ tower), selecting its frame, seed, depth $t$ and guess count $G$, ..."

## F17 [severity: major] prog_prime.tex:296 and 326-328 (standing)
What is wrong: "Two studies passed independent validation". A third, the atlas (curve25519_ic_atlas_20261104), also has an independent validator report (`pass_with_errata`), and it is the source of the scaling paragraph ($r^{1.03}$, optimal t=4, "F4 arm is $42\times$ too expensive", "2.39--3.32"). That paragraph carries no standing label at all. None of the three validated Curve25519 studies has a pcc claim, and the paragraph does not say so.
Evidence: research/curve25519_ic_atlas_20261104/validator-report.json "verdict": "pass_with_errata".
Replacement: "Three studies have independent validator reports (pass with errata); none has a \texttt{pcc} claim:" and add the atlas as a third bullet, labelled at line 326.

## F18 [severity: minor] prog_prime.tex:271-272 (pcc citation)
What is wrong: "Every autolab result is a statement claim with no proof attached, so its standing is \emph{unverified}." That label is correct, but the rule asks for claim ids, and the source lists them.
Evidence: curve25519_autolab_20261104/README.md:146 "R4-I1 `PCC-20261104-c97d81`, re-baseline `PCC-20261104-d91a51`, multi-curve `PCC-20261105-37a78d`".
Replacement: append "(R4-I1 \texttt{PCC-20261104-c97d81}, re-baseline \texttt{PCC-20261104-d91a51}, multi-curve \texttt{PCC-20261105-37a78d}; all unverified)".

## F19 [severity: major] prog_transfer.tex:107-136 and abstract.tex:59-60 (genus 3 below the strongest rho)
What is wrong: The abstract reads "genus 3 via GHS: below the strongest rho at one size". prog_transfer shows this holds only at the registered unit price $c_\ell=1$. At the protocol's own sensitivity $c_\ell=9$ every instance is 2.7-12.4x rho, including 3.27-3.32x at q=2^16. The study has no README and no review. The abstract carries neither qualifier.
Evidence: research/ghs3_vs_rho_20261023/TABLES.md:15-16 (c_l=9 column 3.267, 3.321 at q=2^16).
Replacement (abstract): "genus 3 via GHS: below the strongest rho at one size under the registered unit price, above it at the protocol's sensitivity price, unreviewed".

## F20 [severity: major] Duplication: Joux-Vitse told at length in four places
Locations: prog_transfer.tex:243-305 (about 60 lines: two arms, the validated row, history, 9 Oct whole route, notes); transfer_ext.tex:1509-1686 (November detail, tables jac-nagao and jac-sieve, Hilbert stop, weak class); prog_frontier.tex:683-700 (Nagao p-scaling at length, plus the sieve row); prog_openproblems.tex:135-150. The parts disagree (F3, F5-F7, F9).
Recommendation: keep the full account in transfer_ext §jac-jv. Move into it the validated-row paragraph and the limitations list from prog_transfer (lines 258-286). Reduce prog_transfer to a 5-line summary with the validated number and its standing, and prog_frontier to two sentences and a pointer. Within prog_transfer, "The roughly thirtyfold gap ... is unexplained" appears twice (lines 285-286 and 293-294); delete one.

## F21 [severity: minor] Duplication: ECCp-131 and standard-curve audits split across prog_prime and prime_ext
Locations: prog_prime.tex:190-203 and prime_ext.tex §pt-eccp131 (lines 1623-1661); prog_prime.tex:358-383 and prime_ext.tex §pt-standard (1663-1706). The content is complementary rather than repeated, but the reader must visit two chapters for one target, and "Cheon's attack ... beats rho from 24 bits up" (prog_prime:381-383) sits next to "every Cheon/rho ratio and crossing in that report is withdrawn" (prime_ext, P-224 shape) without a cross-reference.
Recommendation: merge each pair into prime_ext and leave one-line pointers in prog_prime. At prog_prime:381, add "(a different study; the P-224-shape Cheon/rho ratios were withdrawn, \S\ref{sec:pt-standard})".

## F22 [severity: minor] Duplication: conductor gaps, volcano levels and cover censuses
Locations: prog_transfer.tex:35-63, 307-338 vs transfer_ext.tex §iso-conductor (221-384), §iso-volcano (101-220) and §ghs-odd (1145-1226). transfer_ext states that it extends beyond those summaries, so the overlap is mostly summary vs detail.
Recommendation: acceptable. Make sure the prog_transfer summaries carry the corrections given in transfer_ext. For example, transfer_ext's conductor "correction trail" should be signposted from prog_transfer:41-52.

## F23 [severity: minor] prime_ext.tex:1498-1622 (placement of EcGFp5 and $\FF_{p^4}$)
What is wrong: EcGFp5/EcMasFp5 are curves over $\FF_{p^5}$, and the "$\FF_{p^4}$" paragraph extends §gaudry. Both sit in a chapter titled "Prime fields: the tensor route and named targets", while extension.tex and transfer_ext's GLV chapter cover extension fields. transfer_ext.tex:2056-2059 has to point back into the prime chapter for the EcGFp5 quotient.
Recommendation: move §pt-gfp5 and the $\FF_{p^4}$ paragraph to extension.tex (or ch:glvprogramme), and leave a one-line pointer.

## F24 [severity: minor] prime_ext.tex:1561-1563
What is wrong: "Whole-solve CPU falls $1.83$, $1.97$ and $1.72\times$ at the three primes, below the frozen $1.5\times$ bar". The speed-ups exceed 1.5; what is "below" is the new CPU time, under the bar W0/1.5. As written the sentence reads as a failure.
Evidence: ecgfp5_quotient_bw_finish_20261102/README.md:28 "bar (W0/1.5)", :43 "1.5× bar at every prime".
Replacement: "... $1.72\times$ at the three primes, clearing the frozen $1.5\times$ bar, so ...".

## F25 [severity: minor] prime_ext.tex:1733-1734
What is wrong: "the 24- and 28-bit base selections are relabellings (\S\ref{sec:pg-engineering})". §pg-engineering says nothing about 24- or 28-bit selections. The statement is in distance.tex:138.
Replacement: change the reference to "(Chapter~\ref{ch:distance})".

## F26 [severity: minor] prog_prime.tex:292-294; machinery.tex:1173, 1218 (citation)
What is wrong: "F4 trace replay, which is a port of the Joux--Vitse technique" has no citation. machinery cites \cite{jouxvitse2012}, the Eurocrypt 2012 cover-and-decomposition paper, for F4 trace reuse. The F4 variant with trace reuse is Joux and Vitse, "A Variant of the F4 Algorithm", CT-RSA 2011, LNCS 6558.
Action: add a refs.bib entry for the CT-RSA 2011 paper and cite it at prog_prime:293 and machinery:1173/1218. Keep jouxvitse2012 for the cover.

## F27 [severity: minor] transfer_ext.tex:1655-1657 vs prog_frontier.tex:686-688
What is wrong: Two baselines of the plain Nagao arm against two different references appear without reconciliation. transfer_ext gives $S$/rho "1361, 205, 35.2 and 7.19 at $p=53$, 101, 251 and 503, slope $-2.27$" (matched negation rho, Hilbert-stop study). prog_frontier gives "620$\times$ at $p=53$ ... slope $-2.05$" (pooled rho constant 1.344).
Action: in transfer_ext, add "(against the matched negation rho of the Hilbert-stop study; the pooled-constant series of Chapter~\ref{sec:frontier} reads 620 at $p=53$ and slope $-2.05$)".

## F28 [severity: minor] prime_ext.tex:105 and distance.tex:169 (lower end of the Kummer range)
What is wrong: "Kummer tower (Chapter~\ref{sec:prime}) & 20--30 & $28$--$159$". The cited ecbench study's smallest ratio is 29.4 (24 bits). No 28 appears in pkm_tower_ecbench_20261013, and the isogeny-tower variant is 17-137.
Evidence: research/pkm_tower_ecbench_20261013/README.md:56-60.
Action: use "$29$--$159$", or name the source of 28.

## F29 [severity: minor] prime_ext.tex:283-284 (k=5 vs tuned incumbent)
What is wrong: "Tuned for $w=0$, the incumbent is 8--13\% cheaper, and $k=5$ is still $0.25$--$0.49$ of it." The source gives the gap as 2.2-4.6x, which is 0.22-0.45, and the table ratios are 0.217-0.700.
Evidence: research/prime_ksum_pooled_20261029/README.md:113 "against a 2.2–4.6× gap".
Action: replace with the source's "$2.2$--$4.6\times$ cheaper" or cite where 0.25-0.49 comes from.

## F30 [severity: minor] prog_transfer.tex:258-269 (unit of the headline)
What is wrong: The headline everywhere outside prog_transfer is "0.0065x" and "155x" (abstract:63, prog_frontier:696-697, openproblems:140). That is the more favourable unit, group additions. The F_p-multiplication cell (0.0156, 64x) and the S+ cells (37x, 15.5x) are equally validated. Quoting only the most favourable unit in summaries overstates the distance.
Action: wherever 155x is quoted, add "($64\times$ in $\FF_p$ multiplications; $15.5\times$ with additions and lookups charged)".

## F31 [severity: major] Gap: research/jv_sieve_fbline_20261102/ is never cited
What is missing: the frozen controlled study of the factor-base line search on the sieve route (arms P/H/T/U). It has an independent validator report, VAL-20261103-594245 ("Verdict: passed"), plus a red-team review, but no VALIDATION.json ("Validation gate: not admissible ... The protocol names no rho reference"). It is the cover-only half of the H and U arms in Table jac-sieve. No section of the treatise cites it (grep on sections/ is empty).
Action: add one paragraph to transfer_ext §jac-jv before "The sieve route on the same instances", giving its paired ratios against P and its standing (validator passed, gate not admissible).

## F32 [severity: minor] Gap: research/curve25519_autolab_parity_20261105/
What is missing: the 5 November full-width relation recovery and probe correction (all [pcc:unverified]). It records a corrected fast probe and a "legacy affine-equivalent tally" that "must not be treated as a complete native-operation cost ledger". That bears on how far the autolab's w>0 figures can be trusted. Not cited.
Action: one sentence in §c25519 ("A 5 November correction ... no comparative experiment was run").

## F33 [severity: minor] prog_transfer.tex:288-292 (clarity)
What is wrong: "A 9 October whole-route replay ... with an earlier revision of the solver whose source includes the sieve module" does not say which arm produced 0.058. The reader cannot tell whether it is the sieve or the Nagao arm, so it cannot be placed in the two-arms story.
Action: state the arm, or write "(arm not recorded in RESULT.md)".

## F34 [severity: minor] extension.tex:1-12 and prime_ext.tex:1506-1507 (cross-references)
What is wrong: extension.tex's introduction does not mention the GLV-invariant chapter (ch:glvprogramme), which supersedes its odd-characteristic numbers (F12). prime_ext:1506-1507 points to Chapter~\ref{ch:extension} for the JV $(n-1)$ decomposition, but the source of the "$4.5\cdot10^7$--$1.8\cdot10^8\times$" figure is told twice: extension.tex:169-173 and prog_transfer.tex:303-305.
Action: add "Chapter~\ref{ch:glvprogramme} gives the later $k=4$ programme and supersedes the distances below" to extension.tex:12. Delete the duplicate at prog_transfer.tex:303-305 and point to extension.tex.
