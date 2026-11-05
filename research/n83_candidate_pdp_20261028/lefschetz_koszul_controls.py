"""Finite free-chain syzygy controls. No n83 production entry point."""
from pathlib import Path
from itertools import combinations
import json,sys
import lefschetz_controls as c

def multiply(a,b,p):
    out={}
    for x,v in a.items():
        for y,w in b.items():c.put(out,tuple(sorted(x+y)),v*w,p)
    return out

def image(vector,fs,p):
    out={}
    for j,q in enumerate(vector):
        for mon,v in multiply(q,fs[j],p).items():c.put(out,mon,v,p)
    return out

def check(m,q,p):
    n=2*m-2;fs=c.quartics([[i] for i in range(m)],list(range(m,n)),p,{})
    matrix=c.multiplication_map(fs,n,q,{},p)
    domain=[tuple(x) for x in matrix['multiplier_domain']]
    lookup={(j,mon):j*len(domain)+i for j in range(m-1) for i,mon in enumerate(domain)}
    columns=[];all_zero=True;bad_detected=False
    for i,j in combinations(range(m-1),2):
        for h in c.monomials(n,q-4,{}):
            vector=[{} for _ in fs]
            vector[i]=multiply(fs[j],{h:1},p)
            vector[j]={mon:-v%p for mon,v in multiply(fs[i],{h:1},p).items()}
            residual=image(vector,fs,p);all_zero &= not residual
            if not columns:
                bad=[dict(x) for x in vector];mon=next(iter(bad[j]));bad[j][mon]=-bad[j][mon]%p
                bad_detected=bool(image(bad,fs,p))
            entries={lookup[(k,mon)]:v for k,row in enumerate(vector) for mon,v in row.items()}
            columns.append(entries)
    rank=c.sparse_rank(columns,p)
    return {'m':m,'q':q,'p':p,'coefficient_map':matrix,'koszul_columns':[[[i,str(v)] for i,v in sorted(col.items())] for col in columns],
            'koszul_rank':rank,'kernel_dimension':matrix['columns']-matrix['rank'],
            'every_column_maps_to_zero':all_zero,'changed_sign_detected':bad_detected}

def run(out,replay):
    def save(name,value):
        value=json.loads(json.dumps(value));p=out/name
        if replay:assert json.loads(p.read_text())==value
        else:
            with p.open('x') as f:json.dump(value,f,indent=2);f.write('\n')
    save('identity.json',{'development_only':True,'source_sha256':c.sha(__file__),'core_sha256':c.sha(c.__file__)})
    rows=[]
    for m in [3,4]:
        for q in [4,5,6]:
            for p in [11,101]:
                d=check(m,q,p);save(f'm{m}-q{q}-p{p}.json',d)
                assert d['every_column_maps_to_zero'] and d['changed_sign_detected']
                assert d['koszul_rank']==d['kernel_dimension']
                row={k:d[k] for k in ['m','q','p','koszul_rank','kernel_dimension']};rows.append(row)
                print(json.dumps(row),flush=True)
    save('summary.json',{'development_only':True,'rows':rows,'all_passed':True,'new_n83_bound':None})

if __name__=='__main__':
    mode=sys.argv[1];out=Path(sys.argv[2]);assert mode in ['run','replay']
    if mode=='run':assert not out.exists();out.mkdir()
    else:assert out.is_dir()
    run(out,mode=='replay')
