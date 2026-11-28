from math import comb
r=2147446831
def count(b):
    return 1+sum(comb(b,t)*2**t*comb(j-1,t-1)
                 for j in (2,4) for t in range(1,min(b,j)+1))
def closed(b): return (2*b**4+10*b*b+3)//3
def strict(b): return 16*comb(b,4) if b>=4 else 0
for b in range(678):
    assert count(b)==closed(b)
assert closed(238)<=r<closed(239)
assert strict(239)<=r<strict(240)
assert closed(677)==140045175925 and strict(677)==138805851600
assert (closed(677)+r-1)//r==66
assert (strict(677)+r-1)//r==65
for b in (31,224,238,239,240,256,320,677):
    print(b,closed(b),strict(b),max(0,b-238),max(0,b-239))
