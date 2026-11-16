from intfree import *
from tensor_encoding import bits_value
rng=random.Random(7); fails=0; tests=0
for d,ch in [(131,(134,66)),(133,(136,67)),(131,(72,33)),(133,(72,34))]:
    for t in range(40):
        c=FreeInt(); c.CHOICE={d:ch}; a=c.word(d); b=c.word(d); out=c.integer_multiply(a,b)
        x=rng.randrange(1<<d) if t>1 else (1<<d)-1; y=rng.randrange(1<<d) if t>1 else (1<<d)-1
        vals=c.assignment({a:x,b:y}); bad=c.check(vals); ok=not bad
        got=bits_value(tuple(vals[v] for v in out)); tests+=1
        if not ok or got!=x*y: fails+=1; print("FAIL",d,ch,bad[:3],got==x*y)
print("tests",tests,"fails",fails)
