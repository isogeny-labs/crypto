"""Create disclosed constructed controls; never mutate natural pool files."""
import json
from pathlib import Path
import random
import time
import os
from generate import Curve, OUTPUT, dump, verify_fixture

os.sched_setaffinity(0,{3})
manifest=json.loads((OUTPUT/'manifest.json').read_text())
original=list(manifest['fixtures'])
groups={}
for item in original:
    if item['kind']=='natural':groups.setdefault((item['B'],item['curve_index']),[]).append(item)
added=[]
for (B,ci),items in groups.items():
    source=items[0]
    data=json.loads(Path(source['path']).read_text())
    validation=json.loads(Path(source['validation_path']).read_text())
    k=validation['secret_k'];q=data['curve']
    c=Curve(q['p'],q['a'],q['b'],q['r']);P=tuple(q['P']);Q=tuple(q['Q'])
    pools=data['pools'];seen={z['x'] for pool in pools for z in pool}
    # Replacing a point discards its already-charged public generation work.
    discarded=pools[2][-1]
    seen.remove(discarded['x'])
    c.reset();cpu=time.process_time();wall=time.perf_counter()
    chosen=None
    for ii,z in enumerate(pools[0]):
        for jj,zz in enumerate(pools[1]):
            target=c.neg(c.add((z['x'],z['y']),(zz['x'],zz['y'])))
            if target is not None and target[0] not in seen:
                chosen=(ii,jj,z,zz,target);break
        if chosen:break
    if chosen is None:raise RuntimeError('No disjoint planted control available')
    ii,jj,z,zz,target=chosen
    rng=random.Random(120991+B*10+ci)
    beta=rng.randrange(c.r)
    while (z['beta']+zz['beta']+beta)%c.r==0:beta=rng.randrange(c.r)
    alpha=(-z['alpha']-zz['alpha']-(z['beta']+zz['beta']+beta)*k)%c.r
    actual=c.add(c.scale(alpha,P),c.scale(beta,Q))
    assert actual==target
    pools[2][-1]={'index':B-1,'x':actual[0],'y':actual[1],'alpha':alpha,'beta':beta}
    extra={'cpu_seconds':time.process_time()-cpu,'wall_seconds':time.perf_counter()-wall,'counters':dict(c.counts)}
    case_id=f'b{B:03d}_c{ci}_planted_control'
    data['id']=case_id;data['kind']='planted_control'
    data['generation']['cpu_seconds']+=extra['cpu_seconds']
    data['generation']['wall_seconds']+=extra['wall_seconds']
    for name,n in extra['counters'].items():data['generation']['counters'][name]=data['generation']['counters'].get(name,0)+n
    data['generation']['secret_used_for_pool_generation']=True
    data['generation']['control_scope']='Known-answer correctness control only: uses validation k to relabel the forced point informatively. All public point arithmetic plus the discarded random point is charged. Not a natural-yield observation.'
    data['construction']={'origin_fixture_id':source['id'],'replaced_point_discarded':discarded,'additional_public_arithmetic':extra,
                          'expected_informative_witness_is_private':True}
    verify_fixture(c,P,Q,pools,k)
    signs=[1,1,1];indices=[ii,jj,B-1]
    BB=sum(pools[j][indices[j]]['beta'] for j in range(3))%c.r
    AA=sum(pools[j][indices[j]]['alpha'] for j in range(3))%c.r
    assert BB and -AA*pow(BB,-1,c.r)%c.r==k
    path=OUTPUT/(case_id+'.json');vp=OUTPUT/(case_id+'.validation.json')
    dump(path,data);dump(vp,{'schema':'prime-pools-validation/v1','id':case_id,'secret_k':k,'signed_zero_sum_exists':True,
                           'witness':{'indices':indices,'signs':signs,'informative':True},'scope':'Constructed correctness control; not independent natural sample.'})
    added.append({'id':case_id,'B':B,'kind':'planted_control','curve_index':ci,'path':str(path),'validation_path':str(vp)})
    # A separately labelled alias of an exact negative natural fixture. No
    # new independent data point or extra timing sample is claimed.
    neg_source=next((item for item in items if not json.loads(Path(item['validation_path']).read_text())['signed_zero_sum_exists']),None)
    if neg_source is not None:
        neg=json.loads(Path(neg_source['path']).read_text());nv=json.loads(Path(neg_source['validation_path']).read_text())
        nid=f'b{B:03d}_c{ci}_negative_control'
        neg['id']=nid;neg['kind']='exact_negative_control'
        neg['construction']={'origin_fixture_id':neg_source['id'],'control_scope':'Alias of an existing naturally sampled exact negative; not an additional independent sample. Scalar reference is validation-only.'}
        nv['id']=nid;nv['scope']='Alias of exact negative natural fixture; no new natural sample.'
        np=OUTPUT/(nid+'.json');nvp=OUTPUT/(nid+'.validation.json');dump(np,neg);dump(nvp,nv)
        added.append({'id':nid,'B':B,'kind':'exact_negative_control','curve_index':ci,'path':str(np),'validation_path':str(nvp),'origin_fixture_id':neg_source['id']})
manifest['fixtures'].extend(added);manifest['fixture_count']=len(manifest['fixtures'])
dump(OUTPUT/'manifest.json',manifest)
print(json.dumps({'added':len(added),'total':len(manifest['fixtures']),'natural_unchanged':len(original)}))
