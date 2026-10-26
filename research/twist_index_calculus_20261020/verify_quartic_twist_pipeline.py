"""Independent field/group, relation-row, rank and scalar replay of twist24 runs."""
from __future__ import annotations

import json
import pathlib
import sys

from verify_quartic_twist_probe import Curve, Field, ONE, Z, miller_rabin


def point(x, y):
    return tuple(x), tuple(y)


def map_columns(points, curve, psi, cubic, lp, lc, r, combined):
    index = {pt[0][0]: i for i, pt in enumerate(points)}
    assert len(index) == len(points)
    col = [None] * len(points)
    coef = [None] * len(points)
    ncols = 0
    for i, rep in enumerate(points):
        if col[i] is not None:
            continue
        image = rep
        fj = 1
        for j in range(4 if combined else 1):
            if j:
                image = psi(image)
                fj = fj * lp % r
            transformed = image
            ck = 1
            for k in range(3):
                if k:
                    transformed = cubic(transformed)
                    ck = ck * lc % r
                assert transformed[0][1:] == (0, 0, 0)
                idx = index[transformed[0][0]]
                m = fj * ck % r
                if points[idx][1] == curve.f.neg(transformed[1]):
                    m = -m % r
                else:
                    assert points[idx][1] == transformed[1]
                if col[idx] is not None:
                    assert (col[idx], coef[idx]) == (ncols, m)
                else:
                    col[idx], coef[idx] = ncols, m
        ncols += 1
    assert all(v is not None for v in col)
    return col, coef, ncols


def relation_row(terms, cols, coefs, ncols, a, b, r):
    out = {}
    for i, sign in terms:
        c = cols[i]
        out[c] = (out.get(c, 0) + sign * coefs[i]) % r
    row = sorted((c, v) for c, v in out.items() if v)
    row.append((ncols, (-b) % r))
    row.sort()
    return row, a


def target_from_rows(rows, ncols, r):
    """Reduce the target unit vector in the exact row span over F_r."""
    basis = {}
    for row, rhs in rows:
        v = {c: x % r for c, x in row if x % r}
        rhs %= r
        while v:
            piv = min(v)
            if piv not in basis:
                inv = pow(v[piv], -1, r)
                basis[piv] = ({c: x * inv % r for c, x in v.items()}, rhs * inv % r)
                break
            brow, brhs = basis[piv]
            factor = v[piv]
            for c, x in brow.items():
                y = (v.get(c, 0) - factor * x) % r
                if y:
                    v[c] = y
                else:
                    v.pop(c, None)
            rhs = (rhs - factor * brhs) % r
        else:
            assert rhs == 0, "inconsistent relation matrix"
    v = {ncols: 1}
    answer = 0
    while v:
        piv = min(v)
        if piv not in basis:
            return len(basis), None
        brow, brhs = basis[piv]
        factor = v[piv]
        for c, x in brow.items():
            y = (v.get(c, 0) - factor * x) % r
            if y:
                v[c] = y
            else:
                v.pop(c, None)
        answer = (answer + factor * brhs) % r
    return len(basis), answer


def verify_one(rec):
    p, b0, r = rec['p'], rec['b0'], rec['n']
    assert (p, b0) in ((37,2),(73,5),(97,5),(157,25),(229,6),(241,49),(337,100))
    assert miller_rabin(p) and miller_rabin(r)
    d = next(x for x in range(2,p) if pow(x,(p-1)//2,p)==p-1)
    f = Field(p,d)
    t = (0,1,0,0)
    t3 = f.mul(f.mul(t,t),t)
    c = Curve(f,f.mul(t3,(b0,0,0,0)))
    h = f.power(t,f.q-1-(p-1)//2)
    h2, h3 = f.mul(h,h), f.mul(f.mul(h,h),h)
    z = f.power(t,p-1)
    assert z == (rec['z'],0,0,0) and h2[1:] == (0,0,0)
    assert pow(rec['omega'],3,p)==1 and rec['omega']!=1
    assert pow(rec['lambda_frob'],4,r)==r-1
    assert pow(rec['lambda_cubic'],3,r)==1 and rec['lambda_cubic']!=1
    def psi(pt):
        if pt is None:return None
        return f.mul(h2,f.power(pt[0],p)),f.mul(h3,f.power(pt[1],p))
    def cubic(pt):
        if pt is None:return None
        return f.mul(pt[0],(rec['omega'],0,0,0)),pt[1]
    g = point(rec['generator_x'],rec['generator_y'])
    q = point(rec['target_x'],rec['target_y'])
    assert c.on(g) and c.on(q) and c.scale(g,r) is None
    assert rec['known_scalar']==271 and c.scale(g,271)==q
    assert c.scale(g,rec['lambda_frob'])==psi(g)
    assert c.scale(g,rec['lambda_cubic'])==cubic(g)
    if 'negative_control_rejected' in rec:
        assert rec['negative_control_rejected'] and rec['all_coefficients_checked']
        assert c.scale(g,(rec['lambda_frob']+1)%r)!=psi(g)
        for key in ('model_fp_muls','twist_geometry_fp_muls','input_fp_muls',
                    'input_group_ops','cubic_eigen_fp_muls','cubic_eigen_group_ops',
                    'frob_eigen_fp_muls','frob_eigen_group_ops'):
            assert isinstance(rec[key],int) and rec[key]>=0
    pts = [point(x,y) for x,y in rec['base_coordinates']]
    assert len(pts)==rec['base_points'] and len(set(pts))==len(pts)
    line = [x for x in range(1,p) if (rhs:=c.rhs((x,0,0,0)))!=Z
            and f.power(rhs,(f.q-1)//2)==ONE]
    assert [pt[0][0] for pt in pts]==line
    for pt in pts:
        assert c.on(pt) and psi(pt)==c.scale(pt,rec['lambda_frob'])
        assert cubic(pt)==c.scale(pt,rec['lambda_cubic'])
    flatcols=list(range(len(pts)))
    flatcoef=[1]*len(pts)
    cc,cv,cn=map_columns(pts,c,psi,cubic,rec['lambda_frob'],rec['lambda_cubic'],r,False)
    tc,tv,tn=map_columns(pts,c,psi,cubic,rec['lambda_frob'],rec['lambda_cubic'],r,True)
    assert cn*3==tn*12==len(pts)
    maps={'flat':(flatcols,flatcoef,len(pts)),
          'cubic':(cc,cv,cn),'twist24':(tc,tv,tn)}
    all_rows={k:[] for k in maps}
    seen={k:set() for k in maps}
    assert len(rec['selected_relations'])==rec['targets_decomposed']
    for item in rec['selected_relations']:
        a,b=item['a'],item['b']
        terms=[tuple(t) for t in item['terms']]
        assert len(terms)==4 and len({i for i,_ in terms})==4
        assert all(0<=i<len(pts) and s in (-1,1) for i,s in terms)
        scalar=(a+271*b)%r
        actual=c.scale(g,scalar)
        product=None
        for i,s in terms:
            product=c.add(product,pts[i] if s==1 else c.neg(pts[i]))
        assert product==actual, "relation is not a group identity"
        for name,(cols,coefs,ncols) in maps.items():
            limit=rec[name]['relations']
            if len(all_rows[name])>=limit:
                continue
            row=relation_row(terms,cols,coefs,ncols,a,b,r)
            key=(tuple(row[0]),row[1])
            if key not in seen[name]:
                seen[name].add(key)
                all_rows[name].append(row)
    ranks={}
    for name,rows in all_rows.items():
        record=rec[name]
        assert record['columns']==maps[name][2]
        assert record['relations']==len(rows)==len(rec[name+'_rows'])
        assert all(rec[name+'_rows'][i]=={'cols':[list(z) for z in row], 'rhs':rhs}
                   for i,(row,rhs) in enumerate(rows))
        rank,answer=target_from_rows(rows,maps[name][2],r)
        assert answer==record['recovered']==271
        assert record['solved'] and record['correct']
        ranks[name]=rank
    assert rec['residuals']<=rec['targets_computed']
    assert rec['targets_computed']%64==0
    assert rec['decompositions']>=rec['targets_decomposed']
    assert sum(rec['decompositions_per_target'])==rec['residuals']
    assert all(rec['stream_fp_muls_at_solve'][i]<=rec['stream_fp_muls'] for i in range(3))
    for rr in rec['rho']:
        assert rr['fold'] in ('Negation','Psi','Twist24')
        assert rr['correct'] and rr['recovered']==271
        assert c.scale(g,rr['recovered'])==q
    return {'p':p,'seed':rec['seed'],'base_points':len(pts),
            'relations_replayed':len(rec['selected_relations']),
            'ranks':ranks,'rho_solves':len(rec['rho'])}


def main(path):
    rows=[json.loads(s) for s in path.read_text().splitlines() if s.strip()]
    summaries=[verify_one(x) for x in rows]
    return {'status':'pass','runs':len(rows),
            'relations_replayed':sum(x['relations_replayed'] for x in summaries),
            'rho_solves':sum(x['rho_solves'] for x in summaries),
            'rows':summaries}


if __name__=='__main__':
    print(json.dumps(main(pathlib.Path(sys.argv[1])),sort_keys=True))
