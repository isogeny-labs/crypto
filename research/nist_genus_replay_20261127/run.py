#!/usr/bin/env python3
"""Administrative native-process launcher only; no finite-field arithmetic."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import subprocess
import time

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]
END = time.monotonic() + 3600


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def invoke(directory, name, argv, deadline=180):
    deadline = min(deadline, END - time.monotonic())
    if deadline <= 0:
        raise SystemExit('one-hour total wrapper limit; no automatic retry')
    cpu = min(os.sched_getaffinity(0))

    def limits():
        os.sched_setaffinity(0, {cpu})
        resource.setrlimit(resource.RLIMIT_AS, (1073741824, 1073741824))

    started = time.monotonic()
    with (directory / (name + '.stdout')).open('x') as out, (directory / (name + '.stderr')).open('x') as err:
        try:
            status = subprocess.run(argv, stdout=out, stderr=err, preexec_fn=limits, timeout=deadline).returncode
        except subprocess.TimeoutExpired:
            status = 124
    receipt = {'argv': list(map(str, argv)), 'cpu': cpu, 'address_space_bytes': 1073741824,
               'deadline_seconds': deadline, 'elapsed_diagnostic_seconds': time.monotonic() - started, 'exit': status}
    (directory / (name + '.execution.json')).write_text(json.dumps(receipt, indent=2) + '\n')
    (directory / (name + '.status')).write_text(str(status) + '\n')
    return status


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('output', type=Path)
    parser.add_argument('--cohort', action='store_true')
    options = parser.parse_args()
    out = options.output.resolve()
    out.mkdir()  # Refuse all replacement, including incomplete prior runs.
    sources = ['reconstruct.cpp', 'check.cpp', 'json.hpp', 'run.py']
    (out / 'source').mkdir()
    for name in sources:
        (out / 'source' / name).write_bytes((HERE / name).read_bytes())
    models_path = REPO / 'research/nist_cover_separator_20261115/data/models.json'
    (out / 'models.json').write_bytes(models_path.read_bytes())
    (out / 'pins.json').write_text(json.dumps({name: digest(HERE / name) for name in sources} |
                                            {'models.json': digest(models_path)}, indent=2) + '\n')
    for name, source in [('reconstruct', 'reconstruct.cpp'), ('check', 'check.cpp')]:
        argv = ['c++', '-std=c++17', '-O2', '-Wall', '-Wextra', '-Werror', '-pedantic',
                str(out / 'source' / source), '-lgmpxx', '-lgmp', '-o', str(out / name)]
        if invoke(out, 'build-' + name, argv):
            raise SystemExit('native build failed; retained logs')
    (out / 'binary-hashes.json').write_text(json.dumps({name: digest(out / name) for name in ['check', 'reconstruct']}, indent=2) + '\n')
    models = json.loads(models_path.read_text())['models']
    if not options.cohort:
        models = [row for row in models if row['family'] == '192' and row['candidate'] == 139]
    for row in models:
        for d in (range(8) if options.cohort else [7]):
            case = out / (str(row['family']) + '-' + str(row['candidate']) + '-d' + str(d))
            case.mkdir()
            args = [row['p'], row['b'], str(d)]
            status = invoke(case, 'reconstruct', [str(out / 'reconstruct')] + args)
            if status == 0:
                status = invoke(case, 'check', [str(out / 'check'), str(case / 'reconstruct.stdout')] + args)
            print(case.name, status, flush=True)
            if status:
                raise SystemExit('native case failed; no retry or omitted case')
    if not options.cohort:
        packet = out / '192-139-d7' / 'reconstruct.stdout'
        data = json.loads(packet.read_text())
        for key in ['Q', 'L', 'W', 'U']:
            mutant = json.loads(packet.read_text())
            mutant[key][0] = str(int(mutant[key][0]) + 1)  # Literal altered-input control, not research arithmetic.
            path = out / ('bad-' + key + '.json')
            path.write_text(json.dumps(mutant) + '\n')
            code = invoke(out, 'control-' + key, [str(out / 'check'), str(path), data['p'], data['b'], data['d']])
            if code != 1:
                raise SystemExit('altered-input control did not reject normally')
        for key in ['b', 'd']:
            mutant = json.loads(packet.read_text())
            mutant[key] = str(int(mutant[key]) + 1)
            path = out / ('bad-' + key + '.json')
            path.write_text(json.dumps(mutant) + '\n')
            code = invoke(out, 'control-' + key, [str(out / 'check'), str(path), data['p'], data['b'], data['d']])
            if code != 1:
                raise SystemExit('model binding control did not reject normally')
        mutant = json.loads(packet.read_text())
        mutant['b'] = str(int(mutant['b']) + 1)
        path = out / 'wrong-equation.json'
        path.write_text(json.dumps(mutant) + '\n')
        if invoke(out, 'control-equation', [str(out / 'check'), str(path), data['p'], mutant['b'], data['d']]) != 1:
            raise SystemExit('wrong curve equation not rejected')
    (out / 'COMPLETE.json').write_text(json.dumps({'native_cases_completed': '168' if options.cohort else '1',
                                                  'cohort': options.cohort, 'standing': 'unsigned-producer-development'}) + '\n')


if __name__ == '__main__':
    main()
