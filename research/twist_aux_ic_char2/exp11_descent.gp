\\ EXP11 builder: Weil descent of S5(x1..x4, x(R)) with x_i = c0 + c1 w_i, w_i in F_q, over K = F_{q^4}.
\\ Writes msolve input (4 equations over F_q in w1..w4, optionally with field equations w^q - w).
S3f(u,v,w,a,b,c) = (u-v)^2*w^2 - 2*((u+v)*(u*v+b) + 2*a*u*v + 2*c)*w + (u*v-b)^2 - 4*c*(u+v+a);
[w1,w2,w3,w4];
TL = varlower("tz");
YY = varhigher("YY"); XX = varhigher("XX");
\\ S5 restricted to the line, as polynomial in w1..w4 with K-coefficients
s5line(cf, c0, c1, xR) = {
  my(a=cf[1], b=cf[2], c=cf[3], x1=c0+c1*w1, x2=c0+c1*w2, x3=c0+c1*w3, x4=c0+c1*w4);
  my(S4p = polresultant(S3f(x3,x4,YY,a,b,c), S3f(XX,xR,YY,a,b,c), YY));
  polresultant(S3f(x1,x2,XX,a,b,c), S4p, XX);
}
\\ map ffelt leaves to polynomials in tt
toT(P) = if(type(P)=="t_FFELT", subst(P.pol, variable(P.pol), TL), if(type(P)=="t_POL", apply(toT, P), P));
\\ expand polynomial in w1..w4 (integer coeffs mod p) to msolve string
mono(P, vars, k, pre, ~L) = {
  if(k > #vars, my(cc=lift(Mod(P, p_glob))); if(cc!=0, listput(L, Str(cc, pre))); return);
  my(v=vars[k], d=poldegree(P, v));
  for(e=0, d, my(C=polcoef(P, e, v)); if(C==0, next); mono(C, vars, k+1, if(e==0, pre, Str(pre, "*", v, if(e>1, Str("^",e), ""))), ~L));
}
polstr(P) = { my(L=List()); mono(P, [w1,w2,w3,w4], 1, "", ~L); if(#L==0, "0", my(s=L[1]); for(i=2,#L, s=Str(s,"+",L[i])); s); }
descend(S, n) = { my(T=lift(toT(S))); vector(n, b, polcoef(T, b-1, TL)); }
writems(fname, eqs, q, fieldeq) = {
  my(s=Str("w1,w2,w3,w4\n", q, "\n")); my(all=eqs); if(fieldeq, all=concat(eqs, [w1^q-w1, w2^q-w2, w3^q-w3, w4^q-w4]));
  for(i=1,#all, s=Str(s, polstr(all[i]), if(i<#all, ",\n", "\n")));
  write1(fname, s);
}

\\ ---- symmetrisation: write a symmetric polynomial in w1..w4 (coeffs mod p) in terms of e1..e4 ----
[s1,s2,s3,s4];
lexlead(P) = { my(ex=vector(4), C=P, vv=[w1,w2,w3,w4]); for(i=1,4, my(d=poldegree(C,vv[i])); ex[i]=d; C=polcoef(C,d,vv[i])); [ex, C]; }
symred(P0, p) = {
  my(P=P0*Mod(1,p), E=[w1+w2+w3+w4, w1*w2+w1*w3+w1*w4+w2*w3+w2*w4+w3*w4, w1*w2*w3+w1*w2*w4+w1*w3*w4+w2*w3*w4, w1*w2*w3*w4], G=0, it=0);
  while(P!=0, my(L=lexlead(P), a=L[1], c=L[2]);
    if(!(a[1]>=a[2] && a[2]>=a[3] && a[3]>=a[4]), error("not symmetric"));
    my(b=[a[1]-a[2], a[2]-a[3], a[3]-a[4], a[4]]);
    G += c*s1^b[1]*s2^b[2]*s3^b[3]*s4^b[4];
    P -= c*E[1]^b[1]*E[2]^b[2]*E[3]^b[3]*E[4]^b[4]; it++);
  lift(G);
}
polstrS(P) = { my(L=List()); mono(P, [s1,s2,s3,s4], 1, "", ~L); if(#L==0, "0", my(s=L[1]); for(i=2,#L, s=Str(s,"+",L[i])); s); }
writemsS(fname, eqs, q) = { my(s=Str("s1,s2,s3,s4\n", q, "\n")); for(i=1,#eqs, s=Str(s, polstrS(eqs[i]), if(i<#eqs, ",\n", "\n"))); write1(fname, s); }
tdeg(P) = poldegree(substvec(P, [s1,s2,s3,s4], ['h*s1,'h*s2,'h*s3,'h*s4]), 'h);
