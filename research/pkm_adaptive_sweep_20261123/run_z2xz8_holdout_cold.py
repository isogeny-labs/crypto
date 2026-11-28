#!/usr/bin/env python3
"""Resume the frozen six-curve, 96-target cold X1(2,8) comparison."""

import argparse
import datetime as dt
import fcntl
import hashlib
import importlib.util
import json
import os
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = 'a0642f42e205aba4951dcf9732721d62211d7c33'
BINARY_SHA = '8e5ba8c40647a076310a2f4205774caaa6925e7eadda6ca961fe8e4590a7a027'
BUILD_SHA = 'e09abcf0d0ff07d82f1392bca04e3abb159fea853539d8a328d40b4a5f8c63b7'
MANIFEST_SHA = '99140281d9a94c288b8993b503cf91e4226879052e25c63dfdad0fccd20c8bc8'
PROTOCOL_SHA = '723ac30ff07386b4ad1769ce89d353229d8ba9636b5c0582813a3c7986378a3d'
PREFLIGHT_SHA = '4e849aebeb3da8047b7d69fa4c953483dd7d3bb6a7d45659f2971abd0a195cea'
ARMS = ('interval_a','candidate_a','candidate_b','interval_b','mitm','kummer')
SELECTOR = 'prime-isogeny:t=5,seed=0,aux=ordinary,family=z2xz8,selection=equal-size'
F4_M2 = 'pkm-semaev:m=2,engine=f4,degree=8,mode=chain'
F4_INTERVAL = 'pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=0,mode=chain'
F4_M3 = 'pkm-semaev:m=3,mode=chain,engine=f4,descent=1,grid_crossover=0,trace=0,guess=0,degree=12,max_nnz=1000000'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    return json.loads(path.read_bytes())


def encoded(value):
    return (json.dumps(value,sort_keys=True,indent=2)+'\n').encode()


def load_helper(checkout):
    path=checkout/'research/pkm_nonenum_decomp_20261125/run_cold_panel.py'
    spec=importlib.util.spec_from_file_location('pkm_holdout_cold_helper',path)
    module=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    module.WALL_CAP_S=120
    module.RSS_CAP_KIB=2*1024*1024
    return module,sha(path.read_bytes())


def check_inputs(args):
    checkout=args.checkout.resolve(strict=True)
    binary=args.bin.resolve(strict=True)
    source=subprocess.check_output(['git','-C',str(checkout),'rev-parse','HEAD'],text=True).strip()
    dirty=subprocess.check_output(['git','-C',str(checkout),'status','--porcelain=v1','--untracked-files=no'],text=True).strip()
    sparse=subprocess.check_output(['git','-C',str(checkout),'config','--get','core.sparseCheckout'],text=True).strip()
    if source!=SOURCE or dirty or sparse!='false':
        raise ValueError('source checkout differs or is not clean/full')
    if (sha(binary.read_bytes())!=BINARY_SHA
            or sha(args.build.read_bytes())!=BUILD_SHA
            or sha(args.manifest.read_bytes())!=MANIFEST_SHA
            or sha((HERE/'PROTOCOL_Z2XZ8_EQUAL_SIZE_COLD_20261128.md').read_bytes())!=PROTOCOL_SHA
            or sha(args.preflight.read_bytes())!=PREFLIGHT_SHA):
        raise ValueError('binary, build, manifest, protocol or preflight hash differs')
    build=read(args.build)
    if (build['source_commit']!=SOURCE or build['binary_sha256']!=BINARY_SHA
            or not build['full_checkout'] or not build['tracked_checkout_clean']
            or build['cargo_exit_code']!=0):
        raise ValueError('build receipt differs')
    subprocess.run([sys.executable,str(HERE/'verify_z2xz8_holdout.py'),
                    '--dir',str(args.manifest.parent)],check=True,capture_output=True)
    subprocess.run([sys.executable,str(HERE/'verify_z2xz8_holdout_preflight.py'),
                    '--store-dir',str(args.preflight.parent),'--checkout',str(checkout),
                    '--bin',str(binary),'--build',str(args.build)],
                   check=True,capture_output=True)
    return checkout,binary


def arm_command(binary,workload,arm,relation_seed):
    fb=('prime-abscissa:below=32' if arm.startswith('interval') else
        'prime-kummer:t=5,g=1' if arm=='kummer' else SELECTOR)
    oracle=(F4_INTERVAL if arm.startswith('interval') else
            F4_M3 if arm=='kummer' else 'mitm:m=2' if arm=='mitm' else F4_M2)
    max_trials=2_000_000 if arm=='kummer' else 20_000
    return [str(binary),'bench','--prime-workload',str(workload),
            '--factor-base',fb,'--oracle',oracle,'--targets','walk',
            '--linalg','incremental-gauss-full-rank-checked',
            '--audit-rank',
            '--max-trials',str(max_trials),'--solver-budget-seconds','5',
            '--rho-runs','1','--seed',str(relation_seed),'--json']


def positive_unpriced(report):
    found={}
    def visit(node):
        if isinstance(node,dict):
            for key,value in node.items():
                if key.endswith('_uncharged') and isinstance(value,(int,float)) and value>0:
                    found[key]=max(found.get(key,0),value)
                visit(value)
        elif isinstance(node,list):
            for value in node:visit(value)
    visit(report)
    rows=report.get('rows') or []
    if len(rows)==1:
        fb=(rows[0].get('factor_base') or {}).get('cost') or {}
        native=fb.get('native') or {}
        for key in ('aux_selection_sqrt_muls','aux_selection_target_legendre'):
            if native.get(key,0)>0:found[key]=native[key]
        for phase in (fb,(rows[0].get('decomposition') or {}).get('setup') or {},
                      (rows[0].get('decomposition') or {}).get('cost') or {},
                      (rows[0].get('linear_algebra') or {}).get('cost') or {}):
            for key,value in (phase.get('work_native') or {}).items():
                if value>0 and not key.startswith(('counted:','tallied:')):
                    found['work_native:'+key]=value
    return found


def summary(path,workload_sha,scalar,r,arm,helper):
    result=helper.report_summary(path,workload_sha,scalar,r,arm)
    try:report=read(path)
    except (OSError,ValueError):return result
    result['unpriced_native']=positive_unpriced(report)
    if arm.startswith('candidate') or arm=='mitm':
        # There is no complete coefficient/Jacobi/Vélu price in this binary.
        result['known_missing_price']=['selector_coefficient_arithmetic',
                                       'selector_jacobi_arithmetic',
                                       'selector_velu_evaluations']
    else:
        result['known_missing_price']=[]
    result['reported_gae_lower_bound']=result.get('total_gae')
    result['complete_cold_gae']=None
    return result


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--checkout',type=Path,default=Path('/Volumes/SSD990-2/pkm-equalsize-full-20261128'))
    parser.add_argument('--bin',type=Path,default=Path('/Volumes/SSD990/pkm-z2xz8-target-20261128/release/ic'))
    parser.add_argument('--build',type=Path,default=Path('/Volumes/SSD990-2/pkm-equalsize-build-20261128/BUILD.json'))
    parser.add_argument('--manifest',type=Path,default=HERE/'z2xz8_holdout/manifest.json')
    parser.add_argument('--preflight',type=Path,default=Path('/Volumes/SSD990-2/pkm-z2xz8-holdout-preflight/receipts.jsonl'))
    parser.add_argument('--store-dir',type=Path,default=Path('/Volumes/SSD990-2/pkm-z2xz8-holdout-cold-20261128'))
    parser.add_argument('--max-new',type=int,default=0)
    parser.add_argument('--dry-run',action='store_true')
    args=parser.parse_args()
    if args.max_new<0:parser.error('--max-new must be nonnegative')
    checkout,binary=check_inputs(args)
    helper,helper_sha=load_helper(checkout)
    manifest=read(args.manifest)
    roster=read(args.manifest.parent/'roster.json')
    if len(manifest['entries'])!=6 or len(roster['curves'])!=6:
        raise ValueError('six-curve holdout differs')
    cases=[f"b{e['bits']}_c{e['curve_index']}_q{j}:{arm}"
           for e in manifest['entries'] for j in range(16) for arm in ARMS]
    if args.dry_run:
        print(json.dumps({'schema':'pkm-z2xz8-cold-plan/v1','source':SOURCE,
                          'binary_sha256':BINARY_SHA,'manifest_sha256':MANIFEST_SHA,
                          'arms':ARMS,'cases':len(cases),'first':cases[0],
                          'last':cases[-1],'helper_sha256':helper_sha},sort_keys=True))
        return
    args.store_dir.mkdir(parents=True,exist_ok=True)
    store=args.store_dir/'receipts.jsonl'
    lock=(args.store_dir/'receipts.lock').open('a+')
    fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
    header={'id':'header','schema':'pkm-z2xz8-heldout-cold/v1',
            'source_commit':SOURCE,'binary_sha256':BINARY_SHA,
            'build_receipt_sha256':BUILD_SHA,'manifest_sha256':MANIFEST_SHA,
            'protocol_sha256':PROTOCOL_SHA,'preflight_sha256':PREFLIGHT_SHA,
            'runner_sha256':sha(Path(__file__).read_bytes()),'helper_sha256':helper_sha,
            'arms':ARMS,'cases':len(cases),'wall_cap_s':helper.WALL_CAP_S,
            'rss_cap_kib':helper.RSS_CAP_KIB,
            'created_utc':dt.datetime.now(dt.timezone.utc).isoformat()}
    prior=helper.records(store)
    if not prior:
        helper.append(store,header)
        prior['header']=header
    elif {k:v for k,v in prior['header'].items() if k!='created_utc'}!={k:v for k,v in header.items() if k!='created_utc'}:
        raise ValueError('cold store header differs')
    new=0
    for e in manifest['entries']:
        name=f"b{e['bits']}_c{e['curve_index']}"
        full_path=args.manifest.parent/e['path']
        if sha(full_path.read_bytes())!=e['sha256']:
            raise ValueError(f'frozen workload changed: {name}')
        full=read(full_path)
        roster_curve=next(c for c in roster['curves'] if (c['bits'],c['curve_index'])==(e['bits'],e['curve_index']))
        for j,target in enumerate(full['targets']):
            if target['scalar']!=roster_curve['targets'][j]['scalar'] or target['point']!=roster_curve['targets'][j]['point']:
                raise ValueError(f'target differs: {name} q{j}')
            single=dict(full)
            single['targets']=[target]
            single['sampling']+=f'; cold target_index={j}'
            input_path=args.store_dir/'inputs'/f'{name}_q{j}.json'
            helper.preserve(input_path,encoded(single))
            input_sha=sha(input_path.read_bytes())
            seed=int.from_bytes(bytes.fromhex(roster_curve['targets'][j]['relation_seed_sha256'])[:8],'little')
            for arm in ARMS:
                case=f'{name}_q{j}:{arm}'
                rid='result:'+case
                if rid in prior:continue
                if 'launch:'+case in prior:raise RuntimeError(f'unresolved launch: {case}')
                if args.max_new and new>=args.max_new:return
                check=helper.quiet()
                check['quiet']=not check['competing_processes'] and check['loadavg'][0]<=check['logical_cpus']
                load_id=f'load:{case}:{1+sum(k.startswith("load:"+case+":") for k in prior)}'
                load={'id':load_id,**check}
                helper.append(store,load)
                prior[load_id]=load
                if not check['quiet']:
                    print(json.dumps({'waiting_for_quiet_host':case,'load_id':load_id},sort_keys=True))
                    return
                command=arm_command(binary,input_path,arm,seed)
                raw=args.store_dir/'raw'
                raw.mkdir(exist_ok=True)
                out=raw/(case.replace(':','_')+'.stdout.json')
                err=raw/(case.replace(':','_')+'.stderr.txt')
                launch={'id':'launch:'+case,'command':command,'workload_sha256':input_sha,
                        'target_scalar':target['scalar'],'relation_seed':seed,
                        'arm':arm,'stdout_path':str(out),
                        'stderr_path':str(err),'load_id':load_id}
                helper.append(store,launch)
                prior[launch['id']]=launch
                result=helper.run_one(command,out,err)
                post=helper.quiet()
                result['postcall_load']=post
                result['postcall_busy']=bool(post['competing_processes'] or post['loadavg'][0]>post['logical_cpus'])
                result['summary']=summary(out,input_sha,target['scalar'],e['r'],arm,helper)
                result['outcome']=('censored' if result['cap_hit'] or result['load_contamination']
                                   or result['postcall_busy'] else
                                   'parsed' if result['exit_code']==0 and result['summary'].get('identity_match')
                                   else 'failed')
                result['id']=rid
                helper.append(store,result)
                prior[rid]=result
                new+=1
                print(json.dumps({'case':case,'exit':result['exit_code'],
                                  'cap':result['cap_hit'],'status':result['summary'].get('status'),
                                  'verified':result['summary'].get('verified')},sort_keys=True),flush=True)
                if result['cap_hit'] or result['load_contamination'] or result['postcall_busy']:
                    return
    failed=[case for case in cases if not prior['result:'+case]['summary'].get('verified')]
    verdict={'id':'verdict','schema':'pkm-z2xz8-heldout-cold-verdict/v1',
             'cases':len(cases),'failed_or_incomplete':failed,
             'complete_cold_cost':False,
             'reason':'positive unpriced native field and Vélu work; no complete GAE price'}
    if 'verdict' not in prior:helper.append(store,verdict)
    elif prior['verdict']!=verdict:raise ValueError('verdict differs')


if __name__=='__main__':
    main()
