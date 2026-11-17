default(parisizemax,2000000000);
read("scripts/lib.gp");
m=eval(getenv("M")); l=eval(getenv("L")); pop=eval(getenv("POP"));
lib_init(m);
K=if(pop==2,1,if(l<=8,20,if(l==9,8,4)));
print("# stage1_exhaustive m=",m," l=",l," pop=",pop," draws=",K," modulus=",MODINT," gp=",GPVER);
print("# fields: m,l,pop,k,seed,N0V,N1V,np_same,np_cross,rat_same,rat_cross,viol,triples,triples_formula,f4_ok,rootfail,x3zero,pc2_rat,pc2_diff");
{
for(kk=if(pop==2,0,1),if(pop==2,0,K),
  my(seed=2026102100+1000000*m+10000*l+100*pop+kk,VV,el,Vn,n,N0V=0,N1V=0,x1,x2,r,t1,tt,np_same=0,np_cross=0,rat_same=0,rat_cross=0,viol=0,triples=0,rootfail=0,x3zero=0,pc2_rat=0,pc2_diff=0,perm,lab,j,tmp,nr,tf);
  setrand(seed); VV=makeV(l,pop); el=VV[1];
  Vn=vector(#el-1,i,el[i+1]); n=#Vn;
  for(i=1,n, if(TYT[Vn[i]+1]==0,N0V++,N1V++));
  \\ PC2: seeded random permutation of the type labels over V\{0}
  setrand(seed+900000000);
  perm=vector(n,i,i);
  forstep(i=n,2,-1, j=random(i)+1; tmp=perm[i]; perm[i]=perm[j]; perm[j]=tmp);
  lab=vector(n,i,TYT[Vn[perm[i]]+1]);
  for(i=1,n,
    x1=Vn[i]; t1=TYT[x1+1];
    \\ equal abscissas: exactly one finite root x(2P)
    r=roots3(x1,x1); triples+=#r; if(S3(x1,x1,r[1]),rootfail++); if(r[1]==0,x3zero++);
    for(jj=i+1,n,
      x2=Vn[jj]; tt=(t1==TYT[x2+1]);
      r=roots3(x1,x2);
      if(tt,np_same++,np_cross++);
      if(#r>0!=tt, viol++);
      if(#r>0,
        triples+=#r;
        for(u=1,#r, if(S3(x1,x2,r[u]),rootfail++); if(r[u]==0,x3zero++));
        if(tt,rat_same++,rat_cross++);
        pc2_rat++; if(lab[i]!=lab[jj], pc2_diff++))));
  tf=2*binomial(N0V,2)+N0V+2*binomial(N1V,2)+N1V;
  print(csvline([m,l,pop,kk,seed,N0V,N1V,np_same,np_cross,rat_same,rat_cross,viol,triples,tf,if(triples==tf,1,0),rootfail,x3zero,pc2_rat,pc2_diff])));
}
quit
