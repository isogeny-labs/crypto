"""Exact affine quotient of a Boolean gate circuit; no root-completion claim."""
def atoms(nodes,n):
    forms=[0,1]+[1<<(i+1) for i in range(n)]; products=[]; count=n
    for node in nodes[n+2:]:
        kind,a,*rest=node
        if kind=='xor': forms.append(forms[a]^forms[rest[0]])
        elif kind=='not': forms.append(forms[a]^1)
        elif kind=='and':
            count+=1; z=1<<count
            products.append((z,forms[a],forms[rest[0]]));forms.append(z)
        else: raise ValueError(kind)
    return forms,products,count

def bits(f):
    while f:
        b=f&-f;yield b.bit_length()-1;f^=b

def normalize(f,sub,mask):
    selected=f&mask
    for j in bits(selected): f^=(1<<j)^sub[j]
    return f

def quotient(count,products,zeros,max_rounds=16):
    basis={};bad=False;history=[]
    def insert(row):
        nonlocal bad
        while row>1:
            j=row.bit_length()-1
            if j in basis:row^=basis[j]
            else:basis[j]=row;return True
        if row==1:bad=True
        return False
    for f in zeros:insert(f)
    for iteration in range(max_rounds):
        sub={};mask=0
        for j in sorted(basis):
            sub[j]=normalize(basis[j]^(1<<j),sub,mask);mask|=1<<j
        equations=[]
        for z,a,b in products:
            z,a,b=(normalize(f,sub,mask) for f in (z,a,b))
            if a==0 or b==0:equations.append(z)
            elif a==1:equations.append(z^b)
            elif b==1:equations.append(z^a)
            elif a==b:equations.append(z^a)
            elif a^b==1:equations.append(z)
            elif z==1:equations.extend([a^1,b^1])
        added=sum(insert(f) for f in equations)
        history.append({'round':iteration,'rank':len(basis),'new_rows':added})
        if bad or not added:break
    sub={};mask=0
    for j in sorted(basis):sub[j]=normalize(basis[j]^(1<<j),sub,mask);mask|=1<<j
    reduced=[]
    for triple in products:
        z,a,b=(normalize(f,sub,mask) for f in triple)
        trivial=((a==0 or b==0 or a^b==1) and z==0) or (a==1 and z==b) or (b==1 and z==a) or (a==b and z==a)
        if not trivial:reduced.append((z,a,b))
    return {'basis':basis,'sub':sub,'mask':mask,'bad':bad,'products':reduced,'free':[j for j in range(1,count+1)if j not in basis],'history':history,'atoms':count}

def evaluate(form,assignment):return (form&1)^((form&assignment).bit_count()&1)

def restore(q,free_assignment):
    value=free_assignment
    for j,form in q['sub'].items():value|=evaluate(form,free_assignment)<<j
    return value

def accepts(q,free_assignment):
    return not q['bad'] and all(evaluate(z,free_assignment)==(evaluate(a,free_assignment)&evaluate(b,free_assignment))for z,a,b in q['products'])

def profile(q,forms,products):
    unique={f for t in q['products']for f in t}
    return {'atoms_before':q['atoms'],'linear_rank':len(q['basis']),'free_atoms':len(q['free']),'primary_nonconstant':sum(normalize(f,q['sub'],q['mask'])>1 for f in forms[2:78]),'and_before':len(products),'and_after':len(q['products']),'unique_affine_forms':len(unique),'affine_terms':sum(f.bit_count()for f in unique),'max_affine_terms':max(map(int.bit_count,unique),default=0),'contradiction':q['bad'],'rounds':q['history']}
