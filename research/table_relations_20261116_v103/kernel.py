from itertools import combinations

def features(width,degree,cap=2048):
    masks=[0]
    for k in range(1,min(width,degree)+1):masks.extend(sum(1<<j for j in t)for t in combinations(range(width),k))
    assert len(masks)<=cap,'feature cap'
    return masks

def evaluation(value,masks):
    return sum(int(value&m==m)<<j for j,m in enumerate(masks))

def fast_evaluation(value,width,degree,index):
    active=[i for i in range(width)if value>>i&1];row=1
    for k in range(1,min(len(active),degree)+1):
        for t in combinations(active,k):row|=1<<index[sum(1<<j for j in t)]
    return row

def nullspace(rows,columns):
    piv={}
    for row in rows:
        while row:
            j=row.bit_length()-1
            if j in piv:row^=piv[j]
            else:piv[j]=row;break
    free=[j for j in range(columns)if j not in piv];basis=[]
    for j in free:
        v=1<<j
        for p in sorted(piv):
            if (piv[p]&v).bit_count()&1:v|=1<<p
        basis.append(v)
    return basis,len(piv)

def derive(table,width,degree,cap=2048):
    masks=features(width,degree,cap);index={m:i for i,m in enumerate(masks)}
    rows=[fast_evaluation(v,width,degree,index)for v in table];basis,rank=nullspace(rows,len(masks))
    assert len(basis)+rank==len(masks)
    for relation in basis:assert all(not((relation&row).bit_count()&1)for row in rows)
    return masks,basis,rank,rows

def accepted(value,masks,relations):
    row=evaluation(value,masks)
    return all(not((row&r).bit_count()&1)for r in relations)
