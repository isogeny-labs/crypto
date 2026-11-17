import random,subprocess,sys
B=sys.argv[1]
for t in range(30):
    rng=random.Random(t); N=rng.randrange(3,13); ne=rng.randrange(1,10)
    eqs=[]
    for i in range(ne):
        ms=set(rng.getrandbits(N)&rng.getrandbits(N)&rng.getrandbits(N) for _ in range(rng.randrange(1,20)))
        ms={m for m in ms if bin(m).count('1')<=2}; eqs.append(ms)
    s="festest %d 2\n%d\n"%(N,ne)+"".join("%d %s\n"%(len(e)," ".join(map(str,e))) for e in eqs)
    print(subprocess.run([B],input=s,capture_output=True,text=True).stdout.strip())
