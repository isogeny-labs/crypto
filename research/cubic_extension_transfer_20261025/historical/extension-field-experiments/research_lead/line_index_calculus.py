"""Line-assisted index calculus in a prime-order plane-quartic Jacobian.

Split line sections provide cheap homogeneous relations. A few smooth reduced
divisors anchor these relations to S and T. The canonical class is an explicit
unknown: omitting it would give incorrect line relations. This is not the full
Diem large-prime algorithm and no asymptotic running-time claim is made.
"""
import itertools, random
from time import perf_counter
from sage.all import GF, PolynomialRing, vector


class Echelon:
    def __init__(self, field, width):
        self.field=field; self.width=width; self.pivots={}

    def add(self, coefficients, rhs):
        v=vector(self.field,list(coefficients)+[rhs])
        for pivot,row in sorted(self.pivots.items()):
            if v[pivot]:v-=v[pivot]*row
        pivot=next((i for i in range(self.width) if v[i]),None)
        if pivot is None:
            assert not v[-1], 'Inconsistent relation'
            return False
        v/=v[pivot];self.pivots[pivot]=v;return True

    def answer(self):
        assert len(self.pivots)==self.width
        out=vector(self.field,self.width)
        for pivot,row in sorted(self.pivots.items(),reverse=True):
            out[pivot]=row[-1]-sum(row[j]*out[j] for j in range(pivot+1,self.width))
        return out


def normalized(P):
    a=next(c for c in P if c)
    return tuple(c/a for c in P)


def place_coordinates(F, places):
    x=F(F.base_field().gen());y=F.gen();k=F.constant_base_field();out=[]
    for pl in places:
        if not pl.is_infinite_place():P=(x.evaluate(pl),y.evaluate(pl),k.one())
        elif (y/x).valuation(pl)>=0:P=(k.one(),(y/x).evaluate(pl),k.zero())
        else:P=((x/y).evaluate(pl),k.one(),k.zero())
        out.append(normalized(P))
    assert len(set(out))==len(out)
    return out


def split_line(g,P,Q):
    k=g.base_ring();R=PolynomialRing(k,'u');u=R.gen()
    h=R(g(*[u*a+b for a,b in zip(P,Q)]));roots=h.roots()
    infinity=4-h.degree()
    if sum(e for a,e in roots)+infinity!=4:return None
    points={}
    for a,e in roots:
        point=normalized([a*b+c for b,c in zip(P,Q)])
        points[point]=points.get(point,0)+int(e)
    if infinity:points[P]=points.get(P,0)+int(infinity)
    assert sum(points.values())==4 and all(not g(*pt) for pt in points)
    return points,{'parameter_P':list(map(int,P)),'parameter_Q':list(map(int,Q)),
                   'restriction_coefficients':list(map(int,h.list())),
                   'finite_roots':[[int(a),int(e)] for a,e in roots],
                   'infinity_multiplicity':int(infinity)}


def solve(S,T,r,base_div,places,g,seed=20261106,max_trials=10000,line_multiple=10):
    start=perf_counter();G=S.parent();zero=G.zero();F=G.function_field()
    assert not S==zero and r*S==zero and r*T==zero
    rng=random.Random(seed);B=places[0];N=len(places);fbplaces=places[1:]
    fb=[G.point(pl.divisor()-B.divisor()) for pl in fbplaces]
    coords=place_coordinates(F,places);lookup={P:i for i,P in enumerate(coords)}
    # Unknowns are factor-base logarithms, log(K-4B), and log(T).
    echelon=Echelon(GF(r),N+1);canonical_index=N-1;target_index=N
    records=[];seen_lines=set();seen_pairs=set();attempts=0;split=0;checked=0;canonical=None
    budget=min(line_multiple*N,N*(N-1)//2)
    pairs=list(itertools.combinations(range(N),2)) if N<30 else None
    if pairs is not None:rng.shuffle(pairs)
    while attempts<budget:
        if pairs is not None:i,j=pairs[attempts]
        else:
            i,j=sorted(rng.sample(range(N),2))
            if (i,j) in seen_pairs:continue
        seen_pairs.add((i,j));attempts+=1
        line=normalized(vector(g.base_ring(),coords[i]).cross_product(vector(g.base_ring(),coords[j])))
        if line in seen_lines:continue
        seen_lines.add(line);hit=split_line(g,coords[i],coords[j])
        if hit is None:continue
        points,certificate=hit;split+=1;counts=[0]*N
        for P,e in points.items():counts[lookup[P]]+=e
        row=counts[1:]+[-1,0]
        # Check sample relations in the actual Jacobian, in addition to exact
        # intersection factorization for every retained geometric relation.
        if checked<3:
            value=zero
            for c,Fb in zip(counts[1:],fb):value+=c*Fb
            if canonical is None:canonical=value
            else:assert value==canonical
            checked+=1
        if echelon.add(row,0):
            records.append({'kind':'line','counts':counts,'rhs':0,'certificate':certificate})
        if len(echelon.pivots)>=N-1:break
    line_seconds=perf_counter()-start;line_rank=len(echelon.pivots)
    print('LINE PHASE',N,'points',attempts,'pairs',split,'split','rank',line_rank,'s',line_seconds,flush=True)
    trials=0;smooth=0
    while trials<max_trials:
        trials+=1;a=rng.randrange(r);b=rng.randrange(r);V=a*S+b*T
        D=V.divisor()+base_div
        assert D.degree()==3 and D.is_effective()
        if any(pl.degree()!=1 for pl in D.support()):continue
        counts=[int(D.valuation(pl)) for pl in places]
        reconstructed=zero
        for c,Fb in zip(counts[1:],fb):reconstructed+=c*Fb
        assert reconstructed==V;smooth+=1
        row=counts[1:]+[0,-b]
        if echelon.add(row,a):records.append({'kind':'smooth_divisor','a':a,'b':b,'counts':counts,'rhs':a})
        if len(echelon.pivots)==N+1:
            answer=echelon.answer();m=int(answer[target_index]);assert m*S==T
            return {'scalar':m,'seconds':perf_counter()-start,'line_phase_seconds':line_seconds,
                    'line_attempts':attempts,'distinct_lines':len(seen_lines),'split_lines':split,
                    'line_rank':line_rank,'line_relations_checked_in_jacobian':checked,
                    'trials':trials,'smooth_relations':smooth,'factor_base_size':N-1,
                    'linear_system_rank':len(echelon.pivots),'independent_relations':records,
                    'algorithm':'line-assisted rational-point index calculus',
                    'claim':'Exact geometric relations plus target DLP; not full optimized Diem or an exponent claim'}
    raise RuntimeError('Line-assisted index-calculus trial budget exhausted')
