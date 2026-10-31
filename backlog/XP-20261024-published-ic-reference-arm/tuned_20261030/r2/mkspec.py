"""R2 spec generator.  usage: mkspec.py <name> <curve a> <n> <lab l list|-> <pub list like l4s3,l5s5|->  [targets=8]
Same law as E1: target_seed 202610111901, public targets, m=3, rounds 1, alternate, L0, timeout 3600."""
import json,sys
name,a,n,lab,pub=sys.argv[1:6]; tg=int(sys.argv[6]) if len(sys.argv)>6 else 8
arms=[]
def mk(nm,l,orc): return {"name":nm,"role":"candidate","method":{"id":"ic.pipeline","params":{"factor_base":f"binary-standard-subspace:dimension={l}","oracle":orc,"max_trials":"200000"}}}
if lab!="-":
    for l in lab.split(","): arms.append(mk(f"inherited-f4-l{l}",l,"pdp3-binary:m=3,engine=inherited-f4,degree=3,node_budget=8192"))
if pub!="-":
    for p in pub.split(","):
        form="e" if p[0]=="e" else "x"; p=p.lstrip("e"); l,s=p[1:].split("s")
        arms.append(mk(f"msolve-{form}-l{l}-s{s}",l,f"pdp3-binary:m=3,engine=msolve-s4,form={form},spec={s}"))
spec={"schema":"ecbench.spec/v1","label":f"R2 {name}","description":"R2 matched sweep cell (published arm swept per size vs lab arm swept).",
 "workloads":{"curves":[{"kind":"koblitz","a":int(a),"n":int(n)}],"targets_per_curve":tg,"target_seed":202610111901,"target_kind":"public"},
 "arms":arms,"measurement":{"rounds":1,"warmup":0,"order":"alternate","seed":202610111902,"isolation_required":"L0","timeout_seconds":3600}}
json.dump(spec,open(f"r2/specs/SPEC-R2_{name}.json","w"),indent=1)
