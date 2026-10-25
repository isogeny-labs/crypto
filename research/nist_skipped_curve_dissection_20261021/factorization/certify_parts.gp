\\ Certify a factor partition after combining native searches and exact hints.
default(nbthreads,1);
check(ok,msg)=if(!ok,error(msg));
qstr(n)=Str("\"",n,"\"");
certprime(C)=if(type(C)=="t_INT",C,C[1][1]);
jfactors(M)=Str("[",strjoin(vector(matsize(M)[1],i,Str("[",qstr(M[i,1]),",",M[i,2],"]")),","),"]");
certify()={
  read(getenv("CERTIFY_INPUT"));
  my(parts=List(BASE_PARTS));
  check(prod(i=1,#parts,parts[i])==N,"invalid initial partition");
  foreach(EXTRA_FACTORS,q,
    check(q>1 && q<N && N%q==0,"ECM candidate is not a proper divisor");
    for(k=1,#parts,my(d=gcd(parts[k],q));if(d>1 && d<parts[k],my(old=parts[k]);parts[k]=d;listput(parts,old/d))));
  my(primes=Map(),composites=Map());
  foreach(parts,q,
    if(isprime(q),
      my(C=primecert(q));check(certprime(C)==q && primecertisvalid(C),"invalid prime certificate");
      write(Str(OUT,"/certs-final/prime-",q,".gp"),"CERT=",C,";");
      my(e=0);mapisdefined(primes,q,&e);mapput(primes,q,e+1),
      my(e=0);mapisdefined(composites,q,&e);mapput(composites,q,e+1)));
  my(P=Mat(primes),R=Mat(composites),pv=1,square=1);
  for(i=1,matsize(P)[1],pv*=P[i,1]^P[i,2];square*=P[i,1]^(P[i,2]\2));
  for(i=1,matsize(R)[1],pv*=R[i,1]^R[i,2]);
  check(pv==N,"final product mismatch");
  print("{\"id\":\"",ID,"\",\"value\":",qstr(N),",\"bits\":",#binary(N),",\"status\":\"",if(matsize(R)[1],"partial","complete"),
    "\",\"factors\":",jfactors(P),",\"residuals\":",jfactors(R),",\"product_verified\":true,\"known_factors_prime\":true,",
    "\"certificates_replay_valid\":true,\"square_part_root\":",if(matsize(R)[1],"null",qstr(square)),"}");
};
iferr(certify(),err,print("CERTIFICATION FAILED: ",err);quit(1));
quit;
