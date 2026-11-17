#!/usr/bin/env python3
"""Plan audit downloads after the harness hydrates the complete session set.

Use the audit's existing largest-first byte balancing. Exception capsules stay
in every shard so their hash pins and expected failures are still checked.
Only path lists cross jobs; private evidence stays in its original store.
"""
import argparse
import json
from pathlib import Path, PurePosixPath


def checked_paths(lines):
    paths = []
    for line in lines:
        value = line.rstrip('\n')
        p = PurePosixPath(value)
        if not value or p.is_absolute() or '..' in p.parts or any(c in value for c in '\r\t\0'):
            raise ValueError(f'invalid session path: {value!r}')
        if value in paths:
            raise ValueError(f'duplicate session: {value}')
        paths.append(value)
    if not paths:
        raise ValueError('empty session list')
    return paths


def plan(root, sessions, exception_lists, count):
    if count < 1:
        raise ValueError('shard count must be positive')
    exceptions = set()
    for path in exception_lists:
        for row in path.read_text().splitlines():
            if row and not row.startswith('#'):
                name = row.split('\t')[0]
                if name not in sessions:
                    raise ValueError(f'exception not in complete session list: {name}')
                exceptions.add(name)
    weights = []
    for index, name in enumerate(sessions):
        directory = root / name
        if json.loads((directory / 'session.json').read_text()).get('schema') != 'ecbench.session/v1':
            raise ValueError(f'not an ecbench session: {name}')
        if name in exceptions:
            continue
        weight = sum(f.stat().st_size * (8 if f.name.endswith('.gz') else 1)
                     for f in directory.iterdir() if f.is_file() and not f.is_symlink())
        weights.append((weight, index, name))
    shards = [[] for _ in range(count)]
    loads = [0] * count
    for weight, index, name in sorted(weights, key=lambda x: (-x[0], x[1])):
        k = min(range(count), key=lambda i: loads[i])
        shards[k].append(name)
        loads[k] += weight
    assigned = [p for shard in shards for p in shard]
    if len(assigned) != len(set(assigned)) or set(assigned) | exceptions != set(sessions):
        raise ValueError('shards do not exactly cover the complete session set')
    shared = [p for p in sessions if p in exceptions]
    return [shard + shared for shard in shards], loads


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('list', type=Path)
    p.add_argument('--shards', type=int, default=16)
    p.add_argument('--output', type=Path, required=True)
    p.add_argument('--incomplete', type=Path, default=Path('docs/ecbench/INCOMPLETE_SESSIONS.tsv'))
    p.add_argument('--defective', type=Path, default=Path('docs/ecbench/DEFECTIVE_SESSIONS.tsv'))
    a = p.parse_args()
    sessions = checked_paths(a.list.read_text().splitlines())
    shards, loads = plan(Path.cwd(), sessions, [a.incomplete, a.defective], a.shards)
    a.output.mkdir(parents=True, exist_ok=True)
    for k, shard in enumerate(shards):
        (a.output / f'{k}.txt').write_text(''.join(f'{name}\n' for name in shard))
    (a.output / 'summary.json').write_text(json.dumps({'sessions':len(sessions), 'shards':a.shards, 'weighted_bytes':loads},indent=2)+'\n')
    print(f'Planned {len(sessions)} sessions across {a.shards} downloads; exception capsules repeated.')


if __name__ == '__main__':
    main()
