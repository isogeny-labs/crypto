# T7 post-run analysis (written after data; decides P0b-P8 from the frozen rules in PROTOCOL.md). Not part of the frozen plan.
import json,glob,os
D="/home/user/crypto/research/curve25519_leads_wave2_20261103/t7_t0_frames/data"
sel=json.load(open(D+"/selection.json"))["cells"]
C={}
for f in sorted(glob.glob(D+"/engine/*.json")):
    tag=os.path.basename(f)[:-5]
    try: C[tag]=json.loads(open(f).read().strip().splitlines()[-1])
    except Exception as e: print("BAD",tag,e)
keys=["form","columns","base_points","covered_points","coverage_exact","solving_degree_max","poly_terms_per_specialisation","f4_muladds","muladds_per_target","muladds_per_relation_at_exact_coverage","hits","hits_verified","planted_hits","planted_exact","planted_done","undecided","capped","targets_done","wall_s"]
print("|tag|"+"|".join(keys)+"|")
for t,c in C.items(): print("|"+t+"|"+"|".join(str(c.get(k)) for k in keys)+"|")
cells=[(4,0),(4,3),(5,0),(5,3),(6,4)]
def pr(tag): return C[tag]["muladds_per_relation_at_exact_coverage"] if tag in C else None
out={}
for t,g in cells:
    s=f"_t{t}_g{g}"
    arms=[a for a in ["t0lo0","t0sel","bitslo0","bitsbest","isosel","iso1093"] if a+s in C]
    r={a:pr(a+s) for a in arms}
    x=C["t0sel"+s]
    P3=[]
    for ref in ["t0lo0","bitsbest"]:
        y=C[ref+s]
        P3.append((ref, x["solving_degree_max"]==y["solving_degree_max"], x["poly_terms_per_specialisation"]==y["poly_terms_per_specialisation"], x["muladds_per_target"]/y["muladds_per_target"]))
    P3cov = x["covered_points"]==sel[str(t)]["L_star_covered"]
    iso=[r[a] for a in ["isosel","iso1093"] if a in r]
    elig={a:v for a,v in r.items() if C[a+s]["undecided"]==0 and not C[a+s]["capped"]}
    others=min(v for a,v in elig.items() if a!="t0sel")
    cheapest=min(elig,key=elig.get)
    out[(t,g)]=dict(perrel=r,P3=P3,P3cov=P3cov,P4=r["t0sel"]/r["bitsbest"],P5=r["t0sel"]/min(iso),P6=r["t0sel"]/r["t0lo0"],P7=r["t0sel"]<0.99*others,cheapest=cheapest, ratio_to_min_other=r["t0sel"]/others)
    print(t,g,out[(t,g)])
# P0b
ref={"bitsbest_t4_g3":(8884000,78),"bitslo0_t5_g3":(303691176,42),"bitslo0_t5_g0":(12728434431,42),"bitsbest_t5_g3":(303691611,198),"bitsbest_t5_g0":(12728430774,198),"iso1093_t6_g4":(70384751,528)}
for k,(m,c) in ref.items():
    if k in C: print("P0b",k,C[k]["f4_muladds"],C[k]["covered_points"],(C[k]["f4_muladds"],C[k]["covered_points"])==(m,c))
P8=[(t, c["planted_hits"]==c["planted_exact"]==c["planted_done"], c["hits_verified"]==c["hits"], c["undecided"]==0, c["capped"]==False) for t,c in C.items()]
print("P8 ok", sum(all(p[1:]) for p in P8), "/", len(P8), [p for p in P8 if not all(p[1:])])
print("total f4 muladds", sum(c["f4_muladds"] for c in C.values()), "planted", sum(c.get("planted_muladds",0) for c in C.values()))
json.dump({"cells":{f"t{t}_g{g}":v for (t,g),v in out.items()},"P8":P8,"selection":sel},open(os.path.join(os.path.dirname(os.path.abspath(__file__)),"summary.json"),"w"),indent=1,default=str)
