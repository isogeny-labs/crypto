#!/usr/bin/env python3
"""Assemble the four validator-report.md files from the drafts in reports/ and
the validator's outputs in out/. Refuses to write a report that still contains
a placeholder (lesson L-1a5a6b64022c).
Usage: fill_reports.py VALIDATOR_DIR OUT_ROOT"""
import json, os, re, sys
VD, OR = sys.argv[1], sys.argv[2]
rd = lambda f: open(os.path.join(VD, f)).read()
js = lambda f: json.load(open(os.path.join(VD, "out", f)))
common = rd("reports/common.md")
acc = rd("reports/accounting_common.md")
sens = js("pricing_sensitivity.json")
cal = js("calibrated_reading.json")
k = sens["k_window_ops_per_step"]

def certs_summary(files):
    n = rep = dump = ind = 0
    fails = []
    for f in files:
        c = js(f)
        n += c["n"]; rep += c["replay_identical"]; dump += c["dump_identical"]; ind += c["independent_ok"]
        fails += c["failures"]
    return n, rep, dump, ind, fails

def s3(x):
    return f"{x:.2f}"

def sens_line(st, arm, ps):
    parts = []
    for p in ps:
        a = sens[st][f"{arm} p={p} w=0.1"]; b = sens[st][f"{arm} p={p} w=1.0"]
        parts.append(f"p = {p}: w = 0.1 {s3(a['base'])} → IC adds {s3(a['ic_adds'])}, rho window {s3(a['rho_win'])}, both {s3(a['both'])}; "
                     f"w = 1 {s3(b['base'])} → {s3(b['ic_adds'])}, {s3(b['rho_win'])}, {s3(b['both'])}")
    return "; ".join(parts) + "."

def cal_line(st, arm, ps):
    e = cal[st]["per_p"]
    return "; ".join(f"p = {p}: {e[str(p)][arm]['w0']:.3f} / {e[str(p)][arm]['w_cal']:.3f} / {e[str(p)][arm]['10w_cal']:.3f}" for p in ps) + "."

kline = ", ".join(f"{v:.1f} (p = {p})" for p, v in sorted(k.items(), key=lambda t: int(t[0])))

def flips(st):
    """Arms and primes where pricing rho's window compares alone puts the arm
    below R at w = 0.1 and w = 1 (w = 0 is below R at every p >= 257)."""
    out = {}
    for key, v in sens[st].items():
        arm, p, w = key.split(" ")
        if w == "w=0.1":
            other = sens[st][f"{arm} {p} w=1.0"]
            if v["rho_win"] < 1 and other["rho_win"] < 1:
                out.setdefault(arm, []).append(p.split("=")[1])
    return out

def acc_for(st, arm, ps, label):
    a = acc
    a = a.replace("@@ADDS_SENS@@", f"{label}: " + sens_line(st, arm, ps))
    a = a.replace("@@RHO_WORKNATIVE@@",
        f"The validator's dumper (the frozen bin plus a direct `rho_reference_walk` call per rho row, which reproduces each committed rho row's recovered log and group operations) measured {kline} window word compares per rho step, against 1.04–1.13 canonicalisations per step.")
    fl = flips(st)
    if fl:
        txt = "; ".join(f"{a_} at p = {', '.join(v)}" for a_, v in fl.items())
        a = a.replace("@@FLIP@@", f"Under \"rho window\" alone, these arms would be below R at all three prices: {txt}. The band verdict of this study is therefore not robust to pricing rho's compares alone; under \"both\" every arm stays above R at w = 0.1 and 1, and under \"IC adds\" the distances grow 3–5×.")
    else:
        a = a.replace("@@FLIP@@", "No arm of this study is below R at both w = 0.1 and w = 1 under any of the four readings; the verdict \"unresolved\" holds under each.")
    a = a.replace("@@CAL_READING@@", f"{label} " + cal_line(st, arm, ps) + (" W and R were priced with their own runs' w_cal; one common w_cal gives the same order (`out/capped_calibrated_same_wcal.txt`)." if st == "capped" else ""))
    return a

reports = {}
# ---- lucas
n, rep, dump, ind, fails = certs_summary(["certs_lucas_CLMR.json"])
assert n == 400 and rep == 400 and dump == 400 and ind == 400 and not fails
dl = json.load(open(os.path.join(VD, "out", "dump_lucas_CLMR.json")))["rows"]
by = {(r["arm"], r["p"], r["curve_idx"], r["seed"]): r for r in dl}
g2 = sum(1 for (a, p, c, s), r in by.items() if a == "C_double_lp" and r["val_recovered"] % r["r"] == by[("C_lucas", p, c, s)]["val_recovered"] % r["r"])
t = rd("reports/lucas.md")
t = t.replace("@@COMMON@@", common)
t = t.replace("@@ACCOUNTING_COMMON@@", acc_for("lucas", "C_lucas_roots", [16411, 32771, 65537], "arm M"))
t = t.replace("@@LUCAS_G2_LOG@@", f"on all {g2} replayed (p, curve, seed) at p ≤ 2053, L's recovered log equals C's, and both equal M's, R's and the instance's secret mod r. Not checked at p ≥ 4099 (not replayed).")
t = t.replace("@@LUCAS_G2_LOG_VERDICT@@", "consistent at p ≤ 2053; could not check at p ≥ 4099")
t = t.replace("@@LUCAS_REPLAY@@", "(sha256 `6f822439…`; no recorded binary hash to compare) reproduces all 400 rows of run.json at p ≤ 2053 in every deterministic field (all but `wall_ns`, `total_cal`): 400/400 identical. This binds the dirty-tree run to the frozen sources at these primes. At p ≥ 4099 the C and R rows equal the 11-12 control's and the M rows equal the LA study's (whose binary is bound to its freeze commit by hash, A3/A16), so only the L rows at p ≥ 4099 rest on this run alone.")
t = t.replace("@@LUCAS_REPLAY_VERDICT@@", "consistent (p ≤ 2053, 400/400)")
t = t.replace("@@LUCAS_CERTS@@", "400/400 rows pass: f is the documented generator's output for curve seeds 5, 6, 16, 19, 20 (p = 131) … (all 25 curves found in order), squarefree; G and T reduced divisors on the curve; G ≠ 0, [r]G = 0, r prime, r > h, h·r = order inside the Hasse–Weil interval, [order]D = 0 for 4 random divisors; [t]G = T for the recovered t of every C, L, M and R row, and t ≡ t_secret (mod r). The validator's val_dump rows equal the frozen binary's rows (400/400), so each checked log belongs to the committed row. Negative control: corrupting a log, T, f, G or a counted field is caught (`out/negctl_verify.txt`). The 43 test and training instances at every prime up to 131101 were also re-derived from the generator with the validator's arithmetic (curve seed, [order]D = 0, h smallest cofactor; `out/instances_check.json`, negative control `out/instances_negative_control.txt`).")
t = t.replace("@@LUCAS_CERTS_VERDICT@@", "consistent (p ≤ 2053); p ≥ 4099 rows carry no recovered log and were not re-verified")
t = t.replace("@@DISP_LUCAS@@", "validated (measurement, gates and ratios); RESULTS.md needs corrections L4, L11, L13; X2 and X3 must be disclosed")
t = t.replace("@@DISP_TEXT_LUCAS@@", "`validated` for the measurement: the frozen binary rebuilt from `4feb188f47` replays every p ≤ 2053 row exactly, every replayed log passes an independent certificate, the gates recompute, and 48 of 49 transcribed numbers match. Scope limits: rows at p ≥ 4099 (the headline primes) are accepted on determinism, on their identity with the 11-12 control and the LA study, and on the instrument's own `[t]G = T` check, which agreed with independent arithmetic on all 400 replayed rows; run.json records no recovered logs, so they could not be re-verified directly. Corrections needed in RESULTS.md (as an ERRATA file, not in place): 576 runs, not 696 (L11); L's LA share at 32771 is 0.56, not 0.54 (L13); the frozen `uncertainty.py` was a one-stage bootstrap and was deleted at the results commit (L4). The §5a calibrated reading (X3) and rho's dropped window compares with their sensitivity (X2) should be disclosed. No row may be cited beyond `[pcc:unverified]` until a pcc claim records this report.")
reports["ic_positive_control_lucas_20261116"] = t
# ---- la
n, rep, dump, ind, fails = certs_summary(["certs_la_MNSR.json"])
assert n == 400 and rep == 400 and dump == 400 and ind == 400 and not fails
tr = open(os.path.join(VD, "out", "compare_la_train.txt")).read().strip().splitlines()
trn = sum(json.loads(l)["identical"] for l in tr); trt = sum(json.loads(l)["replay_rows"] for l in tr)
t = rd("reports/la.md")
t = t.replace("@@COMMON@@", common)
t = t.replace("@@ACCOUNTING_COMMON@@", acc_for("la", "C_coord_square", [16411, 32771, 65537], "arm S"))
t = t.replace("@@LA_TRAIN_REPLAY@@", f"the binary rebuilt from `caae5eca9f` (bit-identical to `binary.sha256`) reproduces all {trt} training rows at p = 4099 for the four cc values ({trn}/{trt} identical in every deterministic field). The p = 8209 and 16411 training rows were not replayed (dispatch limit), so the training score's binding to the frozen sources is established for a third of its instances.")
t = t.replace("@@LA_TRAIN_VERDICT@@", "consistent at p = 4099; could not check at 8209/16411")
t = t.replace("@@LA_REPLAY@@", "sha256 `d611851a…`, **bit-identical to `binary.sha256`**, so the measured binary is the frozen commit's build. It reproduces all 400 rows at p ≤ 2053 exactly (400/400). The producer's own copy of that binary gives the same 400/400 (`out/producer_bin_replay/`).")
t = t.replace("@@LA_REPLAY_VERDICT@@", "consistent (binary hash and 400/400 rows)")
t = t.replace("@@LA_CERTS@@", "400/400 rows (M, N, S, R × 100) pass the checks listed in the Lucas report (L10): instance from the generator, valid divisors, [r]G = 0, [order]D = 0, [t]G = T, t ≡ t_secret; val_dump rows equal the frozen rows 400/400.")
t = t.replace("@@LA_CERTS_VERDICT@@", "consistent (p ≤ 2053)")
t = t.replace("@@DISP_LA@@", "validated (measurement, gates and ratios); RESULTS.md needs corrections A10, A12, A13; X2 and X3 must be disclosed")
t = t.replace("@@DISP_TEXT_LA@@", "`validated` for the measurement: the test binary is the frozen commit's build (bit-identical rebuild), it replays every p ≤ 2053 row and the p = 4099 training rows exactly, every replayed log passes an independent certificate, the gates and the training score recompute, and all 32 transcribed numbers match. Scope limits as in the Lucas report (p ≥ 4099 test rows and p ≥ 8209 training rows not replayed; no recovered logs in run.json). Corrections needed (ERRATA): 576 runs and 288 N + S runs, not 696 and 348 (A10); the \"incl. additions\" column is total native work, the LA alone counting additions is 0.33–0.34× (A12); the 2.6–5.6 vs 1.9–2.8 comparison is from the test rows (A13). Disclose X2 and X3.")
reports["ic_positive_control_la_20261116"] = t
# ---- unit_t
n, rep, dump, ind, fails = certs_summary(["certs_unit_t_SUVR.json"])
assert n == 400 and rep == 400 and dump == 400 and ind == 400 and not fails
t = rd("reports/unit_t.md")
t = t.replace("@@COMMON@@", common)
t = t.replace("@@ACCOUNTING_COMMON@@", acc_for("unit_t", "C_unit_t", [16411, 32771, 65537, 131101], "arm U"))
t = t.replace("@@UT_REPLAY@@", "sha256 `cd574bde…`, **bit-identical to `binary.sha256`**. It reproduces all 400 rows at p ≤ 2053 exactly (400/400), including the 9 U runs with fallbacks (8 at p = 131, 1 at 2053) and the F1 instance (131, 1, 1001). The producer's copy gives the same 400/400.")
t = t.replace("@@UT_REPLAY_VERDICT@@", "consistent (binary hash and 400/400 rows)")
t = t.replace("@@UT_CERTS@@", "400/400 rows (S, U, V, R × 100) pass the checks of the Lucas report (L10), including every U row that fell back; val_dump rows equal the frozen rows 400/400.")
t = t.replace("@@UT_CERTS_VERDICT@@", "consistent (p ≤ 2053)")
t = t.replace("@@DISP_UT@@", "validated (measurement, gates, D1 and F1); RESULTS.md needs correction U13; X2 and X3 must be disclosed")
t = t.replace("@@DISP_TEXT_UT@@", "`validated` for the measurement: the binary is the frozen commit's build (bit-identical rebuild), it replays every p ≤ 2053 row exactly, every replayed log passes an independent certificate, the gates and all 48 transcribed numbers recompute, D1 is confirmed in every V row and F1's counts are exact (its rank diagnosis is confirmed only on the producer's uncommitted debug dump). Scope limits as in the Lucas report. Corrections needed (ERRATA): the \"LA incl. additions\" column is total native work, and its prose range omits 0.985 at 8209 (U13). Disclose X2, in particular that U would be below rho at all three prices at 16411, 32771 and 131101 if rho's dropped window compares alone were priced (not under the repository's band rule, and not when the IC's additions are priced too), and X3. Note for the reader: the \"first interval to touch 1\" (U at 32771, w = 0.1, lower end 0.957–0.97 depending on the resampling seed) is not a result below rho.")
reports["ic_positive_control_unit_t_20261116"] = t
# ---- capped
n, rep, dump, ind, fails = certs_summary(["certs_capped_W.json", "certs_capped_SR.json"])
assert n == 300 and rep == 300 and dump == 300 and ind == 300 and not fails
t = rd("reports/capped.md")
t = t.replace("@@COMMON@@", common)
t = t.replace("@@ACCOUNTING_COMMON@@", acc_for("capped", "C_unit_t_capped_fast", [16411, 32771, 65537, 131101], "arm W"))
t = t.replace("@@CAP_REPLAY@@", "sha256 `6a97ea07…`, **bit-identical to `binary.sha256`** (two independent rebuilds, 15:49 and in the dumper pair build). It reproduces all 100 W rows of run_w.json and all 200 S/R rows of run_sr.json at p ≤ 2053 exactly (300/300); the producer's copy gives the same.")
t = t.replace("@@CAP_REPLAY_VERDICT@@", "consistent (binary hash and 300/300 rows)")
t = t.replace("@@CAP_CERTS@@", "300/300 rows (W, S, R × 100) pass the checks of the Lucas report (L10), including the 15 W runs that fell back (26 fallbacks); val_dump rows equal the frozen rows 300/300.")
t = t.replace("@@CAP_CERTS_VERDICT@@", "consistent (p ≤ 2053)")
t = t.replace("@@DISP_CAP@@", "validated (measurement, gates and ratios); RESULTS.md needs corrections W8, W11, W13; X2 and X3 must be disclosed")
t = t.replace("@@DISP_TEXT_CAP@@", "`validated` for the measurement: the binary is the frozen commit's build (bit-identical rebuild), it replays every p ≤ 2053 W, S and R row exactly, every replayed log passes an independent certificate, G1 holds field for field, and all 54 transcribed numbers match. Scope limits as in the Lucas report. Corrections needed (ERRATA): state that W equals the unit-T study's U row for row at p = 16411, 32771 and 131101 (and 4 of 12 runs at 4099), so the W/S gain and the W/R values there are U's, already reported, and the capped walk and fast fallback act only at p ≤ 4099 and the fallback primes (W8); relabel the \"LA incl. additions\" column (W11); the w = 0.1 extrapolation is 2^21.7 by the fit, not 2^20–2^21 (W13). Disclose X2, in particular that W (= U there) would be below rho at all three prices at 16411, 32771 and 131101 if rho's dropped window compares alone were priced (not under the repository's band rule, and not when the IC's additions are priced too), and X3.")
reports["ic_positive_control_capped_20261116"] = t
for d, txt in reports.items():
    left = re.findall(r"@@[A-Z_0-9]+@@", txt)
    assert not left, (d, left)
    os.makedirs(os.path.join(OR, d), exist_ok=True)
    open(os.path.join(OR, d, "validator-report.md"), "w").write(txt)
    print("wrote", os.path.join(OR, d, "validator-report.md"), len(txt), "bytes")
