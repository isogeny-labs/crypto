from research.residue_quotient_20261116_v99b.kernel import basis,preimage,apply

class RankCap(Exception):
    pass

def reduce_by(v,piv):
    for j in sorted(piv,reverse=True):
        if v>>j&1:v^=piv[j][0]
    return v

def quotient_map(cols,ambient):
    piv,K=basis(cols);free=[j for j in range(ambient)if j not in piv]
    def q(v):
        v=reduce_by(v,piv)
        return sum(((v>>j)&1)<<i for i,j in enumerate(free))
    return piv,K,free,q

def affine_hull(table):
    table=sorted(set(table))
    if not table:return {'empty':True,'anchor':0,'columns':[],'domain':[]}
    anchor=table[0];piv,_=basis([v^anchor for v in table]);B=[v for p,(v,w)in sorted(piv.items())];bp,_=basis(B)
    domain=[preimage(v^anchor,bp)for v in table]
    assert len(set(domain))==len(table)
    assert all(anchor^apply(B,u)==v for u,v in zip(domain,table))
    return {'empty':False,'anchor':anchor,'columns':B,'domain':domain}

def build_code(observations,affine,widths,tables,n):
    ambient=sum(widths);cp,K,free,q=quotient_map(observations,ambient)
    hulls=[affine_hull(t)for t in tables];blocks=[];anchors=0;offset=0
    for width,h in zip(widths,hulls):
        assert all(v < (1<<width)for v in h['columns'])
        anchors^=h['anchor']<<offset
        blocks.append([q(v<<offset)for v in h['columns']]);offset+=width
    target=q(affine^anchors)
    allcols=[v for block in blocks for v in block];sp,_=basis(allcols)
    return {'n':n,'ambient_bits':ambient,'observation_rank':len(cp),'observation_kernel':K,
            'quotient_bits':len(free),'widths':widths,'hulls':hulls,'blocks':blocks,
            'target':target,'syndrome_rank':len(sp),'affine_feasible':reduce_by(target,sp)==0}

def accepted(code,choice):
    value=code['target']
    for block,u in zip(code['blocks'],choice):value^=apply(block,u)
    return value==0

def recover(code,choice,observations,affine):
    value=affine;offset=0
    for width,h,u in zip(code['widths'],code['hulls'],choice):
        value^=(h['anchor']^apply(h['columns'],u))<<offset;offset+=width
    piv,K=basis(observations)
    theta=preimage(value,piv)
    return theta,K

class Widths:
    def __init__(self,blocks,limit=4096):
        self.blocks=blocks;self.limit=limit;self.calls=0;self.cache={0:0};self.full=(1<<len(blocks))-1
        self.total=self.rank(self.full)
    def rank(self,mask):
        if mask not in self.cache:
            self.calls+=1
            if self.calls>self.limit:raise RankCap('declared rank evaluation cap')
            self.cache[mask]=len(basis([v for i,b in enumerate(self.blocks)if mask>>i&1 for v in b])[0])
        return self.cache[mask]
    def cut(self,mask):return self.rank(mask)+self.rank(self.full^mask)-self.total
    def ordering(self,order):
        mask=0;values=[]
        for i in order[:-1]:mask|=1<<i;values.append(self.cut(mask))
        return {'order':order,'widths':values,'peak_width':max(values,default=0)}
    def profiles(self):
        n=len(self.blocks);natural=self.ordering(list(range(n)));reverse=self.ordering(list(range(n-1,-1,-1)))
        order=[];mask=0
        while len(order)<n:
            i=min((i for i in range(n)if not(mask>>i&1)),key=lambda i:(self.cut(mask|(1<<i)),i))
            order.append(i);mask|=1<<i
        greedy=self.ordering(order)
        return {'natural':natural,'reverse':reverse,'greedy':greedy,'rank_evaluations':self.calls}
