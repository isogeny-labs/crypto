#!/usr/bin/env python3
"""Compile exact Rust expressions and check their F5 rows against pinned Magma formulas."""
import csv
import hashlib
import json
import re
import subprocess
import tempfile
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
STUDY = HERE.parents[1]
RUST = STUDY / 'src/bin/projective_law_gate.rs'
RUST_SHA = '5976e7f30b3e9c5c49a5fc0f4095404b7322832e4b02f467bc3cb7e0d9dcc8c0'
AUTHOR_SHA = '7416a7fffaafff371738e7498a6795057372e1c477e0079297eebf2347c2e72a'
AUTHOR_URL = 'https://raw.githubusercontent.com/ozbayelif/jac-on-jac/ec98e115ed359c8b7facf559eb0d9f240703f6f5/jac-on-jac.m'

def sha(p): return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def demand(v, why):
    if not v: raise AssertionError(why)

def extract(rust):
    demand(re.search(r'fn double22_weighted\([^)]*\)[^{]*\{\s*double22_weighted_variant\(a, curve, f, true\)', rust) is not None,
           'production selector is not true')
    func = rust.split('fn double22_weighted_variant(', 1)[1].split('\nfn double22_weighted(', 1)[0]
    header = 'let [q, r, s, t, z, w] = a;'
    demand(func.count(header) == 1, 'unique coordinate destructure')
    pre = func.split(header, 1)[1].split('if b == 0 {', 1)[0]
    main = func.split('    let z2 = f.sq(z);', 1)[1].split('    let bw = f.mul(b, w);', 1)[0]
    pre = pre.strip(); main = 'let z2 = f.sq(z);\n' + main
    for symbol in ('n1','n2','b'):
        demand(f'let {symbol} =' in pre, f'prelude {symbol}')
    for symbol in ('n5','n6','aa','cc'):
        demand(f'let {symbol} =' in main, f'main {symbol}')
    demand('if n6_uses_squared_w { w2 } else { w }' in main, 'N6 selector')
    demand('if ' not in main.replace('if n6_uses_squared_w { w2 } else { w }', ''), 'unexpected conditional')
    return pre, main

def author_check(author):
    lines = [re.sub(r'\s+', '', x) for x in author.splitlines()]
    needles = [
        'N1:=Q1*S1-T1;N2:=R1*S1;N3:=S1;N4:=T1;',
        'B:=2*(N1*N4-N2*N3);',
        'QQ1:=Q1^2;SS1:=S1^2;WW1:=W1^2;',
        'N5:=SS1-(Q1*(tR1mQQ1+tR1)-(Q1*a3ZZZZ1-a2*ZZZZZZ1))*WW1;',
        'N6:=(tR1mQQ1-2*QQ1-a3ZZZZ1)*WW1;',
        'A:=N1*N5-N2*N6;C:=N3*N5-N4*N6;'
    ]
    positions = [next((i for i,x in enumerate(lines) if x == n), -1) for n in needles]
    demand(all(i >= 0 for i in positions) and positions == sorted(positions), 'ordered author Magma expressions')

def harness(pre, main):
    return '''use std::io::{self, Write};
struct Field { p:u64 }
impl Field {
 fn add(&self,a:u64,b:u64)->u64{(a+b)%self.p}
 fn sub(&self,a:u64,b:u64)->u64{(a+self.p-b)%self.p}
 fn mul(&self,a:u64,b:u64)->u64{(a*b)%self.p}
 fn sq(&self,a:u64)->u64{(a*a)%self.p}
}
fn extracted(a:[u64;6], curve:&[u64], f:&Field, n6_uses_squared_w:bool)->[u64;7]{
 let [q,r,s,t,z,w]=a;
''' + pre + '\n' + main + '\n[n1,n2,b,n5,n6,aa,cc]\n}\n' + '''
fn main(){
 let f=Field{p:5}; let mut out=io::BufWriter::new(io::stdout().lock());
 let squared=std::env::args().nth(1).as_deref()==Some("--squared");
 for q in 0..5 {for r in 0..5 {for s in 0..5 {for t in 0..5 {
 for z in 0..5 {for w in 0..5 {for a2 in 0..5 {for a3 in 0..5 {
   let v=extracted([q,r,s,t,z,w], &[0,0,a2,a3], &f, squared);
   writeln!(out,"{q},{r},{s},{t},{z},{w},{a2},{a3},{},{},{},{},{},{},{}",
       v[0],v[1],v[2],v[3],v[4],v[5],v[6]).unwrap();
 }}}}}}}}
}
'''

def expected(q,r,s,t,z,w,a2,a3):
    p=5
    n1=(q*s-t)%p; n2=r*s%p; b=2*(n1*t-n2*s)%p
    z2=z*z%p; z4=z2*z2%p; z6=z4*z2%p
    qq=q*q%p; ss=s*s%p; ww=w*w%p
    tr=2*r%p; trmqq=(tr-qq)%p
    n5=(ss-(q*(trmqq+tr)-(q*a3*z4-a2*z6))*ww)%p
    n6=((trmqq-2*qq-a3*z4)*ww)%p
    a=(n1*n5-n2*n6)%p; c=(s*n5-t*n6)%p
    return (n1,n2,b,n5,n6,a,c)

def run(author_path):
    demand(sha(RUST)==RUST_SHA, 'production Rust SHA')
    demand(sha(author_path)==AUTHOR_SHA, 'pinned author Magma SHA')
    rust=RUST.read_text(); author=Path(author_path).read_text()
    author_check(author)
    pre, main=extract(rust)
    source=harness(pre,main)
    counts=defaultdict(lambda: {'all':0,'b_nonzero':0,'mismatch':0})
    total=0; mismatch=0; digest=hashlib.sha256(); mutation_digest=hashlib.sha256()
    mutation=defaultdict(lambda: {'n6':0,'a':0,'c':0})
    with tempfile.TemporaryDirectory(prefix='n6-bridge-') as tmp:
        src=Path(tmp)/'extracted.rs'; exe=Path(tmp)/'extracted'; src.write_text(source)
        subprocess.run(['rustc','--edition=2021','-O',str(src),'-o',str(exe)],check=True, capture_output=True,text=True)
        for squared in (True,False):
            process=subprocess.Popen([str(exe)]+(['--squared'] if squared else []),stdout=subprocess.PIPE,text=True)
            assert process.stdout is not None
            seen=0
            for line in process.stdout:
                (digest if squared else mutation_digest).update(line.encode())
                values=tuple(int(x) for x in line.strip().split(','))
                demand(len(values)==15,'row arity')
                q,r,s,t,z,w,a2,a3=values[:8]
                obs=values[8:]; exp=expected(q,r,s,t,z,w,a2,a3)
                if squared:
                    row=counts[(z,w)]; row['all']+=1; row['b_nonzero']+=int(obs[2]!=0)
                    bad=obs!=exp; row['mismatch']+=int(bad); mismatch+=int(bad); total+=1
                    if bad and mismatch<=3: print('mismatch',values,exp)
                else:
                    for key,index in [('n6',4),('a',5),('c',6)]:
                        mutation[(z,w)][key]+=int(obs[index]!=exp[index])
                seen+=1
            demand(process.wait()==0 and seen==5**8,'compiled extractor exit and row count')
    demand(total==5**8,'complete F5^8 universe')
    demand(all(x['all']==5**6 for x in counts.values()) and len(counts)==25,'complete coordinate strata')
    with (HERE/'ROWS.csv').open('w',newline='') as f:
        writer=csv.writer(f, lineterminator='\n'); writer.writerow(['z','w','tested','b_nonzero','b_zero','mismatch','mutation_n6_mismatch','mutation_a_mismatch','mutation_c_mismatch'])
        for (z,w),x in sorted(counts.items()):
            m=mutation[(z,w)]
            writer.writerow([z,w,x['all'],x['b_nonzero'],x['all']-x['b_nonzero'],x['mismatch'],m['n6'],m['a'],m['c']])
    receipt={'schema':'n6-compiled-expression-bridge/v1','field':'F_5','universe':'all F5^8 tuples (q,r,s,t,z,w,a2,a3)',
             'rows':total,'mismatches':mismatch,'rust_sha256':RUST_SHA,'author_magma_sha256':AUTHOR_SHA,
             'author_url':AUTHOR_URL,'compiled_extractor_sha256':hashlib.sha256(source.encode()).hexdigest(),
             'stdout_sha256':digest.hexdigest(),'mutation_stdout_sha256':mutation_digest.hexdigest(),
             'mutation_mismatches':{key:sum(x[key] for x in mutation.values()) for key in ('n6','a','c')},
             'csv_sha256':sha(HERE/'ROWS.csv'),
             'in_scope':'exact source-extracted pre-return N1,N2,B,N5,N6,A,C expressions compiled with F5 field primitives',
             'out_of_scope':'production binary control flow, full complete-law output, Rust field implementation semantics, Pic0 class equation'}
    (HERE/'RESULT.json').write_text(json.dumps(receipt,indent=2,sort_keys=True)+'\n')
    demand(mismatch==0,'mismatch rows')
    demand(all(receipt['mutation_mismatches'][key]>0 for key in ('n6','a','c')),'mutation sensitivity')
    print(json.dumps({'rows':total,'mismatches':mismatch,'stdout_sha256':receipt['stdout_sha256']}))

if __name__=='__main__':
    import argparse
    p=argparse.ArgumentParser(); p.add_argument('--author-source',required=True)
    run(p.parse_args().author_source)
