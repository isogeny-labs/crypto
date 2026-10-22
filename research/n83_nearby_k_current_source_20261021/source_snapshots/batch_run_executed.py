#!/usr/bin/env python3
"""Secondary deterministic relation-probe distribution across 16 prior public points."""
import json
import os
import subprocess
import time
from datetime import datetime, timezone
from pathlib import Path
from research.n83_highk_beyond4405_20261021 import run_k1107_base_stage as base
from research.n83_indexed5_20261013 import analyze as group
from research.n83_online_finalists_20261018.run import ic_environment

ROOT = Path(__file__).resolve().parent
BINARY = Path('/Volumes/SSD990-2/.codex/n83-fresh-cold-target/release/examples/koblitz_orbit_dlp_fast')
CAP = 1200
ORDER = (1120, 1104, 1107)
PUBLIC = base.ROOT/'research/n83_highk_beyond4405_20261021/screen_targets.jsonl'
ANSWERS = base.ROOT/'research/n83_highk_beyond4405_20261021/screen_answers.jsonl'
assert base.sha256(base.SOURCE)==base.SOURCE_SHA256
assert base.sha256(BINARY)=='07f7b5b424d0f1ecea1cd7e6564522c4d9c64557a820441c381e7474b13a388a'
points=[json.loads(line) for line in PUBLIC.read_text().splitlines()[:16]]
answers=[json.loads(line) for line in ANSWERS.read_text().splitlines()[:16]]
assert len(points)==len(answers)==16 and all(
    a['fixture_index']==i and group.multiply(group.G,int(a['scalar']))==
    group.point_from_json(p) for i,(p,a) in enumerate(zip(points,answers)))
(ROOT/'campaign.json').write_text(json.dumps({'schema':'n83.nearby-k-batch16-probe-diagnostic/v1','question':'Does K1120 reduce deterministic relation probes across 16 prior public targets after reusable factor-log setup?','tier':'secondary shared-host probe diagnostic; not a single-target or Tier 1 speed result','target_count':16,'k_order':ORDER,'source_sha256':base.SOURCE_SHA256,'binary_sha256':base.sha256(BINARY),'public_targets_sha256':base.sha256(PUBLIC),'answer_sidecar_sha256':base.sha256(ANSWERS),'cap_seconds_per_child':CAP},indent=2,sort_keys=True)+'\n')
for k in ORDER:
 out=ROOT/f'k{k}';out.mkdir(exist_ok=False)
 target=out/'targets.jsonl';target.write_text(''.join(json.dumps(point)+'\n' for point in points))
 flags={name:value for name,value in ic_environment(out,'root11_dynamic','k1107_root11').items() if name.startswith('KIC_')}
 env={name:value for name,value in os.environ.items() if not name.startswith('KIC_')};env.update(flags)
 command=[str(BINARY),f'construct:83:1:{k}',str(target),'29',str(out/'targets_result.jsonl')]
 started=datetime.now(timezone.utc).isoformat()
 launch={'schema':'n83.nearby-k-batch16-launch/v1','k':k,'command':command,'flags':flags,'target_sha256':base.sha256(target),'source_sha256':base.SOURCE_SHA256,'binary_sha256':base.sha256(BINARY),'started_utc':started,'cap_seconds':CAP}
 (out/'launch.json').write_text(json.dumps(launch,indent=2,sort_keys=True)+'\n')
 began=time.monotonic()
 with (out/'stdout.jsonl').open('wb') as stdout,(out/'stderr.log').open('wb') as stderr:
  try:
   proc=subprocess.run(command,cwd=base.ROOT,env=env,stdout=stdout,stderr=stderr,timeout=CAP,check=False)
   status='PROCESS_EXITED' if proc.returncode==0 else 'PROCESS_FAILED';code=proc.returncode
  except subprocess.TimeoutExpired:
   status='TIMEOUT';code=None
 receipt={'schema':'n83.nearby-k-batch16-receipt/v1','status':status,'exit_code':code,'wall_seconds':time.monotonic()-began,'started_utc':started,'stopped_utc':datetime.now(timezone.utc).isoformat(),'stdout_sha256':base.sha256(out/'stdout.jsonl'),'stderr_sha256':base.sha256(out/'stderr.log'),'base_dump_sha256':base.sha256(out/'base_dump.jsonl') if (out/'base_dump.jsonl').exists() else None,'target_result_sha256':base.sha256(out/'targets_result.jsonl') if (out/'targets_result.jsonl').exists() else None}
 (out/'receipt.json').write_text(json.dumps(receipt,indent=2,sort_keys=True)+'\n')
 print(json.dumps({'k':k,'status':status,'wall_seconds':receipt['wall_seconds']},sort_keys=True),flush=True)
 if status!='PROCESS_EXITED':break
