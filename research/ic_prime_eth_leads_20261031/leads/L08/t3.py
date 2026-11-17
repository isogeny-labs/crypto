from l08lib import *
import json, random, sys
inst = challenge()
rows=[]
cfgs = {"proto":(None,None,False), "free27":(27,27,False), "free27_fold":(27,27,True), "free81k66":(81,66,False), "free81k66_fold":(81,66,True)}
for m in (2,3,4,6):
    st0 = parameters(inst, m)
    prime=int(st0["hash_prime"]); tau=st0["threshold"]
    rng=random.Random(1)
    seeds=[{"prime":prime,"alpha":rng.randrange(1,prime),"beta":rng.randrange(prime),"threshold":tau} for _ in range(m)]
    for name,(J,k,fold) in cfgs.items():
        e = FoldEncoding(inst.curve, J, k, fold)
        pts = e.relation(inst.Q, seeds, inst.order, inst.cofactor)
        iface = tuple(v for p in pts for w in p.words for v in w)
        s = e.circuit.statistics(iface)
        row={"m":m,"cfg":name,"edges":s["boolean_network_edges"],"vars":s["variables"],"maxdeg":s["maximum_degree"],"nmul":len(s["multiplication_encodings"]),"tau":tau}
        rows.append(row); print(json.dumps(row), flush=True)
json.dump(rows, open("t3_rows.json","w"))
