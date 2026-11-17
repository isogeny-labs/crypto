from intfree import *
widths=list(map(int,sys.argv[1:])) or [131,133]
best={}
for d in widths:
    base=gadget_edges(d); res=[]
    for k in range(1,d+1):
        ok0,n,L=valid(d,64,k)
        lg=math.ceil(math.log2(n)) if n>1 else 0
        Jmin=2*k+lg+1; step=max(1,L//2)
        J=((Jmin+step-1)//step)*step
        ok,_,_=valid(d,J,k); assert ok,(d,J,k)
        res.append((gadget_edges(d,(J,k)),J,k,n,L))
    res.sort(); best[d]=res[0]
    print(json.dumps({"d":d,"default_edges":base,"best":res[0],"top5":res[:5],"ratio":base/res[0][0]}),flush=True)
json.dump({str(d):v for d,v in best.items()},open("best.json","w"))
