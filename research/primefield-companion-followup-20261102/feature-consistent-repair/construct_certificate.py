"""One fixed exact certificate construction; see CHECK_SCOPE.md."""
from pathlib import Path
from itertools import combinations
from dataclasses import asdict
import copy,hashlib,json,sys
from rectangular import factor
from verify_program import evaluate,verify
D=Path(__file__).resolve().parent

def write(path,value):
    data=(json.dumps(value,separators=(',',':'))+'\n').encode()
    if len(data)>6*1024**2:raise ValueError('individual output budget')
    with path.open('xb') as f:f.write(data)


def program(labels,coefficients,extra=False):
    terms=[dict(offsets=list(S),coefficient=c) for S,c in zip(labels,coefficients) if c]
    if extra:terms.append(dict(offsets=[15],coefficient=405))
    return dict(schema='public-character-polynomial/v1',r=479,terms=terms)


def main():
    if not __debug__:raise RuntimeError('assertions required for local checks')
    for name,digest in json.loads((D/'check-lock.json').read_text()).items():
        assert hashlib.sha256((D/name).read_bytes()).hexdigest()==digest,name
    O=D/'certificate';O.mkdir(exist_ok=False)
    p=json.loads((D/'repair-input.json').read_text())
    assert p['r']==479 and p['q']==15 and p['pairs']==[[25,173],[292,440]]
    assert p['coefficients']==[405,0] and p['new_offset']==15
    r,q=479,15
    labels=tuple(S for degree in range(4) for S in combinations(range(q),degree))
    assert len(labels)==576
    representatives=tuple(x for x in range(r) if x not in (173,440))
    chi=tuple(pow(x,(r-1)//2,r) for x in range(r))
    A=[]
    for x in representatives:
        row=[]
        for S in labels:
            value=1
            for d in S:value=value*chi[(x+d)%r]%r
            row.append(value)
        A.append(row)
    F=factor(A,r)
    assert len(F.pivots)==477
    corrected=tuple((x-405*chi[(x+15)%r])%r for x in representatives)
    coefficients=F.solve(corrected)
    result=program(labels,coefficients,True)
    checked=verify(result)
    write(O/'program.json',result)
    write(O/'factor.json',dict(schema='rectangular-factor/v1',representatives=representatives,
                              labels=labels,**asdict(F)))
    # All representative rows can fit X while omitted mates still disprove it.
    restricted=program(labels,F.solve(representatives))
    values=evaluate(restricted)
    assert all(values[x]==x for x in representatives)
    failures=[x for x,v in enumerate(values) if x!=v]
    assert failures==[173,440]
    for rhs,expected in [(tuple(1 for x in representatives),[1]*r),
                         (tuple(chi[x] for x in representatives),list(chi))]:
        assert evaluate(program(labels,F.solve(rhs)))==expected
    mutations=0
    for index in (0,len(result['terms'])-1):
        bad=copy.deepcopy(result)
        c=bad['terms'][index]['coefficient'];bad['terms'][index]['coefficient']=c%478+1
        try:verify(bad)
        except ValueError:mutations+=1
        else:raise AssertionError('altered program accepted')
    toy=factor([[1,1],[2,2]],7)
    assert toy.solve((1,2))==(1,0)
    try:toy.solve((1,3))
    except ValueError:pass
    else:raise AssertionError('inconsistent system accepted')
    swap=factor([[0,1],[1,1]],7)
    assert swap.solve((2,5))==(3,2)
    try:swap.solve((2,))
    except ValueError:pass
    else:raise AssertionError('bad RHS dimension accepted')
    summary=dict(**checked,representative_rows=477,old_feature_columns=576,
                 producer_rank=len(F.pivots),independent_rank_replay=False,
                 old_nonzero_terms=len(result['terms'])-1,total_nonzero_terms=len(result['terms']),
                 corrected_rhs_solved=True,factor_rhs_solves=4,
                 omitted_row_negative_control=failures,known_rhs_global_checks=2,
                 program_mutations_rejected=mutations,small_linear_algebra_checks=4,
                 unknown_scalar_targets=0,performance_claims=False,
                 program_sha256=hashlib.sha256((O/'program.json').read_bytes()).hexdigest(),
                 factor_sha256=hashlib.sha256((O/'factor.json').read_bytes()).hexdigest())
    write(O/'summary.json',summary)
    print(json.dumps(summary),flush=True)

if __name__=='__main__':main()
