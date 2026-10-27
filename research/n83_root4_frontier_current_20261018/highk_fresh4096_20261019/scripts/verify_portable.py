#!/usr/bin/env python3
"""Independently replay 4,096-point N83 wide3 panel from committed gzip targets and stored bases."""
import argparse
import gzip
import hashlib
import importlib.util
import json
import math
from pathlib import Path

ARMS=(4096,4300,4405)

def sha(data):return hashlib.sha256(data).hexdigest()
def one(path):
    rows=[json.loads(line) for line in path.read_text().splitlines()]
    if len(rows)!=1:raise ValueError(f'expected one row: {path}')
    return rows[0]
def lines(path):return [json.loads(line) for line in path.read_text().splitlines()]
def canonical(v):return json.dumps(v,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()

def main():
    p=argparse.ArgumentParser()
    p.add_argument('--repo',type=Path,required=True)
    p.add_argument('--evidence',type=Path,default=Path(__file__).resolve().parent.parent)
    p.add_argument('--base-dir',type=Path,required=True)
    p.add_argument('--output',type=Path)
    args=p.parse_args();repo=args.repo.resolve();e=args.evidence.resolve();bases=args.base_dir.resolve()
    fixture=json.loads((e/'fixture_manifest.json').read_text())
    tpath=e/'targets.jsonl';apath=e/'answers.jsonl'
    if (sha(tpath.read_bytes())!=fixture['targets_sha256'] or sha(apath.read_bytes())!=fixture['answers_sha256']
            or fixture['point_count']!=4096):raise ValueError('fixture digest or count differs')
    group_path=repo/'research/n83_indexed5_20261013/analyze.py'
    if sha(group_path.read_bytes())!=fixture['group_source_sha256']:raise ValueError('group source differs')
    spec=importlib.util.spec_from_file_location('n83_fresh_replay_group',group_path)
    group=importlib.util.module_from_spec(spec);spec.loader.exec_module(group)
    public=lines(tpath);answers=lines(apath)
    if len(public)!=4096 or len(answers)!=4096:raise ValueError('fixture row count differs')
    points=[];scalars=[];seen=set()
    for i,(row,ans) in enumerate(zip(public,answers)):
        scalar=int(ans['scalar']);point=group.point_from_json(row)
        seed=hashlib.sha256(fixture['domain'].encode()+i.to_bytes(4,'little')).digest()
        expected=1+int.from_bytes(seed,'little')%(group.R-1)
        if (ans!={'fixture_index':i,'scalar':str(expected),'seed_sha256':seed.hex()}
                or scalar!=expected or point in seen or not group.on_curve(point)
                or group.multiply(point,group.R) is not None or group.multiply(group.G,scalar)!=point):
            raise ValueError(f'fixture differs at {i}')
        seen.add(point);points.append(point);scalars.append(scalar)
    source_sha=sha((repo/'examples/koblitz_orbit_dlp_fast.rs').read_bytes())
    manifest=json.loads((repo/'research/n83_root4_frontier_current_20261018/highk_20261019/store_blobs_manifest.json').read_text())
    stored={row['k']:row for row in manifest}
    archive=json.loads((e/'target_blobs_manifest.json').read_text())
    target_blobs={row['k']:row for row in archive}
    out={'schema':'n83.wide3-fresh4096-portable-replay/v1','status':'PASS_INDEPENDENT_REPLAY','source_sha256':source_sha,'fixture_sha256':fixture['targets_sha256'],'arms':{},'selected_online_candidate_id':None}
    for k in ARMS:
        arm=e/'runs'/f'k{k}_wide3';base_record=stored[k]
        compressed=(bases/f'k{k}_base_dump.jsonl.gz').read_bytes()
        if sha(compressed)!=base_record['gzip_sha256'] or len(compressed)!=base_record['gzip_bytes']:
            raise ValueError(f'compressed base differs: K{k}')
        raw=gzip.decompress(compressed)
        if sha(raw)!=base_record['raw_sha256'] or len(raw)!=base_record['raw_bytes']:
            raise ValueError(f'raw base differs: K{k}')
        summary=one(arm/'stdout.jsonl');src=json.loads((arm/'source.json').read_text());usage=json.loads((arm/'usage.json').read_text());analysis=json.loads((arm/'analysis.json').read_text())
        protocol=json.loads((e/'protocols'/f'k{k}_wide3_fresh4096_protocol.json').read_text())
        if (src['source_sha256']!=source_sha or src['target_sha256']!=fixture['targets_sha256']
                or src['binary_sha256']!=protocol['binary_sha256']
                or src['runner_sha256']!=protocol['pilot_runner_sha256']
                or src['algorithm_flags']['KIC_ROOT_BLOOM_PROFILE']!='wide3'
                or src['algorithm_flags']['KIC_INDEXED5_ID30']!='1'
                or usage['status']!='PASS_PRODUCER' or usage['exit_code']!=0
                or usage['peak_sampled_rss_bytes']>protocol['rss_cap_bytes']
                or usage['wall_seconds']>protocol['cap_seconds']+5
                or analysis['status']!='PASS_INDEPENDENT_REPLAY'
                or analysis['base_dump_sha256']!=base_record['raw_sha256']
                or summary['factor_base_points']!=166*k or summary['rank']!=k
                or summary['root_bloom_profile']!='wide3' or summary['root_bloom_bytes']!=2*(1<<30)
                or summary['targets_solved']!=4096 or summary['targets_failed']!=0):
            raise ValueError(f'run admission differs: K{k}')
        base=group.replay_base(json.loads(raw),summary)
        if len(base)!=166*k:raise ValueError(f'base count differs: K{k}')
        gz=(arm/'targets_result.jsonl.gz').read_bytes();blob=target_blobs[k]
        if sha(gz)!=blob['gzip_sha256'] or len(gz)!=blob['gzip_bytes']:raise ValueError(f'target gzip differs: K{k}')
        results=gzip.decompress(gz)
        if sha(results)!=blob['raw_sha256'] or len(results)!=blob['raw_bytes']:raise ValueError(f'target raw differs: K{k}')
        rows=[json.loads(line) for line in results.splitlines()]
        by_index={r['fixture_index']:r for r in rows}
        if len(rows)!=4096 or len(by_index)!=4096 or set(by_index)!=set(range(4096)):
            raise ValueError(f'target indices differ: K{k}')
        online=[]
        for i in range(4096):
            r=by_index[i];indices=r['point_indices'];scalar=int(r['recovered_scalar'])
            if (r['group_verified'] is not True or group.point_from_json(r['target'])!=points[i]
                    or scalar!=scalars[i] or group.multiply(group.G,scalar)!=points[i]
                    or len(indices)!=4 or any(not 0<=v<len(base) for v in indices)):
                raise ValueError(f'target differs: K{k}, {i}')
            relation=None
            for v in indices:relation=group.add(relation,base[v])
            if relation!=points[i]:raise ValueError(f'relation differs: K{k}, {i}')
            phases=[float(r[f]) for f in ('target_query_ms','target_pdp_ms','target_relation_check_ms','target_descent_ms','target_recovery_check_ms')]
            value=float(r['target_ms'])
            if (not math.isfinite(value) or value<=0 or any(not math.isfinite(x) or x<0 for x in phases)
                    or abs(value-sum(phases))>1):raise ValueError(f'phase sum differs: K{k}, {i}')
            online.append(value)
        mean=sum(online)/4096
        if abs(mean-analysis['mean_online_ms_diagnostic'])>1e-9 or online!=analysis['online_ms_diagnostic']:
            raise ValueError(f'analysis timing differs: K{k}')
        out['arms'][str(k)]={'status':'PASS_INDEPENDENT_REPLAY','verified_targets':4096,'base_points':len(base),'mean_target_ms':mean,'base_raw_sha256':sha(raw),'target_gzip_sha256':sha(gz),'analysis_sha256':sha((arm/'analysis.json').read_bytes())}
    if args.output:args.output.write_text(json.dumps(out,sort_keys=True,indent=2)+'\n')
    print(json.dumps(out,sort_keys=True))
if __name__=='__main__':main()
