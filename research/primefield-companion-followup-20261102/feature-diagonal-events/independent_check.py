"""Independent direct character-feature diagonal check. Not executed this round.

No producer imports. Explicit finite points supplied by a funded validation panel.
This checks diagonal values, not zero columns, Schur closure or an extractor.
"""
def verify(record,r,q,gamma,weights,points):
    if record.prime!=r or record.window!=q:
        raise ValueError('record dimensions')
    if not 0<q<r or len(record.corrections)!=q:
        raise ValueError('shape')
    pats=[(0,)]+[(0,d) for d in range(1,q)]+[(0,d,e) for d in range(1,q) for e in range(d+1,q)]
    if len(weights)!=len(pats)+1:
        raise ValueError('weight count')
    if record.gamma_inverse*gamma%r!=1:
        raise ValueError('inverse gamma')
    expected_background=sum(w*sum(pow(gamma,a,r) for a in range(q-max(P)))
                            for P,w in zip(pats,weights[1:]))%r
    if record.nonconstant_background!=expected_background or record.background!=(weights[0]+expected_background)%r:
        raise ValueError('background')
    if any(type(v) is not int or not 0<=v<r for v in
           (record.gamma_inverse,record.nonconstant_background,record.background)+tuple(record.corrections)):
        raise ValueError('canonical record values')
    checked=0
    for k in points:
        if type(k) is not int or not 0<=k<r:
            raise ValueError('point')
        value=weights[0]
        for P,w in zip(pats,weights[1:]):
            for a in range(q-max(P)):
                feature=1
                for d in P:
                    feature=feature*pow((k+a+d)%r,(r-1)//2,r)%r
                value=(value+w*pow(gamma,a,r)*feature*feature)%r
        z=(-k)%r
        actual=(record.background-(record.corrections[z] if z<q else 0))%r
        if value!=actual:
            raise ValueError('direct character diagonal mismatch')
        checked+=1
    return dict(points_checked=checked,scope='supplied diagonal points only',
                zero_columns_checked=False,primality_checked=False)
