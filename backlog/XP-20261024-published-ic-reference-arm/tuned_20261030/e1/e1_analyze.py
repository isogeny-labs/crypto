"""E1 analysis: tuned-published / lab instruction ratio across sizes (PILOT_20261030.md section 9 pricing rule).
usage: python3 e1_analyze.py <out.json>   (cells hard-coded below; each session dir has records.jsonl and a *_tm dir of
ecbench-child `time -l` files from the shim).  Per target: whole = child Ir + sum of msolve Ir; ir_per_target = whole - 3.51e7
(child start-up); published lower bound = msolve Ir - calls*3.8e7 (solving only).  Lab arm = every inherited-f4 arm
(whole run, start-up removed).  Sweep-symmetric: each arm family takes the setting with the smallest median cost.
"""
import json, glob, os, sys, math, statistics as st
ECB_START, MS_START = 3.51e7, 3.8e7
R = "/tmp/claude-501/e1runs/"
def ir(p):
    for l in open(p):
        if "instructions retired" in l: return int(l.split()[0])
def load(sess, tm):
    recs=[json.loads(l) for l in open(sess+"/records.jsonl")]
    tms=sorted(glob.glob(tm+"/*.time")); assert len(tms)==len(recs),(sess,len(tms),len(recs))
    out=[]
    for i,r in enumerate(recs):
        ex=(r.get("solver") or {}).get("extra") or {}
        child=ir(tms[i]); msi=ex.get("msolve_ir",0); mc=ex.get("msolve_calls",0)
        arm=r["arm"]; arm="inherited-f4-l4" if arm=="inherited-f4" and "T9" in sess else ("inherited-f4-l5" if arm=="inherited-f4" else arm)
        out.append(dict(arm=arm,t=r["workload"]["target_index"],ok=r["outcome"]["status"]=="verified",
            whole=child+msi-ECB_START, lower=msi-mc*MS_START, r=r["workload"]["curve"]["r"], curve=r["workload"]["curve"].get("id")))
    return out
CELLS = {  # name: (family, n, [(session, tm)])
 "K0 n=9":  ("K0", 9,  [(R+"ref/T9",R+"ref/T9_tm"),(R+"E1_k0n9L",R+"E1_k0n9L_tm")]),
 "K1 n=9":  ("K1", 9,  [(R+"E1_k1n9",R+"E1_k1n9_tm")]),
 "K1 n=11": ("K1", 11, [(R+"E1_k1n11",R+"E1_k1n11_tm"),(R+"E1_k1n11L",R+"E1_k1n11L_tm")]),
 "K0 n=13": ("K0", 13, [(R+"ref/T13b",R+"ref/T13b_tm"),(R+"ref/T13",R+"ref/T13_tm"),(R+"E1_k0n13L",R+"E1_k0n13L_tm")]),
 "K0 n=15": ("K0", 15, [(R+"E1_k0n15",R+"E1_k0n15_tm")]),
}
if len(sys.argv)>2: CELLS.update(json.load(open(sys.argv[2])))
def main():
    res={}
    for cell,(fam,n,sess) in CELLS.items():
        if not all(os.path.isdir(s) for s,_ in sess): continue
        rows=[]; seen=set()
        try: loaded=[(s,load(s,tm)) for s,tm in sess]
        except AssertionError: print('skip incomplete',cell); continue
        for s,L in loaded:
            for x in L:
                k=(x["arm"],x["t"])
                if x["arm"].startswith("inherited-f4"):
                    x["arm"]=x["arm"]  # lab arms: duplicates across sessions are A/A repeats; keep the first
                if k in seen: continue
                seen.add(k); rows.append(x)
        arms=sorted(set(x["arm"] for x in rows))
        by={a:{x["t"]:x for x in rows if x["arm"]==a and x["ok"]} for a in arms}
        ntar=len(set(x["t"] for x in rows))
        labs=[a for a in arms if a.startswith("inherited-f4") and by[a]]; pubs=[a for a in arms if a.startswith("msolve") and by[a]]
        r_=rows[0]["r"]
        mat={}
        for la in labs:
            for pa in pubs:
                ts=sorted(set(by[la])&set(by[pa]))
                if not ts: continue
                rw=[by[pa][t]["whole"]/by[la][t]["whole"] for t in ts]; rl=[by[pa][t]["lower"]/by[la][t]["whole"] for t in ts]
                mat[la+"|"+pa]=dict(n=len(ts),whole=rw,lower=rl,whole_med=st.median(rw),lower_med=st.median(rl),
                    lab_ir=st.median(by[la][t]["whole"] for t in ts),pub_ir=st.median(by[pa][t]["whole"] for t in ts),pub_solver_ir=st.median(by[pa][t]["lower"] for t in ts))
        lab_med={la:st.median(x["whole"] for x in by[la].values()) for la in labs}
        pub_med={pa:st.median(x["whole"] for x in by[pa].values()) for pa in pubs}
        pub_low={pa:st.median(x["lower"] for x in by[pa].values()) for pa in pubs}
        res[cell]=dict(family=fam,n=n,r=r_,targets=ntar,lab_med=lab_med,pub_med=pub_med,pub_low=pub_low,matrix=mat,
                       verified={a:len(by[a]) for a in arms})
    json.dump(res,open(sys.argv[1],"w"),indent=1)
    for c,v in res.items():
        print(c,"r=%d"%v["r"],"targets",v["targets"],"verified",v["verified"])
        print("  lab whole-run median Ir:",{k:"%.3g"%x for k,x in v["lab_med"].items()})
        print("  pub whole-run median Ir:",{k:"%.3g"%x for k,x in v["pub_med"].items()},"solver-only:",{k:"%.3g"%x for k,x in v["pub_low"].items()})
        for k,m in v["matrix"].items(): print("   %-34s whole %7.1f  lower %6.1f  (n=%d)"%(k,m["whole_med"],m["lower_med"],m["n"]))
main()
