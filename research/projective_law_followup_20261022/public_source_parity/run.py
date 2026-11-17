#!/usr/bin/env python3
"""Replay bounded then full pinned-source public projective-law comparison."""
from __future__ import annotations

import argparse
import csv
import hashlib
import json
import lzma
import os
import subprocess
import tempfile
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / 'src/bin/projective_law_gate.rs'
TESTS = HERE / 'probe_tests.rs'
GENERATED = ROOT / 'src/bin/projective_public_parity_probe.rs'
RUST_SHA = '5976e7f30b3e9c5c49a5fc0f4095404b7322832e4b02f467bc3cb7e0d9dcc8c0'

def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()

def stage(destination: Path, data: bytes, check: bool) -> None:
    if check:
        assert destination.read_bytes() == data, f'evidence differs: {destination}'
    else:
        destination.write_bytes(data)

def stage_log(destination: Path, data: bytes, check: bool) -> None:
    if check:
        archived=destination.read_bytes()
        for marker in (b'test result: ok. 1 passed; 0 failed',
                       b'PUBLIC_PARITY_RUST_JSON'):
            assert marker in archived and marker in data
    else:
        destination.write_bytes(data)

def probe(outdir: Path, limit: int | None) -> bytes:
    env = os.environ.copy()
    env['TMPDIR'] = '/dev/shm'
    env['PUBLIC_PARITY_ROWS_PATH'] = str(outdir / 'rust-rows.csv')
    env['PUBLIC_PARITY_INPUTS_PATH'] = str(outdir / 'inputs.csv')
    if limit is not None: env['PUBLIC_PARITY_LIMIT'] = str(limit)
    else: env.pop('PUBLIC_PARITY_LIMIT', None)
    command = ['cargo','test','--release','--offline','--locked',
               '--manifest-path',str(ROOT/'Cargo.toml'),'--bin',
               'projective_public_parity_probe','emit_valid_pair_corpus',
               '--','--nocapture']
    result = subprocess.run(command,cwd=ROOT.parent.parent,env=env,
                            stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
    log = result.stdout.rstrip(b'\n')+b'\n'
    assert result.returncode == 0, log.decode(errors='replace')
    assert b'test result: ok. 1 passed; 0 failed' in log
    return log

def compare(author: Path, outdir: Path) -> dict:
    subprocess.run(['python3',str(HERE/'compare.py'),'--author-source',str(author),
                    '--rust-rows',str(outdir/'rust-rows.csv'),
                    '--inputs',str(outdir/'inputs.csv'),'--outdir',str(outdir)],check=True)
    return json.loads((outdir/'SUMMARY.json').read_text())

def split_raw(source: Path) -> dict[str,bytes]:
    groups={(p,w):[] for p in (7,11) for w in (0,1)}
    with source.open(newline='') as h:
        reader=csv.DictReader(h)
        names=reader.fieldnames
        for row in reader:
            groups[(int(row['p']),int(row['weight_index']))].append(row)
    result={}
    for (p,w),rows in groups.items():
        import io
        buffer=io.StringIO(newline='')
        writer=csv.DictWriter(buffer,names,lineterminator='\n')
        writer.writeheader(); writer.writerows(rows)
        result[f'F{p}-weight{w}.csv.xz']=lzma.compress(buffer.getvalue().encode(),preset=9)
    return result

def rows_summary(source: Path) -> bytes:
    groups={(p,w):Counter() for p in (7,11) for w in (0,1)}
    with source.open(newline='') as h:
        for row in csv.DictReader(h):
            c=groups[(int(row['p']),int(row['weight_index']))]
            c['rows']+=1
            c['author_match']+=row['author_matches_rust']=='True'
            c['rust_cantor_match']+=row['rust_matches_cantor']=='True'
            c['invalid_weight']+=row['author_status']=='invalid_weight'
            c['undefined_division']+=row['author_status']=='undefined_division'
            c['other_invalid']+=row['author_status'].startswith('invalid_intermediate')
            c['zero_weights']+=int(row['author_zero_weights'])
    import io
    buffer=io.StringIO(newline='')
    names=['schema','p','weight_index','valid_divisors','ordered_pairs','rows',
           'author_match','rust_cantor_match','invalid_weight',
           'undefined_division','other_invalid','zero_weights']
    writer=csv.DictWriter(buffer,names,lineterminator='\n'); writer.writeheader()
    for (p,w),counts in sorted(groups.items()):
        n={7:66,11:114}[p]
        writer.writerow({'schema':'projective-law-public-source-parity/v1',
                         'p':p,'weight_index':w,'valid_divisors':n,
                         'ordered_pairs':n*n,**counts})
    return buffer.getvalue().encode()

def main() -> None:
    parser=argparse.ArgumentParser()
    parser.add_argument('--author-source',type=Path,required=True)
    parser.add_argument('--check',action='store_true')
    args=parser.parse_args()
    assert digest(SOURCE)==RUST_SHA
    if GENERATED.exists(): raise SystemExit(f'refusing to overwrite {GENERATED}')
    GENERATED.write_bytes(SOURCE.read_bytes()+b'\n'+TESTS.read_bytes())
    try:
        with tempfile.TemporaryDirectory(prefix='public-source-parity-',dir='/dev/shm') as tmp:
            scratch=Path(tmp)
            bounded=scratch/'bounded'; bounded.mkdir()
            bounded_log=probe(bounded,512)
            bounded_summary=compare(args.author_source,bounded)
            assert bounded_summary['rows_by_field_weight']=={
                '11:0':256,'11:1':256,'7:0':256,'7:1':256}
            stage_log(HERE/'bounded-cargo.log',bounded_log,args.check)
            stage(HERE/'BOUNDED.json',(json.dumps(bounded_summary,sort_keys=True,indent=2)+'\n').encode(),args.check)
            full=scratch/'full'; full.mkdir()
            full_log=probe(full,None)
            summary=compare(args.author_source,full)
            assert summary['rows_by_field_weight']=={
                '11:0':12996,'11:1':12996,'7:0':4356,'7:1':4356}
            summary['rust_rows_sha256']=digest(full/'rust-rows.csv')
            summary['input_catalog_sha256']=digest(full/'inputs.csv')
            summary['combined_rows_sha256']=digest(full/'combined-rows.csv')
            summary['branch_matrix_sha256']=digest(full/'branch-matrix.csv')
            stage_log(HERE/'full-cargo.log',full_log,args.check)
            for name in ('branch-matrix.csv','failures.csv'):
                stage(HERE/name,(full/name).read_bytes(),args.check)
            stage(HERE/'SUMMARY.json',(json.dumps(summary,sort_keys=True,indent=2)+'\n').encode(),args.check)
            stage(HERE/'ROWS.csv',rows_summary(full/'combined-rows.csv'),args.check)
            shards=split_raw(full/'combined-rows.csv')
            for name,data in shards.items():
                stage(HERE/name,data,args.check)
            manifest=''.join(f'{hashlib.sha256(data).hexdigest()}  {name}\n'
                             for name,data in sorted(shards.items()))
            stage(HERE/'RAW_SHA256SUMS',manifest.encode(),args.check)
            print(json.dumps(summary,sort_keys=True))
    finally:
        GENERATED.unlink()

if __name__=='__main__': main()
