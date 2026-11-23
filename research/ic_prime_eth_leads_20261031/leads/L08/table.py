import json, math
lg=math.log2
r_bits=lg(680564733841876926932320129493409985129); n=131; RHO=60.81
fit=json.load(open("fit_rows.json")); t3=json.load(open("t3_rows.json"))
E={(r["cfg"],r["m"]):r["edges"] for r in t3}
E[("eccp131_int",2)]=1564506
print("log2 r = %.3f ; rho floor 2^%.2f ; fold factor log2 n = %.3f"%(r_bits,RHO,lg(n)))
hdr="network m edges  log2L  log2N | N/n | dense(N^3) sparse(N^2(m+2)) | dense fold sparse fold | s/sqrt(log2 s) | c* for 1 rel = rho"
print(hdr)
out=[]
for (cfg,m),s in sorted(E.items(), key=lambda t:(t[0][1],t[0][0])):
    fold = cfg.endswith("fold")
    L=max(r_bits/m, lg(10*m)); N=lg(m)+L           # log2 L, log2 N (L = max(r^{1/m},10m))
    Nf = N - lg(n)
    dense=3*N; sparse=2*N+lg(m+2)
    denseF=3*Nf; sparseF=2*Nf+lg(m+2)
    ex=s/math.sqrt(lg(s)); cstar=RHO/ex
    row=dict(cfg=cfg,m=m,edges=s,log2L=L,log2N=N,log2N_over_n=Nf,dense=dense,sparse=sparse,denseF=denseF,sparseF=sparseF,contr_bits=ex,cstar=cstar,
             fold=fold)
    # total relation phase: (#relations needed) * attempts(2.5) * contraction; folded needs N/n relations
    nrel = Nf if fold else N
    row["total_rel_phase_log2"]=nrel+lg(2.5)+ex
    out.append(row)
    print("%-16s %d %9d %6.2f %6.2f %6.2f | %7.1f %7.1f | %7.1f %7.1f | %9.0f | c*=%.2e | total2^%.0f"%(cfg,m,s,L,N,Nf,dense,sparse,denseF,sparseF,ex,cstar,row["total_rel_phase_log2"]))
json.dump(out,open("table_rows.json","w"))
