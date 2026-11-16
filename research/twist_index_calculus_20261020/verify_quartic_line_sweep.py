"""Independent exact character count for all four quartic Frobenius lines."""
from __future__ import annotations

import json
import pathlib
import sys

from verify_quartic_twist_probe import Curve, Field, ONE, Z, miller_rabin

PANEL=[(13,4),(37,2),(73,5),(97,5),(157,25),(229,6),(241,49),(337,100)]


def verify(path):
    rows=[json.loads(s) for s in path.read_text().splitlines()]
    assert len(rows)==32
    assert [(x['p'],x['b0'],x['line_power']) for x in rows]==[
        (p,b,j) for p,b in PANEL for j in range(4)]
    sizes={}
    for row in rows:
        p,b,j=row['p'],row['b0'],row['line_power']
        assert miller_rabin(p)
        d=next(k for k in range(2,p) if pow(k,(p-1)//2,p)==p-1)
        f=Field(p,d);t=(0,1,0,0)
        v=f.power(t,j)
        t3=f.power(t,3)
        curve=Curve(f,f.mul(t3,(b,0,0,0)))
        z=f.power(t,p-1)
        assert z==(row['z'],0,0,0)
        omega=row['omega']
        assert pow(omega,3,p)==1 and omega!=1
        xs=[]
        for s in range(1,p):
            x=f.mul(v,(s,0,0,0))
            y=curve.rhs(x)
            if y!=Z and f.power(y,(p**4-1)//2)==ONE:
                xs.append(s)
        assert xs==row['lifted_abscissae']
        assert len(xs)==row['abscissa_count']
        assert row['physical_points']==2*len(xs)
        expected={} if not xs else {'24':len(xs)//12}
        assert len(xs)%12==0 and row['orbit_hist']==expected
        assert row['status']=='pass'
        if j in (1,3):assert not xs
        sizes.setdefault(p,[]).append(len(xs))
    assert all(sizes[p][0]>=sizes[p][2] for p,_ in PANEL)
    return {'status':'pass','rows':len(rows),'empty_lines':sum(v==0 for s in sizes.values() for v in s),
            'sizes':sizes,'independent_field_character':True}


if __name__=='__main__':
    print(json.dumps(verify(pathlib.Path(sys.argv[1])),sort_keys=True))
