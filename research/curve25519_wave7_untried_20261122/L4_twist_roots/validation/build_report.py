#!/usr/bin/env python3
"""Assemble validator-report.json from the validator's own result files (all keys indexed with [])."""
import hashlib, json, os, subprocess, sys

S = os.path.dirname(os.path.abspath(__file__))
L4 = "/home/user/crypto/research/curve25519_wave7_untried_20261122/L4_twist_roots"
R = "research/curve25519_wave7_untried_20261122/L4_twist_roots"
J = lambda f: json.load(open(os.path.join(S, f)))
facts, ident, exh, pp = J("facts.json"), J("identity.json"), J("exh_k16.json"), J("p5p6.json")
rep = {k: J(f"replay_k{k}.json") for k in ("16_given", "20_given", "20_std", "24_std")}
cmp = {k: open(os.path.join(S, f"cmp_{k}.txt")).read().strip().splitlines()[-1] for k in ("exh16", "16_given", "20_given", "20_std", "24_std")}
for k, v in cmp.items():
    assert v == "ALL CELLS EQUAL", (k, v)


def np_version():
    import numpy
    return numpy.__version__


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def git(*a):
    return subprocess.run(["git", "-C", "/home/user/crypto", *a], capture_output=True, text=True, check=True).stdout.strip()


frozen = ["PROTOCOL.md", "l4core.py", "run_l4.py", "controls.py", "analyze_l4.py", "run_all.sh"]
frozen_hashes = {}
for f in frozen:
    a = hashlib.sha256(subprocess.run(["git", "-C", "/home/user/crypto", "show", f"e897b1a0f8:{R}/{f}"], capture_output=True, check=True).stdout).hexdigest()
    b = hashlib.sha256(subprocess.run(["git", "-C", "/home/user/crypto", "show", f"c47218b9d5:{R}/{f}"], capture_output=True, check=True).stdout).hexdigest()
    frozen_hashes[f] = {"at_e897b1a0f8": a, "at_c47218b9d5": b, "worktree": sha(os.path.join(L4, f)), "identical": a == b == sha(os.path.join(L4, f))}
artifacts = {}
for f in sorted(os.listdir(os.path.join(L4, "data"))):
    artifacts[f"data/{f}"] = sha(os.path.join(L4, "data", f))
for f in ("README.md", "run.log", "REPORT.typ", "REPORT.pdf", "figure.py", "l4_roots.svg", "l4_roots.png"):
    artifacts[f] = sha(os.path.join(L4, f))

ex = {(t["pair_class"], t["x1_eq_x2"]): t for t in exh["table"]}
f16, f20, f24 = facts["16"], facts["20"], facts["24"]

checks = [
 {"name": "binding: validated commit and tree state",
  "result": "pass",
  "detail": f"Validated from HEAD {git('rev-parse', 'HEAD')} on branch claude/curve25519-index-calculus-i674j4 (last commit touching L4: {git('log', '-1', '--format=%h', '--', R)}; HEAD advanced during validation through commits of other leads only); `git status` clean for {R} at validation time "
            f"(other lead directories had uncommitted changes from concurrent agents, outside scope). Run commit c47218b9d5 is an ancestor of HEAD; no L4 file in "
            f"data/ or the frozen set was modified after its first commit (git log --name-status shows only A for data files)."},
 {"name": "(6) frozen files at e897b1a0f8 byte-identical to those at the run commit and in the worktree",
  "result": "pass",
  "detail": "sha256 of PROTOCOL.md, l4core.py, run_l4.py, controls.py, analyze_l4.py, run_all.sh identical at e897b1a0f8, c47218b9d5 and the worktree "
            "(see frozen_hashes). Freeze commit 2026-11-22T03:05:29Z precedes run.log launch 03:05:56Z and the first data commit 2d683358c2 (03:06:28Z)."},
 {"name": "(6) run.log shows the frozen commands",
  "result": "partial",
  "detail": "run.log records 'launch 2026-11-22T03:05:56Z commit e897b1a0f8' and then the outputs in exactly the order and with the parameters of the frozen "
            "run_all.sh (controls k16/k20 n=20000 n3=1e6; exhaustive k16 65437 rows; sample k20 n=2e8; given k16, k20 n=5e7; controls k24 n=5000; sample k24 n=5e7; "
            "analysis). It does not echo the command lines themselves, nor python/numpy versions. The analysis block at the end of run.log is byte-identical to "
            "data/results_main.md, and re-running the frozen analyze_l4.py on copies of the run JSONs regenerates results_main.md and results_main.json byte-identically. "
            "PROTOCOL section 3 lists the k24 controls before the cells; run_all.sh (also frozen) runs them after; no effect on any number. See erratum E7."},
 {"name": "artifact integrity: hist vs hist_rows, totals",
  "result": "pass",
  "detail": "For all five run JSONs the validator's own decode of 'hist' equals 'hist_rows' and sum(hist) == npairs."},
 {"name": "(1) curve facts by two independent methods",
  "result": "pass",
  "detail": f"Euler-criterion character sums give #E = {f16['#E_charsum']}, {f20['#E_charsum']}, {f24['#E_charsum']} and twist orders {f16['#twist_charsum']}, "
            f"{f20['#twist_charsum']}, {f24['#twist_charsum']} for k = 16, 20, 24; x-only ladder point orders on random E-class and T-class x have lcm equal to "
            "these orders and each is the unique multiple of the lcm in the Hasse interval (E(F_p) cyclic of order 8r, twist cyclic of order 4r'). "
            f"r = {f16['r']}, {f20['r']}, {f24['r']} and r' = {f16['r_twist']}, {f20['r_twist']}, {f24['r_twist']} are prime (trial division); "
            "#E + #twist = 2p + 2; A^2 - 4 non-square at every k so Z = {0}; x-class counts Z/E/T = "
            f"{f16['n_x_Z']}/{f16['n_x_E']}/{f16['n_x_T']}, {f20['n_x_Z']}/{f20['n_x_E']}/{f20['n_x_T']}, {f24['n_x_Z']}/{f24['n_x_E']}/{f24['n_x_T']}. "
            "All equal the README table and the run JSON curve_facts."},
 {"name": "(2) discriminant identity disc/4 = 4 f(x1) f(x2), symbolic",
  "result": "pass",
  "detail": "Expanded B2^2 - (x1-x2)^2 (x1 x2-1)^2 - 4 f(x1) f(x2) in Z[x1, x2, A] with the validator's own polynomial arithmetic: the zero polynomial. "
            "The identity therefore holds over every F_p (not only on sampled pairs), and the reversed ('given') polynomial has the same disc/4 because a c is symmetric."},
 {"name": "(2) consequence: mixed pairs have no F_p root (brute force, no discriminant used)",
  "result": "pass",
  "detail": f"At p = 101 and p = 103, for every A with A^2 != 4 (99 and 101 values, about half with full rational 2-torsion), S3 was evaluated at every X in F_p for every pair: "
            f"mixed (E,T) pairs with an F_p root 0 of {ident['bruteforce']['101']['mixed_pairs']} and 0 of {ident['bruteforce']['103']['mixed_pairs']}; non-diagonal non-Z "
            "same-class pairs with != 2 roots 0; same-class roots on the other curve 0; identity failures 0. Derivation: with a = (x1-x2)^2 != 0 (mixed implies x1 != x2), "
            "disc/4 = 4 f(x1) f(x2) is (square)(non-square), a non-square, so no F_p root."},
 {"name": "(3) independent exhaustive census at k = 16 (all classes)",
  "result": "pass",
  "detail": f"All {exh['npairs']} unordered pairs x1 <= x2 classified with the validator's own code (Euler QR table, Atkin square roots, Fermat inverses, disc computed directly "
            f"as B2^2 - a c), {exh['cpu_s']} s CPU on one core. Every cell of all 8 rows equals data/exhaustive_k16_std_main.json and the README table: (E,twist) "
            f"{ex[('ET', False)]['pairs']} pairs, {ex[('ET', False)]['roots0']} with 0 roots, 0 with any root; (E,E) non-diagonal {ex[('EE', False)]['pairs']} all with 2 roots, "
            f"{ex[('EE', False)]['roots_on_E']} on E, 0 on twist, {ex[('EE', False)]['roots_on_Z']} at x = 0; (twist,twist) non-diagonal {ex[('TT', False)]['pairs']}, "
            f"{ex[('TT', False)]['roots_on_twist']} on twist, 0 on E, {ex[('TT', False)]['roots_on_Z']} at x = 0."},
 {"name": "sampled cells replayed from the protocol seed rule (k = 20, 24 std; k = 16, 20 given)",
  "result": "pass",
  "detail": f"Re-drawing the samples from the PROTOCOL seed rule (sha256 first 8 bytes big-endian, default_rng, numpy {np_version()}) and classifying with the validator's "
            "own root code reproduces every cell of sample_k20_std_main.json (601,048,573 pairs), sample_k24_std_main.json (166,777,141), sample_k16_given_main.json "
            "(150,065,437) and sample_k20_given_main.json (151,048,573) exactly."},
 {"name": "(4) P5 2-torsion-translate counts",
  "result": "pass",
  "detail": "S3(x1, x2, 0) = (x1 x2 - 1)^2, so a root at x = 0 (the only Z point) occurs iff x1 x2 = 1; x(P + T0) = 1/x confirmed by explicit addition on 4000 points per k; "
            "f(1/x) = f(x)/x^4 for every x != 0 so the class is preserved. Unordered pairs {x, 1/x}, x != 1/x: "
            f"k16 EE {pp['16']['P5_pairs_x_1overx']['EE']}, TT {pp['16']['P5_pairs_x_1overx']['TT']}, ET 0; k20 {pp['20']['P5_pairs_x_1overx']['EE']}, "
            f"{pp['20']['P5_pairs_x_1overx']['TT']}; k24 {pp['24']['P5_pairs_x_1overx']['EE']}, {pp['24']['P5_pairs_x_1overx']['TT']} (= (|class| - 1)/2; x = 1 and x = -1 are "
            f"the 2 fixed points). The validator's exhaustive census has exactly {ex[('EE', False)]['pairs_with_root_on_Z']} (E,E) and {ex[('TT', False)]['pairs_with_root_on_Z']} "
            "(twist,twist) non-diagonal pairs with a root at x = 0, 0 mixed: equal to 16417 and 16300. Sampled k20: observed 390, 420 vs expected 381.09, 381.85 "
            "(z = 0.46, 1.95); k24: 4, 2 vs 5.96, 5.96 (z = -0.80, -1.62), recomputed from unrounded values."},
 {"name": "(4) P6 diagonal",
  "result": "pass_with_erratum",
  "detail": "For every non-Z x the linear S3 root c/(2 B2) equals x(2P) from the Weierstrass doubling formula, and lies on the point's own curve except x = 1 and x = -1 "
            "(x(2P) = 0) at every k; 0 on the other curve. The single ZZ diagonal pair x1 = x2 = 0 has B2 = 0, c = 1: no finite root (2 T0 = O). The frozen P6 wording "
            "('x1 = x2: exactly one finite root x(2P) ... 100%') does not cover it, and analyze_l4.py evaluates P6 on EE/TT diagonals only. See erratum E1."},
 {"name": "(5) (E,E) and (T,T) roots equal x(P1 +- P2) by the validator's point arithmetic",
  "result": "pass",
  "detail": "Long-Weierstrass affine addition (twist mapped to Y^2 = X^3 + A n X^2 + n^2 X, X = n x, with a large non-residue n, not the producer's smallest one): "
            f"k16 EE {pp['16']['points']['EE']['roots_eq_x(P1+-P2)']}/{pp['16']['points']['EE']['tested']}, TT {pp['16']['points']['TT']['roots_eq_x(P1+-P2)']}/{pp['16']['points']['TT']['tested']}; "
            f"k20 {pp['20']['points']['EE']['roots_eq_x(P1+-P2)']}/{pp['20']['points']['EE']['tested']}, {pp['20']['points']['TT']['roots_eq_x(P1+-P2)']}/{pp['20']['points']['TT']['tested']}; "
            f"k24 {pp['24']['points']['EE']['roots_eq_x(P1+-P2)']}/{pp['24']['points']['EE']['tested']}, {pp['24']['points']['TT']['roots_eq_x(P1+-P2)']}/{pp['24']['points']['TT']['tested']} "
            "root sets equal; S3 vanishes at both sums in every case. Mixed pairs over F_p^2 (2000 per k): both x(P1 +- P2) are S3 roots, conjugate, outside F_p; "
            "300 k16 mixed pairs evaluated at all 65437 X: 0 roots."},
 {"name": "(7) README numbers vs data and recomputation",
  "result": "pass",
  "detail": "Every cell of the README exhaustive and sampled tables equals results_main.json (sampled table has 'roots on twist' and 'roots on E' columns swapped "
            "relative to the exhaustive table; values are under the right headers). Pair-count arithmetic: C(32835,2) = 539,052,195, C(32601,2) = 531,396,300, "
            "32835 x 32601 = 1,070,453,835, p(p+1)/2 = 2,141,033,203. Rule-of-three 3/2e8 = 1.5e-8 (Clopper-Pearson 1.498e-8). P8 fractions 0.5004, 0.4996, 0.4996, "
            "0.4998, 0.5006 match the controls JSONs. P7a 19999/19999, 19998/19998; P7b 20000/20000; P9 20000/20000 match. Timings match the JSONs (but are wall-clock; E6)."},
 {"name": "(7) every result sentence carries [pcc:unverified]",
  "result": "fail_minor",
  "detail": "Untagged result sentences: 'At every k the only rational 2-torsion x is 0 (A^2 - 4 is a non-square).'; 'Positive controls (P7a, P7b) show the counter and "
            "the mechanism are visible.'; the bold lead sentences of Answer items 1-3; Answer item 4 first sentence; and four sentences of 'What the outcome implies "
            "for twist points in a factor base'. See errata E3, E4."},
 {"name": "(7) derived vs measured labelling",
  "result": "fail_moderate",
  "detail": "The S_m (m > 3) argument is labelled 'Argued, not measured' and tagged, and is correct (R in E(F_p) with pi(R) = -R forces 2R = 0). The factor-base "
            "implication is labelled derived only in Answer item 4's last sentence; the section 'What the outcome implies ...' states it as fact without a derived label "
            "or tags, and 'Derived versus measured' omits it. See E3."},
 {"name": "(7) no relabelling; scope; reporting contract",
  "result": "pass",
  "detail": "No rho or BSGS relabelled as index calculus; no distance to rho asserted; no leaderboard or benchmarks row; ratio 1.00 for the lever is stated as derived "
            "and matches STRUCTURE-R4.md line 148. Section 5a band, certificates and the m=83/ECC2K-130 rules are not applicable (no cross-method cost comparison, no "
            "DLP solve, no relation or closure claim)."},
 {"name": "controls present and meaningful",
  "result": "pass",
  "detail": "Positive controls C1/C2 (explicit arithmetic), negative/sensitivity control C3 (A+1 gives about 0.5 F_p-root fraction in every class), identity C4, "
            "Z predictor C5 all present for k = 16, 20, 24; the validator re-derived C1, C2, C4 and C5 independently (above)."},
]

errata = [
 {"id": "E1", "severity": "moderate",
  "text": "P6 as frozen ('x1 = x2: exactly one finite root x(2P) ... 100%') does not hold for the one ZZ diagonal pair x1 = x2 = 0: 2 T0 = O, S3 degenerates to the "
          "constant 1 and has no finite root (README exhaustive table row '(Z,Z) yes 1 pair, 0 roots'; same in the k = 20 and k = 24 Z strata, omitted from the README "
          "sampled table). analyze_l4.py evaluates P6 on the EE and TT diagonals only, so 'P6 held' and 'All 10 numbered predictions (P1 to P10) held; none failed' "
          "overstate the frozen wording. The exception follows from the mathematics and the count is otherwise exact (32,835 E and 32,601 twist diagonal pairs).",
  "suggested_fix": "In the README Errata section (or a sibling ERRATA.md; leave data/ byte-equal): state that P6 as worded fails on 1 of 65,437 diagonal pairs at "
                   "k = 16 (x = 0, 2 T0 = O, 0 finite roots), that P6 held on the EE/TT diagonals, and replace 'none failed' with '9 held as worded; P6 held on "
                   "the EE/TT diagonals and fails on the x = 0 diagonal pair, which the protocol did not except'."},
 {"id": "E2", "severity": "minor",
  "text": "In data/results_main.md the 'given' cells label the leading-coefficient-zero flag as 'x1=x2'. For the given polynomial a = (x1 x2 - 1)^2, so the flag marks "
          "x1 x2 = 1 pairs (the ZZ pair (0,0) appears as 'x1=x2 False'). Validator replay: k = 16 given has 1595 (E,E) and 1594 (twist,twist) pairs with x1 x2 = 1 "
          "but 1474 and 1518 with x1 = x2; k = 20 given 94/99 vs 91/97. The P6 lines of the given cells therefore test x1 x2 = 1 pairs, and README's '100% two-root "
          "same-class pairs' for the given cells holds only for pairs with x1 x2 != 1 (those have one finite root, the other at infinity).",
  "suggested_fix": "Errata note: the column is 'a = 0 (x1 x2 = 1)' in the given cells, the given-cell P6 lines are about x1 x2 = 1 pairs, and the README "
                   "sentence becomes '100% two-root same-class pairs with x1 x2 != 1; the 1595 + 1594 (k = 16) and 94 + 99 (k = 20) pairs with x1 x2 = 1 have one "
                   "finite root'."},
 {"id": "E3", "severity": "moderate",
  "text": "The factor-base implication is not consistently labelled derived and is mostly untagged: 'What the outcome implies for twist points in a factor base' "
          "states 'Twist points therefore add no relations for a target on E', 'They serve only the twist's own discrete logarithm ... which carries no information "
          "about E', 'The only mixed rational relations run through translation by 2-torsion (P5) ... as in the torsion leads' and 'A factor base using twist "
          "x-coordinates lowers the density of usable columns' as facts without [pcc:unverified] or a derived label; Answer item 4's first sentence is also "
          "untagged; 'Derived versus measured' omits the implication (PROTOCOL section 7 calls it 'a derivation plus count'). 'As in the torsion leads' cites a study "
          "that 'Sources checked' says was not read. 'Can use only E-class base points' omits x = 0 (T0), which does give relations S3(x_Q, 0, 1/x_Q) = 0.",
  "suggested_fix": "Add a 'Derived (argued from the S3 counts, not measured): factor-base implication' bullet to 'Derived versus measured', tag each sentence of "
                   "the implication section, drop or source 'as in the torsion leads', and write 'only E-class base points or x = 0'."},
 {"id": "E4", "severity": "minor",
  "text": "Other result sentences without [pcc:unverified]: 'At every k the only rational 2-torsion x is 0 (A^2 - 4 is a non-square).'; 'Positive controls (P7a, "
          "P7b) show the counter and the mechanism are visible.'; the bold lead sentences of Answer items 1-3 (the following sentences are tagged).",
  "suggested_fix": "Append [pcc:unverified] to each."},
 {"id": "E5", "severity": "minor",
  "text": "README heading 'Erratum to the brief' says 'The brief writes S3 = (x1 x2 - 1)^2 x3^2 ...', but research/curve25519_wave7_untried_20261122/BRIEF.md "
          "contains no S3 formula; PROTOCOL section 2 correctly attributes the reversed form to 'the task text'.",
  "suggested_fix": "Retitle to 'Erratum to the task text' and replace 'the brief' by 'the L4 task text (dispatch)' in that section."},
 {"id": "E6", "severity": "minor",
  "text": "Times labelled 'CPU' (305 s, 97 s, 52 s; cpu_seconds in the JSONs) are wall-clock time.time() intervals on a loaded shared host, not CPU time.",
  "suggested_fix": "Relabel as 'wall s (shared host)'; use time.process_time() in future runs."},
 {"id": "E7", "severity": "minor",
  "text": "Reproducibility binding is incomplete: run.log does not echo the commands (only the launch commit and outputs, consistent with run_all.sh), and neither "
          "run.log nor the run JSONs record the git commit, python or numpy version (the validator's replay on this host, numpy 2.5.3, reproduced every sampled "
          "draw, so the binding holds here de facto). results_main.md prints 'complete=True' for sampled cells from d.get('complete', True), a default rather than a "
          "recorded field (harmless: it only gates exhaustive-mode P5).",
  "suggested_fix": "Errata note recording python 3.13.18 / numpy 2.5.3 as the replay environment; future launchers log 'set -x' command lines and write commit and "
                   "versions into each JSON."},
 {"id": "E8", "severity": "minor",
  "text": "P10 has no machine verdict: analyze_l4.py evaluates P1-P9 only, and the code never tests r or r' for primality (it follows from complete trial "
          "division in factor_small). README's 'P10 held' is read off the curve-facts line. The validator confirms P10 independently.",
  "suggested_fix": "Say 'P10 held (read from curve_facts; confirmed by the validator)'."},
 {"id": "E9", "severity": "info",
  "text": "The README sampled table puts 'roots on twist' before 'roots on E', the reverse of the exhaustive table, and omits the diagonal and ZZ rows (370/387 and "
          "3/3 diagonal pairs, 1 ZZ pair) that are included in its pair totals. Values are correct.",
  "suggested_fix": "Use one column order in both tables, or note the swap."},
 {"id": "E10", "severity": "info",
  "text": "'Frozen before any data': the Phase 1 smoke (labelled not data, as BRIEF Phase 1 requires) had already classified 400 of 65,437 k = 16 rows "
          "(26,095,000 pairs, 1.2% of the M16 population) and 2e5-pair samples at k = 16 and 20 before the freeze; the P5 targets came from the predictor run, not the counts.",
  "suggested_fix": "Write 'frozen before the confirmatory run (the smoke saw 1.2% of the k = 16 population)'."},
]

recomputed = {
 "curve_facts": {k: {kk: facts[k][kk] for kk in ("p", "A", "#E_charsum", "#twist_charsum", "r", "r_twist", "r_prime", "r_twist_prime", "n_x_Z", "n_x_E", "n_x_T",
                                                   "Z_x", "A2-4_square", "class_x=1", "class_x=-1")} | {"ladder_E_unique": facts[k]["ladder_E"]["unique_and_equal"],
                                                   "ladder_twist_unique": facts[k]["ladder_twist"]["unique_and_equal"]} for k in ("16", "20", "24")},
 "identity_symbolic_zero": ident["symbolic"]["identity_holds"],
 "bruteforce_small_p": ident["bruteforce"],
 "exhaustive_k16": {"npairs": exh["npairs"], "cpu_s": exh["cpu_s"], "table": exh["table"]},
 "replays": {k: {"npairs": v["npairs"], "cpu_s": v["cpu_s"], "diag_vs_a0": v["diag_vs_a0"], "table": v["table"]} for k, v in rep.items()},
 "P5_pairs_x_1overx": {k: pp[k]["P5_pairs_x_1overx"] for k in ("16", "20", "24")},
 "P5_sampled_expected": {"k20": {"EE": [390, 381.089, 0.46], "TT": [420, 381.853, 1.95]}, "k24": {"EE": [4, 5.958, -0.80], "TT": [2, 5.963, -1.62]},
                         "format": "[observed, expected, z]"},
 "P6": {k: {"diag_classes": pp[k]["P6_diag_classes"], "lin_root_eq_x2P": pp[k]["P6_lin_root_eq_x2P_all_nonZ_x"], "x0_diag": pp[k]["P6_x0_diag"]} for k in ("16", "20", "24")},
 "points": {k: pp[k]["points"] for k in ("16", "20", "24")},
 "pair_arithmetic": {"C(32835,2)": 539052195, "C(32601,2)": 531396300, "32835*32601": 1070453835, "p(p+1)/2_k16": 2141033203,
                     "rule3_k20": 1.5e-8, "clopper_pearson_k20": 1.4978661200260035e-08},
}

commands = [
 "python3 tools/lessons.py brief --scope validator",
 "git -C /home/user/crypto show <commit>:<path> | sha256sum   (frozen-file comparison at e897b1a0f8, c47218b9d5, worktree)",
 "python3 -I v_facts.py facts.json",
 "python3 -I v_identity.py identity.json",
 "nice -n 5 python3 -I v_exhaustive.py exh_k16.json",
 "python3 -I v_compare.py exh_k16.json data/exhaustive_k16_std_main.json",
 "nice -n 5 python3 -I v_p5p6_points.py p5p6.json",
 "nice -n 5 python3 -I v_replay.py replay_k{16,20}_given.json {16,20} given 50000000; v_replay.py replay_k20_std.json 20 std 200000000; v_replay.py replay_k24_std.json 24 std 50000000",
 "python3 -I v_compare.py replay_<cell>.json data/sample_<cell>_main.json leading_coeff_zero",
 "python3 -I analyze_l4.py --dir <scratch copy of data> --tag main --ks 16,20,24 ; cmp with data/results_main.{md,json}",
 "python3 -I build_report.py",
]


report = {
 "schema": "validator-report/v1 (L4 wave 7)",
 "lead": "L4 mixed curve/twist S3 roots",
 "directory": R,
 "validated_commit": git("rev-parse", "HEAD"),
 "last_commit_touching_L4": git("log", "-1", "--format=%H", "--", R),
 "protocol_commit": git("rev-parse", "e897b1a0f8"),
 "run_commit": git("rev-parse", "c47218b9d5"),
 "validator_date": "2026-11-22",
 "verdict": "pass_with_errata",
 "verdict_reason": "Every measured number in README.md and data/ was reproduced exactly by independent code (exhaustive k = 16 census of 2,141,033,203 pairs, "
                   "seed-rule replays of all four sampled cells, curve facts by two methods, the discriminant identity symbolically, P5/P6 and point arithmetic). "
                   "Main result reproduced: 0 of 1,070,453,835 mixed pairs at k = 16 have an F_p root. Errata are scope and labelling: P6's wording "
                   "versus the x = 0 diagonal pair, the given-cell flag label, untagged or unlabelled derived sentences (factor-base implication), attribution, "
                   "timing label and reproducibility logging. No erratum changes a count or the P1 conclusion.",
 "independence": "Validator code in the session scratchpad (vcore.py, v_facts.py, v_identity.py, v_exhaustive.py, v_p5p6_points.py, v_replay.py, v_compare.py) "
                 "imports none of l4core.py, run_l4.py, controls.py, analyze_l4.py; only the protocol text (seed rule, polynomial) was reused. analyze_l4.py was run "
                 "once, on a scratch copy, solely to check that results_main.{md,json} regenerate from the JSONs.",
 "environment": {"python": sys.version.split()[0], "numpy": np_version(), "host": "shared 4-core container, one core used, nice 5"},
 "checks": checks,
 "errata": errata,
 "recomputed": recomputed,
 "frozen_hashes": frozen_hashes,
 "artifact_sha256_at_validation": artifacts,
 "pinning_note": "data/*.json and data/results_main.md are pinned above and must stay byte-equal; record errata in the README Errata section or a sibling ERRATA.md.",
 "commands": commands,
 "not_applicable": ["section 5a band (no cross-method cost comparison)", "DLP/relation/closure certificates (none claimed)", "m=83 / ECC2K-130 disclosure"],
 "edits_by_validator": ["validator-report.json (this file) only"],
}
out = os.path.join(L4, "validator-report.json")
with open(out, "w") as f:
    json.dump(report, f, indent=1)
    f.write("\n")
print("wrote", out, os.path.getsize(out), "bytes")
