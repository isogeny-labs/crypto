"""S1 loaders.  arm names: inherited-f4-l<l> (lab), wdsat-<A|B|C|D>-l<l> (this branch), wdsat-l<l> (earlier sessions, config A).
whole = child Ir (ECB_TIME_DIR shim, record order) + wdsat_ir - 3.51e7; solver-only = wdsat_ir - calls*START[l]; lab whole = child Ir - 3.51e7."""
import json, glob, statistics as st
ECB = 3.51e7
def ir(p):
    for l in open(p):
        if "instructions retired" in l: return int(l.split()[0])
def load(sess, tm, START):
    recs = [json.loads(l) for l in open(sess + "/records.jsonl")]
    tms = sorted(glob.glob(tm + "/*.time")); assert len(tms) == len(recs), (sess, len(tms), len(recs))
    out = []
    for i, r in enumerate(recs):
        if r["outcome"]["status"] == "error" and (r.get("solver") or {}).get("calls") is None: pass
        ex = (r.get("solver") or {}).get("extra") or {}
        p = r["method"]["params"]; orc = p.get("oracle")
        l = int(p["factor_base"].split("=")[1])
        s = r.get("solver") or {}
        base = dict(t=r["workload"]["target_index"], ok=r["outcome"]["status"] == "verified", st=r["outcome"]["status"], r=r["workload"]["curve"]["r"], l=l, calls=s.get("calls"))
        if "inherited-f4" in orc: out.append(dict(base, arm=f"lab-l{l}", whole=(ir(tms[i]) or 0) - ECB, lower=None))
        elif "wdsat-s4" in orc:
            kv = dict(x.split("=") for x in orc.split(",")[1:] if "=" in x)
            cfg = {(1, 1): "A", (0, 0): "B", (1, 0): "C", (0, 1): "D"}[(int(kv.get("xg", 1)), int(kv.get("sym", 1)))]
            wi, wc = ex.get("wdsat_ir", 0), ex.get("wdsat_calls", 0)
            out.append(dict(base, arm=f"wd{cfg}-l{l}", whole=(ir(tms[i]) or 0) + wi - ECB, lower=wi - wc * START.get(str(l), 0), wd_ir=wi, wd_calls=wc, unsat=ex.get("wdsat_unsat", 0),
                            inexact=ex.get("wdsat_inexact", 0), refuted=ex.get("refuted_by_emitter", 0), lift=ex.get("lift_failures", 0)))
    return out
def summarize(recs):
    by = {}
    for x in recs: by.setdefault(x["arm"], {})[x["t"]] = x
    arms = {}
    for a, d in by.items():
        ok = {t: x for t, x in d.items() if x["ok"]}
        e = dict(n=len(ok), n_all=len(d), status=sorted(set(x["st"] for x in d.values())), med_whole=st.median(x["whole"] for x in ok.values()) if ok else None,
                 med_calls=st.median(x["calls"] for x in d.values() if x["calls"] is not None) if any(x["calls"] is not None for x in d.values()) else None)
        if a.startswith("wd"):
            e["med_lower"] = st.median(x["lower"] for x in ok.values()) if ok else None
            e["inexact"] = sum(x.get("inexact", 0) for x in d.values()); e["lift"] = sum(x.get("lift", 0) for x in d.values())
        arms[a] = e
    return by, arms
if __name__ == "__main__":
    import sys
    START = json.load(open(sys.argv[2])); s = sys.argv[1]
    by, arms = summarize(load(s, s + "_tm", START))
    for a in sorted(arms, key=lambda a: (a[:3], int(a.split("l")[-1]), a)):
        e = arms[a]; print(f"{a:12s} ok {e['n']}/{e['n_all']} {','.join(e['status'])} whole {e['med_whole'] and '%.3g' % e['med_whole']} lower {e.get('med_lower') and '%.3g' % e['med_lower']} calls {e['med_calls']}" + (f" inexact {e['inexact']} lift {e['lift']}" if 'inexact' in e else ""))
