\\ Native factor search and proof generation, driven by a frozen task file.
default(nbthreads,1);
default(parisizemax,2000000000);
default(factor_proven,1);
check(ok,msg)=if(!ok,error(msg));
qstr(n)=Str("\"",n,"\"");
jfactors(M)=Str("[",strjoin(vector(matsize(M)[1],i,Str("[",qstr(M[i,1]),",",M[i,2],"]")),","),"]");
certprime(C)=if(type(C)=="t_INT",C,C[1][1]);

save_result(stage,parts)={
  my(primes=Map(),composites=Map(),certdir=Str(OUT,"/certs-",stage));
  foreach(parts,q,
    if(isprime(q),
      my(C=primecert(q));
      check(certprime(C)==q && primecertisvalid(C),"primality certificate mismatch");
      write(Str(certdir,"/prime-",q,".gp"),"CERT=",C,";");
      my(e=0);mapisdefined(primes,q,&e);mapput(primes,q,e+1),
      my(e=0);mapisdefined(composites,q,&e);mapput(composites,q,e+1)));
  my(P=Mat(primes),R=Mat(composites),pv=1,sv=1);
  for(i=1,matsize(P)[1],pv*=P[i,1]^P[i,2];sv*=P[i,1]^(P[i,2]\2));
  for(i=1,matsize(R)[1],pv*=R[i,1]^R[i,2]);
  check(pv==N,"factor product mismatch");
  write(Str(OUT,"/",stage,".json"),"{\"id\":\"",ID,"\",\"value\":",qstr(N),",\"bits\":",#binary(N),
    ",\"status\":\"",if(matsize(R)[1],"partial","complete"),"\",\"factors\":",jfactors(P),",\"residuals\":",jfactors(R),
    ",\"product_verified\":true,\"known_factors_prime\":true,\"certificates_replay_valid\":true,\"square_part_root\":",
    if(matsize(R)[1],"null",qstr(sv)),"}");
  print(ID,": ",stage,"; proven factors ",matsize(P)[1],"; residual composites ",matsize(R)[1]);
};

main()={
  read(getenv("TASK_FILE"));
  check(N>1 && prod(i=1,#PIECES,PIECES[i])==N,"invalid algebraic pieces");
  check(prod(i=1,#HINTS,HINTS[i][1]^HINTS[i][2])==N,"invalid remote hint product");
  my(parts=List(PIECES));
  foreach(HINTS,h,for(j=1,h[2],for(k=1,#parts,
    my(d=gcd(parts[k],h[1]));if(d>1 && d<parts[k],my(old=parts[k]);parts[k]=d;listput(parts,old/d)))));
  my(initial=List());
  foreach(parts,q,my(F=factor(q,1000000));for(i=1,matsize(F)[1],for(e=1,F[i,2],listput(initial,F[i,1]))));
  save_result("initial",Vec(initial));
  my(final=List(),start=getwalltime());
  foreach(initial,q,
    if(isprime(q),listput(final,q);next);
    my(remaining=600-ceil((getwalltime()-start)/1000));
    if(remaining<=0,listput(final,q);next);
    print(ID,": PARI factor search on ",#binary(q),"-bit residual; budget ",remaining," seconds");
    my(F=0);
    iferr(F=alarm(remaining,factor(q)),err,print("SEARCH ERROR: ",err));
    if(type(F)=="t_MAT",for(i=1,matsize(F)[1],for(e=1,F[i,2],listput(final,F[i,1]))),listput(final,q)));
  save_result("pari",Vec(final));
};
iferr(main(),err,print("FATAL: ",err);quit(1));
quit;
