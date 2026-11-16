\\ REPAIR (audit must_fix 2): runnable A0 hypothesis check with full monomial enumeration.
\\ Same instance as armA0.gp (p = nextprime(2^61), setrand(1)). Run: gp -q -f armA0_v2.gp
default(parisizemax, 4000000000);
p = nextprime(2^61); setrand(1);
a = random(p); b = random(p); xR = random(p);
S3(u,v,w) = (u-v)^2*w^2 - 2*((u+v)*(u*v+a)+2*b)*w + (u*v-a)^2 - 4*b*(u+v);
S4g(x1,x2,x3,x4) = polresultant(S3(x1,x2,'Y), S3(x3,x4,'Y), 'Y);
\\ list of [exponent vector, coeff] of nonzero terms of f in vars
terms(f, vars) = {
  if(#vars == 0, return(if(f == 0, [], [[[], f]])));
  my(v = vars[1], rest = vars[2..#vars], out = List(), d = poldegree(f, v));
  for(e = 0, max(d,0), my(c = polcoef(f, e, v));
    if(c != 0, foreach(terms(c, rest), t, listput(out, [concat([e], t[1]), t[2]]))));
  Vec(out) };
report(name, f, vars) = {
  my(T = terms(f, vars), degs = vector(#vars, i, poldegree(f, vars[i])), tot, top);
  tot = vecmax(apply(t -> vecsum(t[1]), T));
  top = select(t -> vecsum(t[1]) == tot, T);
  print(name, ": nonzero monomials = ", #T, "; per-variable degrees = ", degs, "; top total degree = ", tot);
  print(name, ": top-degree form has ", #top, " monomial(s): ", top);
  print(name, ": constant term = ", polcoef(substvec(f, vars, vector(#vars, i, 0)), 0)) };
print("p = ", p, "; a = ", a, "; b = ", b, "; xR = ", xR);
f2 = Mod(1,p)*S3(x1,x2,xR); report("S3", f2, [x1,x2]);
f3 = Mod(1,p)*S4g(x1,x2,x3,xR); report("S4", f3, [x1,x2,x3]);
t0 = getabstime();
f4 = polresultant(Mod(1,p)*S3(x1,x2,'X), Mod(1,p)*S4g(x3,x4,xR,'X), 'X);
report("S5", f4, [x1,x2,x3,x4]);
print("S5 build+report ms ", getabstime()-t0);
