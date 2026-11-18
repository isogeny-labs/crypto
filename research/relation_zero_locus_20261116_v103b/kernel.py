def variable_truth(width,j):
    size=1<<width
    if width<3:return sum(((x>>j)&1)<<x for x in range(size))
    count=size//8
    if j<3:return int.from_bytes(bytes([0xaa if j==0 else 0xcc if j==1 else 0xf0])*count,'little')
    half=(1<<j)//8;return int.from_bytes((bytes(half)+b'\xff'*half)*(count//(2*half)),'little')

def zero_locus(width,masks,relations,cap_bytes):
    states=1<<width;estimated=((states+7)//8)*(len(masks)+width+4)
    assert estimated<=cap_bytes,'truth-storage cap'
    universe=(1<<states)-1;variables=[variable_truth(width,j)for j in range(width)];cache={0:universe};vectors=[]
    for m in masks:
        if m:
            low=m&-m;cache[m]=cache[m^low]&variables[low.bit_length()-1]
        vectors.append(cache[m])
    accepted=universe
    for r in relations:
        value=0
        while r:
            low=r&-r;value^=vectors[low.bit_length()-1];r^=low
        accepted&=universe^value
    return accepted,estimated

def table_mask(width,table):
    states=1<<width;data=bytearray((states+7)//8)
    for x in table:data[x//8]|=1<<(x%8)
    return int.from_bytes(data,'little')
