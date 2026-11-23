import math
def inv(k): return (k-1)+(int(math.log2(k-1))+bin(k-1).count('1')-1)
def ops(k,l):
    a=inv(k)+3+3+1
    h1=(k-1)+inv(k)+2
    h2=h1+2*(k-l)
    pre=inv(k)+2
    return a,h1,h2,pre
for m,ks in ((2,[17,19,23,29,31,131]),(3,[17,19,23,29,131])):
    for k in ks:
        l=-(-k//m); a,h1,h2,pre=ops(k,l)
        r=(2*a+h2)/(a+h2) if m==2 else (8*a+2*h1+2*h2)/(3*a+h1+2*h2)
        lam=2**(m*l-k)/ (2 if m==2 else 6)
        print(m,k,l,"a=%d h1=%d h2=%d pre=%d ratio=%.3f lam=%.3g"%(a,h1,h2,pre,r,lam))
