default(parisizemax,2000000000);
read("scripts/lib.gp");
m=eval(getenv("M")); l=eval(getenv("L")); pop=eval(getenv("POP"));
lib_init(m);
E0=ellinit([1,0,0,0,1],GEN);
K=if(pop==2,1,if(l<=8,20,if(l==9,8,4)));
FIELDS="m,l,pop,k,seed,N0V,N1V,nT0,nT1,tgt_bad,E_solves0,S_solves0,Esh_solves0,E_rel0,S_rel0,S_k0pure0,S_cross0,S_k1pure0,mism,mism_sh,rows_E,rows_S,rows_eq,cls_E,cls_E_used,cls_S,cls_S_t1,cls_S_used,cls_S_t1_used,E_solves1,S_solves1,E_rel1,S_rel1,S_k1pure1,S_cross1,S_k0pure1,pc1_ok,P_N0,P_solves0,P_rel0,tE1,tS1,tEa1,tEs1,tE2,tS2,tEa2,tEs2,tE3,tS3,tEa3,tEs3";
print("# stage2_sieve m=",m," l=",l," pop=",pop," draws=",K," modulus=",MODINT," gp=",GPVER);
print("# fields: ",FIELDS);

splitrel(v) = { my(a=List(),b=List(),c=List(),x,y,tx,ty);
  for(i=1,#v, x=v[i]\Q; y=v[i]%Q; tx=TYT[x+1]; ty=TYT[y+1]; if(tx!=ty, listput(b,v[i]), if(tx==0, listput(a,v[i]), listput(c,v[i]))));
  [Vecsmall(a),Vecsmall(b),Vecsmall(c)] }
nrel(R) = { my(s=0); for(i=1,#R, s+=#R[i]); s }
\\ orbit-canonical rows (target orbit b, canonical classes of x1,x2) of an arm's relations
rowset(R) = { my(L=List(),b,x,y,cx,cy);
  for(ti=1,#R, b=(ti-1)\m; for(i=1,#R[ti], x=R[ti][i]\Q; y=R[ti][i]%Q; cx=canon(x); cy=canon(y); listput(L,b*Q*Q+min(cx,cy)*Q+max(cx,cy))));
  vecsort(Vecsmall(L),,8) }
usedcls(R) = { my(L=List(),x,y); for(ti=1,#R, for(i=1,#R[ti], x=R[ti][i]\Q; y=R[ti][i]%Q; listput(L,canon(x)); listput(L,canon(y)))); vecsort(Vecsmall(L),,8) }
clsof(base) = { my(L=List()); for(i=1,#base, listput(L,canon(base[i]))); vecsort(Vecsmall(L),,8) }
tm(base,inV,tg,armE) = { my(t0=getabstime()); sieve(base,inV,tg,armE); getabstime()-t0 }
orbit(x) = { my(v=vector(m),y=x); for(j=1,m, v[j]=y; y=sq(y)); v }

{
for(kk=if(pop==2,0,1),if(pop==2,0,K),
  my(seed=2026102100+1000000*m+10000*l+100*pop+kk,VV,el,inV,w,BE,BS,BEsh,TG0,TG1,seen,xs,P,x,tgt_bad=0,perm,n,j,tmp,
     el2,inV2,BE2,r,rE,rS,rEs,rE1,rS1,sp,spk,mism=0,mism_sh=0,rE_k,rowsE,rowsS,uE,uS,uSall,t1used,
     pc1_ok=0,P_N0=-1,P_solves0=-1,P_rel0=-1,Tt=vector(12),ord,tA,tB,N0V=0,N1V=0,nsE,nsS,nsEs,nsE1,nsS1,sp1,cls_S_t1=0,rS_k0);
  setrand(seed); VV=makeV(l,pop); el=VV[1]; inV=VV[2]; w=VV[3];
  BE=mkbase(el,1); BS=mkbase(el,0); N0V=#BE; N1V=#BS-#BE;
  \\ targets (own seed stream)
  setrand(seed+500000000);
  seen=List(); xs=List();
  while(#xs<40, P=random(E0); if(#P<2,next); x=fint(P[1]); if(x==0,next); if(TYT[x+1]!=0,tgt_bad++);
    if(setsearch(Set(Vec(seen)),canon(x)),next); listput(seen,canon(x)); listput(xs,x));
  TG0=Vecsmall(concat(vector(40,b,orbit(xs[b]))));
  seen=List(); xs=List();
  while(#xs<40, x=random(QM1)+1; if(TYT[x+1]!=1,next); if(setsearch(Set(Vec(seen)),canon(x)),next); listput(seen,canon(x)); listput(xs,x));
  TG1=Vecsmall(concat(vector(40,b,orbit(xs[b]))));
  \\ NC1 shuffle (own seed stream)
  setrand(seed+900000000);
  perm=vector(#BE,i,i); forstep(i=#BE,2,-1, j=random(i)+1; tmp=perm[i]; perm[i]=perm[j]; perm[j]=tmp);
  BEsh=Vecsmall(vector(#BE,i,BE[perm[i]]));
  \\ ---- PC1 first (positive control, E-only base at dimension l+1 vs l), l<=9 ----
  if(l<=9,
    el2=concat(el,vectorsmall(#el,i,bitxor(el[i],w))); inV2=vectorsmall(Q); for(i=1,#el2,inV2[el2[i]+1]=1);
    BE2=mkbase(el2,1); r=sieve(BE2,inV2,TG0,1); pc1_ok=1; P_N0=#BE2; P_solves0=r[1]; P_rel0=nrel(r[2]));
  \\ ---- arms on K0 targets ----
  rE=sieve(BE,inV,TG0,1); rS=sieve(BS,inV,TG0,0); rEs=sieve(BEsh,inV,TG0,1);
  nsE=rE[1]; nsS=rS[1]; nsEs=rEs[1];
  sp=vector(#TG0,i,splitrel(rS[2][i]));
  rS_k0=vector(#TG0,i,sp[i][1]);
  for(i=1,#TG0, if(rE[2][i]!=sp[i][1], mism++); if(rE[2][i]!=rEs[2][i], mism_sh++));
  rowsE=rowset(rE[2]); rowsS=rowset(rS_k0);
  uE=usedcls(rE[2]); uS=usedcls(rS_k0); uSall=usedcls(rS[2]);
  t1used=0; for(i=1,#uSall, if(TYT[uSall[i]+1]==1,t1used++));
  my(cs=clsof(BS)); for(i=1,#cs, if(TYT[cs[i]+1]==1,cls_S_t1++));
  \\ ---- K1-type targets ----
  rE1=sieve(BE,inV,TG1,1); rS1=sieve(BS,inV,TG1,0);
  sp1=vector(#TG1,i,splitrel(rS1[2][i]));
  \\ ---- timing: ES scheme and NC1 scheme, 3 repeats ordered ABBA / BAAB / ABBA (CPU ms, getabstime) ----
  for(rep=1,3,
    ord=if(rep==2,[1,0,0,1],[0,1,1,0]); tA=0; tB=0;
    for(u=1,4, if(ord[u]==0, tA+=tm(BE,inV,TG0,1), tB+=tm(BS,inV,TG0,0)));
    Tt[4*(rep-1)+1]=tA; Tt[4*(rep-1)+2]=tB;
    tA=0; tB=0;
    for(u=1,4, if(ord[u]==0, tA+=tm(BE,inV,TG0,1), tB+=tm(BEsh,inV,TG0,1)));
    Tt[4*(rep-1)+3]=tA; Tt[4*(rep-1)+4]=tB);
  print(csvline(concat([[m,l,pop,kk,seed,N0V,N1V,#TG0,#TG1,tgt_bad,nsE,nsS,nsEs,nrel(rE[2]),nrel(rS[2]),
     nrel(rS_k0),nrel(vector(#TG0,i,sp[i][2])),nrel(vector(#TG0,i,sp[i][3])),mism,mism_sh,#rowsE,#rowsS,if(rowsE==rowsS,1,0),
     #clsof(BE),#uE,#clsof(BS),cls_S_t1,#uS,t1used,rE1[1],rS1[1],nrel(rE1[2]),nrel(rS1[2]),nrel(vector(#TG1,i,sp1[i][3])),
     nrel(vector(#TG1,i,sp1[i][2])),nrel(vector(#TG1,i,sp1[i][1])),pc1_ok,P_N0,P_solves0,P_rel0],Vec(Tt)])));
);
}
quit
