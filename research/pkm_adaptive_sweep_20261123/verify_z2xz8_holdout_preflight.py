#!/usr/bin/env python3
"""Replay the six source-pinned held-out map preflight records and raw hashes."""

import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent


def sha(data):
    return hashlib.sha256(data).hexdigest()


def records(path):
    result = {}
    for line in path.read_text().splitlines():
        row = json.loads(line)
        if row['id'] in result:
            raise ValueError(f'duplicate id {row["id"]}')
        result[row['id']] = row
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--store-dir', type=Path,
                        default=Path('/Volumes/SSD990-2/pkm-z2xz8-holdout-preflight'))
    parser.add_argument('--checkout', type=Path,
                        default=Path('/Volumes/SSD990-2/pkm-equalsize-full-20261128'))
    parser.add_argument('--bin', type=Path,
                        default=Path('/Volumes/SSD990/pkm-z2xz8-target-20261128/release/ic'))
    parser.add_argument('--build', type=Path,
                        default=Path('/Volumes/SSD990-2/pkm-equalsize-build-20261128/BUILD.json'))
    args = parser.parse_args()
    manifest_path = HERE/'z2xz8_holdout/manifest.json'
    manifest_raw = manifest_path.read_bytes()
    manifest = json.loads(manifest_raw)
    rows = records(args.store_dir/'receipts.jsonl')
    head = rows.get('header') or {}
    runner = HERE/'run_z2xz8_holdout_preflight.py'
    protocol = HERE/'PROTOCOL_Z2XZ8_EQUAL_SIZE_COLD_20261128.md'
    if (len(rows) != 13 or head.get('schema') != 'pkm-z2xz8-holdout-map-preflight/v1'
            or head.get('runner_sha256') != sha(runner.read_bytes())
            or head.get('protocol_sha256') != sha(protocol.read_bytes())
            or head.get('manifest_sha256') != sha(manifest_raw)
            or head.get('build_receipt_sha256') != sha(args.build.read_bytes())
            or head.get('binary_sha256') != sha(args.bin.read_bytes())):
        raise ValueError('store identity or six-case coverage differs')
    build = json.loads(args.build.read_bytes())
    if (head['source_commit'] != build['source_commit']
            or build['binary_sha256'] != head['binary_sha256']
            or not build['full_checkout'] or not build['tracked_checkout_clean']
            or build['cargo_exit_code'] != 0):
        raise ValueError('build identity differs')
    if subprocess.check_output(['git','-C',str(args.checkout),'rev-parse','HEAD'],text=True).strip()!=head['source_commit']:
        raise ValueError('source commit differs')
    subprocess.run([sys.executable,str(HERE/'verify_z2xz8_holdout.py'),
                    '--dir',str(manifest_path.parent)],check=True,capture_output=True)
    selected=[]
    for e in manifest['entries']:
        key=f"b{e['bits']}_c{e['curve_index']}"
        launch=rows['launch:'+key]
        result=rows['result:'+key]
        command=launch['command']
        if (command[0]!=str(args.bin) or command[1]!='bench'
                or command[2]!='--prime-workload'
                or command[4:6]!=['--factor-base',head['factor_base']]
                or command[6:8]!=['--oracle',head['oracle']]
                or command[-1]!='--json'):
            raise ValueError(f'command differs {key}')
        input_path=Path(command[3])
        input_raw=input_path.read_bytes()
        if sha(input_raw)!=launch['input_sha256']:
            raise ValueError(f'input hash differs {key}')
        input_doc=json.loads(input_raw)
        frozen=json.loads((manifest_path.parent/e['path']).read_bytes())
        if (input_doc['curve']!=frozen['curve']
                or input_doc['targets']!=frozen['targets'][:1]
                or launch['target_scalar']!=e['target_scalars'][0]):
            raise ValueError(f'input target differs {key}')
        for kind in ('stdout','stderr'):
            part=result[kind]
            raw=Path(part['path']).read_bytes()
            if sha(raw)!=part['sha256'] or len(raw)!=part['bytes']:
                raise ValueError(f'raw {kind} differs {key}')
        raw_report=json.loads(Path(result['stdout']['path']).read_bytes())
        report_rows=raw_report.get('rows') or []
        if (raw_report.get('operation')!='bench'
                or raw_report.get('prime_workload_sha256')!=launch['input_sha256']
                or raw_report.get('frozen_target_scalars')!=[launch['target_scalar']]
                or len(report_rows)!=1):
            raise ValueError(f'report identity differs {key}')
        fb=report_rows[0].get('factor_base') or {}
        native=(fb.get('cost') or {}).get('native') or {}
        p=e['p']
        a=int(frozen['curve']['a'],16)
        b=int(frozen['curve']['b'],16)
        interval=sum(pow((x*x*x+a*x+b)%p,(p-1)//2,p)!=p-1 for x in range(32))
        if (fb.get('abscissae')!=interval
                or native.get('aux_selection_m')!=native.get('aux_curves_tried')
                or not 1<=native.get('aux_selection_m',0)<=64
                or not 1<=native.get('aux_selection_cosets_tested',0)<=64*native['aux_curves_tried']
                or native.get('aux_selection_sqrt_muls',0)<=0
                or native.get('aux_search_gae',0)<=0
                or native.get('aux_tower_build_gae',0)<=0
                or result['summary']['factor_base_abscissae']!=interval):
            raise ValueError(f'map size or work counters differ {key}')
        selected.append({'curve':key,'p':p,'interval_columns':interval,
                         'm':native['aux_selection_m'],
                         'cosets':native['aux_selection_cosets_tested'],
                         'sqrt_calls':native['aux_selection_sqrt_calls'],
                         'sqrt_muls':native['aux_selection_sqrt_muls'],
                         'search_gae':native['aux_search_gae'],
                         'exit_code':result['exit_code'],
                         'status':raw_report.get('status')})
    print(json.dumps({'schema':'pkm-z2xz8-holdout-preflight-replay/v1',
                      'store_sha256':sha((args.store_dir/'receipts.jsonl').read_bytes()),
                      'selected':selected},sort_keys=True))


if __name__=='__main__':
    main()
