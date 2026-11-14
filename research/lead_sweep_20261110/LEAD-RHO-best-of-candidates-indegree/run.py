import sys, json, random, math
M=(1<<64)-1
def mix(x,s):
    x=(x^s)&M; x=(x^(x>>33))*0xff51afd7ed558ccd&M; x=(x^(x>>33))*0xc4ceb9fe1a85ec53&M
    return x^(x>>33)
def steps(arm,N,rng,tab,s1,s2):
    x=rng.randrange(N); seen={x}; n=0
    while True:
        if arm=="rand":
            y=mix(x,s1)%N
        else:
            a=tab[mix(x,s1)&31]
            y=(x+a)%N
            if arm=="best":
                z=(x-a)%N
                if mix(z,s2)<mix(y,s2): y=z
        n+=1
        if y in seen: return n
        seen.add(y); x=y
def arm_stats(arm,N,T,seed):
    rng=random.Random(seed); out=[]
    for t in range(T):
        tab=[rng.randrange(1,N) for _ in range(32)]
        out.append(steps(arm,N,rng,tab,rng.getrandbits(64),rng.getrandbits(64)))
    return out
def boot(a,b,B,seed):
    r=random.Random(seed); n=len(a); m=len(b); rs=[]
    for _ in range(B):
        sa=sum(a[r.randrange(n)] for _ in range(n))/n
        sb=sum(b[r.randrange(m)] for _ in range(m))/m
        rs.append(sa/sb)
    rs.sort(); return rs[int(.025*B)],rs[int(.975*B)]
def main():
    smoke="--smoke" in sys.argv
    sizes=[2**8] if smoke else [2**14,2**18]
    T=40 if smoke else 1500; B=100 if smoke else 400
    res={}
    for N0 in sizes:
        N=N0+1 if N0%2==0 else N0  # odd modulus
        plain=arm_stats("plain",N,T,101); aa=arm_stats("plain",N,T,202)
        best=arm_stats("best",N,T,303); rnd=arm_stats("rand",N,T,404)
        mean=lambda v:sum(v)/len(v)
        res[str(N)]={"mean_plain":mean(plain),"mean_best":mean(best),"mean_aa":mean(aa),"mean_randfn":mean(rnd),
          "ratio_plain_over_best":mean(plain)/mean(best),"ci_ratio":boot(plain,best,B,1),
          "aa_ratio":mean(plain)/mean(aa),"aa_ci":boot(plain,aa,B,2),
          "plain_over_randfn":mean(plain)/mean(rnd),"ref_sqrt_pi_N_over_2":math.sqrt(math.pi*N/2)}
    big=res[str(max(int(k) for k in res))]
    lo,hi=big["ci_ratio"]; alo,ahi=big["aa_ci"]
    if not(alo<=1<=ahi): v="inconclusive"
    elif lo>1.05 and hi<1.30: v="consistent_with_prediction"
    elif hi<1.03: v="falsified"
    else: v="inconclusive"
    if smoke: v="inconclusive"
    print(json.dumps({"lead":"LEAD-RHO-best-of-candidates-indegree","smoke":smoke,"seeds":[101,202,303,404],
      "params":{"sizes":sizes,"trials":T,"boot":B,"jump_table":32},"results":res,"verdict":v}))
main()
