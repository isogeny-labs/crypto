default(parisize, 10^9);
read("curves4.gp");
KMAX = 2^20; XMAX = 12;
{
for(ci = 1, #CV,
  my(name = CV[ci][1], p = CV[ci][2], b = CV[ci][3], gx = CV[ci][4], E = ellinit([-3, b], p), found = [], n0 = 0);
  for(x0 = 0, XMAX,
    my(y2 = Mod(x0, p)^3 - 3*x0 + b);
    if(issquare(y2),
      n0++;
      my(P0 = [Mod(x0, p), sqrt(y2)], Q = P0);
      for(k = 1, KMAX,
        if(lift(Q[1]) == gx, found = concat(found, [[x0, k]]));
        Q = elladd(E, Q, P0))));
  printf("%s: %d base points with x0 <= %d, k <= 2^20: %s\n", name, n0, XMAX, if(#found, found, "none")));
\\ control: synthetic G = 777777*P0 on P-256 must be found
my(p = CV[2][2], b = CV[2][3], E = ellinit([-3, b], p), x0 = 0, P0);
while(!issquare(Mod(x0,p)^3 - 3*x0 + b), x0++);
P0 = [Mod(x0, p), sqrt(Mod(x0,p)^3 - 3*x0 + b)];
my(G = ellmul(E, P0, 777777), Q = P0, hit = 0);
for(k = 1, KMAX, if(Q[1] == G[1], hit = k; break); Q = elladd(E, Q, P0));
printf("control (P-256, x0=%d, k=777777): found k = %d\n", x0, hit);
}
