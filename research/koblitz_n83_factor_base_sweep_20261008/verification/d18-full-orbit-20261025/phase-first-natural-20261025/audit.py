import base64
import copy
import gzip
import hashlib
import importlib.util
import json
import pathlib
import re
import sys

import blake3

HERE = pathlib.Path(__file__).resolve().parent
VERIFY = HERE.parent
if len(sys.argv) != 3:
    raise SystemExit('usage: audit.py FRESH_BASE_DIR EXACT_SOURCE_BINARY')
BASE = pathlib.Path(sys.argv[1]).resolve()
BINARY = pathlib.Path(sys.argv[2]).resolve()
SOURCE = '43a422ce66ebce82b48af036d15dcbef8a5ac785'
BINARY_SHA = '9605c08a4aeeb61e6ce3196cb74d5a87ddea9d13f9f1d79a0a4e643d85ac5152'
OBJECT_SHA = 'eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b'
MANIFEST_SHA = '1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616'
OBJECT = BASE / 'objects/c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz'
POSIT = HERE / 'positive'
NAT = HERE / 'natural'

def read(path):
    return json.loads(path.read_text())

def sha(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b''):
            h.update(chunk)
    return h.hexdigest()

spec = importlib.util.spec_from_file_location('n83_independent_group', VERIFY / 'root-pinned-20261025/verify.py')
group = importlib.util.module_from_spec(spec)
spec.loader.exec_module(group)

assert sha(OBJECT) == OBJECT_SHA and OBJECT.stat().st_size == 160747529
assert sha(BASE / 's3-manifest.json') == MANIFEST_SHA
wrapper = read(BASE / 's3-manifest.json')
object_get = read(BASE / 'object-get-metadata.json')
manifest_get = read(BASE / 'manifest-get-metadata.json')
assert wrapper['compressed_sha256'] == OBJECT_SHA and wrapper['producer_manifest_sha256'] == sha(VERIFY / 'manifest.json')
assert object_get['ChecksumSHA256'] == base64.b64encode(bytes.fromhex(OBJECT_SHA)).decode()
assert object_get['ContentLength'] == 160747529 and object_get['ServerSideEncryption'] == 'AES256'
assert manifest_get['ChecksumSHA256'] == base64.b64encode(bytes.fromhex(MANIFEST_SHA)).decode()
assert sha(BINARY) == BINARY_SHA


def replay_relation(result):
    indices = result['relation']['indices']
    assert len(indices) == 4 and all(isinstance(i, int) and i >= 0 for i in indices)
    wanted = set(indices)
    rows = {}
    with gzip.open(OBJECT, 'rt') as f:
        header = json.loads(next(f))
        assert header['curve_a'] == 0 and header['subgroup_order'] == str(group.ORDER)
        for idx, line in enumerate(f):
            if idx in wanted:
                rows[idx] = json.loads(line)
            if len(rows) == len(wanted):
                break
    assert len(rows) == len(wanted)
    generator = tuple(map(int, header['generator']))
    assert group.on_curve(generator) and group.scalar_mul(generator, group.ORDER) is None
    target = tuple(map(int, result['target']))
    assert group.on_curve(target) and group.scalar_mul(target, group.ORDER) is None
    total = None
    coeffs = [0] * 32643
    for idx in indices:
        row = rows[idx]
        pt = tuple(map(int, row['point']))
        assert group.on_curve(pt) and group.scalar_mul(pt, group.ORDER) is None
        total = group.add(total, pt)
        col = int(row['column'])
        assert 0 <= col < 32643
        coeffs[col] = (coeffs[col] + int(row['coefficient'])) % group.ORDER
    assert total == target
    digest = blake3.blake3(json.dumps([str(x) for x in coeffs], separators=(',', ':')).encode()).hexdigest()
    assert digest == result['relation']['row_decimal_blake3']
    assert result['relation']['rank_after_insert'] == 1 and result['relation']['tamper_rejected'] is True
    assert group.add(target, generator) != total
    return {'indices': indices, 'target': list(map(str, target)), 'row_decimal_blake3': digest, 'group_replay': 'PASS', 'tamper_rejected': True}


def arm(path, mode):
    if mode == 'natural' and (path / 'result.json.cap.json').exists():
        assert (path / 'exit-code.txt').read_text().strip() == '124'
        assert (path / 'outer-cap.txt').read_text().strip() == 'not_reached'
        assert (path / 'binary-sha256.txt').read_text().split()[0] == BINARY_SHA
        args = read(path / 'command.json')
        assert args[3] == 'primary-full-d18-m4-l0-shared-phase-first-natural'
        cap = read(path / 'result.json.cap.json')
        assert cap['schema'] == 'n83.full-d18-m4-cap/v1' and cap['status'] == 'UNKNOWN_wall_cap'
        assert cap['mode'] == mode and cap['budget_seconds'] == 300
        assert cap['choice_pin_policy'] == 'none' and cap['search_policy'] == 'phase_zero_first'
        assert cap['selected_best_total_runtime'] is None
        shape = read(path / 'result.json.shape.json')
        assert shape['variables'] == 329726 and shape['clauses'] == 2807254 and shape['xor_rows'] == 3901
        assert shape['search_policy'] == 'phase_zero_first' and shape['choice_pin_policy'] == 'none'
        phases = [json.loads(x) for x in (path / 'result.json.phases.jsonl').read_text().splitlines()]
        assert [x['phase'] for x in phases] == ['started', 'import_checked', 'model_constructed']
        assert phases[0]['detail']['source_commit'] == SOURCE and phases[0]['detail']['binary_sha256'] == BINARY_SHA
        assert phases[2]['detail'] == shape
        rss_match = re.search(r'(\d+)\s+maximum resident set size', (path / 'stderr-time.txt').read_text())
        assert rss_match
        return {'status': cap['status'], 'solver_answer': None, 'solver_conflicts': None, 'import_ms': shape['import_ms'], 'model_ms': shape['model_ms'], 'search_ms': None, 'observed_post_model_window_ms': cap['elapsed_process_wall_ms'] - phases[2]['elapsed_process_wall_ms'], 'process_wall_ms': cap['elapsed_process_wall_ms'], 'supervisor_max_rss_bytes': int(rss_match.group(1)), 'relation': None}
    assert (path / 'exit-code.txt').read_text().strip() == '0'
    assert (path / 'outer-cap.txt').read_text().strip() == 'not_reached'
    assert (path / 'binary-sha256.txt').read_text().split()[0] == BINARY_SHA
    args = read(path / 'command.json')
    assert args[3] == 'primary-full-d18-m4-l0-shared-phase-first' + ('-natural' if mode == 'natural' else '')
    result = read(path / 'result.json')
    assert read(path / 'stdout.json') == result
    assert result['mode'] == mode and result['source_commit'] == SOURCE and result['binary_sha256'] == BINARY_SHA
    assert result['source_attestation_mode'] == 'clean_checkout'
    assert result['choice_pin_policy'] == 'none' and result['search_policy'] == 'phase_zero_first'
    assert result['root_encoding'] == 'shared_root_bits' and result['root_choice_pinned'] is False
    assert result['selected_best_total_runtime'] is None
    assert result['shape'] == read(path / 'result.json.shape.json')
    shape = result['shape']
    assert shape['variables'] == 329726 and shape['xor_rows'] == 3901
    assert shape['clauses'] == (2807752 if mode == 'positive' else 2807254)
    phases = [json.loads(x) for x in (path / 'result.json.phases.jsonl').read_text().splitlines()]
    assert [x['phase'] for x in phases] == ['started', 'import_checked', 'model_constructed', 'solver_returned']
    assert phases[0]['detail']['source_commit'] == SOURCE and phases[2]['detail'] == shape
    rss_match = re.search(r'(\d+)\s+maximum resident set size', (path / 'stderr-time.txt').read_text())
    assert rss_match
    relation = None
    if mode == 'positive':
        assert result['status'] == 'PASS_CONSTRUCTED_CONTROL' and result['solver_answer'] == 'Sat'
        assert result['relation']['matches_planted_row'] is True
        relation = replay_relation(result)
    elif result['status'] == 'PENDING_INDEPENDENT_NATURAL_REPLAY':
        relation = replay_relation(result)
    else:
        assert result['status'] in ('UNKNOWN_search_cap', 'MODEL_GROUP_REJECTED', 'UNVERIFIED_UNSAT', 'INVALID_MODEL')
    return {'status': result['status'], 'solver_answer': result['solver_answer'], 'solver_conflicts': result['solver_conflicts'], 'import_ms': shape['import_ms'], 'model_ms': shape['model_ms'], 'search_ms': result['search_ms'], 'process_wall_ms': result['process_wall_ms'], 'supervisor_max_rss_bytes': int(rss_match.group(1)), 'relation': relation}

positive = arm(POSIT, 'positive')
natural = arm(NAT, 'natural')
original = read(POSIT / 'result.json')
negative_controls = {}
for label in ('changed_target', 'changed_row_digest'):
    mutated = copy.deepcopy(original)
    if label == 'changed_target':
        mutated['target'][0] = str(int(mutated['target'][0]) ^ 1)
    else:
        mutated['relation']['row_decimal_blake3'] = '0' * 64
    try:
        replay_relation(mutated)
    except AssertionError:
        negative_controls[label] = 'REJECTED'
    else:
        raise AssertionError(label)
status = 'PASS_CENSORED_NATURAL_WALL_CAP' if natural['status'] == 'UNKNOWN_wall_cap' else 'PASS_REPLAY'
print(json.dumps({'schema': 'n83.full-d18-phase-first-natural-independent-audit/v1', 'status': status, 'source_commit': SOURCE, 'binary_sha256': BINARY_SHA, 'object_sha256': OBJECT_SHA, 'positive': positive, 'natural': natural, 'negative_controls': negative_controls, 'selected_best_total_runtime': None}, indent=2, sort_keys=True))
