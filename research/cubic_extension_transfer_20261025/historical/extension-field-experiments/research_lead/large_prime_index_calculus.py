"""A small-factor-base, double-large-prime plane-quartic DLP solver.

Implements split-line FP/PP relations and rooted-tree substitution as in
Diem--Thome. Exact intersection certificates accompany the relations. Dense
incremental elimination is used at these small experimental sizes; this is
not an implementation of the full asymptotically optimized sparse solver.
"""
import itertools,math,random
from collections import deque
from time import perf_counter
from sage.all import GF,vector
from line_index_calculus import Echelon,normalized,place_coordinates,split_line


def graph_data(g,coords,r,seed=20261112,factor_base_constant=3):
    start=perf_counter();rng=random.Random(seed);N=len(coords);p=int(g.base_ring().cardinality())
    size=min(N,max(4,math.ceil(factor_base_constant*math.sqrt(p))))
    chosen=[0]+rng.sample(list(range(1,N)),size-1);chosen_set=set(chosen)
    field=GF(r);width=size # FB logs excluding B, plus canonical class.
    lookup={P:i for i,P in enumerate(coords)};expressions={0:vector(field,width)}
    for j,idx in enumerate(chosen[1:]):
        v=vector(field,width);v[j]=1;expressions[idx]=v
    canonical=vector(field,width);canonical[-1]=1
    relations=[];adjacency={};seeds=[];seen=set();split=0;attempts=0
    for i,j in itertools.combinations(chosen,2):
        attempts+=1
        line=normalized(vector(g.base_ring(),coords[i]).cross_product(vector(g.base_ring(),coords[j])))
        if line in seen:continue
        seen.add(line);hit=split_line(g,coords[i],coords[j])
        if hit is None:continue
        points,cert=hit;split+=1;counts={lookup[P]:e for P,e in points.items()}
        large={idx:e for idx,e in counts.items() if idx not in chosen_set}
        assert sum(large.values())<=2
        small=sum((e*expressions[idx] for idx,e in counts.items() if idx in chosen_set),vector(field,width))
        residual=canonical-small
        record={'counts':counts,'large':large,'residual':residual,'certificate':cert}
        edge=len(relations);relations.append(record)
        if len(large)==1:seeds.append((next(iter(large)),edge))
        elif len(large)==2:
            a,b=large;assert large[a]==large[b]==1
            adjacency.setdefault(a,[]).append((b,edge));adjacency.setdefault(b,[]).append((a,edge))
    depth={idx:0 for idx in chosen};parents={};queue=deque();max_depth=max(2,math.ceil(math.log2(p)))
    for idx,edge in seeds:
        if idx in expressions:continue
        relation=relations[edge];expressions[idx]=relation['residual']/field(relation['large'][idx])
        depth[idx]=1;parents[idx]={'parent':None,'edge':edge};queue.append(idx)
    while queue:
        a=queue.popleft()
        if depth[a]>=max_depth:continue
        for b,edge in adjacency.get(a,[]):
            if b in expressions:continue
            expressions[b]=relations[edge]['residual']-expressions[a]
            depth[b]=depth[a]+1;parents[b]={'parent':a,'edge':edge};queue.append(b)
    return {'chosen':chosen,'expressions':expressions,'relations':relations,'parents':parents,
            'depth':depth,'max_depth_limit':max_depth,'width':width,'canonical':canonical,
            'line_attempts':attempts,'distinct_lines':len(seen),'split_lines':split,
            'seconds':perf_counter()-start,'factor_base_constant':factor_base_constant}


def solve(S,T,r,base_div,places,g,seed=20261112,max_trials=10000,factor_base_constant=3):
    start=perf_counter();G=S.parent();F=G.function_field();zero=G.zero();B=places[0]
    assert not S==zero and r*S==zero and r*T==zero
    coords=place_coordinates(F,places);data=graph_data(g,coords,r,seed,factor_base_constant)
    expressions=data['expressions'];width=data['width'];field=GF(r)
    echelon=Echelon(field,width+1);records=[];checked=0;canonical_point=None
    point_cache={0:zero}
    def point(idx):
        if idx not in point_cache:point_cache[idx]=G.point(places[idx].divisor()-B.divisor())
        return point_cache[idx]
    for record in data['relations']:
        if any(idx not in expressions for idx in record['counts']):continue
        relation=sum((e*expressions[idx] for idx,e in record['counts'].items()),vector(field,width))-data['canonical']
        if checked<3:
            value=sum((e*point(idx) for idx,e in record['counts'].items()),zero)
            if canonical_point is None:canonical_point=value
            else:assert value==canonical_point
            checked+=1
        if echelon.add(list(relation)+[0],0):
            records.append({'kind':'recombined_line','coefficients':list(map(int,relation)),
                            'counts':record['counts'],'rhs':0})
    geometric_rank=len(echelon.pivots);precompute_seconds=perf_counter()-start
    print('LARGE PRIME GRAPH','FB',width,'covered',len(expressions),'of',len(places),
          'rank',geometric_rank,'s',precompute_seconds,flush=True)
    rng=random.Random(seed+1);trials=0;smooth=0;usable=0;walk_resets=0;walk_additions=0
    lookup={pl:i for i,pl in enumerate(places)}
    while trials<max_trials:
        # Consecutive translates of S avoid two full scalar multiplications per
        # trial. Random restarts are explicit; independence is not asserted.
        if trials%64==0:
            a,b=rng.randrange(r),rng.randrange(1,r);V=a*S+b*T;walk_resets+=1
        else:V+=S;a=(a+1)%r;walk_additions+=1
        trials+=1;D=V.divisor()+base_div
        assert D.degree()==3 and D.is_effective()
        if any(pl.degree()!=1 for pl in D.support()):continue
        smooth+=1;counts={lookup[pl]:int(e) for pl,e in D.list()}
        if any(idx not in expressions for idx in counts):continue
        usable+=1;assert sum((e*point(idx) for idx,e in counts.items()),zero)==V
        reduced=sum((e*expressions[idx] for idx,e in counts.items()),vector(field,width))
        if echelon.add(list(reduced)+[-b],a):
            records.append({'kind':'smooth_divisor','a':a,'b':b,'counts':counts,
                            'coefficients':list(map(int,reduced)),'rhs':a})
        if len(echelon.pivots)==width+1:
            answer=echelon.answer();m=int(answer[-1]);assert m*S==T
            return {'scalar':m,'seconds':perf_counter()-start,'precompute_seconds':precompute_seconds,
                    'factor_base_size':width-1,'factor_base_including_reference':width,
                    'curve_rational_points':len(places),'covered_points':len(expressions),
                    'tree_maximum_depth':max(data['depth'].values()),'tree_depth_limit':data['max_depth_limit'],
                    'line_attempts':data['line_attempts'],'distinct_lines':data['distinct_lines'],
                    'split_lines':data['split_lines'],'geometric_rank':geometric_rank,
                    'linear_system_rank':len(echelon.pivots),'trials':trials,'smooth_relations':smooth,
                    'usable_smooth_relations':usable,'walk_resets':walk_resets,'walk_additions':walk_additions,
                    'graph_parent_edges':data['parents'],
                    'graph_expressions':{i:list(map(int,v)) for i,v in expressions.items()},
                    'chosen_points':data['chosen'],
                    'line_certificates':[{'counts':z['counts'],'certificate':z['certificate']} for z in data['relations']],
                    'independent_relations':records,'seed':seed,
                    'algorithm':'split-line double-large-prime tree index calculus with translate-walk anchors',
                    'limits':['Dense elimination for small experiments; no new exponent claim.',
                              'The translate walk is a practical heuristic, not independent uniform relation sampling.',
                              'No universal graph-coverage or rank guarantee; bounded trial budget.']}
    raise RuntimeError('Large-prime index-calculus trial budget exhausted')
