#!/usr/bin/env python3
"""Counter model of the default index-calculus path of `ic solve-prime-file` (pair-sum table, joint relation
point, fb <= 64), checked against the measured counters of the band panel, then projected to the real
E-222 and M-221.  Stdlib only.  The projection is a MODEL EXTRAPOLATION, not a measurement.

Model (all counters are per solve; F = factor-base size, R = F + 2 relations, n = subgroup order, h = cofactor,
projected = the H1 cofactor projection, so every table point lies in <G>):
  table inserts          2 F^2                      (exact)
  table point negations  (3 F^2 - F) / 2            (exact)
  table group additions  F^2                        (exact)
  usable table points    m = 2 F^2 (projected or h = 1) else 2 F^2 / h ;  distinct in <G>: n (1 - exp(-m / n))
  success per trial      p = distinct / n ;  trials = R / p ;  probes = trials ; scalars drawn = 2 trials
  per trial              doublings E[bits(max(a,b))] - 1 ,  additions 0.75 (E[bits] - 1) + 1  (popcount(a|b))
  projection of F points by [h] (H1): F (popcount(h) - 1) additions , F (bits(h) - 1) doublings
  verification           bits(n) - 1 doublings , (bits(n) - 1)/2 additions  (x uniform)
  factor-base square roots  2 F + 6 (fitted to the panel, see --validate)
  elimination operations    4.7 F^2 (measured at F = 32 .. 256 on cofactor-1 instances of 16-17 bits)
Rho (matched negation walk): GAE ~ S sqrt(r) with S in [0.886, 1.8] (floor sqrt(pi/4) = 0.886 for A = 2, up to the
2x spread seen between the floor and the measured mean at 10..16 bits), unpriced ~ 0.43 GAE (canonicalisations, panel).
usage: native_band_model.py --validate BAND_DIR | --project | --json BAND_DIR"""
import glob,json,math,statistics,sys

def e_bits_max(n):
    b=n.bit_length();return sum(1-(min(2**(k-1),n)/n)**2 for k in range(1,b+1))
def ladder(k): return (bin(k).count("1")-1,k.bit_length()-1)
def model(n,h,F,projected,sqrt_c=(2.0,6.0),elim_c=4.7):
    R=F+2;b=n.bit_length();eb=e_bits_max(n)
    usable=2*F*F if (projected or h==1) else 2*F*F/h
    distinct=n*(-math.expm1(-usable/n)) if n<1e15 else usable
    p=distinct/n;trials=R/p
    dbl=trials*(eb-1);add=trials*(0.75*(eb-1)+1)
    dbl+=b-1;add+=(b-1)/2                                    # verification
    add+=F*F                                                 # table build
    if projected and h>1:
        a,d=ladder(h);add+=F*a;dbl+=F*d
    c={"group_additions":add,"group_doublings":dbl,
       "table_probes_uncharged":trials,"random_scalars_drawn_uncharged":2*trials,
       "table_insert_attempts_uncharged":2*F*F,"point_negations_uncharged":(3*F*F-F)/2,
       "sqrt_mod_p_calls_uncharged":sqrt_c[0]*F+sqrt_c[1],"elimination_ops_uncharged":elim_c*F*F}
    return c,trials
def cost(c,w):
    return c["group_additions"]+c["group_doublings"]+w*sum(v for k,v in c.items() if k.endswith("_uncharged"))
def rho_gae(r,S): return S*math.sqrt(r)
def rho_cost(r,S,w): return rho_gae(r,S)*(1+0.43*w)

def load(band_dir):
    rows=[]
    for cfg in ("default","h1"):
        for f in sorted(glob.glob(f"{band_dir}/{cfg}/*.json")):
            o=json.load(open(f));rows.append((cfg,o))
    return rows
def validate(band_dir):
    out={};per=[]
    for cfg,o in load(band_dir):
        n=o["input"]["n"];h=o["input"]["cofactor"];F=o["ic"]["stages"]["factor_base_size"]
        c,tr=model(n,h,F,cfg=="h1")
        ob=o["ic"]["stages"]["counters"];st=o["ic"]["stages"]
        obs={"group_additions":ob["group_additions"],"group_doublings":ob["group_doublings"],"trials_total":st["trials_total"],
             "table_probes_uncharged":ob["table_probes_uncharged"],"random_scalars_drawn_uncharged":ob["random_scalars_drawn_uncharged"],
             "table_insert_attempts_uncharged":ob["table_insert_attempts_uncharged"],"point_negations_uncharged":ob["point_negations_uncharged"],
             "sqrt_mod_p_calls_uncharged":ob["sqrt_mod_p_calls_uncharged"],
             "elimination_ops_uncharged":sum(v for k,v in ob.items() if k.startswith("elimination_"))}
        pred=dict(c);pred["trials_total"]=tr
        per.append((cfg,o["_meta"]["family"],o["_meta"]["r_bits"],{k:obs[k]/pred[k] for k in obs}))
    keys=list(per[0][3])
    for cfg in ("default","h1"):
        sel=[p for p in per if p[0]==cfg]
        out[cfg]={"n":len(sel),**{k:{"median_obs_over_pred":statistics.median(p[3][k] for p in sel),
              "q1":sorted(p[3][k] for p in sel)[len(sel)//4],"q3":sorted(p[3][k] for p in sel)[3*len(sel)//4]} for k in keys}}
    return out,per
def project():
    res=[]
    curves=[("E-222",int("ffffffffffffffffffffffffffff70cbc95e932f802f31423598cbf",16),4),
            ("M-221",int("040000000000000000000000000015A08ED730E8A2F77F005042605B",16),8)]
    for name,n,h in curves:
        for proj in (False,True):
            for lgF in (5,10,15,20,25,30):
                F=2**lgF;c,tr=model(n,h,F,proj)
                row={"curve":name,"n_bits":n.bit_length(),"h":h,"H1":proj,"log2_F":lgF,"log2_table_entries":math.log2(2*F*F),
                     "log2_trials":math.log2(tr),"log2_ic":{str(w):math.log2(cost(c,w)) for w in (0,0.1,1)}}
                r=n
                row["log2_rho"]={str(S):{str(w):math.log2(rho_cost(r,S,w)) for w in (0,0.1,1)} for S in (0.886,1.8)}
                row["log2_ic_over_rho_best_case_for_ic"]={str(w):row["log2_ic"][str(w)]-row["log2_rho"]["1.8"][str(w)] for w in (0,0.1,1)}
                row["log2_ic_over_rho_worst_case_for_ic"]={str(w):row["log2_ic"][str(w)]-row["log2_rho"]["0.886"][str(w)] for w in (0,0.1,1)}
                res.append(row)
    return res
if __name__=="__main__":
    if sys.argv[1]=="--validate":
        v,_=validate(sys.argv[2])
        for cfg,d in v.items():
            print(f"== {cfg} ({d['n']} instances): median observed / predicted [q1, q3]")
            for k,x in d.items():
                if k!="n": print(f"  {k:36} {x['median_obs_over_pred']:.3f} [{x['q1']:.3f}, {x['q3']:.3f}]")
    elif sys.argv[1]=="--project":
        for r in project():
            print(f"{r['curve']} H1={str(r['H1']):5} F=2^{r['log2_F']:<2} table 2^{r['log2_table_entries']:.0f} entries  trials 2^{r['log2_trials']:.1f}  IC 2^{r['log2_ic']['0']:.1f}/{r['log2_ic']['1']:.1f}  IC/rho 2^{r['log2_ic_over_rho_best_case_for_ic']['0']:.1f}..2^{r['log2_ic_over_rho_worst_case_for_ic']['0']:.1f} (w=0)")
    elif sys.argv[1]=="--json":
        v,_=validate(sys.argv[2]);print(json.dumps({"validation":v,"projection":project()},indent=1))
