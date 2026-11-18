"""Pinned progress-runner admission guard; does not grant execution authority."""
import argparse
import datetime
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
spec = importlib.util.spec_from_file_location('bounded_capture', ROOT / 'feature-native-monomial-capture/capture.py')
capture = importlib.util.module_from_spec(spec)
spec.loader.exec_module(capture)


def validate(protocol, admission, binary, binding, now):
    """Pure gate, evaluated before any output directory or process is created."""
    digest = capture.canonical_protocol(protocol)
    if protocol['freeze']['status'] != 'frozen' or protocol['freeze']['protocol_sha256'] != digest:
        raise ValueError('unfrozen or changed protocol')
    if admission['status'] != 'admitted' or admission['protocol_sha256'] != digest:
        raise ValueError('matching admission required')
    if [arm['id'] for arm in protocol['arms']] != ['baseline', 'candidate', 'aa']:
        raise ValueError('unexpected arm schedule')
    for arm in protocol['arms']:
        instrument = arm['instrument']
        if instrument['commit'] != binding['source_commit'] or instrument['binary_sha256'] != binding['binary_sha256']:
            raise ValueError('instrument differs from published binding')
    if admission['source_commit'] != binding['source_commit']:
        raise ValueError('admission source mismatch')
    if binary != binding['binary_sha256'] or admission['binary_sha256'] != binary:
        raise ValueError('binary mismatch')
    for field, expected in [('max_attempts', 1), ('timeout_seconds', 120)]:
        if type(admission[field]) is not int or admission[field] != expected:
            raise ValueError('wrong finite envelope')
    deadline = datetime.datetime.fromisoformat(admission['deadline_utc'].replace('Z', '+00:00'))
    if deadline.tzinfo is None or now.tzinfo is None:
        raise ValueError('timezone required')
    remaining = (deadline - now).total_seconds()
    if remaining <= 0:
        raise ValueError('expired admission')
    return digest, min(120, remaining)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ['runner', 'protocol', 'admission', 'out']:
        parser.add_argument('--' + name, type=Path, required=True)
    args = parser.parse_args()
    binding = json.loads((HERE / 'source-binding.json').read_text())
    for path, expected in binding['dependencies'].items():
        if capture.sha(ROOT / path) != expected:
            raise ValueError('dependency mismatch: ' + path)
    protocol = json.loads(args.protocol.read_text())
    admission = json.loads(args.admission.read_text())
    binary = capture.sha(args.runner)
    digest, timeout = validate(protocol, admission, binary, binding, datetime.datetime.now(datetime.timezone.utc))
    record = capture.capture([str(args.runner.resolve()), '--run-admitted'], args.out, timeout, {
        'protocol_sha256': digest, 'protocol_file_sha256': capture.sha(args.protocol),
        'binary_sha256': binary, 'source_commit': binding['source_commit'],
        'admission_sha256': capture.sha(args.admission),
        'binding_sha256': capture.sha(HERE / 'source-binding.json'),
    })
    raise SystemExit(0 if record['exit_code'] == 0 and not record['timed_out'] else 1)

if __name__ == '__main__':
    main()
