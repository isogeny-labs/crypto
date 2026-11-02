import sys,json
def ms(d):
    t,p,c=d['t'],d['p'],d['coeffs']
    V=[f"y{i}_{j}" for i in (1,2) for j in range(t)]
    def z(i,e):
        if e==0: return "1"
        if e==1: return f"y{i}_0"
        return f"y{i}_1" if t>=2 else "1"
    terms=[f"{c[e1][e2]}*{z(1,e1)}*{z(2,e2)}" for e1 in range(3) for e2 in range(3) if c[e1][e2]]
    eqs=[]
    for i in (1,2):
        for j in range(t-1): eqs.append(f"y{i}_{j}^2-y{i}_{j+1}")
        eqs.append(f"y{i}_{t-1}^2-1")
    eqs.append("+".join(terms))
    return ",".join(V)+"\n"+str(p)+"\n"+",\n".join(eqs)+"\n"
if __name__=='__main__':
    print(ms(json.loads(sys.stdin.readline())),end='')
