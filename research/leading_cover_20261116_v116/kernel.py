from research.prefix_translation_20261116_v115.kernel import translate

def limits(d,k):return [(5-j)*(d-2)+min(5-j,k)for j in range(5)]
def fits(H,bounds):return all(v.bit_length()<=b+1 for v,b in zip(H[:5],bounds))

def equations(base,directions,d,k,shift):
    if shift:
        c=1<<(d-1);base=translate(base,c);directions=[translate(v,c)for v in directions]
    bounds=limits(d,5-k if shift else k);rows=[];labels=[]
    for j,bound in enumerate(bounds):
        upper=max([base[j].bit_length()]+[v[j].bit_length()for v in directions])
        for bit in range(bound+1,upper):
            mask=sum(((v[j]>>bit)&1)<<i for i,v in enumerate(directions));rhs=(base[j]>>bit)&1
            if mask or rhs:rows.append((mask,rhs));labels.append({'coefficient':j,'bit':bit,'shifted':shift,'bound':bound})
    return rows,labels
