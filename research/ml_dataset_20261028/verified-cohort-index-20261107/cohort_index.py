"""Verify existing strict exports and emit provenance-only cohort index, never training rows."""
import argparse, hashlib, json, re
from pathlib import Path

def require(ok, message):
    if not ok: raise ValueError(message)

def pairs(items):
    out={}
    for k,v in items:
        require(k not in out, 'duplicate JSON key')
        out[k]=v
    return out

def parse(data):
    def bad(x): raise ValueError('nonfinite JSON number')
    return json.loads(data, object_pairs_hook=pairs, parse_constant=bad,
                      parse_float=lambda x: finite_float(x))

def finite_float(x):
    import math
    value=float(x); require(math.isfinite(value),'nonfinite JSON number'); return value

def digest(data): return hashlib.sha256(data).hexdigest()

def safe(root, name):
    require(isinstance(name,str) and name and '\\' not in name and not any(ord(c)<32 for c in name),'unsafe artifact path')
    require(not name.startswith('/') and all(c not in ('','.','..') for c in name.split('/')),'unsafe artifact path')
    p=root
    for component in name.split('/'):
        p=p/component; require(not p.is_symlink(),'symlink artifact path')
    require(p.resolve().is_relative_to(root.resolve()) and p.is_file(),'artifact escapes or is absent')
    return p

def table(rows, key):
    out={}
    for r in rows:
        require(isinstance(r,dict) and isinstance(r.get(key),str) and r[key],'missing row identity')
        require(r[key] not in out,'duplicate row ID'); out[r[key]]=r
    return out

def split(row, global_splits):
    c,s=row.get('split_component'),row.get('split')
    if c is None and s is None: return
    require(isinstance(c,str) and re.fullmatch(r'field-size-n[1-9][0-9]*',c) and s in ('train','validation','test'),'invalid split')
    expected=int(hashlib.sha256(('ml-dataset-whole-field-size-split-v1\n'+c).encode()).hexdigest()[:16],16)%100
    require(s==('train' if expected<80 else 'validation' if expected<90 else 'test'),'split conflicts with deterministic assignment')
    require(c not in global_splits or global_splits[c]==s,'cross-cohort split conflict');global_splits[c]=s

def public_split(row, global_splits, core=False):
    require(row.get('split_component') is not None and row.get('split') is not None,'public feature split required')
    inputs=row.get('inputs');require(isinstance(inputs,dict),'public feature inputs required')
    if core or 'n' in inputs:
        n=inputs.get('n');require(type(n) is int and 0<n<2**64,'public feature field size required')
        require(row['split_component']=='field-size-n'+str(n),'public feature field size split mismatch')
    split(row,global_splits)

def typed_plan_binding(plan):
    """Exact supported DatasetPlan Serde layout, Rust mod.rs:318-363, x86_64.
    Only legacy plan/v1 is supported; absent channels/policies are skipped.
    Option<String> protocol_sha256 defaults to None and serializes as null.
    """
    fields=['schema','field_sizes','arities','families','solvers','curve_seeds','generators','keys','seed','factor_base_size','degree_bound','work_limit','twists','protocol_sha256','split_rule']
    require(isinstance(plan,dict) and set(plan)<=set(fields),'unsupported typed plan fields')
    require(plan.get('schema')=='crypto.ml-dataset.plan/v1','unsupported typed plan schema')
    require(set(fields)-{'protocol_sha256'}<=set(plan),'missing typed plan field')
    typed={k:plan.get(k) for k in fields}
    for key,bits in [('curve_seeds',64),('generators',64),('keys',64),('seed',64),('factor_base_size',64),('degree_bound',32),('work_limit',64)]:
        require(type(typed[key]) is int and 0<=typed[key]<2**bits,'invalid typed plan integer')
    for key,bits in [('field_sizes',32),('arities',64)]:
        require(type(typed[key]) is list and all(type(v) is int and 0<=v<2**bits for v in typed[key]),'invalid typed plan integer array')
    for key in ['families','solvers']:
        require(type(typed[key]) is list and all(type(v) is str for v in typed[key]),'invalid typed plan string array')
    require(type(typed['twists']) is bool and type(typed['split_rule']) is str,'invalid typed plan scalar')
    require(typed['protocol_sha256'] is None or type(typed['protocol_sha256']) is str,'invalid typed plan optional digest')
    return digest(json.dumps(typed,separators=(',',':'),ensure_ascii=False,allow_nan=False).encode('utf-8'))

def verify_export(entry, global_splits):
    manifest_path=Path(entry['manifest']); require(manifest_path.is_absolute(),'manifest must be absolute')
    raw=manifest_path.read_bytes();require(digest(raw)==entry['manifest_sha256'],'manifest digest mismatch');m=parse(raw)
    require(m['schema'] in ('ml-dataset.training-export/v1','ml-dataset.training-export/v3'),'unsupported manifest schema')
    bindings={}
    require('source_bindings' in entry,'source_bindings required')
    if 'source_bindings' in entry:
        values={}
        for name in ['run','plan','protocol']:
            ref=entry['source_bindings'][name];bp=Path(ref['path']);require(bp.is_absolute(),'binding path must be absolute');data=bp.read_bytes();require(digest(data)==ref['sha256'],'source binding digest mismatch');values[name]=parse(data);bindings[name]={'path':str(bp),'sha256':digest(data)}
        require(bindings['run']['sha256']==m['source_run_sha256'],'source run digest mismatch')
        require(values['run']['source_commit']==m['source_commit'] and values['run']['binary_sha256']==m['binary_sha256'] and values['run']['plan_sha256']==m['plan_sha256'],'source provenance mismatch')
        bindings['plan']['typed_binding_sha256']=typed_plan_binding(values['plan'])
        require(bindings['plan']['typed_binding_sha256']==values['run']['plan_sha256'],'typed plan binding mismatch')
        require(values['plan']['protocol_sha256']==bindings['protocol']['sha256'],'protocol digest binding mismatch')
    policy=m['split_policy'];require(policy['domain']=='ml-dataset-whole-field-size-split-v1' and policy['row_random_split_permitted'] is False,'unsupported split policy')
    rows={};artifacts=[]
    for a in m['artifacts']:
        name=a['path'];require(name not in rows,'duplicate artifact path')
        p=safe(manifest_path.parent,name);data=p.read_bytes()
        require(type(a['bytes']) is int and len(data)==a['bytes'] and digest(data)==a['sha256'],'artifact digest or size mismatch')
        require(a.get('encoding') in ('jsonl','json'),'unsupported artifact encoding')
        parsed=([parse(line) for line in data.splitlines() if line.strip()] if a['encoding']=='jsonl' else [parse(data)])
        require(all(isinstance(r,dict) for r in parsed),'nonobject row')
        if name!='export-options.json':
            schemas={'features.jsonl':'ml-dataset.features/v1','restricted/targets.jsonl':'ml-dataset.restricted-targets/v1','restricted/index.jsonl':'ml-dataset.export-index/v1'}
            for stem in ['prefix','algebraic','boolean-equation','sat','ecdlp']:
                schemas[stem+'-features.jsonl']='ml-dataset.'+stem+'-features/v1'
                schemas['restricted/'+stem+'-records.jsonl']='ml-dataset.restricted-'+stem+'-record/v1'
            require(name in schemas and all(r.get('schema')==schemas[name] for r in parsed),'unexpected row schema')
        rows[name]=parsed;artifacts.append({'path':name,'bytes':len(data),'sha256':digest(data),'rows':len(parsed)})
    features=table(rows['features.jsonl'],'feature_id');targets=table(rows['restricted/targets.jsonl'],'target_id');index=table(rows['restricted/index.jsonl'],'record_id')
    for r in features.values(): public_split(r,global_splits,core=True)
    for r in targets.values():
        split(r,global_splits);f=r.get('feature_id')
        if f is not None:
            require(f in features,'dangling target feature join')
            require((r.get('split_component'),r.get('split'))==(features[f].get('split_component'),features[f].get('split')),'target feature split mismatch')
    indexed=set()
    for r in index.values():
        require(r.get('source_run_sha256')==m['source_run_sha256'],'source run join mismatch')
        t=r.get('target_id');f=r.get('feature_id')
        if t is None:
            require(r.get('status')=='unsealed_attempt_excluded' and f is None,'invalid unsealed index row')
        else:
            require(t in targets and t not in indexed,'dangling or duplicate index target join');indexed.add(t)
            require(r['record_id']==t and f==targets[t].get('feature_id'),'index feature join mismatch')
    require(indexed==set(targets),'target missing index join')
    supported={'features.jsonl','restricted/targets.jsonl','restricted/index.jsonl'}
    for stem in ['prefix','algebraic','boolean-equation','sat','ecdlp']:
        fp=stem+'-features.jsonl';rp='restricted/'+stem+'-records.jsonl'
        if fp not in rows and rp not in rows:continue
        require(fp in rows and rp in rows,'missing channel pair');supported.update([fp,rp]);fs=table(rows[fp],'feature_id');rs=table(rows[rp],'record_id')
        for r in fs.values():public_split(r,global_splits)
        for r in rs.values():
            split(r,global_splits);require(r.get('source_target_id') in targets,'dangling channel target join');f=r.get('ecdlp_feature_id' if stem=='ecdlp' else 'feature_id')
            if stem=='ecdlp':require(r.get('source_index_record_id') in index and index[r['source_index_record_id']].get('target_id')==r['source_target_id'],'dangling ecdlp index join')
            if f is not None:
                require(f in fs,'dangling channel feature join')
                require((r.get('split_component'),r.get('split'))==(fs[f].get('split_component'),fs[f].get('split')),'channel feature split mismatch')
    if 'export-options.json' in rows:supported.add('export-options.json')
    require(set(rows)==supported,'unknown artifact channel')
    for key,name in [('unique_feature_rows','features.jsonl'),('restricted_target_rows','restricted/targets.jsonl'),('unique_prefix_feature_rows','prefix-features.jsonl'),('restricted_prefix_record_rows','restricted/prefix-records.jsonl'),('unique_algebraic_feature_rows','algebraic-features.jsonl'),('restricted_algebraic_record_rows','restricted/algebraic-records.jsonl')]:
        if key in m:require(m[key]==len(rows.get(name,[])),'manifest row count mismatch')
    for a in artifacts:
        again=safe(manifest_path.parent,a['path']).read_bytes()
        require(len(again)==a['bytes'] and digest(again)==a['sha256'],'artifact changed during validation')
    require(manifest_path.read_bytes()==raw,'manifest changed during validation')
    # Only explicit manifest-level provenance and aggregate row counts leave this function.
    return {'manifest':str(manifest_path),'manifest_schema':m['schema'],'source_pooling_permission':m.get('feature_policy',{}).get('cross_population_pooling_permitted'),'manifest_sha256':digest(raw),'source_run_sha256':m['source_run_sha256'],'plan_sha256':m['plan_sha256'],'source_commit':m['source_commit'],'binary_sha256':m['binary_sha256'],'artifacts':artifacts,'source_bindings':bindings,'restricted_labels_copied':False,'failure_and_censor_records':'preserved unchanged in hash-bound restricted source artifacts; not promoted to labels'}

def build(spec):
    require(spec['schema']=='ml-dataset.cohort-index-input/v1','unsupported input schema')
    splits={};cohorts=[];seen=set();manifests=set()
    for c in spec['cohorts']:
        require(c['cohort_id'] not in seen,'duplicate cohort ID');seen.add(c['cohort_id']);exports=[]
        for e in c['exports']:
            require(e['manifest'] not in manifests,'duplicate manifest identity');manifests.add(e['manifest'])
            exports.append(verify_export(e,splits))
        cohorts.append({'cohort_id':c['cohort_id'],'exports':exports,'protocol_refs':c.get('protocol_refs',[]),'qualification_notes':c.get('qualification_notes',[])})
    return {'schema':'ml-dataset.verified-cohort-index/v1','input_sha256':digest(json.dumps(spec,sort_keys=True,separators=(',',':'),allow_nan=False).encode()),'cross_population_pooling_permitted':False,'purpose':'provenance-only cohort index, not pooled training data','native_replay_performed':False,'solved_instruction_or_degree_labels_inferred':False,'restricted_values_copied':False,'cohorts':cohorts,'component_splits':splits}

def main():
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--inventory',required=True,type=Path);ap.add_argument('--output',required=True,type=Path);args=ap.parse_args()
    result=build(parse(args.inventory.read_bytes()))
    with args.output.open('x') as f:json.dump(result,f,indent=2,allow_nan=False);f.write('\n')
if __name__=='__main__':main()
