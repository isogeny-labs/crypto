#!/usr/bin/env python3
"""Fail-closed source and packet validator for the bounded N6 correction."""
import argparse
import csv
import hashlib
import json
import re
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
AUTHOR_SHA = '7416a7fffaafff371738e7498a6795057372e1c477e0079297eebf2347c2e72a'
RUST_SHA = '5976e7f30b3e9c5c49a5fc0f4095404b7322832e4b02f467bc3cb7e0d9dcc8c0'

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def require(ok, message):
    if not ok:
        raise AssertionError(message)

def check(args):
    author = Path(args.author_source)
    rust = ROOT / 'src/bin/projective_law_gate.rs'
    require(digest(author) == AUTHOR_SHA, 'pinned Magma source hash')
    require(digest(rust) == RUST_SHA, 'runtime Rust source hash')
    a = author.read_text()
    m = re.search(r'WW1\s*:=\s*W1\s*\^\s*2\s*;', a)
    n = re.search(r'N6\s*:=\s*\([^;]*\)\s*\*\s*WW1\s*;', a)
    require(m is not None and n is not None and m.end() < n.start(), 'Magma alias and ordered N6 assignment')
    r = rust.read_text()
    require('if n6_uses_squared_w { w2 } else { w }' in r, 'Rust variant expression')
    require(re.search(r'fn double22_weighted\([^)]*\)[^{]*\{\s*double22_weighted_variant\(a, curve, f, true\)', r) is not None,
            'Rust production squared-w selector')
    log = (HERE/'lean.log').read_text()
    require('error:' not in log.lower() and 'sorry' not in log.lower(), 'Lean error or sorry')
    theorem_names = ['author_a_matches_printed_affine', 'author_b_matches_printed_affine',
                     'author_c_matches_printed_affine', 'linear_mutation_a_defect',
                     'linear_mutation_c_defect']
    for name in theorem_names:
        require(f"'N6AffineBridge.{name}' depends on axioms: [propext, Quot.sound]" in log,
                f'Lean axiom audit {name}')
    require(len(log.splitlines()) == 5, 'five theorem axiom lines')
    with (HERE/'scaling-weights.csv').open(newline='') as f:
        rows = list(csv.DictReader(f))
    require([(row['coefficient'],row['author_rust_weight_z'],row['author_rust_weight_w']) for row in rows]
            == [('A','11','3'),('B','10','2'),('C','9','3')], 'coefficient weight CSV')
    svg = (HERE/'scaling-weights.svg').read_text()
    require('<svg ' in svg and 'synthetic mutation' in svg, 'SVG legend')
    for pdf in [HERE/'REPORT.pdf', HERE.parent/'REPORT.pdf',
                HERE.parents[1]/'independent_complete_law/SOURCE_AUDIT.pdf']:
        require(pdf.read_bytes().startswith(b'%PDF-'), f'PDF signature: {pdf}')
    frozen = HERE.parent/'N6Homogeneity.lean'
    require(digest(frozen) == '99e4d0288be44ac496d146b6471930a155a28ab606ae59aa696043f649878189',
            'frozen Lean proof bytes')
    require(digest(HERE.parents[1]/'results/n6-w-power-regression.json') ==
            '3c58257325810f0d306222b5e7a69bf17536ed4595bf9dc966152e87a2e138fb',
            'frozen mutation record')
    return {'schema':'n6-source-correction/v1', 'status':'verified',
            'author_magma_sha256':AUTHOR_SHA, 'runtime_rust_sha256':RUST_SHA,
            'lean_source_sha256':digest(HERE/'N6AffineBridge.lean'),
            'pdf_sha256':digest(HERE/'REPORT.pdf'),
            'graph_svg_sha256':digest(HERE/'scaling-weights.svg'),
            'author_alias':'WW1:=W1^2', 'author_n6_uses':'WW1',
            'rust_production_squared_w':True, 'lean_theorems':theorem_names,
            'lean_axioms':['propext','Quot.sound'],
            'coefficient_weights':rows,
            'scope':'source attribution and universal commutative-ring coefficient identities; no compiled semantics or Pic0 bridge'}

def main():
    p = argparse.ArgumentParser()
    p.add_argument('--author-source', required=True)
    p.add_argument('--check', action='store_true')
    args = p.parse_args()
    result = check(args)
    target = HERE/'VALIDATION.json'
    if args.check:
        require(json.loads(target.read_text()) == result, 'validation receipt differs')
    else:
        target.write_text(json.dumps(result, indent=2, sort_keys=True)+'\n')
    print(json.dumps({'status':'verified', 'receipt':str(target)}, sort_keys=True))

if __name__ == '__main__':
    main()
