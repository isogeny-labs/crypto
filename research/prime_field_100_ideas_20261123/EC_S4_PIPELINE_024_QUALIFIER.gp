\\ Independent accepted-row replay of frozen #24 producer stdout.
p=1009;r=1013;B=16;X='X;Y='Y;W='W;T='T;
E=ellinit([Mod(1,p),Mod(14,p)]);G=[Mod(0,p),Mod(425,p)];Q=[Mod(42,p),sqrt(Mod(42^3+42+14,p))];
if(ellcard(E)!=r || ellorder(E,G)!=r || ellorder(E,Q)!=r,error("qualifier curve guard"));
s3q(u,v,z)=(u-v)^2*z^2-2*((u+v)*(u*v+Mod(1,p))+2*Mod(14,p))*z+(u*v-Mod(1,p))^2-4*Mod(14,p)*(u+v);
build_base()={my(n=0,xx=1,q);fb=vector(B);fbx=vector(B);while(n<B && xx<p,q=Mod(xx^3+xx+14,p);if(issquare(q),n++;fb[n]=[Mod(xx,p),sqrt(q)];fbx[n]=xx);xx++);if(n!=B || fbx[B]!=40,error("qualifier base guard"))};
build_base();
row_for(i,j,k,m,b)={my(v=vector(B+1,h,0));v[i]+=if(m%2,-1,1);v[j]+=if((m\2)%2,-1,1);v[k]+=if((m\4)%2,-1,1);v[B+1]=-b;v};
rankrows(V)={my(M);M=matrix(#V,B+1,ii,jj,Mod(V[ii][jj],r));matrank(M)};
verify_file()={my(lines=readstr("research/prime_field_100_ideas_20261123/EC_S4_PIPELINE_024.stdout.txt"),V=List(),C=List(),count=0,head=0,solLine=0,summary=0,rho=0,parts,t,a0,b0,i,j,k,m,R,S4v,P1,P2,P3,Psum,row,A,Yrhs,Sol,d);
  for(h=1,#lines,
    parts=strsplit(lines[h],"=");
    if(#parts>=2 && parts[1]=="header p",head++);
    if(#parts>=2 && parts[1]=="rel",
      t=eval(parts[2]);if(#t!=6,error("qualifier row width"));
      a0=t[1];b0=t[2];i=t[3];j=t[4];k=t[5];m=t[6];
      if(a0<0 || a0>64 || b0<0 || b0>1 || i<1 || i>B || j<1 || j>B || k<1 || k>B || m<0 || m>7,error("qualifier row range"));
      R=elladd(E,ellmul(E,G,a0),ellmul(E,Q,b0));if(R==[0],error("qualifier infinity"));
      P1=if(m%2,ellsub(E,[0],fb[i]),fb[i]);P2=if((m\2)%2,ellsub(E,[0],fb[j]),fb[j]);P3=if((m\4)%2,ellsub(E,[0],fb[k]),fb[k]);
      Psum=elladd(E,elladd(E,P1,P2),P3);if(Psum!=R,error("qualifier point relation"));
      S4v=polresultant(s3q(X,Y,T),s3q(T,W,R[1]),T);
      if(subst(subst(subst(S4v,X,fbx[i]),Y,fbx[j]),W,fbx[k])!=0,error("qualifier S4 relation"));
      row=row_for(i,j,k,m,b0);listput(~V,row);listput(~C,a0);count++;
    );
    if(#parts>=2 && parts[1]=="ic_solution target_log",solLine++);
    if(#parts>=2 && parts[1]=="ic_summary phase0_rank",summary++);
    if(#parts>=2 && parts[1]=="rho_summary collision",rho++);
  );
  if(count!=17 || solLine!=1 || summary!=1 || rho!=1,error("qualifier line count/sentinel"));
  V=Vec(V);C=Vec(C);if(rankrows(V)!=17,error("qualifier rank"));
  A=matrix(#V,B+1,ii,jj,Mod(V[ii][jj],r));Yrhs=vector(#C,ii,Mod(C[ii],r))~;Sol=matsolve(A,Yrhs);
  if(A*Sol!=Yrhs,error("qualifier linear residual"));
  d=lift(Sol[B+1]);if(ellmul(E,G,d)!=Q,error("qualifier recovered target"));
  for(i=1,B,if(ellmul(E,G,lift(Sol[i]))!=fb[i],error("qualifier base log")));
  print("qualifier accepted_rows=",count," exact_group_and_S4_rows=",count," rank=",rankrows(V)," target_log=",d," all_base_logs_verified=true");
};
verify_file();print("complete independent S4 row replay");
