\\ Independent scalar replay of the frozen #6 V2 point-orbit receipt.
\\ Point coordinates and scalars are fixed from V2 stdout, not imported from V2 code.
p=1009;r=79;B=12;E=ellinit([Mod(0,p),Mod(3,p)]);
G=[Mod(378,p),Mod(580,p)];R=[Mod(385,p),Mod(631,p)];
ks=[1,55,23,2,31,46,3,7,69,4,62,13];
xs=[378,112,519,311,279,419,308,166,535,914,794,310];
ys=[580,580,580,105,105,105,353,353,353,680,680,680];
if(ellcard(E)!=948 || ellorder(E,G)!=r || ellmul(E,G,5)!=R,error("curve or target anchor"));
if(Mod(374,p)^3!=1 || Mod(374,p)==1 || ellmul(E,G,55)!=[Mod(374*378,p),Mod(580,p)],error("automorphism eigenvalue anchor"));
for(i=1,B,
  if(ellmul(E,G,ks[i])!=[Mod(xs[i],p),Mod(ys[i],p)],error("base scalar/point mismatch"));
  if(ks[if(i%3==0,i-2,i+1)]!=(55*ks[i])%r,error("orbit scalar mismatch"));
);
index(i,j,k,m)=8*((i-1)*B*B+(j-1)*B+k-1)+m+1;
nextidx(i)=if(i%3==0,i-2,i+1);
signed_scalar(i,j,k,m)=lift(Mod(if(m%2,-ks[i],ks[i])+if((m\2)%2,-ks[j],ks[j])+if((m\4)%2,-ks[k],ks[k]),r));
make_row(i,j,k,m)={my(v=vector(B+1,t,0));v[i]+=if(m%2,-1,1);v[j]+=if((m\2)%2,-1,1);v[k]+=if((m\4)%2,-1,1);v[B+1]=-1;v};
rank_of(rows,ncols)={my(M);if(#rows==0,return(0));M=matrix(ncols,#rows,i,j,Mod(rows[j][i],r));matrank(M)};
project(v)={my(q=vector(5,t,0));for(i=1,B,q[1+(i-1)\3]+=v[i]*lift(Mod(55,r)^((i-1)%3)));q[5]=v[13];q};
verify()={my(mask=vector(B^3*8,t,0),qmask=vector(B^3*8,t,0),rows=List(),proj=List(),
  hits=0,canon=0,qhits=0,s,i1,j1,k1,i2,j2,k2,key,key1,key2,pre=[5,36,38],v);
  for(i=1,B,for(j=1,B,for(k=1,B,for(m=0,7,
    if(signed_scalar(i,j,k,m)==5,
      hits++;mask[index(i,j,k,m)]=1;v=make_row(i,j,k,m);listput(rows,v);listput(proj,project(v));
    );
  ))));
  for(i=1,B,for(j=1,B,for(k=1,B,
    key=(i-1)*B*B+(j-1)*B+k-1;
    i1=nextidx(i);j1=nextidx(j);k1=nextidx(k);
    key1=(i1-1)*B*B+(j1-1)*B+k1-1;
    i2=nextidx(i1);j2=nextidx(j1);k2=nextidx(k1);
    key2=(i2-1)*B*B+(j2-1)*B+k2-1;
    if(key<=key1 && key<=key2,
      canon++;
      for(m=0,7,
        s=signed_scalar(i,j,k,m);
        if(s==pre[1],qhits++;qmask[index(i,j,k,m)]=1);
        if(s==pre[2],qhits++;qmask[index(i1,j1,k1,m)]=1);
        if(s==pre[3],qhits++;qmask[index(i2,j2,k2,m)]=1);
      );
    );
  )));
  rows=Vec(rows);proj=Vec(proj);
  if(canon!=576 || hits!=153 || qhits!=hits || qmask!=mask,error("scalar witness orbit mismatch"));
  for(t=1,#proj,if((proj[t][1]+2*proj[t][2]+3*proj[t][3]+4*proj[t][4]-5)%r!=0,error("projected scalar identity")));
  if(rank_of(rows,13)!=12 || rank_of(proj,5)!=4,error("rank replay mismatch"));
  print("qualifier anchored_points=12 target_scalar=5 lambda=55 target_preimage_scalars=",pre,
        " full_candidates=",B^3*8," orbit_reps=",canon," full_hits=",hits," quotient_hits=",qhits,
        " witness_mask_equal=true full_rank=",rank_of(rows,13)," projected_rank=",rank_of(proj,5),
        " projected_scalar_identities=true");
};
verify();print("complete independent scalar replay");
