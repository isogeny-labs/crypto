"""Reload only the exact published reference factor; no arbitrary factor trust."""
import hashlib
import json
from pathlib import Path
import importlib.util
import sys


def load():
    root = Path(__file__).resolve().parent.parent/'feature-consistent-repair'
    lock = json.loads((root/'check-lock.json').read_text())
    source = root/'rectangular.py'
    if hashlib.sha256(source.read_bytes()).hexdigest() != lock['rectangular.py']:
        raise ValueError('factor implementation differs from checked source')
    data = (root/'certificate/factor.json').read_bytes()
    summary = json.loads((root/'certificate/summary.json').read_text())
    if hashlib.sha256(data).hexdigest() != summary['factor_sha256']:
        raise ValueError('hydrate exact saved factor')
    spec = importlib.util.spec_from_file_location('decoder_binding_rectangular', source)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    z = json.loads(data)
    if z['schema'] != 'rectangular-factor/v1' or (z['prime'], z['rows'], z['columns']) != (479,477,576):
        raise ValueError('fixed factor scope')
    factor = module.Factor(z['prime'], z['rows'], z['columns'], tuple(z['pivots']),
                           tuple(tuple(row) for row in z['upper']),
                           tuple((swap, inv, tuple(factors)) for swap, inv, factors in z['steps']))
    return factor, tuple(z['representatives']), tuple(tuple(S) for S in z['labels'])
