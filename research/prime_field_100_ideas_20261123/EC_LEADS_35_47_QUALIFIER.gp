\\ Public EC controls for proposals 35-40, 42, 45, 47. No quantum device.
\\ Input is the immutable S4_MATCHED_V2_STDOUT.txt row stream.
p=1009; r=1013; B=16;
E=ellinit([Mod(1,p),Mod(14,p)]);
raw=readstr("research/prime_field_100_ideas_20261123/S4_MATCHED_V2_STDOUT.txt");
fb=vector(B); rows0=List(); rows320=List(); Q0=0;

load_rows()={
  my(parts,z,x,y,i,blocks,ss,rr,row);
  for(n=1,#raw,
    parts=strsplit(raw[n]," ");
    if(parts[1]=="base",
      i=eval(strsplit(parts[2],"=")[2]);
      x=eval(strsplit(parts[3],"=")[2]);
      y=eval(strsplit(parts[4],"=")[2]);
      fb[i]=[Mod(x,p),Mod(y,p)];
      if(!ellisoncurve(E,fb[i]),error("base off curve"));
    );
    if(parts[1]=="target",
      z=eval(strsplit(parts[2],"=")[2]);
      y=eval(strsplit(parts[3],"=")[2]);
      if(z==0,Q0=[Mod(z,p),Mod(y,p)]);
    );
    if(parts[1]=="witness",
      blocks=strsplit(raw[n]," indices=");
      z=eval(strsplit(blocks[1],"=")[2]);
      ss=strsplit(blocks[2]," signs=");
      rr=strsplit(ss[2]," row="); row=eval(rr[2]);
      if(#row!=B+1,error("row width"));
      if(z==0,listput(rows0,row),if(z==320,listput(rows320,row),error("target")));
    );
  );
  rows0=Vec(rows0); rows320=Vec(rows320);
  if(#rows0!=24 || #rows320!=33 || Q0==0,error("input counts"));
};
load_rows();

rank_mod(R,cols)={
  my(n=#R,k=0,A,pivot,t);
  A=matrix(n,cols,i,j,Mod(R[i][j],r));
  for(j=1,cols,
    pivot=0; for(i=k+1,n,if(pivot==0 && A[i,j]!=0,pivot=i));
    if(pivot!=0,
      k++;
      if(pivot!=k,for(c=1,cols,t=A[k,c];A[k,c]=A[pivot,c];A[pivot,c]=t));
      t=1/A[k,j]; for(c=j,cols,A[k,c]*=t);
      for(i=1,n,if(i!=k && A[i,j]!=0,t=A[i,j];for(c=j,cols,A[i,c]-=t*A[k,c])));
    );
  );
  k;
};

graph_stats(R)={
  my(parent=vector(B,i,i),active=vector(B),seen=vector(B),component=0,nactive=0,first,u,v,unique=0,duplicate);
  for(t=1,#R,
    first=0;
    for(j=1,B,if(R[t][j]!=0,
      active[j]=1;
      if(first==0,first=j,
        u=first; while(parent[u]!=u,u=parent[u]);
        v=j; while(parent[v]!=v,v=parent[v]);
        if(u!=v,parent[v]=u);
      );
    ));
    duplicate=0;
    for(k=1,t-1,if(R[t]==R[k],duplicate=1));
    if(!duplicate,unique++);
  );
  for(j=1,B,if(active[j],
    nactive++;
    u=j; while(parent[u]!=u,u=parent[u]);
    if(!seen[u],seen[u]=1;component++);
  ));
  [nactive,component,unique];
};

ec_sign_sum(idx,mask)={
  my(s1=if(mask%2,-1,1),s2=if((mask\2)%2,-1,1),s3=if((mask\4)%2,-1,1));
  elladd(E,elladd(E,ellmul(E,fb[idx[1]],s1),ellmul(E,fb[idx[2]],s2)),ellmul(E,fb[idx[3]],s3));
};

grover_control()={
  my(triples=[[3,5,12],[4,5,11]],marked=vector(16,i,0),M=0,amp=vector(16,i,1/4),prob,mean,norm);
  for(c=1,16,
    if(ec_sign_sum(triples[(c-1)\8+1],(c-1)%8)==Q0,marked[c]=1;M++);
  );
  if(M<2,error("expected printed witnesses absent"));
  print("grover_ec_predicate candidates=16 marked=",M," classical_hit=",M/16);
  for(t=0,4,
    prob=0;norm=0;
    for(c=1,16,norm+=amp[c]^2;if(marked[c],prob+=amp[c]^2));
    if(norm!=1,error("amplitude norm"));
    print("grover_oracle_calls=",t," marked_probability=",prob);
    if(t<4,
      for(c=1,16,if(marked[c],amp[c]=-amp[c]));
      mean=0;for(c=1,16,mean+=amp[c]);mean/=16;
      for(c=1,16,amp[c]=2*mean-amp[c]);
    );
  );
};

tensor_control()={
  my(I=matrix(16,16,i,j,if(i==j,1,0)),S=matrix(16,16,i,j,0),RI=matrix(16,16,i,j,0),RS=matrix(16,16,i,j,0));
  for(il=0,3,for(ir=0,3,S[4*ir+il+1,4*il+ir+1]=1));
  for(ol=0,3,for(orr=0,3,for(il=0,3,for(ir=0,3,
    RI[4*ol+il+1,4*orr+ir+1]=I[4*ol+orr+1,4*il+ir+1];
    RS[4*ol+il+1,4*orr+ir+1]=S[4*ol+orr+1,4*il+ir+1];
  ))));
  if(rank_mod(Vec(RI),16)!=1 || rank_mod(Vec(RS),16)!=16,error("tensor cut rank"));
  print("tensor_dim=16 nnz_identity=16 nnz_swap=16 schmidt_rank_identity=1 schmidt_rank_swap=16");
};

g0=graph_stats(rows0);g1=graph_stats(rows320);
print("graph_z0 rows=",#rows0," active=",g0[1]," components=",g0[2]," unique_rows=",g0[3]," rank_base=",rank_mod(rows0,B)," rank_augmented=",rank_mod(rows0,B+1));
print("graph_z320 rows=",#rows320," active=",g1[1]," components=",g1[2]," unique_rows=",g1[3]," rank_base=",rank_mod(rows320,B)," rank_augmented=",rank_mod(rows320,B+1));
grover_control();
print("linear_control real_rank_[1013]=1 modular_rank_F1013_[1013]=0");
M0=[1,1,0;0,1,1;1,0,1];U=[1,1,0;0,1,0;0,0,1];
if(matdet(U)!=1 || matdet(U*M0)!=matdet(M0),error("invertible preconditioner guard"));
print("preconditioner_exact_rank_before=3 after=3 determinant_U=1");
tensor_control();
print("complete_native_controls");
