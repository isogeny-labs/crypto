#!/usr/bin/env python3
"""Add source-layout-checked known-witness orbit choices to an N83 XCNF control."""
import argparse
import json
import sys
from pathlib import Path

from verify import DOMAINS_SHA256, N, mul, sha256


def domain_vars(representatives):
    bits = (N - 1).bit_length()
    return 2 * representatives - 1 + N + bits + 2 * N * bits


def choice(reps, target):
    hits = []
    ordered = sorted(map(int, reps))
    if len(set(ordered)) != len(ordered):
        raise ValueError('duplicate representative')
    for index, value in enumerate(ordered):
        for phase in range(N):
            if value == target:
                hits.append((index, phase))
            value = mul(value, value)
    if len(hits) != 1:
        raise ValueError(f'expected one orbit/phase choice, found {len(hits)}')
    return hits[0]


def main(parent, export, domains, output, receipt_path, mode='both'):
    if mode not in ('both', 'representative', 'phase'):
        raise ValueError('choice mode differs')
    if sha256(domains) != DOMAINS_SHA256:
        raise ValueError('retained domains differ')
    data = json.loads(Path(domains).read_text())
    base = json.loads(Path(export).read_text())
    if base.get('status') != 'EXPORTED_XCNF' or base.get('identity_mask') != 0:
        raise ValueError('parent export differs')
    xs = list(map(int, data['ordered_xs']))
    domains_per_summand = [data['small_reps']] * 3 + [data['residual_reps']] * 2
    sizes = [domain_vars(len(reps)) for reps in domains_per_summand]
    # Exact all-affine m=5 chained-S3 circuit; order_first_pair adds N-1
    # variables after all domain blocks and therefore shifts no choice wire.
    circuit_vars = base['variables'] - sum(sizes) - (N - 1)
    if circuit_vars != 84743:
        raise ValueError('chained-S3 source layout differs')
    clauses = []
    details = []
    offset = circuit_vars
    for summand, (reps, x, size) in enumerate(zip(domains_per_summand, xs, sizes)):
        index, phase = choice(reps, x)
        selector = offset + 1 + index
        phase_first = offset + 1 + 2 * len(reps) - 1 + N
        if mode in ('both', 'representative'):
            clauses.append(f'{selector} 0\n')
        for bit in range((N - 1).bit_length()):
            variable = phase_first + bit
            if mode in ('both', 'phase'):
                clauses.append(f'{variable if (phase >> bit) & 1 else -variable} 0\n')
        details.append({'summand': summand, 'representative_index': index,
                        'phase': phase, 'selector_variable': selector,
                        'phase_first_variable': phase_first})
        offset += size
    if offset + N - 1 != base['variables']:
        raise ValueError('domain sizes do not fill solver variable space')
    parent_bytes = Path(parent).read_bytes()
    header, newline, body = parent_bytes.partition(b'\n')
    expected = f"p cnf {base['variables']} {base['clauses'] + base['xors']}".encode()
    if header != expected or not newline:
        raise ValueError('parent XCNF header differs')
    new_header = f"p cnf {base['variables']} {base['clauses'] + base['xors'] + len(clauses)}\n".encode()
    output_bytes = new_header + body + ''.join(clauses).encode()
    with Path(output).open('xb') as stream:
        stream.write(output_bytes)
    result = dict(base)
    result.update({
        'scope': ('known_answer_domain_choice_control_only' if mode == 'both'
                  else f'known_answer_{mode}_choice_control_only'),
        'parent_xcnf_sha256': sha256(parent),
        'added_choice_unit_clauses': len(clauses),
        'chosen_orbit_phases': details,
        'clauses': base['clauses'] + len(clauses),
        'xcnf_bytes': len(output_bytes),
        'xcnf_sha256': sha256(output),
    })
    result.pop('xcnf_blake3', None)
    with Path(receipt_path).open('x') as stream:
        json.dump(result, stream, indent=2, sort_keys=True)
        stream.write('\n')
    print(json.dumps({'status': 'PINNED_ORBIT_CHOICE_CONTROL',
                      'added_clauses': len(clauses),
                      'xcnf_sha256': result['xcnf_sha256']}, sort_keys=True))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('parent')
    parser.add_argument('export')
    parser.add_argument('domains')
    parser.add_argument('output')
    parser.add_argument('receipt')
    parser.add_argument('--mode', choices=('both', 'representative', 'phase'),
                        default='both')
    args = parser.parse_args()
    main(args.parent, args.export, args.domains, args.output, args.receipt,
         args.mode)
