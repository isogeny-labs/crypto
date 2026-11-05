import json,glob,numpy as np
r=np.random.default_rng(1); out={}
cf=[c for n in (17,19,23) for c in json.load(open(f"/Volumes/SSD990-2/ic-prime-eth-leads-20261031/leads/RESIDUES/legendre/result_{n}.json"))["configs"]]
print("configs",len(cf))
for m in ("LR","LR2","MLP"):
    L=np.array([c["models"][m]["lift"] for c in cf]); a0=np.array([c["models"][m]["auc_base"] for c in cf]); a1=np.array([c["models"][m]["auc_feat"] for c in cf])
    lo=np.array([c["models"][m]["lift_ci"][0] for c in cf]); hi=np.array([c["models"][m]["lift_ci"][1] for c in cf])
    bs=[L[r.integers(0,len(L),len(L))].mean() for _ in range(2000)]
    ex=sum(c["models"][m]["feat_exceeds_q99"] for c in cf)
    print(m,"pooled lift %.4f CI[%.4f,%.4f]"%(L.mean(),*np.percentile(bs,[2.5,97.5])),"meanAUC base %.4f feat %.4f"%(a0.mean(),a1.mean()),"max lift %.4f (CI %.4f..%.4f)"%(L.max(),lo[L.argmax()],hi[L.argmax()]),"q99 exceed %d/%d"%(ex,len(cf)))
    print("  configs with lift CI excluding 1:",int(((lo>1)|(hi<1)).sum()),"; >1.3:",int((L>=1.3).sum()))
for n in (17,19,23):
    print("control",n,json.load(open(f"/Volumes/SSD990-2/ic-prime-eth-leads-20261031/leads/RESIDUES/legendre/result_{n}.json"))["controls"][0]["models"])
