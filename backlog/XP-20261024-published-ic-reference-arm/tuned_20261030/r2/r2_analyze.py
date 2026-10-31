"""R2 analysis: best-published / best-lab instruction ratio per cell, swept on both sides, and the fit of ln(ratio) on n.
Pricing = E1 / PILOT section 9: per target whole = ecbench-child Ir (shim time -l file, in record order) + msolve Ir - 3.51e7;
solver-only lower bound = msolve Ir - calls*3.8e7.  Arms are keyed by (factor_base, oracle) so sessions are pooled across E1/T/R2;
first record per (arm,target) wins (lab duplicates are A/A repeats).  usage: python3 r2_analyze.py <out.json>   (run from tuned_20261030/)"""
import json, glob, os, sys, math, statistics as st
import numpy as np
from scipy import stats
ECB, MS = 3.51e7, 3.8e7
def ir(p):
    for l in open(p):
        if "instructions retired" in l: return int(l.split()[0])
def load(sess, tm):
    recs=[json.loads(l) for l in open(sess+"/records.jsonl")]
    tms=sorted(glob.glob(tm+"/*.time")); assert len(tms)==len(recs),(sess,len(tms),len(recs))
    out=[]
    for i,r in enumerate(recs):
        ex=(r.get("solver") or {}).get("extra") or {}
        p=r["method"]["params"]; orc=p.get("oracle")
        if not orc: continue
        l=int(p["factor_base"].split("=")[1])
        if "msolve" in orc:
            kv=dict(x.split("=") for x in orc.split(",")[1:]) ; key=f"pub-{kv.get('form','e')}-l{l}-s{kv.get('spec','0')}"
        elif "inherited-f4" in orc: key=f"lab-l{l}"
        else: continue
        out.append(dict(arm=key,t=r["workload"]["target_index"],ok=r["outcome"]["status"]=="verified",
            whole=ir(tms[i])+ex.get("msolve_ir",0)-ECB, lower=ex.get("msolve_ir",0)-ex.get("msolve_calls",0)*MS,
            r=r["workload"]["curve"]["r"]))
    return out
E="e1/runs/"; T="runs/"; R="r2/runs/"
CELLS={ # name: (family a, n, [sessions])
 "K0 n=9":("K0",9,[(T+"T9",T+"T9_tm"),(E+"E1_k0n9L",E+"E1_k0n9L_tm")]+[("r2/runs/"+x,"r2/runs/"+x+"_tm") for x in ("R2_k0n9","R2_k0n9L")]),
 "K1 n=9":("K1",9,[(E+"E1_k1n9",E+"E1_k1n9_tm"),("r2/runs/R2_k1n9","r2/runs/R2_k1n9_tm"),("r2/runs/R2_k1n9L","r2/runs/R2_k1n9L_tm")]),
 "K1 n=11":("K1",11,[(E+"E1_k1n11",E+"E1_k1n11_tm"),(E+"E1_k1n11L",E+"E1_k1n11L_tm"),("r2/runs/R2_k1n11","r2/runs/R2_k1n11_tm")]),
 "K0 n=13":("K0",13,[(T+"T13b",T+"T13b_tm"),(T+"T13",T+"T13_tm"),(E+"E1_k0n13L",E+"E1_k0n13L_tm"),("r2/runs/R2_k0n13","r2/runs/R2_k0n13_tm")]),
 "K0 n=15":("K0",15,[(E+"E1_k0n15",E+"E1_k0n15_tm"),(E+"E1_k0n15_partial_l5s5",E+"E1_k0n15_partial_l5s5_tm"),("r2/runs/R2_k0n15","r2/runs/R2_k0n15_tm"),("r2/runs/R2_k0n15p","r2/runs/R2_k0n15p_tm"),("r2/runs/R2_k0n15L","r2/runs/R2_k0n15L_tm")]),
}
def main():
    res={}
    for cell,(fam,n,sess) in CELLS.items():
        by={}; rr=None
        for s,tm in sess:
            if not os.path.isdir(s) or not os.path.isdir(tm): continue
            try: L=load(s,tm)
            except AssertionError as e: print("skip",e); continue
            for x in L:
                rr=x["r"]
                by.setdefault(x["arm"],{}).setdefault(x["t"],x)
        arms={}
        for a,d in by.items():
            ok={t:x for t,x in d.items() if x["ok"]}
            arms[a]=dict(n=len(ok),n_all=len(d),med_whole=st.median(x["whole"] for x in ok.values()) if ok else None,
                         med_lower=st.median(x["lower"] for x in ok.values()) if ok else None)
        labs=[a for a in arms if a.startswith("lab") and arms[a]["med_whole"]]
        pubs=[a for a in arms if a.startswith("pub") and arms[a]["med_whole"]]
        def pair(pa,la):
            ts=sorted(t for t in by[pa] if by[pa][t]["ok"] and t in by[la] and by[la][t]["ok"])
            rw=[by[pa][t]["whole"]/by[la][t]["whole"] for t in ts]; rl=[by[pa][t]["lower"]/by[la][t]["whole"] for t in ts]
            return ts,rw,rl
        # best lab: smallest median over its targets (all lab arms have the full target set)
        bl=min(labs,key=lambda a:arms[a]["med_whole"])
        # candidate published arms compared on a common footing: median over the targets they share with best lab
        cand={}
        for pa in pubs:
            ts,rw,rl=pair(pa,bl)
            if ts: cand[pa]=dict(targets=ts,whole_med=st.median(rw),lower_med=st.median(rl),whole=rw,lower=rl,
                                 pub_whole_med=st.median(by[pa][t]["whole"] for t in ts),pub_lower_med=st.median(by[pa][t]["lower"] for t in ts))
        bpw=min(cand,key=lambda a:cand[a]["whole_med"]); bpl=min(cand,key=lambda a:cand[a]["lower_med"])
        res[cell]=dict(family=fam,n=n,r=rr,arms=arms,best_lab=bl,lab_ir=arms[bl]["med_whole"],cand={a:{k:v for k,v in c.items() if k not in("whole","lower")} for a,c in cand.items()},
            best_pub_whole=bpw,best_pub_lower=bpl,ratio_whole=cand[bpw]["whole_med"],ratio_lower=cand[bpl]["lower_med"],
            per_target_whole=cand[bpw]["whole"],per_target_lower=cand[bpl]["lower"],ntargets_best_pub=len(cand[bpw]["targets"]))
        print(cell,"r=%s"%rr,"best lab",bl,"%.3g"%arms[bl]["med_whole"],"| best pub(whole)",bpw,"%.3g"%cand[bpw]["pub_whole_med"],"x%.1f (%d tg)"%(cand[bpw]["whole_med"],len(cand[bpw]["targets"])),
              "| best pub(solver-only)",bpl,"%.3g"%cand[bpl]["pub_lower_med"],"x%.1f"%cand[bpl]["lower_med"])
        for a in sorted(arms): print("    %-18s n=%d/%d whole %s lower %s"%(a,arms[a]["n"],arms[a]["n_all"],"%.3g"%arms[a]["med_whole"] if arms[a]["med_whole"] else "-","%.3g"%arms[a]["med_lower"] if arms[a]["med_lower"] else "-"))
    # fits
    fits={}
    for kind in ("whole","lower"):
        pts=[(c,v["n"],v["ratio_"+kind]) for c,v in res.items()]
        for label,sel in (("all cells",lambda c:True),("excluding n=15",lambda c:c!="K0 n=15"),("K0 only",lambda c:c.startswith("K0"))):
            p=[x for x in pts if sel(x[0])]
            if len(p)<3: continue
            x=np.array([q[1] for q in p],float); y=np.log([q[2] for q in p]); k=len(x)
            sl,ic,rv,pv,se=stats.linregress(x,y); t=stats.t.ppf(0.975,k-2)
            fits[f"{kind}|{label}"]=dict(k=k,slope=sl,half_ci=t*se,p=pv,lo=sl-t*se,hi=sl+t*se,x4=math.exp(4*sl),x4_lo=math.exp(4*(sl-t*se)),x4_hi=math.exp(4*(sl+t*se)),cells=[q[0] for q in p])
            print(f"fit {kind:5s} {label:15s} k={k}: slope {sl:+.3f} +- {t*se:.3f} per bit (95% CI {sl-t*se:+.3f}..{sl+t*se:+.3f}, p={pv:.2f}); x{math.exp(4*sl):.2f} per 4 bits [{math.exp(4*(sl-t*se)):.2f},{math.exp(4*(sl+t*se)):.2f}]")
    json.dump(dict(cells=res,fits=fits),open(sys.argv[1],"w"),indent=1,default=float)
main()
