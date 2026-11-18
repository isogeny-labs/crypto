import json,time,random,itertools,hashlib,math
from pathlib import Path
from .common import P,finish
from research.residue_quotient_20261116_v99b.problem import build
from research.residue_quotient_20261116_v99b.kernel import basis,preimage,apply,code,rem,pmul
from research.residue_tables_20261116_v102b.kernel import multiplication,allowed,literal_split
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard
START=time.monotonic();out={'original_complete':False,'original_exact_root_count':None,'cells':[]};error=None
try:
    d=json.loads((P/'DATA.json').read_text());config=json.loads(Path(d['config']).read_text());mapping=json.loads(Path(d['coordinates']).read_text());rows=matrix(config['matrix'])
    E,eb,widths,first,F,fb,last,details=build(Path(d['old_problem']))
    assert len(mapping['R'])==72 and len(mapping['K'])==4
    R,K=mapping['R'],mapping['K'];kp,_=basis([apply(F,k)for k in K]);assert len(kp)==4
    free=[i for i in range(16)if i not in kp]
    def quotient(v):
        for i in sorted(kp,reverse=True):
            if v>>i&1:v^=kp[i][0]
        return sum(((v>>i)&1)<<j for j,i in enumerate(free))
    buckets={}
    for x in sorted(last):buckets.setdefault(quotient(x),[]).append(x)
    tables=first+[last];directions=rows[1:]+[[config['modulus']<<j,0,0,0,0,0]for j in range(3)];observations=[];literal_controls=0
    rng=random.Random(d['seed'])
    for i,(s,m)in enumerate(config['fields']):
        observations.append(([code(v,s,m)for v in directions],code(rows[0],s,m)))
        if i>=8:
            G=multiplication(s,m);trace=rem(rows[0][4],m);table=allowed(s,trace,G);assert len(table)==math.comb((1<<s)+4,5)//(1<<s);tables.append(table)
            controls=sorted(table)[:12]+[rng.randrange(1<<(4*s))for _ in range(12)]
            for value in controls:
                H=[(value>>(s*j))&((1<<s)-1)for j in range(4)]+[trace,1]
                assert (value in table)==literal_split(H,1<<s,G);literal_controls+=1
    offsets=[0,4,8,16,28,40,56]
    fixed_indices=[3,4,5,6];sorted_first=[sorted(t)for t in first]
    cells=[tuple(sorted_first[i][rng.randrange(len(sorted_first[i]))]for i in fixed_indices)for _ in range(d['cells'])]
    assert len(set(cells))==len(cells)
    setup=time.monotonic()-START;work=0;all_selected=set();stage_counts=[0]*8
    for cell_id,cell in enumerate(cells):
        cell_start=time.monotonic();base=sum(x<<offsets[i]for i,x in zip(fixed_indices,cell));ys=[base^a^(b<<4)^(c<<8)for a,b,c in itertools.product(*[sorted_first[i]for i in range(3)])]
        direct=set();begin=time.monotonic()
        for y in ys:
            theta0=mapping['offset']^apply(R,y)
            for z in range(16):
                work+=1
                if work>d['tuple_cap']:raise RuntimeError('declared tuple cap')
                theta=theta0^apply(K,z)
                if apply(F,theta)^fb in last:direct.add(theta)
        direct_seconds=time.monotonic()-begin;lookup=set();begin=time.monotonic();bucket_visits=0
        for y in ys:
            theta0=mapping['offset']^apply(R,y);value=apply(F,theta0)^fb
            for hit in buckets.get(quotient(value),[]):
                bucket_visits+=1;z=preimage(value^hit,kp);theta=theta0^apply(K,z)
                assert apply(F,theta)^fb==hit
                lookup.add(theta)
        lookup_seconds=time.monotonic()-begin
        assert lookup==direct
        stage_counts[0]+=len(lookup);kept=[];per=[len(lookup)]
        for theta in sorted(lookup):
            H=decode(rows,theta);lifted=list(H);lifted[0]^=pmul(theta>>73,config['modulus'])
            assert code(lifted,*config['fields'][7])==apply(F,theta)^fb
            if not valid_guard(H,theta>>73):continue
            stage_counts[1]+=1
            for j in range(8,14):
                cols,b=observations[j];v=apply(cols,theta)^b
                assert code(lifted,*config['fields'][j])==v
                if v not in tables[j]:break
                stage_counts[j-6]+=1
            else:kept.append(theta);all_selected.add(theta)
        out['cells'].append({'id':cell_id,'fixed_codes':cell,'first_three_choices':len(ys),'direct_kernel_evaluations':16*len(ys),'bucket_visits':bucket_visits,'eighth_count':len(lookup),'accepted_fourteen_and_guard':kept,'direct_eighth_seconds':direct_seconds,'quotient_eighth_seconds':lookup_seconds,'total_seconds':time.monotonic()-cell_start})
    out.update({'literal_controls':literal_controls,'direct_enumeration_controls':work,'stage_counts_eighth_guard_then_six':stage_counts,'setup_seconds':setup,'total_seconds':time.monotonic()-START,'selected_cell_count':len(cells),'outer_cover_cells':math.prod(len(first[i])for i in fixed_indices),'selected_necessary_parameters':sorted(all_selected),'selected_necessary_cells_complete':True,'scope':'exact guarded fourteen-residue parameters only in the declared32cells; no full support/source/U/outer completeness','field_table_hashes':[hashlib.sha256(json.dumps(sorted(t)).encode()).hexdigest()for t in tables]})
except Exception as e:error=repr(e)
finish(out,error,START,'Conditional algebraic kernel recovery checked against every direct kernel assignment in frozen cells.','Measure whether the remaining outer four-block choices share exact separator signatures; do not enumerate the whole cover or infer50x from a4bit conditional kernel.')
