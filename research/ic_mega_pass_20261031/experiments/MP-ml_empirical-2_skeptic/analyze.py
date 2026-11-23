"""AUC of each degree-d Macaulay statistic for SAT vs UNSAT, with a label-permutation null."""
import json, sys, random, itertools
def auc(pos, neg):
    # Mann-Whitney AUC with ties = 0.5
    s=0.0
    for p in pos:
        for q in neg:
            s += 1.0 if p>q else (0.5 if p==q else 0.0)
    return s/(len(pos)*len(neg))
def main():
    rng=random.Random(1)
    out={}
    for path in sys.argv[1:]:
        res=json.load(open(path))
        cell=path.split('/')[-1].replace('.json','')
        sat=[r for r in res if r['sat']]; uns=[r for r in res if not r['sat']]
        cellout={'n_sat':len(sat),'n_unsat':len(uns),'stats':{}}
        degs=[k for k in res[0] if k.startswith('d') and k[1:].isdigit()]
        for d in degs:
            for st in ('rank','HF','fall','le2','le1','const'):
                pos=[r[d][st] for r in sat]; neg=[r[d][st] for r in uns]
                a=auc(pos,neg)
                # two-sided: max(a,1-a)
                allv=pos+neg; n1=len(pos)
                null=[]
                for _ in range(200):
                    rng.shuffle(allv); b=auc(allv[:n1],allv[n1:]); null.append(max(b,1-b))
                null.sort()
                pval=sum(1 for b in null if b>=max(a,1-a))/len(null)
                cellout['stats'][f'{d}.{st}']=dict(auc=round(a,4), null95=round(null[int(0.95*len(null))],4), p_perm=round(pval,3),
                    sat_values=sorted(set(pos)), unsat_values=sorted(set(neg)),
                    sat_mean=round(sum(pos)/len(pos),3), unsat_mean=round(sum(neg)/len(neg),3))
        out[cell]=cellout
    print(json.dumps(out, indent=1))
if __name__=='__main__': main()
