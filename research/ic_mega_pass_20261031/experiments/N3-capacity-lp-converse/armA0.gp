\\ check support/lead monomial of specialised S3,S4,S5 mod p (S5 via resultant)
p = nextprime(2^61); setrand(1);
a = random(p); b = random(p); xR = random(p);
S3(u,v,w) = (u-v)^2*w^2 - 2*((u+v)*(u*v+a)+2*b)*w + (u*v-a)^2 - 4*b*(u+v);
\\ S4(x1,x2,x3,x4) = Res_X(S3(x1,x2,X), S3(x3,x4,X))
S4g(x1,x2,x3,x4) = polresultant(S3(x1,x2,'Y), S3(x3,x4,'Y), 'Y);
info(f, vars) = {my(degs = vector(#vars, i, poldegree(f, vars[i])), tot, lc, g);
  g = substvec(f, vars, vector(#vars,i,vars[i]*'T)); tot = poldegree(g,'T);
  lc = pollead(g,'T); [degs, tot, lc]};
f2 = Mod(1,p)*S3(x1,x2,xR); print("S3 ", info(f2,[x1,x2]), " const ", substvec(f2,[x1,x2],[0,0]));
f3 = Mod(1,p)*S4g(x1,x2,x3,xR); print("S4 ", info(f3,[x1,x2,x3]), " const ", substvec(f3,[x1,x2,x3],[0,0,0]));
\\ S5(x1..x4,xR) = Res_X(S3(x1,x2,X), S4(x3,x4,xR,X))
t0=getabstime();
f4 = polresultant(Mod(1,p)*S3(x1,x2,'X), Mod(1,p)*S4g(x3,x4,xR,'X), 'X);
print("S5 ", info(f4,[x1,x2,x3,x4]), " const ", substvec(f4,[x1,x2,x3,x4],[0,0,0,0]), " ms ", getabstime()-t0);
