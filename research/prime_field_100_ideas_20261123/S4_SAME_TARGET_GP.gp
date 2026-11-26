\\ Frozen same-target S4 generic construction vs direct specialization.
\\ Public curve and factor base match S4_MATCHED_GP_V2.gp; four ABBA passes.
p=1009; a=Mod(1,p); b=Mod(14,p); r=1013; B=16;
X='X; Y='Y; W='W; T='T; Z='Z;
s3(u,v,z)=(u-v)^2*z^2-2*((u+v)*(u*v+a)+2*b)*z+(u*v-a)^2-4*b*(u+v);
E=ellinit([a,b]); R=[Mod(0,p),sqrt(b)];
if(ellcard(E)!=r || !isprime(r) || !ellisoncurve(E,R) || ellorder(E,R)!=r,error("curve/target guard"));
build_base()={my(n=0,xx=1,q);fbx=vector(B);fb=vector(B);while(n<B && xx<p,q=Mod(xx^3+xx+14,p);if(issquare(q),n++;fbx[n]=xx;fb[n]=[Mod(xx,p),sqrt(q)];if(!ellisoncurve(E,fb[n]),error("base point guard")));xx++);if(n!=B,error("base count guard"))};
build_base();
print("header p=",p," order=",r," target=",R," base_x=",fbx," ordered_triples=",B^3," passes=generic,special,special,generic");
referenceRows=0;
run_arm(mode,pass)={
  my(t0=getabstime(),w0=getwalltime(),Sfull,S,constructionCpu,constructionWall,
     t1,w1,t2,w2,setupCpu,setupWall,evaluationCpu,evaluationWall,totalCpu,totalWall,
     support=0,zeros=0,hits=0,adds=0,rows=List(),rowVec,M,rankRows=0,fw,value,
     e1,e2,e3,P1,P2,P3,Qsum,found,v);
  if(mode==1,
    Sfull=polresultant(s3(X,Y,T),s3(T,W,Z),T);
    S=subst(Sfull,Z,Mod(0,p)),
    S=polresultant(s3(X,Y,T),s3(T,W,Mod(0,p)),T)
  );
  constructionCpu=getabstime()-t0;constructionWall=getwalltime()-w0;
  if([poldegree(S,X),poldegree(S,Y),poldegree(S,W)]!=[4,4,4],error("S4 degree guard"));
  for(ii=0,4,for(jj=0,4,for(kk=0,4,v=polcoef(polcoef(polcoef(S,ii,X),jj,Y),kk,W);if(v!=0,support++))));
  if(support!=88,error("same-target support guard"));
  t1=getabstime();w1=getwalltime();setupCpu=t1-t0;setupWall=w1-w0;
  for(i=1,B,for(j=1,B,
    fw=subst(subst(S,X,Mod(fbx[i],p)),Y,Mod(fbx[j],p));
    for(k=1,B,
      value=subst(fw,W,Mod(fbx[k],p));if(value==0,zeros++);
      found=0;
      for(mask=0,7,
        e1=if(mask%2,-1,1);e2=if((mask\2)%2,-1,1);e3=if((mask\4)%2,-1,1);
        P1=if(e1<0,ellsub(E,[0],fb[i]),fb[i]);
        P2=if(e2<0,ellsub(E,[0],fb[j]),fb[j]);
        P3=if(e3<0,ellsub(E,[0],fb[k]),fb[k]);
        Qsum=elladd(E,elladd(E,P1,P2),P3);adds+=2;
        if(Qsum==R,found++;hits++;rowVec=vector(B+1,h,0);rowVec[i]+=e1;rowVec[j]+=e2;rowVec[k]+=e3;rowVec[B+1]=-1;listput(rows,rowVec));
      );
      if((value==0)!=(found>0),error("S4/group mismatch"));
    );
  ));
  rows=Vec(rows);
  if(#rows>0,M=matrix(B+1,#rows,ii,jj,Mod(rows[jj][ii],r));rankRows=matrank(M));
  if(zeros!=24 || hits!=24 || rankRows!=4 || adds!=65536,error("V2 result guard"));
  if(referenceRows==0,referenceRows=rows,if(rows!=referenceRows,error("cross-arm witness mismatch")));
  t2=getabstime();w2=getwalltime();evaluationCpu=t2-t1;evaluationWall=w2-w1;totalCpu=t2-t0;totalWall=w2-w0;
  print("pass=",pass," mode=",if(mode==1,"generic4_to_z0","direct_z0"),
        " construct_cpu_ms=",constructionCpu," construct_wall_ms=",constructionWall,
        " setup_cpu_ms=",setupCpu," setup_wall_ms=",setupWall,
        " eval_verify_rank_cpu_ms=",evaluationCpu," eval_verify_rank_wall_ms=",evaluationWall,
        " total_cpu_ms=",totalCpu," total_wall_ms=",totalWall,
        " support=",support," zeros=",zeros," signed_rows=",hits," rank=",rankRows,
        " reference_group_adds=",adds," exact_rows_agree=true");
};
run_arm(1,1);run_arm(2,2);run_arm(2,3);run_arm(1,4);
print("complete same-target S4 comparison");
