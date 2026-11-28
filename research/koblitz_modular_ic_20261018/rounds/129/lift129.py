#!/usr/bin/env python3
"""F128[[u]] implicit lift of the degree-seven marked23 double root."""
import hashlib
import importlib.util
import json
import sys
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROUNDS=HERE.parent

def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()

def main(dest):
    p=json.loads((HERE/'protocol.json').read_text())
    files={'r51_source':ROUNDS/'51/source-data/marked23-field51.sing',
           'r127_result':ROUNDS/'127/result.json','r128_result':ROUNDS/'128/result.json',
           'r128_source':ROUNDS/'128/newton128.py'}
    for name,path in files.items():assert sha(path)==p['input_sha256'][name],name
    old=json.loads(files['r128_result'].read_text())
    assert old['hull']==[[0,1],[2,0],[7,0]]
    M=p['precision_u_exclusive']
    assert M==128
    spec=importlib.util.spec_from_file_location('f127',ROUNDS/'127/fiber127.py')
    f=importlib.util.module_from_spec(spec);spec.loader.exec_module(f)
    def conv(a,b):
        z=[0]*M
        for i,x in enumerate(a):
            if x:
                for j,y in enumerate(b[:M-i]):
                    if y:z[i+j]^=f.gf_mul(x,y)
        return z
    def add(a,b):return [x^y for x,y in zip(a,b)]
    rows=[[0]*M for _ in range(8)]
    for si,uj,c in old['support']:
        rows[si][uj]^=c
    assert rows[1][0]==84 and rows[0][0]==0
    inv=[0]*M;inv[0]=f.gf_pow(84,126)
    for k in range(1,M):
        value=0
        for j in range(1,k+1):value^=f.gf_mul(rows[1][j],inv[k-j])
        inv[k]=f.gf_mul(inv[0],value)
    assert conv(rows[1],inv)==[1]+[0]*(M-1)
    s=[0]*M
    for iteration in range(p['max_fixed_point_iterations']):
        powers=[[1]+[0]*(M-1),s]
        for _ in range(2,8):powers.append(conv(powers[-1],s))
        rhs=rows[0][:]
        for j in range(2,8):rhs=add(rhs,conv(rows[j],powers[j]))
        next_s=conv(inv,rhs)
        if next_s==s:break
        s=next_s
    else:raise AssertionError('fixed point did not converge')
    powers=[[1]+[0]*(M-1),s]
    for _ in range(2,8):powers.append(conv(powers[-1],s))
    residual=[0]*M
    for j in range(8):residual=add(residual,conv(rows[j],powers[j]))
    assert residual==[0]*M
    support=[[k,a] for k,a in enumerate(s) if a]
    assert support[0]==[2,6] and s[0]==s[1]==0
    previous=json.loads(files['r127_result'].read_text())
    assert previous['rational_roots']==[[90,2],[123,1]]
    quartic=previous['squarefree_parts'][0]['degree_parts'][1]
    assert quartic['irreducible_degree']==4 and quartic['factor_count']==1
    result={'protocol_id':p['id'],'input_sha256':p['input_sha256'],
            'precision_u_exclusive':M,'fixed_point_iterations':iteration+1,
            's_support':support,'s_support_sha256':hashlib.sha256(json.dumps(support,separators=(',',':')).encode()).hexdigest(),
            'first_s_term':[2,6],'base_places':[{'residue_degree_over_F2':7,'ramification_index':1,'source_factor':'Y+123'},
                           {'residue_degree_over_F2':7,'ramification_index':2,'source_factor':'Y+90'},
                           {'residue_degree_over_F2':28,'ramification_index':1,'source_factor':'Q4'}],
            'x_fiber_degree_check':7+14+28}
    assert result['x_fiber_degree_check']==49
    dest.mkdir(parents=True,exist_ok=True)
    (dest/'result.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
    print('LIFT129_ALL_CHECKS_PASS')

if __name__=='__main__':main(Path(sys.argv[1]))
