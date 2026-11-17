#!/usr/bin/env python3
"""Render pinned reports in a new external directory; preserve archived outputs."""
from __future__ import annotations

import argparse
import hashlib
import importlib.metadata
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = Path(__file__).with_name('report_reproduction_v1.json')
INPUTS = ('build_report.py', 'ledger.json', 'REPORT.md', 'PROOF_MAP.md')


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def new_destination(path: Path, repo: Path) -> Path:
    path = Path(os.path.abspath(path))
    for part in (path, *path.parents):
        if part.is_symlink():
            raise ValueError('output and its ancestors must not be symlinks')
    if path.is_relative_to(repo.resolve()):
        raise ValueError('output must be outside the source repository')
    if path.exists():
        raise FileExistsError('output already exists; choose a fresh directory')
    if not path.parent.is_dir():
        raise ValueError('output parent must already exist')
    return path


def source(repo: Path, revision: str, path: str, expected: str) -> bytes:
    data = subprocess.check_output(
        ['git', '-C', str(repo), 'cat-file', '--filters', '--path=' + path,
         revision + ':' + path])
    if digest(data) != expected:
        raise ValueError('frozen source digest mismatch: ' + path)
    return data


def reproduce(rounds: list[str], output: Path) -> dict:
    manifest = json.loads(MANIFEST.read_text())
    versions = {name: importlib.metadata.version(name) for name in manifest['dependencies']}
    if versions != manifest['dependencies']:
        raise ValueError('install the exact patched report requirements: ' + str(versions))
    output = new_destination(output, ROOT)
    # Authenticate every frozen input and comparison baseline before any execution.
    captured = {}
    for number in rounds:
        pins = manifest['rounds'][number]
        captured[number] = {
            name: source(ROOT, manifest['source_revision'],
                         manifest['study'] + '/' + number + '/' + name, pins[name])
            for name in (*INPUTS, 'routes.svg', 'report.typ', 'report.pdf')}
    output.mkdir()  # exclusive, without exist_ok
    receipt = {'schema': 'crypto.report-reproduction-result/v1',
               'source_revision': manifest['source_revision'],
               'dependencies': versions, 'python': sys.version,
               'scope': 'layout reproduction only; original scientific evidence unchanged',
               'rounds': {}}
    import pymupdf
    for number, files in captured.items():
        here = output / 'study' / 'rounds' / number
        here.mkdir(parents=True)
        for name in INPUTS:
            (here / name).write_bytes(files[name])
        result = subprocess.run([sys.executable, str(here / 'build_report.py')],
                                cwd=here, capture_output=True, text=True, timeout=120)
        (here / 'build.stdout.txt').write_text(result.stdout)
        (here / 'build.stderr.txt').write_text(result.stderr)
        if result.returncode:
            raise RuntimeError(f'round {number} builder failed: {result.stderr}')
        with pymupdf.open(stream=files['report.pdf'], filetype='pdf') as old, \
                pymupdf.open(here / 'report.pdf') as new:
            text_equal = [p.get_text() for p in old] == [p.get_text() for p in new]
            pixels_equal = len(old) == len(new) and all(
                a.get_pixmap().samples == b.get_pixmap().samples for a, b in zip(old, new))
            comparisons = {'pages_original': len(old), 'pages_regenerated': len(new),
                           'page_text_equal': text_equal, 'page_pixels_equal': pixels_equal}
        comparisons.update({name + '_equal': files[name] == (here / name).read_bytes()
                            for name in ('routes.svg', 'report.typ', 'report.pdf')})
        if (not comparisons['routes.svg_equal'] or not comparisons['report.typ_equal']
                or not text_equal or comparisons['pages_original'] != comparisons['pages_regenerated']):
            raise ValueError(f'round {number} content/layout comparison failed: {comparisons}')
        receipt['rounds'][number] = {
            'input_sha256': {name: digest(files[name]) for name in INPUTS},
            'output_sha256': {str(p.relative_to(here)): digest(p.read_bytes())
                              for p in sorted(here.rglob('*')) if p.is_file() and p.name not in INPUTS},
            'comparison': comparisons}
    (output / 'REPRODUCTION.json').write_text(json.dumps(receipt, indent=2) + '\n')
    return receipt


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--round', choices=('07', '09', '10'), action='append', dest='rounds')
    parser.add_argument('--output', required=True, type=Path)
    args = parser.parse_args()
    reproduce(args.rounds or ['07', '09', '10'], args.output)
    print('Isolated report reproduction complete:', args.output)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
