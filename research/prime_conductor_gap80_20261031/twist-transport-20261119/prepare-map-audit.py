#!/usr/bin/env python3
"""Serialize native JSON integers for independent PARI checks, no arithmetic."""
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
case = json.loads((HERE/'case0.gpvec').read_text())


def native(label):
    row = json.loads((HERE/'results'/label/'stdout').read_text())
    assert row['status'] == 'complete_evaluation'
    assert not (HERE/'results'/label/'stderr').read_bytes()
    point = lambda p: [0] if p is None else [int(x) for x in p]
    return [[int(x) for x in row['codomain']],
            [point(r['input']) for r in row['images']],
            [point(r['image']) for r in row['images']],
            int(row['kernel_pairs_covered']), int(row['ell']), int(row['p'])]


data = [case, native('case0-main-b'), native('case0-probe-b')]
(HERE/'main-audit.gpvec').write_text(json.dumps(data)+'\n')
dual = HERE/'results/case0-dual-b/stdout'
if dual.is_file():
    (HERE/'dual-audit.gpvec').write_text(json.dumps([*data, native('case0-dual-b')])+'\n')
