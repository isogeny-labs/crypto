#!/usr/bin/env python3
"""Evaluate actual N51 residues against the degree-seven lambda factors."""
import hashlib
import importlib.util
import json
import re
import sys
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROUNDS=HERE.parent
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()

def load(path,name):
    spec=importlib.util.spec_from_file_location(name,path)
    obj=importlib.util.module_from_spec(spec);spec.loader.exec_module(obj)
    return obj

def eval_bits(bits,base):
    out=0
    for e in range(bits.bit_length()):
        if bits>>e&1:out^=base.gf_pow(2,e)
    return out

def main(dest):
    p=json.loads((HERE/'protocol.json').read_text())
    paths={'n51_excerpt':HERE/'source-data/N51.sing','r102_parser':ROUNDS/'102/audit102.py',
           'r130_result':ROUNDS/'130/result.json','r130_source':ROUNDS/'130/lambda130.py'}
    for label,path in paths.items():assert sha(path)==p['input_sha256'][label],label
    parse=load(paths['r102_parser'],'audit102_pinned')
    fieldmod=load(paths['r130_source'],'lambda130_pinned')
    base=load(ROUNDS/'127/fiber127.py','fiber127_for_r131')
    source=paths['n51_excerpt'].read_text()
    assignments=re.findall(r'(?:^|;)poly N51=(.*?);',source)
    assert len(assignments)==1
    parts=parse.split_sum(assignments[0])
    assert len(parts)==106 and parts[0]=='n'
    terms=[];seen=set()
    for part in parts[1:]:
        match=parse.FRACTION.fullmatch(part)
        assert match,part[:100]
        numerator,denominator,bare,suffix=match.groups()
        den=denominator or bare
        powers={'l':0,'y':0}
        for atom in (suffix[1:].split('*') if suffix else []):
            m=re.fullmatch(r'([ly])(?:\^([1-9][0-9]*))?',atom)
            assert m and powers[m.group(1)]==0
            powers[m.group(1)]=int(m.group(2) or 1)
        key=(powers['l'],powers['y'])
        assert key not in seen and key[0]<16 and key[1]<7
        seen.add(key)
        num=eval_bits(parse.parse_xpoly(numerator),base)
        dv=eval_bits(parse.parse_xpoly(den),base)
        assert dv!=0,('nonunit denominator',key)
        terms.append((key[0],key[1],base.gf_mul(num,base.gf_pow(dv,126))))
    assert len(terms)==105 and len(seen)==105
    old=json.loads(paths['r130_result'].read_text())
    q4=old['quartic_over_F128'];result={'protocol_id':p['id'],'input_sha256':p['input_sha256'],
        'term_count':105,'all_denominators_units':True,'fibers':[]}
    for fiber in old['fibers']:
        F=fieldmod.Field(4 if fiber['field_degree_over_F2']==28 else 1,base,q4)
        y=fiber['y_residue'];h=[0]*16
        for le,ye,c in terms:h[le]^=F.mul(c,F.power(y,ye))
        h=fieldmod.trim(h)
        groups=[]
        for part in fiber['degree_parts']:
            factor=part['factor'];degree=part['lambda_degree'];count=part['count']
            residue=fieldmod.divmod_poly(h,factor,F)[1]
            common=fieldmod.gcd(factor,residue,F)
            assert (len(common)-1)%degree==0
            zero_count=(len(common)-1)//degree
            assert 0<=zero_count<=count
            groups.append({'lambda_degree':degree,'count':count,'zero_residue_count':zero_count,
                           'unit_residue_count':count-zero_count,'residue_remainder':residue,
                           'gcd_with_N51':common,
                           'F2_residue_degree':part['residue_degree_over_F2'],
                           'ramification_index':part['ramification_index']})
        result['fibers'].append({'base_factor':fiber['base_factor'],
                                 'N51_lambda_coefficients':h,'groups':groups})
    result['place_count_check']=sum(g['count'] for f in result['fibers'] for g in f['groups'])
    result['unit_place_count']=sum(g['unit_residue_count'] for f in result['fibers'] for g in f['groups'])
    result['zero_residue_place_count']=sum(g['zero_residue_count'] for f in result['fibers'] for g in f['groups'])
    assert result['place_count_check']==12
    assert result['unit_place_count']+result['zero_residue_place_count']==12
    dest.mkdir(parents=True,exist_ok=True)
    (dest/'result.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
    print('RESIDUE131_ALL_CHECKS_PASS')

if __name__=='__main__':main(Path(sys.argv[1]))
