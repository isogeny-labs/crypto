#!/usr/bin/env python3
"""Replay the sealed primary-N83 K1182/866 ordinary-query stage receipts."""
import gzip
import hashlib
import json
import subprocess
import tempfile
from pathlib import Path

from k1182_verify import verify as verify_model
from natural_audit import scalar_mul
from verify import on_curve, point_add, sha256

HERE = Path(__file__).resolve().parent
EVIDENCE = HERE / 'evidence/k1182'
PLAN = (HERE.parent / 'koblitz_n83_factor_base_sweep_20261008/verification/'
        'k600-gray-hash-paired-protocol-20261014/plan.json')
DOMAINS = (HERE.parent / 'koblitz_n83_factor_base_sweep_20261008/verification/'
           'known-witness-ordered-sat-20261014/domains.json')
SOURCE_COMMIT = 'b1482bac70f3fc261c5caa76d4d00a6d02248865'


def require(condition, message):
    if not condition:
        raise ValueError(message)


def main():
    protocol = json.loads((HERE / 'k1182_protocol.json').read_text())
    source = subprocess.run(
        ['git', 'show', f'{SOURCE_COMMIT}:examples/n83_chain_xcnf_control.rs'],
        cwd=HERE.parent.parent, check=True, capture_output=True,
    ).stdout
    require(protocol['status'] == 'FROZEN_BEFORE_SOLVER_CALLS' and
            protocol['candidate_id'] is None and
            protocol['selected_best_total_runtime'] is None and
            sha256(PLAN) == protocol['plan_sha256'] and
            sha256(DOMAINS) == protocol['domain_input_sha256'] and
            hashlib.sha256(source).hexdigest() == protocol['exporter_source_sha256'] and
            sha256(HERE / 'k1182_verify.py') == protocol['verifier_source_sha256'],
            'sealed protocol, inputs, or source bytes differ')
    plan = json.loads(PLAN.read_text())
    generator = (int('477f77103dfad59850800', 16),
                 int('2fa5e737d542c4e4fd5c3', 16))
    public = tuple(map(int, plan['public_target']))
    require(on_curve(generator) and on_curve(public), 'source point fails curve equation')
    query = point_add(scalar_mul(generator, int(plan['first_trial_a'])),
                      scalar_mul(public, int(plan['first_trial_b'])))
    require(query == tuple(map(int, protocol['query_point'])),
            'ordinary query point differs')
    export = json.loads((EVIDENCE / 'k1182_export.json').read_text())
    require(export['status'] == 'EXPORTED_NATURAL_XCNF' and
            export['scope'] == protocol['scope'] and
            export['plan_sha256'] == protocol['plan_sha256'] and
            export['domain_input_sha256'] == protocol['domain_input_sha256'] and
            export['query_point'] == protocol['query_point'] and
            export['query_coefficient_a'] == protocol['query_coefficient_a'] and
            export['query_coefficient_b'] == protocol['query_coefficient_b'] and
            export['small_orbits'] == protocol['small_signed_orbits'] and
            export['residual_orbits'] == protocol['residual_signed_orbits'] and
            export['identity_mask'] == protocol['identity_mask'] and
            export['order_first_pair'] == protocol['order_first_pair'] and
            export['variables'] == protocol['variables'] and
            export['clauses'] == protocol['cnf_clauses'] and
            export['xors'] == protocol['xor_rows'] and
            export['xcnf_bytes'] == protocol['xcnf_bytes'],
            'export receipt differs')
    with tempfile.TemporaryDirectory(prefix='n83-k1182-audit-') as temporary:
        xcnf = Path(temporary) / 'first_query.xcnf'
        with gzip.open(EVIDENCE / 'first_query.xcnf.gz', 'rb') as source_file, \
                xcnf.open('wb') as output:
            for chunk in iter(lambda: source_file.read(1024 * 1024), b''):
                output.write(chunk)
        require(sha256(xcnf) == protocol['xcnf_sha256'] and
                xcnf.stat().st_size == protocol['xcnf_bytes'],
                'archived XCNF differs')
        with xcnf.open() as stream:
            require(stream.readline().strip() ==
                    f"p cnf {protocol['variables']} "
                    f"{protocol['cnf_clauses'] + protocol['xor_rows']}",
                    'XCNF header differs')
            clauses = xors = 0
            for line in stream:
                require(line.endswith(' 0\n') or line.endswith(' 0 \n'),
                        'malformed XCNF row')
                xors += line.startswith('x ')
                clauses += not line.startswith('x ')
        require((clauses, xors) == (protocol['cnf_clauses'], protocol['xor_rows']),
                'XCNF row counts differ')
        rows = []
        for suffix, cap in [('5k', 5000), ('100k', 100000), ('1m', 1000000)]:
            folder = EVIDENCE / f'k1182_run_{suffix}'
            receipt = json.loads((folder / 'run.json').read_text())
            stdout = folder / 'solver.stdout'
            stderr = folder / 'solver.stderr'
            require(receipt['xcnf_sha256'] == protocol['xcnf_sha256'] and
                    receipt['xcnf_bytes'] == protocol['xcnf_bytes'] and
                    receipt['cap_conflicts'] == cap and
                    receipt['cap_seconds'] == (180 if cap == 1000000 else
                                               120 if cap == 100000 else 60) and
                    receipt['status'] == 'UNKNOWN_solver_cap' and
                    receipt['solver_status'] == 'INDETERMINATE' and
                    receipt['exit_code'] == 15 and
                    receipt['stdout_sha256'] == sha256(stdout) and
                    receipt['stderr_sha256'] == sha256(stderr) and
                    receipt['candidate_id'] is None and
                    receipt['selected_best_total_runtime'] is None and
                    stdout.read_text().splitlines() == ['s INDETERMINATE'],
                    f'{suffix}: bounded raw receipt differs')
            require(verify_model(xcnf, stdout, EVIDENCE / 'k1182_export.json') == 0,
                    f'{suffix}: independent model verifier failed')
            rows.append({'conflicts': cap, 'wall_ms': receipt['elapsed_wall_ms'],
                         'status': receipt['status']})
    print(json.dumps({'status': 'PASS_FROZEN_K1182_BOUNDED_RECEIPTS',
                      'query_point': list(map(str, query)), 'calls': rows,
                      'candidate_id': None, 'selected_best_total_runtime': None},
                     sort_keys=True))


if __name__ == '__main__':
    main()
