default(parisizemax,2000000000);
\\ EXPLORATORY supplement (written after seeing out/verdict.txt; not part of the frozen protocol, does not change the verdict):
\\ PC1 for every cell incl. the single-draw poly cells: ratio P_rel/E_rel and a normal z for P/(P+E) against p=0.8
\\ (the value implied by an exact factor 4). Reads out/stage2_m*.csv only.
{
for(i=1,4, my(m=[11,13,17,19][i],L=readstr(Str("out/stage2_m",m,".csv")),names=[],tot=Map());
  for(k=1,#L, my(s=L[k]); if(#s==0,next);
    if(Vec(s)[1]=="#", my(v=Vec(s)); if(#s>10 && concat(vector(10,j,v[j]))=="# fields: " && #names==0, names=strsplit(concat(vector(#v-10,j,v[j+10])),",")); next);
    my(r=eval(Str("[",s,"]")),ix(nm)=my(f=0); for(q=1,#names, if(names[q]==nm,f=q)); f);
    if(r[ix("pc1_ok")]==1,
      my(l=r[ix("l")],pop=r[ix("pop")],P=r[ix("P_rel0")],E=r[ix("E_rel0")],n=P+E,z,key=[l,pop],cur);
      cur=if(mapisdefined(tot,key),mapget(tot,key),[0,0,0]); mapput(tot,key,[cur[1]+P,cur[2]+E,cur[3]+1])));
  my(ks=Vec(Mat(tot)[,1]~));
  ks=vecsort(Vec(tot)[1..#Vec(tot)],,0);
  my(M2=Mat(tot),keys=M2[,1]);
  for(j=1,#keys, my(key=keys[j],v=mapget(tot,key),P=v[1],E=v[2],n=P+E,z=(P-0.8*n)/sqrt(n*0.16));
    print("SUPP m=",m," l=",key[1]," pop=",if(key[2]==1,"rand","poly")," draws=",v[3]," P_rel=",P," E_rel=",E," ratio=",Strprintf("%.4f",P/E)," z(P/(P+E) vs 0.8)=",Strprintf("%.3f",z))));
}
quit
