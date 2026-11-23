#!/usr/bin/env python3
"""Serialize the frozen H4 n13 systems for a complete msolve replay."""
from sage.all import *
import ast
import hashlib
import json
from pathlib import Path
import random
import numpy as np

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[3]
def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def main():
    protocol=json.loads((HERE/'protocol.json').read_text())
    assert protocol['status']=='frozen' and sha(HERE/'prepare.py')==protocol['preparer_sha256']
    assert sha(protocol['source']['path'])==protocol['source']['sha256']
    assert sha(HERE/'sage-runtime-info.json')==protocol['sage_runtime_sha256']
    assert json.loads((HERE/'sage-runtime-info.json').read_text())['status']=='verified'
    source=Path(protocol['source']['path'])
    needed={'gfmul','gfmul_s','to_int','popcount','build_instance','eval_eqs_at','enumerate_solutions'}
    tree=ast.parse(source.read_text(),filename=str(source))
    functions=[node for node in tree.body if isinstance(node,ast.FunctionDef) and node.name in needed]
    assert {node.name for node in functions}==needed
    global M64,SEED
    M64=np.uint64;SEED=20261022
    exec(compile(ast.Module(body=functions,type_ignores=[]),str(source),'exec'),globals())
    records=[]
    for arm in protocol['arms']:
        sd=build_instance(17,0,arm)
        assert sd is not None and sd['N']==18 and sd['l']==6 and len(sd['eqs'])==17
        roots=sorted(set(enumerate_solutions(sd)))
        assert len(roots)==protocol['expected_root_counts'][arm]
        if arm=='planted':
            assert sd['planted'] in roots and eval_eqs_at(sd['eqs'],sd['planted'])==0
        equations=[]
        for eq in sd['eqs']:
            monomials=[]
            for raw in eq.tolist():
                mask=int(raw)
                monomials.append('1' if mask==0 else '*'.join(f'v{i}' for i in range(sd['N']) if mask>>i&1))
            equations.append('+'.join(monomials))
        equations.extend(f'v{i}^2+v{i}' for i in range(sd['N']))
        path=HERE/f'{arm}.ms'
        path.write_text(','.join(f'v{i}' for i in range(sd['N']))+'\n2\n'+',\n'.join(equations)+'\n')
        records.append({'arm':arm,'n':17,'instance_index':0,'variables':18,'equations':len(equations),
                        'source_equations':17,'exact_roots':roots,'planted_assignment':sd['planted'],
                        'input_file':path.name,'input_sha256':sha(path),
                        'field_polynomial_bits':sd['poly'],'basis_bits':sd['basis'],
                        'a6_bits':sd['a6'],'residual_x_bits':sd['xR']})
    receipt={'status':'PASS','protocol_sha256':sha(HERE/'protocol.json'),'records':records}
    (HERE/'preparation.json').write_text(json.dumps(receipt,sort_keys=True,indent=2)+'\n')
    print(json.dumps({'status':'PASS','root_counts':{x['arm']:len(x['exact_roots']) for x in records}}))

if __name__=='__main__':main()
