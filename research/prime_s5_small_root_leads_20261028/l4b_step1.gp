\\ L4-B step 1 (README §8): premises P1 (irreducibility) and P2 (full box
\\ Newton polytope) for Semaev's S4(x1,x2,x3; z) on the frontier campaign's
\\ 40-bit a = -3 curve icv1-fp40-tm924137-0ecaaeb9 (p = 2^40 - 213, b = 56),
\\ at 20 seeded random z.  S4 = Res_X(S3(x1,x2,X), S3(x3,z,X)).
\\ P2: the 8 vertex coefficients of the box [0,4]^3 are nonzero.
\\ P1 (over F_p only): S4(.;z) restricted to up to 50 random lines
\\ x_i = u_i + v_i t; one irreducible restriction of full degree 12 proves
\\ S4(.;z) irreducible over F_p.  Absolute irreducibility is NOT tested:
\\ a degree-12 restriction irreducible over F_p splits over F_{p^k} into
\\ gcd(k,12) factors whatever S4 does, so line restrictions cannot see it.
p = 1099511627563; a = Mod(-3, p); b = Mod(56, p);
S3(u, v, w) = (u - v)^2*w^2 - 2*((u + v)*(u*v + a) + 2*b)*w + (u*v - a)^2 - 4*b*(u + v);
setrand(20261028);
vtx = 0; irrp = 0; zs = 20; lines = 50; mindeg = 99;
{
for (iz = 1, zs,
  z = Mod(random(p), p);
  S = polresultant(S3(x1, x2, 'X), S3(x3, z, 'X), 'X);
  \\ vertices of [0,4]^3
  ok = 1;
  forvec(e = [[0,1],[0,1],[0,1]],
    c = polcoef(polcoef(polcoef(S, 4*e[1], x1), 4*e[2], x2), 4*e[3], x3);
    if (c == 0, ok = 0));
  vtx += ok;
  \\ line restrictions
  fp = 0;
  for (il = 1, lines,
    u = vector(3, i, Mod(random(p), p)); v = vector(3, i, Mod(random(p), p));
    L = subst(subst(subst(S, x1, u[1] + v[1]*t), x2, u[2] + v[2]*t), x3, u[3] + v[3]*t);
    d = poldegree(L, t); mindeg = min(mindeg, d);
    F = factor(L);
    if (#F~ == 1 && F[1,2] == 1 && d == 12, fp = 1);
    if (fp, nl = il; break));
  irrp += fp;
  printf("z %2d: vertices %s, irreducible over F_p %s\n", iz, if(ok,"all 8 nonzero","MISSING"), if(fp,Str("yes (line ", nl, ")"),"not shown in 50 lines")));
printf("SUMMARY: %d/%d z with full box vertices; %d/%d shown irreducible over F_p (line degree min %d, total degree 12); absolute irreducibility not tested\n", vtx, zs, irrp, zs, mindeg);
}
quit
