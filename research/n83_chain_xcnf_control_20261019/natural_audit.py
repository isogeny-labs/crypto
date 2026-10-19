#!/usr/bin/env python3
"""Independently replay frozen primary-N83 K600 ordinary-query stage evidence."""
import contextlib
import gzip
import io
import json
import hashlib
import subprocess
import tempfile
from pathlib import Path

from natural_domains import main as extract_domains
from verify import on_curve, point_add, sha256

HERE = Path(__file__).resolve().parent
EVIDENCE = HERE / 'evidence/natural'
PLAN = (HERE.parent / 'koblitz_n83_factor_base_sweep_20261008/verification/'
        'k600-gray-hash-paired-protocol-20261014/plan.json')
FROZEN_EXPORTER_COMMIT = '1e7b35ddaafebd277083a24b741bd26bd7d7316e'


def require(condition, message):
    if not condition:
        raise ValueError(message)


def scalar_mul(point, scalar):
    result = None
    while scalar:
        if scalar & 1:
            result = point_add(result, point)
        point = point_add(point, point)
        scalar >>= 1
    return result


def main():
    protocol = json.loads((HERE / 'natural_protocol.json').read_text())
    frozen_source = subprocess.run(
        ['git', 'show', f'{FROZEN_EXPORTER_COMMIT}:examples/n83_chain_xcnf_control.rs'],
        cwd=HERE.parent.parent, check=True, capture_output=True,
    ).stdout
    require(protocol['status'] == 'FROZEN_BEFORE_1M_CALLS' and
            protocol['selected_best_total_runtime'] is None and
            sha256(PLAN) == protocol['plan_sha256'] and
            sha256(HERE / 'natural_domains.py') == protocol['domain_extractor_source_sha256'] and
            hashlib.sha256(frozen_source).hexdigest() == protocol['exporter_source_sha256'],
            'sealed protocol or source bytes differ')
    plan = json.loads(PLAN.read_text())
    generator = (int('477f77103dfad59850800', 16),
                 int('2fa5e737d542c4e4fd5c3', 16))
    public = tuple(map(int, plan['public_target']))
    require(on_curve(generator) and on_curve(public), 'source point fails curve equation')
    a, b = int(plan['first_trial_a']), int(plan['first_trial_b'])
    query = point_add(scalar_mul(generator, a), scalar_mul(public, b))
    require(query == tuple(map(int, protocol['query_point'])),
            'independent ordinary query point differs')
    with tempfile.TemporaryDirectory(prefix='n83-natural-audit-') as temporary:
        temporary = Path(temporary)
        with contextlib.redirect_stdout(io.StringIO()):
            extract_domains(EVIDENCE / 'archives', temporary / 'domains')
        for arm in protocol['arms']:
            role = arm['role']
            domain = EVIDENCE / f'{role}.json'
            require(domain.read_bytes() == (temporary / 'domains' / f'{role}.json').read_bytes() and
                    sha256(domain) == arm['domain_input_sha256'],
                    f'{role}: archived domain extraction differs')
            archive = EVIDENCE / f'{role}.xcnf.gz'
            xcnf = temporary / f'{role}.xcnf'
            with gzip.open(archive, 'rb') as source, xcnf.open('wb') as output:
                for chunk in iter(lambda: source.read(1024 * 1024), b''):
                    output.write(chunk)
            require(sha256(xcnf) == arm['xcnf_sha256'],
                    f'{role}: XCNF digest differs')
            export = json.loads((EVIDENCE / f'{role}_export.json').read_text())
            require(export['status'] == 'EXPORTED_NATURAL_XCNF' and
                    export['scope'] == 'frozen_ordinary_first_query_stage_only' and
                    export['plan_sha256'] == protocol['plan_sha256'] and
                    export['query_point'] == protocol['query_point'] and
                    export['query_coefficient_a'] == protocol['query_coefficient_a'] and
                    export['query_coefficient_b'] == protocol['query_coefficient_b'] and
                    export['variables'] == arm['variables'] and
                    export['clauses'] == arm['clauses'] and
                    export['xors'] == arm['xor_rows'] and
                    export['xcnf_bytes'] == xcnf.stat().st_size,
                    f'{role}: export receipt differs')
            with xcnf.open() as stream:
                require(stream.readline().strip() ==
                        f"p cnf {arm['variables']} {arm['clauses'] + arm['xor_rows']}",
                        f'{role}: XCNF header differs')
                clauses = xors = 0
                for line in stream:
                    require(line.endswith(' 0\n') or line.endswith(' 0 \n'),
                            f'{role}: malformed XCNF row')
                    xors += line.startswith('x ')
                    clauses += not line.startswith('x ')
            require((clauses, xors) == (arm['clauses'], arm['xor_rows']),
                    f'{role}: XCNF row counts differ')
            for suffix, cap in [('5k', 5000), ('100k', 100000), ('1m', 1000000)]:
                folder = EVIDENCE / f'cms_{role.split("_")[0]}_{suffix}'
                receipt = json.loads((folder / 'run.json').read_text())
                stdout = folder / 'solver.stdout'
                stderr = folder / 'solver.stderr'
                require(receipt['xcnf_sha256'] == arm['xcnf_sha256'] and
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
                        f'{role}/{suffix}: bounded raw receipt differs')
    print(json.dumps({'status': 'PASS_FROZEN_ORDINARY_QUERY_AND_BOUNDED_RECEIPTS',
                      'query_point': list(map(str, query)),
                      'arms': 2, 'capped_calls': 6,
                      'selected_best_total_runtime': None}, sort_keys=True))


if __name__ == '__main__':
    main()
