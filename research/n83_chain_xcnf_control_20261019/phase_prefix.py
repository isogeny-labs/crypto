#!/usr/bin/env python3
"""Pin the known-witness Frobenius phases of the first k N83 summands."""
import argparse
import hashlib
import json
from pathlib import Path

PARENT_SHA256 = 'c2cc36eee7a72dbc583a217a0d42d02c74d5d3fde241536941b6614fc565b81e'


def sha256(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def main(parent, export, full_choice, prefix, output, receipt_path):
    if not 1 <= prefix <= 4:
        raise ValueError('prefix must be 1..4; zero and five already have receipts')
    if sha256(parent) != PARENT_SHA256:
        raise ValueError('parent XCNF bytes differ')
    base = json.loads(Path(export).read_text())
    full = json.loads(Path(full_choice).read_text())
    if (base['status'] != 'EXPORTED_XCNF' or
            full['parent_xcnf_sha256'] != PARENT_SHA256 or
            len(full['chosen_orbit_phases']) != 5 or
            [row['summand'] for row in full['chosen_orbit_phases']] != list(range(5))):
        raise ValueError('choice receipt differs')
    units = []
    for row in full['chosen_orbit_phases'][:prefix]:
        for bit in range(7):
            variable = row['phase_first_variable'] + bit
            units.append(variable if (row['phase'] >> bit) & 1 else -variable)
    parent_bytes = Path(parent).read_bytes()
    header, newline, body = parent_bytes.partition(b'\n')
    if header != f"p cnf {base['variables']} {base['clauses'] + base['xors']}".encode() or not newline:
        raise ValueError('parent header differs')
    result_bytes = (f"p cnf {base['variables']} {base['clauses'] + base['xors'] + len(units)}\n".encode()
                    + body + ''.join(f'{lit} 0\n' for lit in units).encode())
    with Path(output).open('xb') as stream:
        stream.write(result_bytes)
    receipt = dict(base)
    receipt.update({
        'scope': 'known_answer_phase_prefix_control_only',
        'parent_xcnf_sha256': PARENT_SHA256,
        'phase_prefix': prefix,
        'added_phase_unit_clauses': len(units),
        'clauses': base['clauses'] + len(units),
        'xcnf_bytes': len(result_bytes),
        'xcnf_sha256': sha256(output),
    })
    receipt.pop('xcnf_blake3', None)
    with Path(receipt_path).open('x') as stream:
        stream.write(json.dumps(receipt, sort_keys=True, indent=2) + '\n')
    print(json.dumps({'prefix': prefix, 'status': 'EXPORTED_PHASE_PREFIX_CONTROL',
                      'xcnf_sha256': receipt['xcnf_sha256']}, sort_keys=True))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('parent', type=Path)
    parser.add_argument('export', type=Path)
    parser.add_argument('full_choice', type=Path)
    parser.add_argument('prefix', type=int)
    parser.add_argument('output', type=Path)
    parser.add_argument('receipt', type=Path)
    args = parser.parse_args()
    main(args.parent, args.export, args.full_choice, args.prefix,
         args.output, args.receipt)
