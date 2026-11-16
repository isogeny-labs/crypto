#!/usr/bin/env python3
"""Bind the added three-branch orbit checks to their source and prior receipt."""
from pathlib import Path
import hashlib
import json

here = Path(__file__).resolve().parent
old_path = here / 'POSITIONS.json'
new_path = here / 'POSITIONS_THREE.json'
source = here / 'positions_three.py'
old = json.loads(old_path.read_text())
new = json.loads(new_path.read_text())

assert new['schema'] == 'ghs.cyclic11-branch-orbits/v2'
assert new['source_sha256'] == hashlib.sha256(source.read_bytes()).hexdigest()
for key in ('PGL2_F3_order', 'orbits', 'quadratic_pair_orbit_size'):
    assert new[key] == old[key]
assert new['three_branch_three_rational_orbit_size'] == 4
assert new['three_branch_rational_plus_quadratic_orbit_size'] == 12

print(json.dumps({
    'schema': 'ghs.cyclic11-three-branch-orbit-audit/v1',
    'source_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    'old_receipt_sha256': hashlib.sha256(old_path.read_bytes()).hexdigest(),
    'new_receipt_sha256': hashlib.sha256(new_path.read_bytes()).hexdigest(),
    'three_rational_configurations': 4,
    'rational_plus_quadratic_configurations': 12,
}, sort_keys=True))
