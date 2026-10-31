"""R2W analysis: best WDSat-arm / best lab-arm instruction ratio per cell (swept over l on the WDSat side) and the fit of ln(ratio) on n.
Lab side = R2's pooled records (loaded exactly as r2_analyze.py: first record per (arm,target) wins across E1/T/R2 sessions), plus any lab arm
measured in the R2W sessions at an l that R2 did not have for that cell (key lab-l<l>); lab arms repeated at an l R2 had are kept apart as labAA-l<l> (A/A check).
WDSat side: sessions R2W_*; whole = ecbench child Ir (shim time -l, record order) + wdsat_ir - 3.51e7; solver-only = wdsat_ir - calls*START[l] (start-up of one WDSat process, evid/startup).
usage: r2w_analyze.py <runs_dir_with_R2W_sessions> <tuned_20261030 dir> <out.json>"""
import json, glob, os, sys, math, statistics as st
import numpy as np
from scipy import stats
ECB, MS = 3.51e7, 3.8e7
START = {3: 13.90e6, 4: 14.30e6, 5: 13.97e6, 6: 14.31e6, 7: 14.74e6, 8: 14.73e6, 9: 15.56e6, 10: 16.43e6, 11: 17.61e6, 12: 18.88e6, 13: 20.96e6}   # median of the first-mode binaries on the contradictory one-line instance
def ir(p):
    for l in open(p):
        if "instructions retired" in l: return int(l.split()[0])
def load(sess, tm):
    recs = [json.loads(l) for l in open(sess + "/records.jsonl")]
    if all(r["outcome"]["status"] == "error" for r in recs): return []   # l >= n: invalid base dimension, no relations attempted
    tms = sorted(glob.glob(tm + "/*.time")); assert len(tms) == len(recs), (sess, len(tms), len(recs))
    out = []
    for i, r in enumerate(recs):
        ex = (r.get("solver") or {}).get("extra") or {}
        p = r["method"]["params"]; orc = p.get("oracle")
        if not orc: continue
        l = int(p["factor_base"].split("=")[1])
        base = dict(t=r["workload"]["target_index"], ok=r["outcome"]["status"] == "verified", st=r["outcome"]["status"], r=r["workload"]["curve"]["r"], l=l, calls=r["solver"]["calls"])
        if "msolve" in orc:
            kv = dict(x.split("=") for x in orc.split(",")[1:]); key = f"pub-{kv.get('form','e')}-l{l}-s{kv.get('spec','0')}"
            out.append(dict(base, arm=key, kind="msolve", whole=ir(tms[i]) + ex.get("msolve_ir", 0) - ECB, lower=ex.get("msolve_ir", 0) - ex.get("msolve_calls", 0) * MS))
        elif "inherited-f4" in orc:
            out.append(dict(base, arm=f"lab-l{l}", kind="lab", whole=ir(tms[i]) - ECB, lower=None))
        elif "wdsat-s4" in orc:
            wi, wc = ex.get("wdsat_ir", 0), ex.get("wdsat_calls", 0)
            out.append(dict(base, arm=f"wd-l{l}", kind="wd", whole=ir(tms[i]) + wi - ECB, lower=wi - wc * START[l], wd_calls=wc, inexact=ex.get("wdsat_inexact", 0), wd_ir=wi,
                            unsat=ex.get("wdsat_unsat", 0), refuted=ex.get("refuted_by_emitter", 0)))
    return out
def main():
    rdir, tdir, outp = sys.argv[1:4]
    LMAX = int(sys.argv[4]) if len(sys.argv) > 4 else 7   # primary grid l in 3..7; 8 = extension
    os.chdir(tdir)
    E, T, R = "e1/runs/", "runs/", "r2/runs/"
    CELLS = {
     "K0 n=9": ("K0", 9, "k0n9", [(T+"T9", T+"T9_tm"), (E+"E1_k0n9L", E+"E1_k0n9L_tm")] + [(R+x, R+x+"_tm") for x in ("R2_k0n9", "R2_k0n9L")]),
     "K1 n=9": ("K1", 9, "k1n9", [(E+"E1_k1n9", E+"E1_k1n9_tm"), (R+"R2_k1n9", R+"R2_k1n9_tm"), (R+"R2_k1n9L", R+"R2_k1n9L_tm")]),
     "K1 n=11": ("K1", 11, "k1n11", [(E+"E1_k1n11", E+"E1_k1n11_tm"), (E+"E1_k1n11L", E+"E1_k1n11L_tm"), (R+"R2_k1n11", R+"R2_k1n11_tm")]),
     "K0 n=13": ("K0", 13, "k0n13", [(T+"T13b", T+"T13b_tm"), (T+"T13", T+"T13_tm"), (E+"E1_k0n13L", E+"E1_k0n13L_tm"), (R+"R2_k0n13", R+"R2_k0n13_tm")]),
     "K0 n=15": ("K0", 15, "k0n15", [(E+"E1_k0n15", E+"E1_k0n15_tm"), (E+"E1_k0n15_partial_l5s5", E+"E1_k0n15_partial_l5s5_tm"), (R+"R2_k0n15", R+"R2_k0n15_tm"), (R+"R2_k0n15p", R+"R2_k0n15p_tm"), (R+"R2_k0n15L", R+"R2_k0n15L_tm")]),
    }
    res = {}
    for cell, (fam, n, short, sess) in CELLS.items():
        by = {}; rr = None
        for s, tm in sess:
            if not (os.path.isdir(s) and os.path.isdir(tm)): continue
            for x in load(s, tm):
                if x["kind"] != "lab": continue
                rr = x["r"]; by.setdefault(x["arm"], {}).setdefault(x["t"], x)
        r2labs = set(by)
        # R2W sessions for this cell
        for s in sorted(glob.glob(f"{rdir}/R2W_*_{short}")):
            if s.endswith("_tm") or "R2W_V3_" in s: continue
            try: L = load(s, s + "_tm")
            except AssertionError as e: print("SKIP incomplete session", e); continue
            for x in L:
                if x["kind"] == "lab":
                    key = x["arm"] if x["arm"] not in r2labs else "labAA-" + x["arm"][4:]
                    rr = x["r"]; by.setdefault(key, {}).setdefault(x["t"], x)
                elif x["kind"] == "wd":
                    rr = x["r"]; by.setdefault(x["arm"], {}).setdefault(x["t"], x)
        arms = {}
        for a, d in by.items():
            ok = {t: x for t, x in d.items() if x["ok"]}
            e = dict(n=len(ok), n_all=len(d), status=sorted(set(x["st"] for x in d.values())),
                     med_whole=st.median(x["whole"] for x in ok.values()) if ok else None,
                     med_calls=st.median(x["calls"] for x in d.values()))
            if a.startswith("wd"):
                e["med_lower"] = st.median(x["lower"] for x in ok.values()) if ok else None
                e["inexact"] = sum(x.get("inexact", 0) for x in d.values()); e["refuted"] = sum(x.get("refuted", 0) for x in d.values())
            arms[a] = e
        labs = [a for a in arms if a.startswith("lab-") and arms[a]["n"] == 8]
        wds = [a for a in arms if a.startswith("wd-") and arms[a]["n"] == 8 and int(a.split("l")[-1]) <= LMAX]
        bl = min(labs, key=lambda a: arms[a]["med_whole"])
        def pair(wa, key):
            ts = sorted(t for t in by[wa] if by[wa][t]["ok"] and t in by[bl] and by[bl][t]["ok"])
            return ts, [by[wa][t][key] / by[bl][t]["whole"] for t in ts]
        cand = {}
        for wa in wds:
            ts, rw = pair(wa, "whole"); _, rl = pair(wa, "lower")
            cand[wa] = dict(whole_med=st.median(rw), lower_med=st.median(rl), whole=rw, lower=rl,
                            wd_whole_med=st.median(by[wa][t]["whole"] for t in ts), wd_lower_med=st.median(by[wa][t]["lower"] for t in ts))
        bpw = min(cand, key=lambda a: cand[a]["whole_med"]); bpl = min(cand, key=lambda a: cand[a]["lower_med"])
        res[cell] = dict(family=fam, n=n, r=rr, arms=arms, best_lab=bl, lab_ir=arms[bl]["med_whole"], r2_best_lab=min([a for a in r2labs if arms[a]["n"] == 8], key=lambda a: arms[a]["med_whole"]),
            cand={a: {k: v for k, v in c.items() if k not in ("whole", "lower")} for a, c in cand.items()},
            best_wd_whole=bpw, best_wd_lower=bpl, ratio_whole=cand[bpw]["whole_med"], ratio_lower=cand[bpl]["lower_med"],
            per_target_whole=cand[bpw]["whole"], per_target_lower=cand[bpl]["lower"],
            ratio_l4_whole=cand["wd-l4"]["whole_med"] if "wd-l4" in cand else None, ratio_l4_lower=cand["wd-l4"]["lower_med"] if "wd-l4" in cand else None)
        print(f"{cell} r={rr}  best lab {bl} {arms[bl]['med_whole']:.3g} (R2 best {res[cell]['r2_best_lab']})")
        print(f"    best WDSat whole {bpw} {cand[bpw]['wd_whole_med']:.3g} -> ratio {cand[bpw]['whole_med']:.2f} | solver-only {bpl} {cand[bpl]['wd_lower_med']:.3g} -> ratio {cand[bpl]['lower_med']:.2f}")
        for a in sorted(arms, key=lambda a: (a[:3], int(a.split('l')[-1]))):
            e = arms[a]; print(f"      {a:10s} ok {e['n']}/{e['n_all']} {','.join(e['status'])} whole {e['med_whole'] and '%.3g' % e['med_whole']} lower {e.get('med_lower') and '%.3g' % e['med_lower']} calls {e['med_calls']}" + (f" inexact {e['inexact']}" if 'inexact' in e else ''))
    fits = {}
    def fit(pts, label, kind):
        x = np.array([q[1] for q in pts], float); y = np.log([q[2] for q in pts]); k = len(x)
        sl, ic, rv, pv, se = stats.linregress(x, y); t = stats.t.ppf(0.975, k - 2)
        f = dict(k=k, slope=sl, half_ci=t * se, p=pv, lo=sl - t * se, hi=sl + t * se, x4=math.exp(4 * sl), x4_lo=math.exp(4 * (sl - t * se)), x4_hi=math.exp(4 * (sl + t * se)), cells=[q[0] for q in pts])
        fits[f"{kind}|{label}"] = f
        v = "growing" if f["lo"] > 0 else "shrinking" if f["hi"] < 0 else "flat" if (f["lo"] >= -0.1 and f["hi"] <= 0.1) else "undecided"
        v_r2 = "flat(R2 wording)" if (f["lo"] <= 0 <= f["hi"] and f["half_ci"] <= 0.1) else ""
        f["verdict"] = v
        print(f"fit {kind:9s} {label:22s} k={k}: slope {sl:+.3f} +- {t*se:.3f} /bit (95% CI {sl-t*se:+.3f}..{sl+t*se:+.3f}, p={pv:.3f}); x{math.exp(4*sl):.2f} per 4 bits [{math.exp(4*(sl-t*se)):.2f},{math.exp(4*(sl+t*se)):.2f}] -> {v} {v_r2}")
    for kind, field in (("whole", "ratio_whole"), ("lower", "ratio_lower"), ("whole_l4", "ratio_l4_whole"), ("lower_l4", "ratio_l4_lower")):
        pts = [(c, v["n"], v[field]) for c, v in res.items() if v[field]]
        for label, sel in (("all 5 cells", lambda c: True), ("without n=15", lambda c: c != "K0 n=15")):
            p = [x for x in pts if sel(x[0])]
            if len(p) >= 3: fit(p, label, kind)
    json.dump(dict(cells=res, fits=fits), open(outp, "w"), indent=1, default=float)
main()
