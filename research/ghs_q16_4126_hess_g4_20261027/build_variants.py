#!/usr/bin/env python3
"""Freeze direct-Hess control scripts for the other three exact sources."""
from hashlib import sha256
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPLAY = HERE.parent/'ghs_q16_residual_quartic_search_20261027/verification.json'


def replace_once(source,old,new):
    if source.count(old) != 1:
        raise ValueError(f'expected one template occurrence of {old!r}')
    return source.replace(old,new)


def main():
    replay = json.loads(REPLAY.read_text())
    rows=[]
    for rec in replay['checked'][:3]:
        order=rec['source_order']
        r=rec['subgroup_order']
        b=rec['source_a6']
        gx,gy=rec['G']
        qx,qy=rec['Q']
        assert order in (4006,4054,4078) and order==2*r
        outputs={}
        for name in ('attack','rho'):
            source=(HERE/f'{name}.magma').read_text()
            replacements={
                'fromint(63)]':f'fromint({b})]',
                'G := E![fromint(62),fromint(1499),1];':
                    f'G := E![fromint({gx}),fromint({gy}),1];',
                'Q := E![fromint(2633),fromint(1798),1];':
                    f'Q := E![fromint({qx}),fromint({qy}),1];',
                'r := 2063;':f'r := {r};',
            }
            if name=='attack':
                replacements.update({
                    'assert #E eq 4126 and Order(G) eq 2063;':
                        f'assert #E eq {order} and Order(G) eq {r};',
                    'assert #J eq 57764;':
                        'assert #J mod r eq 0;',
                    'assert cofactor eq 28;':
                        'assert GCD(cofactor,r) eq 1;',
                })
            else:
                replacements['assert #E eq 4126 and Order(G) eq r and Order(Q) eq r;']=\
                    f'assert #E eq {order} and Order(G) eq r and Order(Q) eq r;'
            for old,new in replacements.items():
                source=replace_once(source,old,new)
            path=HERE/f'{name}_{order}.magma'
            path.write_text(source)
            outputs[name]={'file':path.name,'sha256':sha256(path.read_bytes()).hexdigest()}
        rows.append({'source_order':order,'subgroup_order':r,'source_a2':32,
                     'source_a6':b,'G':rec['G'],'Q':rec['Q'],
                     'scripts':outputs})
    manifest={'schema':'ghs-q16-residual-hess-controls/v1',
              'source_replay_sha256':sha256(REPLAY.read_bytes()).hexdigest(),
              'template_sha256':{name:sha256((HERE/f'{name}.magma').read_bytes()).hexdigest()
                                 for name in ('attack','rho')},
              'variants':rows}
    (HERE/'variants.json').write_text(json.dumps(manifest,indent=2)+'\n')
    print([z['source_order'] for z in rows])

if __name__=='__main__':
    main()
