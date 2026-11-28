#!/usr/bin/env python3
"""Resumable, diagnostic one-query preflight on the frozen PKM holdout."""

import argparse
import hashlib
import json
import os
import resource
import signal
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = 'a0642f42e205aba4951dcf9732721d62211d7c33'
BINARY_SHA = '8e5ba8c40647a076310a2f4205774caaa6925e7eadda6ca961fe8e4590a7a027'
MANIFEST_SHA = '99140281d9a94c288b8993b503cf91e4226879052e25c63dfdad0fccd20c8bc8'
PROTOCOL_SHA = '723ac30ff07386b4ad1769ce89d353229d8ba9636b5c0582813a3c7986378a3d'
BUILD_SHA = 'e09abcf0d0ff07d82f1392bca04e3abb159fea853539d8a328d40b4a5f8c63b7'
FACTOR_BASE = 'prime-isogeny:t=5,seed=0,aux=ordinary,family=z2xz8,selection=equal-size'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def encoded(value):
    return (json.dumps(value,sort_keys=True,indent=2)+'\n').encode()


def append(path,row):
    with path.open('ab') as out:
        out.write((json.dumps(row,sort_keys=True)+'\n').encode())
        out.flush()
        os.fsync(out.fileno())


def records(path):
    if not path.exists():return {}
    rows={}
    for line in path.read_text().splitlines():
        row=json.loads(line)
        if row['id'] in rows:raise ValueError(f'duplicate receipt id {row["id"]}')
        rows[row['id']]=row
    return rows


def preserve(path,data):
    if path.exists():
        if path.read_bytes()!=data:raise ValueError(f'frozen input changed: {path}')
        return
    with path.open('xb') as out:
        out.write(data)
        out.flush()
        os.fsync(out.fileno())


def run(command,stdout_path,stderr_path):
    start=time.perf_counter()
    pre_load=os.getloadavg()
    with stdout_path.open('xb') as out,stderr_path.open('xb') as err:
        proc=subprocess.Popen(command,stdout=out,stderr=err,start_new_session=True)
        status=usage=None
        while True:
            pid,status,usage=os.wait4(proc.pid,os.WNOHANG)
            if pid:break
            if time.perf_counter()-start>120:
                os.killpg(proc.pid,signal.SIGKILL)
                _,status,usage=os.wait4(proc.pid,0)
                break
            time.sleep(0.05)
    wall=time.perf_counter()-start
    exit_code=os.waitstatus_to_exitcode(status)
    cap_hit='wall_120s' if wall>120 else None
    stdout=stdout_path.read_bytes()
    stderr=stderr_path.read_bytes()
    try:
        report=json.loads(stdout)
        rows=report.get('rows') or []
        row=rows[0] if len(rows)==1 else {}
        fb=row.get('factor_base') or {}
        summary={'parsed':True,'operation':report.get('operation'),
                 'prime_workload_sha256':report.get('prime_workload_sha256'),
                 'status':report.get('status'),'rows':len(rows),
                 'factor_base_abscissae':fb.get('abscissae'),
                 'factor_base_native':(fb.get('cost') or {}).get('native'),
                 'total_gae':row.get('total_gae'),'verified':row.get('verified'),
                 'recovered':row.get('recovered'),
                 'cost_qualification':row.get('cost_qualification')}
    except (ValueError,UnicodeDecodeError):
        summary={'parsed':False}
    return {'exit_code':exit_code,'cap_hit':cap_hit,'wall_s':wall,
            'child_cpu_s':usage.ru_utime+usage.ru_stime,
            'peak_rss_kib':usage.ru_maxrss//1024 if sys.platform=='darwin' else usage.ru_maxrss,
            'pre_loadavg':pre_load,'post_loadavg':os.getloadavg(),
            'stdout':{'path':str(stdout_path),'sha256':sha(stdout),'bytes':len(stdout)},
            'stderr':{'path':str(stderr_path),'sha256':sha(stderr),'bytes':len(stderr)},
            'summary':summary,'timing_qualification':'diagnostic_contended'}


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--checkout',type=Path,default=Path('/Volumes/SSD990-2/pkm-equalsize-full-20261128'))
    p.add_argument('--bin',type=Path,default=Path('/Volumes/SSD990/pkm-z2xz8-target-20261128/release/ic'))
    p.add_argument('--build',type=Path,default=Path('/Volumes/SSD990-2/pkm-equalsize-build-20261128/BUILD.json'))
    p.add_argument('--manifest',type=Path,default=HERE/'z2xz8_holdout/manifest.json')
    p.add_argument('--store-dir',type=Path,default=Path('/Volumes/SSD990-2/pkm-z2xz8-holdout-preflight'))
    p.add_argument('--max-new',type=int,default=0)
    args=p.parse_args()
    if args.max_new<0:p.error('--max-new must be nonnegative')
    source=subprocess.check_output(['git','-C',str(args.checkout),'rev-parse','HEAD'],text=True).strip()
    dirty=subprocess.check_output(['git','-C',str(args.checkout),'status','--porcelain=v1','--untracked-files=no'],text=True).strip()
    if source!=SOURCE or dirty or subprocess.check_output(['git','-C',str(args.checkout),'config','--get','core.sparseCheckout'],text=True).strip()!='false':
        p.error('source checkout differs or is not clean/full')
    if sha(args.bin.read_bytes())!=BINARY_SHA or sha(args.build.read_bytes())!=BUILD_SHA:
        p.error('binary or build receipt differs')
    build=json.loads(args.build.read_bytes())
    if (build['source_commit']!=SOURCE or build['binary_sha256']!=BINARY_SHA
            or build['cargo_exit_code']!=0 or not build['full_checkout']):
        p.error('build receipt identity differs')
    if sha(args.manifest.read_bytes())!=MANIFEST_SHA or sha((HERE/'PROTOCOL_Z2XZ8_EQUAL_SIZE_COLD_20261128.md').read_bytes())!=PROTOCOL_SHA:
        p.error('manifest or protocol differs')
    subprocess.run([sys.executable,str(HERE/'verify_z2xz8_holdout.py'),'--dir',str(args.manifest.parent)],check=True,capture_output=True)
    manifest=json.loads(args.manifest.read_bytes())
    args.store_dir.mkdir(parents=True,exist_ok=True)
    store=args.store_dir/'receipts.jsonl'
    prior=records(store)
    header={'id':'header','schema':'pkm-z2xz8-holdout-map-preflight/v1',
            'source_commit':SOURCE,'binary_sha256':BINARY_SHA,'build_receipt_sha256':BUILD_SHA,
            'manifest_sha256':MANIFEST_SHA,'protocol_sha256':PROTOCOL_SHA,
            'runner_sha256':sha(Path(__file__).read_bytes()),
            'factor_base':FACTOR_BASE,'oracle':'mitm:m=2',
            'max_trials':1,'rho_runs':0,'timing_qualification':'diagnostic_contended'}
    if not prior:append(store,header);prior['header']=header
    elif prior.get('header')!=header:raise ValueError('store header differs')
    new=0
    for e in manifest['entries']:
        name=f"b{e['bits']}_c{e['curve_index']}"
        rid='result:'+name
        if rid in prior:continue
        if 'launch:'+name in prior:raise RuntimeError(f'unresolved launch: {name}')
        if args.max_new and new>=args.max_new:break
        raw=(args.manifest.parent/e['path']).read_bytes()
        if sha(raw)!=e['sha256']:raise ValueError(f'workload changed: {name}')
        full=json.loads(raw)
        assert full['targets'][0]['scalar']==e['target_scalars'][0]
        one=dict(full)
        one['targets']=[full['targets'][0]]
        one['sampling']+= '; diagnostic target_index=0'
        input_path=args.store_dir/'inputs'/f'{name}.json'
        input_path.parent.mkdir(exist_ok=True)
        one_bytes=encoded(one)
        preserve(input_path,one_bytes)
        roster=json.loads((args.manifest.parent/'roster.json').read_bytes())
        row=next(c for c in roster['curves'] if (c['bits'],c['curve_index'])==(e['bits'],e['curve_index']))
        seed=int.from_bytes(bytes.fromhex(row['targets'][0]['relation_seed_sha256'])[:8],'little')
        command=[str(args.bin),'bench','--prime-workload',str(input_path),
                 '--factor-base',FACTOR_BASE,'--oracle','mitm:m=2','--targets','walk',
                 '--linalg','incremental-gauss-full-rank-checked','--max-trials','1',
                 '--solver-budget-seconds','5','--rho-runs','0','--seed',str(seed),'--json']
        stdout=args.store_dir/'raw'/f'{name}.stdout.json'
        stderr=args.store_dir/'raw'/f'{name}.stderr.txt'
        stdout.parent.mkdir(exist_ok=True)
        launch={'id':'launch:'+name,'command':command,'input_sha256':sha(one_bytes),
                'target_scalar':one['targets'][0]['scalar'],'stdout_path':str(stdout),
                'stderr_path':str(stderr)}
        append(store,launch)
        prior[launch['id']]=launch
        result={'id':rid,**run(command,stdout,stderr)}
        append(store,result)
        prior[rid]=result
        print(json.dumps({'curve':name,'exit':result['exit_code'],'cap':result['cap_hit'],
                          'abscissae':result['summary'].get('factor_base_abscissae'),
                          'wall_s':result['wall_s']},sort_keys=True),flush=True)
        new+=1


if __name__=='__main__':
    main()
