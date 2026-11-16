from itertools import combinations_with_replacement
from research.residue_quotient_20261116_v99b.kernel import fmul

def multiplication(s,m):return [[fmul(a,b,m)for b in range(1<<s)]for a in range(1<<s)]

def allowed(s,trace,G):
    table=set()
    for roots in combinations_with_replacement(range(1<<s),4):
        last=trace
        for x in roots:last^=x
        if last<roots[-1]:continue
        c=[1]
        for x in roots+(last,):
            out=[0]*(len(c)+1)
            for i,a in enumerate(c):out[i]^=G[a][x];out[i+1]^=a
            c=out
        assert c[4]==trace
        code=sum(a<<(s*i)for i,a in enumerate(c[:4]));assert code not in table
        table.add(code)
    return table

def diagram(width,table):
    cur=bytearray(1<<width)
    for x in table:cur[x]=1
    nodes=[]
    for bit in range(width):
        memo={};next_level=[]
        for i in range(0,len(cur),2):
            pair=(cur[i],cur[i+1])
            if pair[0]==pair[1]:v=pair[0]
            elif pair in memo:v=memo[pair]
            else:v=len(nodes)+2;nodes.append((bit,*pair));memo[pair]=v
            next_level.append(v)
        cur=next_level
    return nodes,cur[0]

def evaluate(nodes,root,value):
    while root>1:
        bit,lo,hi=nodes[root-2];root=hi if value>>bit&1 else lo
    return root

def literal_split(H,q,G):
    c=list(H)
    for x in range(q):
        while len(c)>1:
            v=0
            for a in reversed(c):v=G[v][x]^a
            if v:break
            quotient=[0]*(len(c)-1);quotient[-1]=c[-1]
            for i in range(len(c)-2,0,-1):quotient[i-1]=c[i]^G[x][quotient[i]]
            assert c[0]==G[x][quotient[0]];c=quotient
    return len(c)==1
