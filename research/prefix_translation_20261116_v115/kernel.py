from research.residue_quotient_20261116_v99b.kernel import pmul,apply

def product(roots):
    H=[1]
    for x in roots:
        out=[0]*(len(H)+1)
        for j,a in enumerate(H):out[j]^=pmul(a,x);out[j+1]^=a
        H=out
    return H

def translate(H,c):
    assert len(H)==6
    c2=pmul(c,c);c3=pmul(c2,c);c4=pmul(c2,c2);c5=pmul(c4,c)
    h0,h1,h2,h3,h4,h5=H
    return [h0^pmul(c,h1)^pmul(c2,h2)^pmul(c3,h3)^pmul(c4,h4)^pmul(c5,h5),
            h1^pmul(c2,h3)^pmul(c4,h5),h2^pmul(c,h3),h3,h4^pmul(c,h5),h5]

def independent_compose(H,c):
    out=[0]*6;power=[1]
    for coefficient in H:
        for j,v in enumerate(power):out[j]^=pmul(coefficient,v)
        new=[0]*(len(power)+1)
        for j,v in enumerate(power):new[j]^=pmul(v,c);new[j+1]^=v
        power=new
    return out

def bounds(d):return [(5-j)*(d-2)for j in range(5)]
def bounded(H,d):return all(v.bit_length()<=bound+1 for v,bound in zip(H[:5],bounds(d)))

def degree_equations(base,directions,d):
    c=1<<(d-1);b=translate(base,c);cols=[translate(v,c)for v in directions];rows=[];labels=[]
    for j,bound in enumerate(bounds(d)):
        upper=max([b[j].bit_length()]+[v[j].bit_length()for v in cols])
        for bit in range(bound+1,upper):
            mask=sum(((v[j]>>bit)&1)<<i for i,v in enumerate(cols));rhs=(b[j]>>bit)&1
            if mask or rhs:rows.append((mask,rhs));labels.append({'coefficient':j,'bit':bit,'bound':bound})
    return rows,labels
