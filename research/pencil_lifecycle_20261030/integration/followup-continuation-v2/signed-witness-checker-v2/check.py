"""Pinned read-only direct certificate validation; no producer/elimination import."""
from pathlib import Path
import copy
import hashlib
import importlib.util
import json
import tarfile
from collections import Counter

P = Path(__file__).resolve().parent
BASE = P.parent

def require(ok, message):
    if not ok:
        raise ValueError(message)

def digest(data):
    return hashlib.sha256(data).hexdigest()

def unique_index(items, key):
    result = {}
    for item in items:
        name = item[key]
        require(name not in result, 'duplicate ' + key)
        result[name] = item
    return result

def check_bundle(artifacts, streams, summary, verify):
    index = unique_index(artifacts, 'source_stream')
    records = unique_index(summary['records'], 'stream')
    require(len(index) == 64 and set(index) == set(streams) == set(records), 'complete source/summary coverage')
    counts = Counter()
    total = 0
    for name, artifact in index.items():
        raw = streams[name]
        records_raw = [json.loads(line) for line in raw.splitlines()]
        require([r['sequence'] for r in records_raw] == list(range(len(records_raw))), 'sequence')
        events = [r['event'] for r in records_raw]
        header, terminal = events[0], events[-1]
        require(header['kind'] == 'header' and terminal['kind'] == 'terminal', 'event endpoints')
        require(header['zero_node'] == 0 and header['convention'] == 'su_xu_plus_sv_xv_eq_a_plus_bk', 'equation convention')
        p, n = header['r'], terminal['node_count'] - 1
        require(artifact['schema'] == 'signed.target-witness/v1' and artifact['modulus'] == p and artifact['nuisance_columns'] == n, 'domain binding')
        require(artifact['source_sha256'] == digest(raw) and artifact['convention'] == 'U*x-b*k-a=0', 'source binding')
        candidates = unique_index([e for e in events if e['kind'] == 'candidate'], 'attempt')
        rows, origins = [], []
        for sequence, event in enumerate(events):
            if event['kind'] not in ('edge', 'identity'):
                continue
            row = [0] * (n + 2)
            if event['kind'] == 'edge':
                nodes, signs = event['nodes'], event['signs']
                require(len(nodes) == len(signs) == 2, 'edge arity')
                require(all(type(v) is int and 0 <= v <= n for v in nodes), 'node domain')
                require(all(type(v) is int and v in (-1, 1) for v in signs), 'sign domain')
                for node, sign in zip(nodes, signs):
                    if node:
                        row[node - 1] = (row[node - 1] + sign) % p
                a, b = event['a'], event['b']
            else:
                candidate = candidates[event['attempt']]
                a, b = candidate['a'], candidate['b']
            row[-2:] = [-b % p, -a % p]
            rows.append(row)
            origins.append({'sequence': sequence, 'kind': event['kind'], 'attempt': event['attempt']})
        require(rows == artifact['rows'] and origins == artifact['row_origins'], 'row extraction')
        cert = artifact['certificate']
        require(verify(p, n, rows, cert), 'certificate equality')
        retained = terminal['recovered']
        require(artifact['retained_scalar'] == retained, 'retained scalar')
        if retained is None:
            require(terminal['exhausted'] is True and cert['kind'] == 'undetermined', 'capped classification')
        else:
            require(type(retained) is int and terminal['exhausted'] is False and cert['kind'] == 'determined' and cert['scalar'] == retained, 'recovered classification/scalar')
        require(artifact['direct_check_pass'] is True and artifact['matches_retained'] is True, 'artifact flags')
        expected = {'stream': name, 'rows': len(rows), 'nuisance_columns': n, 'kind': cert['kind'], 'scalar': cert.get('scalar'), 'retained_scalar': retained, 'matches_retained': True, 'direct_check_pass': True}
        require(records[name] == expected, 'summary row join')
        counts[cert['kind']] += 1
        total += len(rows)
    require(summary['counts'] == dict(counts) and summary['all_direct_checks_pass'] is True and summary['all_match_retained'] is True, 'summary aggregate')
    return {'certificates': len(index), 'rows': total, 'counts': dict(counts)}

def load():
    pins = json.loads((P / 'input-pins.json').read_text())
    for name, pin in pins['files'].items():
        raw = (BASE / name).read_bytes()
        require(len(raw) == pin['bytes'] and digest(raw) == pin['sha256'], 'pinned input ' + name)
    old = BASE / 'signed-witness'
    source = BASE / 'signed-native-restarts'
    cm = unique_index(json.loads((old / 'certificate-manifest.json').read_text())['members'], 'file')
    sm = unique_index(json.loads((source / 'stream-manifest.json').read_text())['members'], 'file')
    artifacts, streams = [], {}
    with tarfile.open(old / 'certificates.tar.gz') as ct, tarfile.open(source / 'native-streams.tar') as st:
        require(len(ct.getnames()) == len(set(ct.getnames())) and set(ct.getnames()) == set(cm), 'certificate archive set')
        require(len(st.getnames()) == len(set(st.getnames())) and set(st.getnames()) == set(sm), 'source archive set')
        for name, entry in sm.items():
            raw = st.extractfile(name).read()
            require(digest(raw) == entry['sha256'] and len(raw) == entry['bytes'], 'source member hash')
            if name.endswith('.jsonl'):
                streams[name] = raw
        for name, entry in cm.items():
            raw = ct.extractfile(name).read()
            require(digest(raw) == entry['sha256'] and len(raw) == entry['bytes'], 'certificate member hash')
            artifact = json.loads(raw)
            require(name == artifact['source_stream'][:-6] + '.certificate.json', 'certificate filename join')
            artifacts.append(artifact)
    require(set(cm) == {name[:-6] + '.certificate.json' for name in streams}, 'complete certificate set')
    spec = importlib.util.spec_from_file_location('pinned_direct_checker', old / 'check_certificate.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return artifacts, streams, json.loads((old / 'summary.json').read_text()), module.verify

def main():
    artifacts, streams, summary, verify = load()
    result = check_bundle(artifacts, streams, summary, verify)
    controls = []
    def reject(name, mutate):
        aa, ss, su = copy.deepcopy(artifacts), dict(streams), copy.deepcopy(summary)
        mutate(aa, ss, su)
        try:
            check_bundle(aa, ss, su, verify)
        except ValueError as error:
            controls.append({'name': name, 'rejected': True, 'reason': str(error)})
        else:
            raise RuntimeError('negative control accepted: ' + name)
    reject('duplicate_certificate_omits_stream', lambda a,s,u: a.__setitem__(1, copy.deepcopy(a[0])))
    reject('missing_certificate', lambda a,s,u: a.pop())
    reject('unexpected_source', lambda a,s,u: s.update({'unexpected.jsonl': b''}))
    reject('duplicate_summary', lambda a,s,u: u['records'].__setitem__(1, copy.deepcopy(u['records'][0])))
    reject('missing_summary', lambda a,s,u: u['records'].pop())
    reject('summary_scalar', lambda a,s,u: u['records'][0].update(scalar=3))
    reject('summary_counts', lambda a,s,u: u['counts'].update(determined=38))
    reject('certificate_scalar', lambda a,s,u: a[0]['certificate'].update(scalar=3))
    reject('source_row', lambda a,s,u: a[0]['rows'][0].__setitem__(-1, (a[0]['rows'][0][-1]+1)%a[0]['modulus']))
    def terminal_mutation(a,s,u,field,value):
        name = a[0]['source_stream']
        records = [json.loads(x) for x in s[name].splitlines()]
        records[-1]['event'][field] = value
        raw = ('\n'.join(json.dumps(x) for x in records)+'\n').encode()
        s[name] = raw
        a[0]['source_sha256'] = digest(raw)
        if field == 'recovered':
            a[0]['retained_scalar'] = value
    reject('terminal_scalar_self_consistent_binding', lambda a,s,u: terminal_mutation(a,s,u,'recovered',3))
    reject('terminal_exhaustion', lambda a,s,u: terminal_mutation(a,s,u,'exhausted',True))
    print(json.dumps({'schema': 'signed.target-witness.checker-v2/v1', **result, 'negative_controls': controls, 'producer_invoked': False, 'elimination_invoked': False, 'collector_invoked': False, 'wrapper_independently_reviewed': False}, indent=2))

if __name__ == '__main__':
    main()
