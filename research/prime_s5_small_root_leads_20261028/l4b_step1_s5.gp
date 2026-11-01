\\ L4-B step 1 at m = 4: S5(x1,x2,x3,x4; z) on the same 40-bit curve as
\\ l4b_step1.gp, 20 seeded random z.  S4(u,v,w) = Res_Y(S3(u,v,Y), S3(w,z,Y));
\\ S5 = Res_X(S3(x1,x2,X), S4(X,x3,x4)).  P2: the 16 vertex coefficients of
\\ the box [0,8]^4 are nonzero.  P1 over F_p: one random-line restriction of
\\ full degree 32 irreducible over F_p.  Absolute irreducibility not tested
\\ (see l4b_step1.gp).
p = 1099511627563; a = Mod(-3, p); b = Mod(56, p);
S3(u, v, w) = (u - v)^2*w^2 - 2*((u + v)*(u*v + a) + 2*b)*w + (u*v - a)^2 - 4*b*(u + v);
setrand(20261029);
vtx = 0; irrp = 0; zs = 20; lines = 100; mindeg = 99;
{
for (iz = 1, zs,
  z = Mod(random(p), p);
  S4 = polresultant(S3('X, x3, 'Y), S3(x4, z, 'Y), 'Y);
  S = polresultant(S3(x1, x2, 'X), S4, 'X);
  ok = 1;
  forvec(e = [[0,1],[0,1],[0,1],[0,1]],
    c = polcoef(polcoef(polcoef(polcoef(S, 8*e[1], x1), 8*e[2], x2), 8*e[3], x3), 8*e[4], x4);
    if (c == 0, ok = 0));
  vtx += ok;
  fp = 0; nl = 0;
  for (il = 1, lines,
    u = vector(4, i, Mod(random(p), p)); v = vector(4, i, Mod(random(p), p));
    L = subst(subst(subst(subst(S, x1, u[1] + v[1]*t), x2, u[2] + v[2]*t), x3, u[3] + v[3]*t), x4, u[4] + v[4]*t);
    d = poldegree(L, t); mindeg = min(mindeg, d);
    F = factor(L);
    if (#F~ == 1 && F[1,2] == 1 && d == 32, fp = 1; nl = il; break));
  irrp += fp;
  printf("z %d: vertices %s, irreducible over F_p %s\n", iz, if(ok,"all 16 nonzero","MISSING"), if(fp,Str("yes (line ", nl, ")"),"not shown in 100 lines")));
printf("SUMMARY: %d/%d z with full box vertices; %d/%d shown irreducible over F_p (line degree min %d, total degree 32); absolute irreducibility not tested\n", vtx, zs, irrp, zs, mindeg);
}
quit
