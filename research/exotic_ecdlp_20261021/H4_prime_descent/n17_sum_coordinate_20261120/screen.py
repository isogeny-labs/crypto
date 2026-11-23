#!/usr/bin/env python3
"""Exact Boolean-ring XOR-coordinate screen on frozen H4 n17 inputs."""
import hashlib
import json
import re
from pathlib import Path

HERE=Path(__file__).resolve().parent
def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def parse(poly):
    terms=set()
    for term in poly.split('+'):
        mask=0
        for factor in term.strip().split('*'):
            if factor=='1':continue
            match=re.fullmatch(r'v(\d+)(?:\^(\d+))?',factor.strip())
            assert match,factor
            bit=int(match.group(1));assert 0<=bit<18
            mask|=1<<bit
        terms.symmetric_difference_update((mask,))
    return terms

def read(path):
    lines=path.read_text().strip().splitlines()
    assert lines[:2]==[','.join(f'v{i}' for i in range(18)),'2']
    polys='\n'.join(lines[2:]).split(',\n')
    assert len(polys)==35
    result=[parse(poly) for poly in polys]
    assert all(result[i]==set() for i in range(17,35))
    return result

def transform(polys,kind):
    output=[]
    for poly in polys:
        result=set()
        for mono in poly:
            expansion={0}
            for bit in range(18):
                if not mono>>bit&1:continue
                if bit<12:choices=(1<<bit,)
                elif kind=='pair_sum':choices=(1<<(bit-12),1<<bit)
                else:choices=(1<<(bit-12),1<<(bit-6),1<<bit)
                next_terms=set()
                for prior in expansion:
                    for choice in choices:next_terms.symmetric_difference_update((prior|choice,))
                expansion=next_terms
            result.symmetric_difference_update(expansion)
        output.append(result)
    return output

def serialize(poly):
    def term(mask):return '*'.join(f'v{i}' for i in range(18) if mask>>i&1) or '1'
    return '+'.join(term(m) for m in sorted(poly,key=lambda m:(m.bit_count(),m)))

def assignment_map(value,kind):
    mask=(1<<6)-1
    u=value&mask;v=(value>>6)&mask;w=(value>>12)&mask
    s=w^u^(v if kind=='total_sum' else 0)
    return u|(v<<6)|(s<<12)

def vanishes(polys,assignment):
    return all(sum((term&assignment)==term for term in poly)%2==0 for poly in polys)

def stats(polys):
    return {'source_terms':sum(map(len,polys[:17])),
            'max_source_terms':max(map(len,polys[:17])),
            'max_boolean_degree':max(term.bit_count() for poly in polys[:17] for term in poly)}

def main():
    protocol=json.loads((HERE/'protocol.json').read_text())
    assert protocol['status']=='frozen' and sha(HERE/'screen.py')==protocol['screener_sha256']
    prepared=json.loads(Path(protocol['preparation']['path']).read_text())
    assert sha(protocol['preparation']['path'])==protocol['preparation']['sha256']
    records=[]
    for arm in protocol['arms']:
        source=Path(protocol['inputs'][arm]['path'])
        assert sha(source)==protocol['inputs'][arm]['sha256']
        original=read(source);original_stats=stats(original)
        expected=next(x['exact_roots'] for x in prepared['records'] if x['arm']==arm)
        assert all(vanishes(original,x) for x in expected)
        cases=[]
        for kind in protocol['transforms']:
            candidate=transform(original,kind)
            assert transform(candidate,kind)==original
            assert all(vanishes(candidate,assignment_map(x,kind)) for x in expected)
            for seed in range(256):
                x=(seed*0x9e37d)&((1<<18)-1)
                assert vanishes(original,x)==vanishes(candidate,assignment_map(x,kind))
            path=HERE/f'{arm}_{kind}.ms'
            path.write_text(','.join(f'v{i}' for i in range(18))+'\n2\n'+
                            ',\n'.join(serialize(poly) for poly in candidate[:17])+'\n'+
                            ',\n'.join(f'v{i}^2+v{i}' for i in range(18))+'\n')
            candidate_stats=stats(candidate)
            cases.append({'kind':kind,'input_file':path.name,'input_sha256':sha(path),**candidate_stats,
                          'term_ratio':candidate_stats['source_terms']/original_stats['source_terms']})
        records.append({'arm':arm,'original':original_stats,'cases':cases})
    eligible=[]
    for kind in protocol['transforms']:
        if all(next(c for c in r['cases'] if c['kind']==kind)['term_ratio']<=0.9 and
               next(c for c in r['cases'] if c['kind']==kind)['max_boolean_degree']<=r['original']['max_boolean_degree']
               for r in records):
            total=sum(next(c for c in r['cases'] if c['kind']==kind)['source_terms'] for r in records)
            eligible.append((total,protocol['transforms'].index(kind),kind))
    chosen=min(eligible)[2] if eligible else None
    result={'status':'PASS','protocol_sha256':sha(HERE/'protocol.json'),'records':records,
            'selected_transform':chosen,'selection_rule':'at least 10 percent term reduction in both arms and no maximum-degree increase'}
    (HERE/'screen-result.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
    print(json.dumps({'status':'PASS','selected_transform':chosen,'records':records}))

if __name__=='__main__':main()
