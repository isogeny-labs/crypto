#!/usr/bin/env python3
"""Map N31 repaired columns through the exact trace-zero leaf slice."""
import hashlib
import itertools
import json
import struct
import subprocess
import sys
import tempfile
from pathlib import Path

HERE=Path(__file__).resolve().parent
HEADER=struct.Struct('<8sIIII')
U16=struct.Struct('<H')

def sha(path): return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def gf_mul(a,b,modulus):
    output=0
    while b:
        if b&1:output^=a
        b>>=1;a<<=1
        if a&(1<<31):a^=modulus
    return output

def field_trace(a,modulus):
    total=0
    for _ in range(31):
        total^=a
        a=gf_mul(a,a,modulus)
    assert total in (0,1)
    return total

def gf_power(a,n,modulus):
    output=1
    while n:
        if n&1:output=gf_mul(output,a,modulus)
        a=gf_mul(a,a,modulus)
        n>>=1
    return output

def from_coordinates(mask,normal):
    value=0
    while mask:
        bit=mask&-mask
        value^=normal[bit.bit_length()-1]
        mask^=bit
    return value

def basis_solver(columns):
    pivots={}
    for i,original in enumerate(columns):
        value,mask=original,1<<i
        while value:
            lead=value.bit_length()-1
            if lead not in pivots:
                pivots[lead]=(value,mask)
                break
            prior,prior_mask=pivots[lead]
            value^=prior;mask^=prior_mask
    def solve(value):
        mask=0
        while value:
            lead=value.bit_length()-1
            prior,prior_mask=pivots[lead]
            value^=prior;mask^=prior_mask
        return mask
    return solve,len(pivots)

def transform_equations(raw,solve):
    coefficient={}
    for bit,polynomial in enumerate(raw):
        for term in polynomial: coefficient[term]=coefficient.get(term,0)^(1<<bit)
    output=[set() for _ in range(31)]
    for term,value in coefficient.items():
        mask=solve(value)
        while mask:
            bit=mask&-mask
            output[bit.bit_length()-1].add(term)
            mask^=bit
    return output

def substitution_map(trace_bits):
    assert len(trace_bits)==16 and trace_bits[0]==1
    variables=[]
    for leaf in range(2):
        base=15*leaf
        for j in range(16):
            if j==0:
                variables.append({1<<(base+k-1) for k in range(1,16) if trace_bits[k]})
            else: variables.append({1<<(base+j-1)})
    return variables

def substitute_monomial(monomial,variables):
    answer={0}
    while monomial:
        bit=monomial&-monomial
        replacement=variables[bit.bit_length()-1]
        newer=set()
        for left in answer:
            for right in replacement:
                combined=left|right
                if combined in newer:newer.remove(combined)
                else:newer.add(combined)
        answer=newer
        monomial^=bit
    return answer

def substitute_polynomial(poly,variables):
    answer=set()
    for monomial in poly:
        for term in substitute_monomial(monomial,variables):
            if term in answer:answer.remove(term)
            else:answer.add(term)
    return answer

def write_rows(path,equations,multipliers,monomials,selected=None):
    positions={term:i for i,term in enumerate(monomials if selected is None else selected)}
    nnz=0
    with Path(path).open('wb') as out:
        out.write(HEADER.pack(b'G2M4R1\0\0',13980,len(positions),30,4))
        for eq in equations:
            for multiplier in multipliers:
                row=set()
                for term in eq:
                    value=term|multiplier
                    if value in row:row.remove(value)
                    else:row.add(value)
                indices=sorted(positions[term] for term in row if term in positions)
                out.write(U16.pack(len(indices)))
                if indices:out.write(struct.pack('<'+'H'*len(indices),*indices))
                nnz+=len(indices)
    return nnz

def main():
    protocol=json.loads((HERE/'protocol.json').read_text())
    assert protocol['status']=='frozen' and sha(HERE/'bridge.py')==protocol['producer_sha256']
    for name,item in protocol['sources'].items():assert sha(item['path'])==item['sha256'],name
    sys.path.insert(0,str(Path(protocol['sources']['boolean_core']['path']).parent))
    from boolean_core import ANFField,monomials
    algebra=json.loads(Path(protocol['sources']['algebra']['path']).read_text())
    coupling=json.loads(Path(protocol['sources']['coupling']['path']).read_text())
    original=json.loads(Path(protocol['sources']['original']['path']).read_text())
    pivot=json.loads(Path(protocol['sources']['pivot']['path']).read_text())
    panel=json.loads(Path(protocol['sources']['panel']['path']).read_text())
    repairs=json.loads(Path(protocol['sources']['repairs']['path']).read_text())
    normal=coupling['normal_basis_bits']
    assert all(field_trace(bit,algebra['field_modulus_bits'])==1 for bit in normal)
    solve,rank=basis_solver(normal)
    assert rank==31
    codes=original['code_basis_masks']
    old_basis=[from_coordinates(code,normal) for code in codes]
    trace=[code.bit_count()%2 for code in codes]
    assert trace[0]==1
    new_basis=[old_basis[j]^(old_basis[0] if trace[j] else 0) for j in range(1,16)]
    variables=substitution_map(trace)
    masks=monomials(30,4)
    assert len(masks)==31931
    multipliers=monomials(30,2)
    assert len(multipliers)==466
    pivot_positions=pivot['measurement']['pivots']
    full_old_monomials=monomials(32,4)
    assert len(pivot_positions)==18352 and len(full_old_monomials)==41449
    base_columns=panel['information_set_columns']
    result={'status':'PASS','protocol_sha256':sha(HERE/'protocol.json'),
            'trace_bits':trace,'cases':[]}
    with tempfile.TemporaryDirectory(prefix='g2-trace-bridge-') as directory:
        temp=Path(directory)
        for repair in repairs['cases']:
            label,q=repair['label'],repair['q_bits']
            assert field_trace(q,algebra['field_modulus_bits'])==0
            inverse=gf_power(q,(1<<31)-2,algebra['field_modulus_bits'])
            assert gf_mul(q,inverse,algebra['field_modulus_bits'])==1
            assert field_trace(inverse,algebra['field_modulus_bits'])==0
            anf=ANFField(31,algebra['field_modulus_bits'])
            old=transform_equations(anf.s3(anf.linear(range(16),old_basis),
                                           anf.linear(range(16,32),old_basis),anf.constant(q)),solve)
            new=transform_equations(anf.s3(anf.linear(range(15),new_basis),
                                           anf.linear(range(15,30),new_basis),anf.constant(q)),solve)
            assert [substitute_polynomial(eq,variables) for eq in old]==new
            inverse_squared=gf_mul(inverse,inverse,algebra['field_modulus_bits'])
            trace_mask=sum(1<<j for j,b in enumerate(normal)
                           if field_trace(gf_mul(b,inverse_squared,algebra['field_modulus_bits']),
                                          algebra['field_modulus_bits']))
            assert trace_mask
            relation=set()
            for j,eq in enumerate(new):
                if trace_mask>>j&1:relation.symmetric_difference_update(eq)
            assert not relation
            dropped=(trace_mask&-trace_mask).bit_length()-1
            equations=[eq for j,eq in enumerate(new) if j!=dropped]
            assert len(equations)==30
            repaired=sorted(base_columns+repair['selected_columns'])
            support=set()
            for index in repaired:
                support.update(substitute_monomial(full_old_monomials[pivot_positions[index]],variables))
            selected=sorted(support,key=lambda term:(term.bit_count(),term))
            assert set(selected).issubset(set(masks))
            full=temp/'full.bin';narrow=temp/'narrow.bin'
            full_nnz=write_rows(full,equations,multipliers,masks)
            narrow_nnz=write_rows(narrow,equations,multipliers,masks,selected)
            runs=[]
            for kind,path in (('full',full),('mapped_support',narrow)):
                ranked=json.loads(subprocess.run([protocol['sources']['ranker']['path'],str(path)],
                                                 capture_output=True,text=True,timeout=120,check=True).stdout)
                assert ranked['rows']==13980 and ranked['nnz']==(full_nnz if kind=='full' else narrow_nnz)
                assert ranked['maxrss_bytes_process']<protocol['rss_gate_bytes']
                runs.append({'kind':kind,'columns':ranked['columns'],'rank':ranked['rank'],
                             'nnz':ranked['nnz'],'stream_sha256':sha(path),
                             'wall_ns_exploratory':ranked['wall_ns_exploratory'],
                             'peak_rss_bytes_process':ranked['maxrss_bytes_process']})
            result['cases'].append({'label':label,'q_bits':q,'symbolic_substitution':'exact',
                                    'output_trace_mask':trace_mask,'dropped_equation':dropped,
                                    'mapped_support_columns':len(selected),'runs':runs})
            full.unlink();narrow.unlink()
    print(json.dumps(result,sort_keys=True,indent=2))

if __name__=='__main__':main()
