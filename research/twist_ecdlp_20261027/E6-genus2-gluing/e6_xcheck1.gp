default(parisizemax,2000000000);
setrand(777);
E2of(h,p)={my(hz=lift(h),c0=polcoeff(hz,0),c2=polcoeff(hz,2),c4=polcoeff(hz,4),c6=polcoeff(hz,6));
  ellinit([0,Mod(c2,p),0,Mod(c4*c0,p),Mod(c6*c0^2,p)])};
chk(p,A,B)={
  my(f=x^3+A*x+B,E=ellinit([A,B],p),jE=E.j,nE=ellcard(E),t=p+1-nE,fac,typ,nperm,d=2,mus=List(),sing=0,bad=0,tot=0);
  while(kronecker(d,p)!=-1,d++);
  fac=factormod(f,p);typ=vecsort(vector(matsize(fac)[1],i,poldegree(fac[i,1])));
  nperm=if(typ==[3],2,if(typ==[1,2],1,5));
  for(mu=0,p-1,
    my(h=Mod(1,p)*subst(f,x,x^2-mu));
    if(polcoeff(lift(h),0)==0,sing++;next);
    my(W=E2of(h,p));
    if(W.j==jE,listput(mus,mu)));
  \\ for each mu, both classes a in {1,d}: Jacobian order + quotient orders
  my(outs=List());
  foreach(mus,mu,foreach([1,d],a,
     my(h=Mod(1,p)*subst(f,x,a*x^2-mu));
     if(poldegree(gcd(h,deriv(h)))>0,next);
     my(PT=hyperellcharpoly(h),nJ=subst(PT,x,1),W=E2of(h,p),n2=ellcard(W));
     tot++;
     if(!(nJ==nE*(p+1+t)||nJ==nE^2),bad++);
     \\ class rule: nJ==nE*nEt  <=> n2==nEt ; else n2==nE
     if(n2==p+1+t && nJ!=nE*(p+1+t) && t!=0,bad++);
     if(n2==nE && nJ!=nE^2,bad++);
     listput(outs,[mu,a,n2,nJ])));
  [typ,nperm,#mus,sing,tot,bad,t]};
main()={
foreach([101,211,409,1009],p,
  my(cnt=0,mism=0,badall=0,n=0);
  while(n<25,
    my(A=random(p),B=random(p));
    if(A==0||B==0||(4*A^3+27*B^2)%p==0,next);
    my(r=chk(p,A,B));n++;
    print("p=",p," A=",A," B=",B," typ=",r[1]," nonid_perms=",r[2]," mu_with_jE2==jE=",r[3]," singular_mu=",r[4]," tested=",r[5]," bad=",r[6]," t=",r[7]);
    if(r[3]!=r[2],mism++);badall+=r[6]);
  print("SUMMARY p=",p," curves=",n," count_mismatch=",mism," bad=",badall))};
main();
quit;
