default(parisizemax, 2000000000);
print("\\\\ chainsearch.gp  gp ", version());
\\ Exhaustive search of the cheapest Lucas (differential addition) chain for each k in [2,53].
\\ chain = increasing sequence 1 = c_0 < c_1 < ... < c_m = k;  new element = x + y (x >= y in chain),
\\ allowed if x = y (doubling, cost D = 3M+2S) or x - y is in the chain (differential addition, cost A = 4M+2S).
\\ cost unit: 5 M-equivalents with S = 0.8 M  ->  D = 23, A = 28.  ties broken by fewer operations.
CD = 23; CA = 28;
BESTC = 0; BESTCH = 0; BESTD = 0; BESTA = 0; TARGET = 0;
dfs(ch, nd, na, cost) =
{
  my(m = #ch, mx = ch[m], lb, x, y, c, v, isin, sd, tot);
  if(mx == TARGET,
    if(cost < BESTC || (cost == BESTC && nd + na < BESTD + BESTA), BESTC = cost; BESTCH = ch; BESTD = nd; BESTA = na);
    return);
  \\ lower bound on remaining cost: ceil(log2(TARGET/mx)) operations of at least CD each
  lb = 0; c = mx; while(c < TARGET, c *= 2; lb += CD);
  if(cost + lb > BESTC, return);
  for(i = 1, m, for(j = 1, i,
    v = ch[i] + ch[j];
    if(v <= mx || v > TARGET, next);
    if(i == j, sd = 1, sd = 0);
    if(!sd, isin = 0; for(t = 1, m, if(ch[t] == ch[i] - ch[j], isin = 1; break)); if(!isin, next));
    dfs(concat(ch, [v]), nd + sd, na + (1 - sd), cost + if(sd, CD, CA))));
}
\\ plain ladder cost for comparison: l = bitlength(k): l xDBL, (l-1) xADD
ladder_cost(k) = my(l = #binary(k)); l * CD + (l - 1) * CA;
tabD = 0; tabA = 0; tabLD = 0; tabLA = 0;
{
for(k = 2, 53,
  TARGET = k; BESTC = ladder_cost(k) + 1; BESTCH = 0; BESTD = 0; BESTA = 0;
  dfs([1], 0, 0, 0);
  my(l = #binary(k));
  print("chk(\"chain_", k, "\",[", BESTCH, ",", BESTD, ",", BESTA, ",", l, ",", l - 1, "]);  \\\\ [chain, #xDBL, #xADD, ladder #xDBL, ladder #xADD]"));
}
