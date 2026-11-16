#!/usr/bin/env python3
"""Compare source-translated author public outputs with compiled Rust/Cantor."""
from __future__ import annotations

import argparse
import csv
import hashlib
import json
from collections import Counter
from pathlib import Path

from author_eval import F, TRACE, set_field, translate

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
RUST = ROOT / 'src/bin/projective_law_gate.rs'
RUST_SHA = '5976e7f30b3e9c5c49a5fc0f4095404b7322832e4b02f467bc3cb7e0d9dcc8c0'
WEIGHTS = [(1, 1, 1, 1), (2, 3, 3, 5)]
FIELDS = ['p','a_index','b_index','weight_index','a_degree','b_degree',
          'author_status','author_path','author_degree','author_u0','author_u1',
          'author_u2','author_v0','author_v1','author_zero_weights',
          'rust_branch','rust_degree','rust_u0','rust_u1','rust_u2',
          'rust_v0','rust_v1','cantor_degree','cantor_u0','cantor_u1',
          'cantor_u2','cantor_v0','cantor_v1','author_matches_rust',
          'rust_matches_cantor']

def polyrem(poly, divisor, p):
    v = [x % p for x in poly]
    while v and v[-1] == 0: v.pop()
    while len(v) >= len(divisor):
        c = v[-1] * pow(divisor[-1], -1, p) % p
        off = len(v) - len(divisor)
        for i, x in enumerate(divisor): v[off+i] = (v[off+i] - c*x) % p
        while v and v[-1] == 0: v.pop()
    return v

def all_reduced(p):
    out = [(0,(1,0,0),(0,0))]
    for x in range(p):
        fx = (x**5+x+2)%p
        for y in range(p):
            if y*y%p == fx: out.append((1,(-x%p,1,0),(y,0)))
    for q in range(p):
        for r in range(p):
            for s in range(p):
                for t in range(p):
                    u=(r,q,1)
                    f_minus_v2=[(2-t*t)%p,(1-2*t*s)%p,(-s*s)%p,0,0,1]
                    if not polyrem(f_minus_v2,u,p):
                        out.append((2,u,(t,s)))
    return sorted(set(out))

def weighted(d,z,w,p):
    deg,u,v=d
    if deg == 0: return []
    if deg == 1:
        x=-u[0]%p
        return [F(x*z*z),F(v[0]*pow(z,5,p)*w),F(z),F(w)]
    assert deg == 2
    return [F(u[1]*z*z),F(u[0]*pow(z,4,p)),F(v[1]*pow(z,3,p)*w),
            F(v[0]*pow(z,5,p)*w),F(z),F(w)]

def normalize_author(result,p):
    if len(result) != 7: raise ValueError('author arity')
    q,r,s,t,z,w,degree=result
    degree=int(degree)
    if degree == 0: return (0,1,0,0,0,0),0
    if degree not in (1,2): raise ValueError('author degree')
    z,w=int(z)%p,int(w)%p
    zero_weights=(z==0)+(w==0)
    if zero_weights: return None,zero_weights
    iz,iw=pow(z,-1,p),pow(w,-1,p)
    if degree == 1:
        x=int(q)*iz*iz%p
        y=int(s)*pow(iz,5,p)*iw%p
        return (1,-x%p,1,0,y,0),0
    return (2,int(r)*pow(iz,4,p)%p,int(q)*pow(iz,2,p)%p,1,
            int(t)*pow(iz,5,p)*iw%p,int(s)*pow(iz,3,p)*iw%p),0

def input_catalog(path):
    got={7:[],11:[]}
    with path.open(newline='') as h:
        for row in csv.DictReader(h):
            p=int(row['p'])
            assert p in got and int(row['index'])==len(got[p])
            got[p].append((int(row['degree']),
                 tuple(int(row['u'+str(i)]) for i in range(3)),
                 tuple(int(row['v'+str(i)]) for i in range(2))))
    for p in got:
        assert got[p]==all_reduced(p), f'input catalog mismatch F{p}'
    return got

def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--author-source',type=Path,required=True)
    parser.add_argument('--rust-rows',type=Path,required=True)
    parser.add_argument('--inputs',type=Path,required=True)
    parser.add_argument('--outdir',type=Path,required=True)
    args=parser.parse_args()
    assert hashlib.sha256(RUST.read_bytes()).hexdigest()==RUST_SHA
    ns,program=translate(args.author_source)
    inputs=input_catalog(args.inputs)
    args.outdir.mkdir(parents=True,exist_ok=True)
    combined=args.outdir/'combined-rows.csv'
    failures=args.outdir/'failures.csv'
    branch=Counter(); stats=Counter(); by_field=Counter(); seen=set()
    with args.rust_rows.open(newline='') as src, combined.open('w',newline='') as out, failures.open('w',newline='') as bad:
        writer=csv.DictWriter(out,FIELDS,lineterminator='\n'); writer.writeheader()
        bad_writer=csv.DictWriter(bad,FIELDS,lineterminator='\n'); bad_writer.writeheader()
        for item in csv.DictReader(src):
            p=int(item['p']); ia=int(item['a_index']); ib=int(item['b_index']); wi=int(item['weight_index'])
            assert p in inputs and wi in (0,1)
            key=(p,ia,ib,wi); assert key not in seen; seen.add(key)
            a,b=inputs[p][ia],inputs[p][ib]
            set_field(p); za,wa,zb,wb=WEIGHTS[wi]
            ns['TRACE']=[]
            status='ok'; author=None; zero_weights=0
            try:
                result=ns['ADDDIVISOR'](weighted(a,za,wa,p),a[0],weighted(b,zb,wb,p),b[0],F(0),F(0),F(1),F(2))
                author,zero_weights=normalize_author(result,p)
                if author is None: status='invalid_weight'
            except ZeroDivisionError:
                status='undefined_division'
            except (ValueError,TypeError,AssertionError,IndexError) as exc:
                status='invalid_intermediate:'+type(exc).__name__
            path='>'.join(ns['TRACE'])
            rust=(int(item['result_degree']),)+tuple(int(item['result_u'+str(i)]) for i in range(3))+tuple(int(item['result_v'+str(i)]) for i in range(2))
            cantor=(int(item['cantor_degree']),)+tuple(int(item['cantor_u'+str(i)]) for i in range(3))+tuple(int(item['cantor_v'+str(i)]) for i in range(2))
            match=author==rust if author is not None else False
            control=rust==cantor
            row={'p':p,'a_index':ia,'b_index':ib,'weight_index':wi,'a_degree':a[0],'b_degree':b[0],
                 'author_status':status,'author_path':path,'author_zero_weights':zero_weights,
                 'rust_branch':item['rust_branch'],'author_matches_rust':match,'rust_matches_cantor':control}
            for prefix,values in [('author',author),('rust',rust),('cantor',cantor)]:
                for name,value in zip(['degree','u0','u1','u2','v0','v1'],values or [None]*6):
                    row[prefix+'_'+name]=value
            writer.writerow(row)
            if not match or not control: bad_writer.writerow(row)
            branch[(p,wi,item['rust_branch'],path,status)]+=1
            stats[status]+=1; stats['author_match' if match else 'author_mismatch']+=1
            stats['rust_cantor_match' if control else 'rust_cantor_mismatch']+=1
            stats['author_zero_weights']+=zero_weights; by_field[(p,wi)]+=1
    for p in inputs:
        full=len(inputs[p])**2*2
        assert by_field[(p,0)]==by_field[(p,1)], ('unpaired weights',p)
        assert by_field[(p,0)]<=full//2
    with (args.outdir/'branch-matrix.csv').open('w',newline='') as h:
        w=csv.writer(h,lineterminator='\n'); w.writerow(['p','weight_index','rust_branch','author_path','author_status','count'])
        for key,count in sorted(branch.items()): w.writerow([*key,count])
    for name in ('ok','invalid_weight','undefined_division','author_match',
                 'author_mismatch','rust_cantor_match','rust_cantor_mismatch',
                 'author_zero_weights'):
        stats.setdefault(name,0)
    summary={'schema':'projective-law-public-source-parity/v1',
             'author_sha256':hashlib.sha256(args.author_source.read_bytes()).hexdigest(),
             'rust_sha256':RUST_SHA,'translated_python_sha256':hashlib.sha256(program.encode()).hexdigest(),
             'counts':dict(stats),'rows_by_field_weight':{f'{p}:{w}':n for (p,w),n in sorted(by_field.items())},
             'divisors':{str(p):len(v) for p,v in inputs.items()},
             'comparison':'source-translated author ADDDIVISOR vs compiled Rust vs independent Cantor',
             'magma_executed':False}
    (args.outdir/'SUMMARY.json').write_text(json.dumps(summary,sort_keys=True,indent=2)+'\n')
    print(json.dumps(summary,sort_keys=True))

if __name__=='__main__': main()
