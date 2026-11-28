"""Deterministic count model of the candidate levers on per-record trial counts (pilot: separate-seed sessions)."""
import json,gzip,sys,collections,os
def recs(path):
    op = gzip.open if path.endswith('.gz') else open
    for l in op(path,'rt'):
        x=json.loads(l)
        if not x["warmup"] and x["outcome"]["status"]=="verified": yield x
def table_counts(n,k):
    h=(n-1)//2; adds=sum(1+h+n*(k-1-o) for o in range(k)); sibs=sum(h+n*(k-1-o) for o in range(k))
    canon=sum(1+(n-1)+2*n*(k-1-o) for o in range(k)); return adds,sibs,canon
def walk_counts(K,m,r):
    """adds (incl. precompute, chain, pairs, lift) and sib seconds and extra canon/lookup to test k=0..K-1 with the hit at K-1."""
    if m==0: return K-1,0,0
    if K==1: return 0,0,0
    adds=(m+1)  # precompute: 2G..mG (m-1), (m+1)G (1), (2m+1)G (1)
    adds+=1     # C0 = Q + (m+1)G
    sibs=0; extra=0
    kc=1+m; last=K-1
    while True:
        lo=kc-m
        if lo>last: break
        # block: test kc-m..kc-1 (form pair i before testing kc-i), kc, kc+1..kc+m
        if lo>1: adds+=1  # chain step to this centre (C0 already counted)
        for i in range(m,0,-1):
            adds+=1; sibs+=1
            if kc-i==last:
                adds+=1; extra+=2  # lift + re-probe (canon+lookup)
                return adds,sibs,extra
        if kc>=last: return adds,sibs,extra
        if kc+m>=last: return adds,sibs,extra
        kc+=2*m+1
    return adds,sibs,extra
def cost(rec):
    """priced(w) = group ops + w * native ops, from the record itself."""
    NONOP={"factor_base_vec_bytes_floor_uncharged"}
    CAL=["lookups","row_ops","frobenius_maps","canonicalisations","target_guard_probes","as_solves"]
    unp=[u for u in rec["cost"]["unpriced"] if u not in NONOP]
    marks={k[8:] for p in rec["phases"] for k in (p.get("work_native") or {}) if k.startswith("counted:")}
    g=0; nat=0
    for p in rec["phases"]:
        g+=(p.get("adds") or 0)+(p.get("doubles") or 0)
        nv=p.get("native") or {}; wn=p.get("work_native") or {}
        nat+=sum(nv.get(c,0) for c in CAL)
        for u in unp:
            b=u[:-10] if u.endswith("_uncharged") else u
            nat+=nv.get(u,nv.get(b,0))
        nat+=sum(wn.get(t,0) for t in ("table_insert_attempts","sibling_second_points") if t in marks)
    return g,nat
CFG={23:dict(k=4,d=5),29:dict(k=1,d=0),37:dict(k=8,d=5)}
def model(rec,n,k,d,m=0,ladder=False,minkey_reuse=False):
    K=rec["counters"]["targets_tried"]; r=rec["workload"]["curve"]["r"]
    D=int(rec["outcome"]["recovered"])
    la=sum((p.get("native") or {}).get("row_ops",0) for p in rec["phases"])
    g=0; nat=0
    # factor base
    g+=d*(k-1); nat+=n*k+n*k+k+(0 if (k==1 or minkey_reuse) else k*(n-1))
    At,St,Ct=table_counts(n,k); g+=At; nat+=2*Ct+St
    wa,ws,wx=walk_counts(K,m,r); g+=wa+1; nat+=2*K+ws+wx
    nat+=la
    bl=D.bit_length(); pc=bin(D).count("1")
    if D==0: vg=0
    else:
        dbl=bl-1
        if ladder: dbl=max(0,bl-1-d*(k-1)) if k>1 else dbl
        vg=dbl+pc-1
    g+=vg
    return g,nat
if __name__=="__main__":
    S="/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/lsc-sessions"
    bad=0;tot=0
    for n in (23,29,37):
        for rr in (1,2,3):
            for x in recs(f"{S}/n{n}-r{rr}/records.jsonl.gz"):
                if x["arm"]!="ls": continue
                tot+=1
                a=cost(x); b=model(x,n,**CFG[n])
                if a!=b:
                    bad+=1
                    if bad<5: print(n,a,b,x["counters"]["targets_tried"],x["outcome"]["recovered"])
    print("model check: mismatches",bad,"of",tot)

# --- koblitz_k1_setup_w1_20261118: the set-up levers lazy_neg (base) and orbit_ids (table), on top of the above ---
INC = {23: dict(k=4, d=5, m=8, ladder=True, minkey_reuse=True),   # sp-c4-d5 + lean + sib + walk_sib=8 + ladder + key reuse
       29: dict(k=1, d=0),                                          # gen-c1 + lean + sib
       37: dict(k=8, d=5, m=8, ladder=True, minkey_reuse=True)}    # sp-c8-d5 + lean + sib + walk_sib=8 + ladder + key reuse
def model_w1(rec, n, k, d, m=0, ladder=False, minkey_reuse=False, lazy_neg=False, orbit_ids=False, hit_negations=0):
    """(group ops, priced native ops) of the incumbent's record moved to the candidate.
    lazy_neg: the base forms no negatives (-k*n native); a hit that names a stored negative forms it (+hit_negations,
    0..2, read from the candidate's own record).  orbit_ids: per orbit one addition, two sibling second points,
    three canonicalisations and three table inserts fewer (the doubling P+P, the plus half of rotation 2, the
    sign-mu half of rotation 3).  The walk, the relation's targets, the linear algebra and the check are unchanged."""
    g, nat = model(rec, n, k, d, m=m, ladder=ladder, minkey_reuse=minkey_reuse)
    if lazy_neg:
        nat -= k * n
        nat += hit_negations
    if orbit_ids:
        g -= k
        nat -= 2 * k + 3 * k + 3 * k
    return g, nat
