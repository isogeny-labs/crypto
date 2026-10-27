\\ Independent replay of complete or partial factorization certificates.
\\ REPLAY holds [id, expected N, supplied N, factors, residuals, certificate dir].
check(ok,msg)=if(!ok,error(msg));
qstr(n)=Str("\"",n,"\"");
certprime(C)=if(type(C)=="t_INT",C,C[1][1]);
replay_certificates()={
  read(getenv("REPLAY_INPUT"));
  my(r=REPLAY,id=r[1],N=r[2],P=r[4],R=r[5],prodv=1,maxprime=0,maxresidual=0,square=1);
  check(N==r[3] && N>1,"wrong target integer");
  for(i=1,#P,
    my(p=P[i][1],e=P[i][2]);
    check(type(e)=="t_INT" && e>0 && p>1,"invalid factor or exponent");
    if(i>1,check(p>P[i-1][1],"prime factors not unique and sorted"));
    read(Str(r[6],"/prime-",p,".gp"));
    check(certprime(CERT)==p && primecertisvalid(CERT),"wrong or invalid primality certificate");
    check(isprime(p),"independent primality check failed");
    prodv*=p^e;square*=p^(e\2);maxprime=max(maxprime,#binary(p)));
  for(i=1,#R,
    my(c=R[i][1],e=R[i][2]);
    check(type(e)=="t_INT" && e>0 && c>1 && !isprime(c),"invalid composite residual");
    prodv*=c^e;maxresidual=max(maxresidual,#binary(c)));
  check(prodv==N,"factor product does not equal target");
  print("{\"id\":\"",id,"\",\"verified\":true,\"status\":\"",if(#R,"partial","complete"),
    "\",\"target_bits\":",#binary(N),",\"largest_proven_prime_bits\":",maxprime,
    ",\"largest_residual_bits\":",maxresidual,",\"square_part_root\":",if(#R,"null",qstr(square)),"}");
};
iferr(replay_certificates(),err,print("VERIFY FAILED: ",err);quit(1));
quit;
