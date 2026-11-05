"""One-owner entry point. Requires frozen, committed and published admission.

No retry/resume path exists. Producer, analyzer and export share one deadline.
"""
from pathlib import Path
import argparse
import json
import os
import resource
import subprocess
import sys
import time
from admission import published, safe_relative, sha
from bounded_process import run

D = Path(__file__).resolve().parent
ROOT = D.parent
WALL = 3600
MEMORY = 8*1024**3


def control(name, value):
    data = (json.dumps(value, indent=2) + '\n').encode()
    if len(data) > 65536: raise ValueError('control receipt exceeds 64 KiB')
    with (D/name).open('xb') as f: f.write(data)


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--execute', action='store_true')
    ap.add_argument('--publication', type=Path, required=True)
    args = ap.parse_args()
    if not __debug__: raise SystemExit('Assertions must remain enabled')
    started, epoch = time.monotonic(), time.time()
    deadline = started + WALL
    cpu = min(os.sched_getaffinity(0))
    os.sched_setaffinity(0, {cpu})
    resource.setrlimit(resource.RLIMIT_AS, (MEMORY, MEMORY))
    resource.setrlimit(resource.RLIMIT_CPU, (WALL, WALL))
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    os.environ['PYTHONDONTWRITEBYTECODE'] = '1'
    if not sys.dont_write_bytecode:
        raise SystemExit('Use python -B to suppress bytecode output from the owner')
    if (D/'launch.json').exists() or (D/'runs').exists():
        raise SystemExit('Existing owner/output is preserved; no restart allowed')
    old = ROOT/'companion-fused-validation'
    for name in ['exit-code.txt', 'postprocessor-exit.txt']:
        if (old/name).read_text().strip() != '124':
            raise SystemExit('Reconcile the original terminal invocation first')
    for file, key in [('launch.json','producer_pid'),('postprocess-launch.json','postprocessor_pid')]:
        receipt = json.loads((old/file).read_text())
        stat = Path('/proc')/str(receipt[key])/'stat'
        if stat.exists() and stat.read_text().split()[21] == receipt['start_ticks']:
            raise SystemExit('Original owner is still live')
    pbytes = (D/'protocol.json').read_bytes()
    protocol = json.loads(pbytes)
    sys.path.insert(0, str(ROOT/'tools'))
    from experiment_protocol import protocol_hash, seeds_table
    if protocol['freeze']['status'] != 'frozen' or protocol_hash(protocol) != protocol['freeze']['protocol_sha256']:
        raise SystemExit('Exact frozen protocol required')
    if protocol['execution']['timeout_seconds'] != WALL or protocol['execution']['retries']['max_attempts'] != 1:
        raise SystemExit('Execution budget differs from launcher')
    plan = json.loads((D/'plan.json').read_text())
    expected = [dict(arm=a, instance=str(i), replicate=0, seed=z['seed'])
                for a in ['candidate','aa'] for i,z in enumerate(plan)]
    if len(expected) != 1698 or seeds_table(protocol) != expected:
        raise SystemExit('Full cohort seed schedule required')
    sources = json.loads((D/'source-lock.json').read_text())
    inputs = json.loads((D/'input-lock.json').read_text())
    for rel, expected_hash in {**sources, **inputs}.items():
        safe_relative(rel)
        if sha(ROOT/rel) != expected_hash: raise SystemExit('Lock mismatch: ' + rel)
    required = [str(p.relative_to(ROOT)) for p in D.glob('*.py')]
    if not set(required) <= set(sources): raise SystemExit('Every local Python source must be locked')
    tracked = subprocess.check_output(['git','ls-files','--',*sources], cwd=ROOT, text=True).splitlines()
    if set(tracked) != set(sources): raise SystemExit('All locked source must be committed')
    subprocess.run(['git','diff','--exit-code','HEAD','--',*sources], cwd=ROOT, check=True)
    publication = published(json.loads(args.publication.read_text()), sources, inputs,
                            pbytes, ROOT, deadline)
    if not args.execute:
        print(json.dumps(dict(status='admission_verified_no_execution', publication=publication)))
        return
    control('launch.json', dict(owner_pid=os.getpid(), owner_start_ticks=Path('/proc/self/stat').read_text().split()[21],
            start_epoch=epoch, deadline_epoch=epoch+WALL, publication=publication,
            protocol_sha256=protocol['freeze']['protocol_sha256'],cpu=cpu,
            per_process_address_space_bytes=MEMORY, total_output_bytes=4*1024**3,
            command=[v.decode() for v in Path('/proc/self/cmdline').read_bytes().split(b'\0')[:-1]],
            attempts=1, restart_allowed=False))
    py = sys.executable
    steps = [('producer',[py,str(D/'check.py')],8*1024**2),
             ('analysis',[py,str(D/'analyze.py')],4*1024**2),
             ('export-check',[py,str(ROOT/'tools/experiment_protocol.py'),'check-export',
              str(D/'runs/export.json'),'--protocol',str(D/'protocol.json'),'--verify-files'],4*1024**2)]
    results = []
    for name, argv, log_limit in steps:
        remaining = deadline-time.monotonic()
        if remaining <= 0:
            results.append(dict(stage=name,exit_code=124,stop_reason='shared_deadline_before_stage'))
            break
        def on_start(pid, name=name):
            control(name+'-start.json', dict(pid=pid,start_ticks=Path('/proc',str(pid),'stat').read_text().split()[21],
                    deadline_epoch=epoch+WALL,command=argv))
        try:
            receipt = run(argv, ROOT, D/(name+'.log'), wall_seconds=remaining,
                          memory_bytes=MEMORY, log_bytes=log_limit, cpu=cpu, on_start=on_start)
        except Exception as error:
            receipt = dict(exit_code=1, stop_reason='supervisor_error',
                           error_type=type(error).__name__, error=str(error)[:2000], retries=0)
        receipt['stage'] = name
        results.append(receipt)
        control(name+'-exit.json', receipt)
        if receipt['exit_code'] != 0: break
    success = len(results)==3 and all(r['exit_code']==0 for r in results)
    control('terminal.json',dict(status='complete' if success else 'failed_or_capped',
            results=results, restarted=False, deadline_epoch=epoch+WALL))
    if not success: raise SystemExit(1)


if __name__ == '__main__': main()
