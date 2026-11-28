\\ Frozen public EC S4 relation pipeline and same-target Floyd rho.
p=1009;r=1013;B=16;X='X;Y='Y;W='W;T='T;
s3(u,v,z)=(u-v)^2*z^2-2*((u+v)*(u*v+Mod(1,p))+2*Mod(14,p))*z+(u*v-Mod(1,p))^2-4*Mod(14,p)*(u+v);
tCold=getabstime();wCold=getwalltime();
E=ellinit([Mod(1,p),Mod(14,p)]);G=[Mod(0,p),Mod(425,p)];Q=[Mod(42,p),sqrt(Mod(42^3+42+14,p))];
if(ellcard(E)!=r || !isprime(r) || !ellisoncurve(E,G) || ellorder(E,G)!=r || !ellisoncurve(E,Q) || ellorder(E,Q)!=r,error("curve/G/Q guard"));
build_base()={my(n=0,xx=1,q);fbx=vector(B);fb=vector(B);negfb=vector(B);while(n<B && xx<p,q=Mod(xx^3+xx+14,p);if(issquare(q),n++;fbx[n]=xx;fb[n]=[Mod(xx,p),sqrt(q)];negfb[n]=ellsub(E,[0],fb[n]);if(!ellisoncurve(E,fb[n]),error("base point guard")));xx++);if(n!=B || fbx[B]!=40 || Q[1]==fb[B][1],error("base/target guard"))};
build_base();commonSetupCpu=getabstime()-tCold;commonSetupWall=getwalltime()-wCold;
print("header p=",p," order=",r," G=",G," Q=",Q," base_x=",fbx," common_setup_cpu_ms=",commonSetupCpu," common_setup_wall_ms=",commonSetupWall);
rows=List();rhs=List();rankNow=0;targetsTried=0;polyEvals=0;signedPatterns=0;groupAdds=0;resultants=0;allZeros=0;allWitnesses=0;targetScalarMuls=0;targetComposeAdds=0;
row_for(i,j,k,mask,b)={my(v=vector(B+1,h,0));v[i]+=if(mask%2,-1,1);v[j]+=if((mask\2)%2,-1,1);v[k]+=if((mask\4)%2,-1,1);v[B+1]=-b;v};
rank_of(L)={my(V=Vec(L),M);if(#V==0,return(0));M=matrix(#V,B+1,ii,jj,Mod(V[ii][jj],r));matrank(M)};
sum_for(i,j,k,mask)={my(P1=if(mask%2,negfb[i],fb[i]),P2=if((mask\2)%2,negfb[j],fb[j]),P3=if((mask\4)%2,negfb[k],fb[k]));elladd(E,elladd(E,P1,P2),P3)};
collect_target(a0,b0)={my(R=elladd(E,ellmul(E,G,a0),ellmul(E,Q,b0)),z,S,zeros=0,witness=0,oldrank=rankNow,fw,value,found,v,newrank);
  targetScalarMuls+=2;targetComposeAdds++;
  if(R==[0],print("skip_infinity a=",a0," b=",b0);return());
  z=R[1];S=polresultant(s3(X,Y,T),s3(T,W,z),T);resultants++;
  if([poldegree(S,X),poldegree(S,Y),poldegree(S,W)]!=[4,4,4],error("S4 degree guard"));
  targetsTried++;
  for(i=1,B,for(j=1,B,
    fw=subst(subst(S,X,fbx[i]),Y,fbx[j]);
    for(k=1,B,
      value=subst(fw,W,fbx[k]);polyEvals++;if(value==0,zeros++);
      found=0;
      for(mask=0,7,
        signedPatterns++;groupAdds+=2;
        if(sum_for(i,j,k,mask)==R,
          found++;witness++;
          v=row_for(i,j,k,mask,b0);
          listput(~rows,v);newrank=rank_of(rows);
          if(newrank>rankNow,
            listput(~rhs,a0);rankNow=newrank;
            print("rel=",[a0,b0,i,j,k,mask]);
          ,
            listpop(~rows)
          );
        );
      );
      if((value==0)!=(found>0),error("S4 versus signed group mismatch"));
    );
  ));
  allZeros+=zeros;allWitnesses+=witness;
  print("target a=",a0," b=",b0," x=",lift(z)," S4_zero_triples=",zeros," signed_witnesses=",witness," rank_before=",oldrank," rank_after=",rankNow);
};
run_ic()={my(collect0=getabstime(),collectW0=getwalltime(),rankB0,collectCpu,collectWall,V,C,A,Yrhs,Sol,d);
  for(a0=1,64,if(rankNow<16,collect_target(a0,0)));
  rankB0=rankNow;
  for(a0=0,64,if(rankNow<17,collect_target(a0,1)));
  if(rankB0!=16 || rankNow!=17,
    print("ic_censored phase0_rank=",rankB0," final_rank=",rankNow," cap=64_per_phase targets=",targetsTried);
  ,
    V=Vec(rows);C=Vec(rhs);A=matrix(#V,B+1,ii,jj,Mod(V[ii][jj],r));Yrhs=vector(#C,ii,Mod(C[ii],r))~;
    if(matrank(A)!=17,error("solve matrix rank guard"));
    Sol=matsolve(A,Yrhs);if(A*Sol!=Yrhs,error("modular solve residual"));
    d=lift(Sol[B+1]);
    if(ellmul(E,G,d)!=Q,error("target log verification"));
    for(i=1,B,if(ellmul(E,G,lift(Sol[i]))!=fb[i],error("base log verification")));
    print("ic_solution target_log=",d," base_logs=",vector(B,i,lift(Sol[i]))," all_point_checks=true");
  );
  collectCpu=getabstime()-collect0;collectWall=getwalltime()-collectW0;
  print("ic_summary phase0_rank=",rankB0," final_rank=",rankNow," targets=",targetsTried," resultants=",resultants,
        " poly_evals=",polyEvals," signed_patterns=",signedPatterns," signed_sum_adds=",groupAdds,
        " target_scalar_muls=",targetScalarMuls," target_compose_adds=",targetComposeAdds,
        " S4_zeros=",allZeros," signed_witnesses=",allWitnesses,
        " cold_setup_cpu_ms=",commonSetupCpu," cold_setup_wall_ms=",commonSetupWall,
        " collection_solve_cpu_ms=",collectCpu," collection_solve_wall_ms=",collectWall,
        " cold_total_cpu_ms=",getabstime()-tCold," cold_total_wall_ms=",getwalltime()-wCold);
};
rho_step(S)={my(P=S[1],u=S[2],v=S[3],bucket=if(P==[0],0,lift(P[1])%3));
  if(bucket==0,return([elladd(Erho,P,Grho),(u+1)%r,v]));
  if(bucket==1,return([elladd(Erho,P,Qrho),u,(v+1)%r]));
  [elladd(Erho,P,P),(2*u)%r,(2*v)%r]
};
rho_run()={my(t0=getabstime(),w0=getwalltime(),setupC,setupW,P,S,H,collision=0,den,delta,logq=-1,iterations=0);
  Erho=ellinit([Mod(1,p),Mod(14,p)]);Grho=[Mod(0,p),Mod(425,p)];Qrho=[Mod(42,p),sqrt(Mod(42^3+42+14,p))];
  if(ellcard(Erho)!=r || !ellisoncurve(Erho,Grho) || !ellisoncurve(Erho,Qrho) || ellorder(Erho,Grho)!=r || ellorder(Erho,Qrho)!=r,error("rho setup guard"));
  setupC=getabstime()-t0;setupW=getwalltime()-w0;
  P=elladd(Erho,Grho,Qrho);S=[P,1,1];H=S;
  while(!collision && iterations<10000,
    S=rho_step(S);H=rho_step(rho_step(H));iterations++;
    if(S[1]==H[1],collision=1);
  );
  if(collision,
    den=Mod(S[3]-H[3],r);delta=Mod(H[2]-S[2],r);
    if(den!=0,logq=lift(delta/den));
  );
  if(logq>=0 && ellmul(Erho,Grho,logq)!=Qrho,error("rho log verification"));
  print("rho_summary collision=",collision," degenerate=",collision && logq<0," iterations=",iterations,
        " step_group_adds=",3*iterations," initial_group_adds=1 target_log=",logq," verified=",logq>=0,
        " setup_cpu_ms=",setupC," setup_wall_ms=",setupW,
        " total_cpu_ms=",getabstime()-t0," total_wall_ms=",getwalltime()-w0);
};
run_ic();rho_run();print("complete S4 pipeline and same-target rho");
