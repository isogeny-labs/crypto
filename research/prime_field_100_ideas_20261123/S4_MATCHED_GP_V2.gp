\\ Exact bounded comparison for proposal 24 on a public prime-order EC.
\\ V2 parser repair only; same frozen inputs and checks as V1.
\\ Admitted for one local run by explicit user checkpoint-bypass instruction.
\\ Source: Semaev (2004), Theorem 1, S3 formula and S4 resultant recurrence.
\\ Output: stdout/stderr captured separately; preserve V1 parse-failure files.

p=1009; a=Mod(1,p); b=Mod(14,p); r=1013; B=16;
X='X; Y='Y; W='W; T='T;
s3(u,v,z)=(u-v)^2*z^2-2*((u+v)*(u*v+a)+2*b)*z+(u*v-a)^2-4*b*(u+v);
E=ellinit([a,b]);
if(ellcard(E)!=r || !isprime(r),error("curve cardinality/subgroup guard failed"));

\\ GP's stdin parser requires the multiline loop inside a function body.
\\ Shared factor base: first B nonzero, liftable x in ascending order.
build_base()={
  my(nfb=0,xx=1,q);
  fbx=vector(B); fb=vector(B);
  while(nfb<B && xx<p,
    q=Mod(xx^3+xx+14,p);
    if(issquare(q),
      nfb++;
      fbx[nfb]=xx;
      fb[nfb]=[Mod(xx,p),sqrt(q)];
      if(!ellisoncurve(E,fb[nfb]),error("factor-base point guard failed"));
    );
    xx++;
  );
  if(nfb!=B,error("factor-base size guard failed"));
};
build_base();
print("header p=",p," a=1 b=14 order=",r," B=",B," triples_per_target=",B^3);
print("factor_base_x=",fbx);
for(i=1,B,print("base index=",i," x=",fbx[i]," y=",lift(fb[i][2])));

compare_target(z0)={
  my(z=Mod(z0,p), R, S, support=0, corners=0, degX,degY,degW,
     pairCoeff=0, pairHornerMuls=0, pairHornerAdds=0,
     algebraicZeros=0, signedTriples=0, signedWitnesses=0,
     groupAdds=0, rows=List(), rowVec, M, rankRows=0,
     fw, d, nz, value, found, signs, e1,e2,e3,P1,P2,P3,Qsum,v);

  if(!issquare(z^3+a*z+b),error("target abscissa not on curve"));
  R=[z,sqrt(z^3+a*z+b)];
  if(!ellisoncurve(E,R),error("target point guard failed"));
  if(ellorder(E,R)!=r,error("target subgroup guard failed"));
  S=polresultant(s3(X,Y,T),s3(T,W,z),T);
  degX=poldegree(S,X); degY=poldegree(S,Y); degW=poldegree(S,W);
  if([degX,degY,degW]!=[4,4,4],error("S4 multidegree guard failed"));
  for(ii=0,4,for(jj=0,4,for(kk=0,4,
    v=polcoef(polcoef(polcoef(S,ii,X),jj,Y),kk,W);
    if(v!=0,support++);
    if((ii==0||ii==4)&&(jj==0||jj==4)&&(kk==0||kk==4)&&v!=0,corners++);
  )));
  print("target z=",z0," y=",lift(R[2])," support=",support,"/125 corners=",corners,"/8 degrees=",[degX,degY,degW]);

  \\ Pair-specialize S4 into a univariate in W. The arithmetic proxy is
  \\ Horner operations for 16 candidate W values per pair, excluding
  \\ symbolic resultant, specialization, memory traffic, and group costs.
  for(i=1,B,for(j=1,B,
    fw=subst(subst(S,X,Mod(fbx[i],p)),Y,Mod(fbx[j],p));
    if(fw==0,
      d=-1; nz=0,
      d=poldegree(fw,W); nz=0;
      for(h=0,d,if(polcoef(fw,h,W)!=0,nz++));
    );
    pairCoeff+=nz;
    if(d>=0,
      pairHornerMuls+=B*d;
      pairHornerAdds+=B*(nz-1);
    );

    for(k=1,B,
      value=subst(fw,W,Mod(fbx[k],p));
      if(value==0,algebraicZeros++);
      found=0;
      \\ Enumerate all 8 signs and require the sum to equal the chosen
      \\ affine target R, not merely to share its x coordinate.
      for(mask=0,7,
        e1=if(mask%2,-1,1);
        e2=if((mask\2)%2,-1,1);
        e3=if((mask\4)%2,-1,1);
        P1=if(e1<0,ellsub(E,[0],fb[i]),fb[i]);
        P2=if(e2<0,ellsub(E,[0],fb[j]),fb[j]);
        P3=if(e3<0,ellsub(E,[0],fb[k]),fb[k]);
        Qsum=elladd(E,elladd(E,P1,P2),P3);
        groupAdds+=2;
        if(Qsum==R,
          found++;
          signedWitnesses++;
          rowVec=vector(B+1,h,0);
          rowVec[i]+=e1; rowVec[j]+=e2; rowVec[k]+=e3;
          rowVec[B+1]=-1;
          listput(rows,rowVec);
          print("witness z=",z0," indices=",[i,j,k]," signs=",[e1,e2,e3]," row=",rowVec);
        );
      );
      if(found>0,signedTriples++);
      if((value==0)!=(found>0),
        print("mismatch z=",z0," indices=",[i,j,k]," S4_zero=",value==0," signed_count=",found);
        error("S4 versus signed point set mismatch");
      );
    );
  ));
  rows=Vec(rows);
  if(#rows>0,
    M=matrix(B+1,#rows,ii,jj,Mod(rows[jj][ii],r));
    rankRows=matrank(M);
  );
  if(algebraicZeros!=signedTriples,error("zero/witness-count guard failed"));
  print("summary z=",z0,
        " polynomial_evaluations=",B^3,
        " algebraic_zero_triples=",algebraicZeros,
        " signed_triples=",signedTriples,
        " signed_witness_rows=",signedWitnesses,
        " modular_rank=",rankRows,"/",B+1,
        " pair_quartic_nonzero_coeff_sum=",pairCoeff,
        " horner_mul_proxy=",pairHornerMuls,
        " horner_add_proxy=",pairHornerAdds,
        " signed_group_adds_reference=",groupAdds);
};

compare_target(0);
compare_target(320);
print("complete matched exact comparison");
