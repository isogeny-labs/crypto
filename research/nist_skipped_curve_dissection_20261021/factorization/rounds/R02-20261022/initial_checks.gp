\\ Exact structural screens; no probabilistic factor-size claims.
check(ok,msg)=if(!ok,error(msg));
qstr(n)=Str("\"",n,"\"");
{
  read(getenv("CHECK_INPUT"));
  foreach(RESIDUALS,r,
    my(id=r[1],C=r[2],hits=List(),root=0,k=ispower(C,,&root));
    check(C>1 && !isprime(C),"baseline residual is not composite");
    foreach(CURVES,c,
      my(p=c[2],n=c[5],t=p+1-n);
      foreach([p,n,t,(p-1)^2+t^2],v,
        my(d=gcd(C,v));if(d>1 && d<C,listput(hits,d)));
      foreach(CURVES,d,
        my(td=d[2]+1-d[5]);
        foreach([t+td,t-td],v,my(g=gcd(C,v));if(g>1 && g<C,listput(hits,g)))));
    my(H=Set(Vec(hits)));
    print("{\"id\":\"",id,"\",\"residual\":",qstr(C),",\"bits\":",#binary(C),
      ",\"composite\":true,\"perfect_power_exponent\":",k,",\"perfect_power_root\":",if(k,qstr(root),"null"),
      ",\"proper_gcd_hints\":[",strjoin(apply(qstr,H),","),"]}");
  );
}
quit;
