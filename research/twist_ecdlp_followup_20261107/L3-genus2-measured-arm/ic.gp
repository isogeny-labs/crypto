\\ ic.gp -- index calculus in Jac(C') : walk, decomposition, sparse structured elimination, arms (IC-G, IC-S, IC-D, IC-F).
\\ Requires jac.gp.  Works for any Jacobian given by jinit (glued or random) and generator Gt / target Ht of prime order l.
\\ Column ids: vertex (x-coordinate) c in [0,Q-1] stored as c+1 in arrays; the unknown x (the DLP) is column id Q (stored Q+1).

\\ ---------- arm configuration: [name, theta, accept ("FF","FL","LL" bit mask 1,2,4), first threshold, growth, trial cap] ----------
icconf(arm, q) =
{
  my(nest = ceil(q / 2), th, tF, t, ts, thD = min(1., 2 / sqrt(q)));
  if(arm == "G", return(["G", 1., 7, nest, 1.05, 12 * q]));
  if(arm == "S",
    th = min(1., 2 * q^(-2/5.));
    t = if(th >= 1, 1., (-th + sqrt(th^2 + 4 * th * (1 - th))) / (2 * th * (1 - th)));
    ts = t * q / 2;
    return(["S", th, 3, ceil(0.2 * nest), 1.1, ceil(12 * ts)]));
  if(arm == "D", return(["D", thD, 7, ceil(0.2 * nest), 1.1, 12 * q]));
  if(arm == "F", return(["F", thD, 1, max(2, ceil(0.2 * thD * nest)), 1.1, ceil(8 * q^1.5 / 4)]));
  error("bad arm");
}
tsformula(q) =
{
  my(th = min(1., 2 * q^(-2/5.)), t);
  t = if(th >= 1, 1., (-th + sqrt(th^2 + 4 * th * (1 - th))) / (2 * th * (1 - th)));
  t * q / 2;
}

\\ ---------- table / start elements ----------
icmake(Gt, Ht, l) =
{
  my(al = 1 + random(l - 1), be = 1 + random(l - 1), Xj, R, Dd);
  Xj = jrand(); R = jmul(l, Xj);
  Dd = jadd(jadd(jmul(al, Gt), jmul(be, Ht)), R);
  [Dd, al, be];
}
\\ one decomposition-walk position key (for duplicate detection)
jkey(D) = if(#D == 4, ((D[1] * Q + D[2]) * Q + D[3]) * Q + D[4], -1 - 7 * lift(polcoeff(D[1], 0)) - 13 * lift(polcoeff(D[2], 0)));
jconst(D) = if(#D == 4, D[2], lift(polcoeff(D[1], 0)));

\\ ---------- rows ----------
\\ merge: m1*r1 + m2*r2 mod l over sorted column lists; rows are [cols, coefs, rhs]
rowcomb(r1, m1, r2, m2, l) =
{
  my(c1 = r1[1], k1 = r1[2], c2 = r2[1], k2 = r2[2], n1 = #c1, n2 = #c2, i = 1, j = 1, oc = vector(n1 + n2), ok = vector(n1 + n2), n = 0, cc, kk);
  while(i <= n1 || j <= n2,
    if(j > n2 || (i <= n1 && c1[i] < c2[j]),
      cc = c1[i]; kk = (m1 * k1[i]) % l; i++,
      if(i > n1 || c2[j] < c1[i],
        cc = c2[j]; kk = (m2 * k2[j]) % l; j++,
        cc = c1[i]; kk = (m1 * k1[i] + m2 * k2[j]) % l; i++; j++));
    if(kk != 0, n++; oc[n] = cc; ok[n] = kk));
  [oc[1..n], ok[1..n], (m1 * r1[3] + m2 * r2[3]) % l];
}

\\ ---------- structured elimination ----------
\\ Global solver state: SR (rows), AL (alive flags), CR (column -> row ids, may contain dead ids), W (alive weight per column), STK/SP (work stack), NID.
s_push(c) = { SP++; if(SP > #STK, STK = concat(STK, vector(#STK))); STK[SP] = c; }
s_addrow(rw) =
{
  my(id, cs = rw[1], t, c2);
  NID++; if(NID > #SR, SR = concat(SR, vector(#SR)); AL = concat(AL, vector(#AL)));
  id = NID; SR[id] = rw; AL[id] = 1;
  for(t = 1, #cs, if(cs[t] < Q, c2 = cs[t] + 1; CR[c2] = if(CR[c2] == 0, [id], concat(CR[c2], id)); W[c2]++));
  id;
}
s_kill(id) =
{
  my(cs = SR[id][1], t, c2);
  AL[id] = 0;
  for(t = 1, #cs, if(cs[t] < Q, c2 = cs[t] + 1; W[c2]--; if(W[c2] >= 1 && W[c2] <= 2, s_push(c2))));
}
s_coef(id, c0) = { my(cs = SR[id][1], t); for(t = 1, #cs, if(cs[t] == c0, return(SR[id][2][t]))); 0; }
\\ result row has no vertex column: returns x if it is [Q] with nonzero coefficient, else -1 (nothing) ; -2 = no-vertex row without information
s_chk(row, l) =
{
  my(cols = row[1], t, nvc = 0);
  for(t = 1, #cols, if(cols[t] < Q, nvc++));
  if(nvc > 0, return(-3));
  if(#cols == 1 && cols[1] == Q, return((row[3] * lift(Mod(row[2][1], l)^-1)) % l));
  -2;
}

\\ peel stage: process the stack of columns of weight 1 or 2 (weight 1: delete the row; weight 2: combine the two rows).  Returns x (>= 0) on success else -1.
s_peel(l) =
{
  my(c, w, lst, r1, r2, a1, a2, row, chk, cols, i);
  while(SP > 0,
    c = STK[SP]; SP--; w = W[c];
    if(w < 1 || w > 2, next);
    lst = select((t) -> AL[t], CR[c]); CR[c] = lst;
    if(#lst != w, W[c] = #lst; if(#lst >= 1 && #lst <= 2, s_push(c)); next);
    if(#lst == 1, s_kill(lst[1]); ROWDELS++; next);
    r1 = lst[1]; r2 = lst[2];
    a1 = s_coef(r1, c - 1); a2 = s_coef(r2, c - 1);
    row = rowcomb(SR[r1], a2, SR[r2], (l - a1) % l, l);
    ROWOPS++; NNZ += #SR[r1][1] + #SR[r2][1];
    s_kill(r1); s_kill(r2);
    chk = s_chk(row, l);
    if(chk >= 0, return(chk));
    if(chk == -3, s_addrow(row); cols = row[1]; for(i = 1, #cols, if(cols[i] < Q && W[cols[i] + 1] >= 1 && W[cols[i] + 1] <= 2, s_push(cols[i] + 1)))));
  -1;
}
\\ pivot on column c (Markowitz step): the shortest row containing c is the pivot, all other rows of c are combined with it, the pivot row is deleted.
s_pivot(c, l) =
{
  my(lst, pivot, a1, a2, rr, row, chk, cols, i, j);
  lst = select((t) -> AL[t], CR[c]); CR[c] = lst;
  pivot = lst[1]; for(i = 2, #lst, if(#SR[lst[i]][1] < #SR[pivot][1], pivot = lst[i]));
  a1 = s_coef(pivot, c - 1);
  for(j = 1, #lst,
    rr = lst[j]; if(rr == pivot, next);
    a2 = s_coef(rr, c - 1);
    row = rowcomb(SR[rr], a1, SR[pivot], (l - a2) % l, l);
    ROWOPS++; NNZ += #SR[rr][1] + #SR[pivot][1];
    s_kill(rr);
    chk = s_chk(row, l);
    if(chk >= 0, return(chk));
    if(chk == -3, s_addrow(row); cols = row[1]; for(i = 1, #cols, if(cols[i] < Q && W[cols[i] + 1] >= 1 && W[cols[i] + 1] <= 2, s_push(cols[i] + 1)))));
  s_kill(pivot); ROWDELS++;
  -1;
}

\\ ROWS[1..nr] accepted relations (global).  Returns [ok, x, rowops, rowdels, nnzupd]
\\ orderlarge: IC-D order restriction (columns outside the factor base pivot first), uses FBIN(c).
\\ Markowitz stage is run in rounds: columns sorted by current weight (large columns first when orderlarge), each pivoted if its weight has not grown by more than 1.
icsolve(nr, l, orderlarge) =
{
  my(cap = 2 * nr + 2000, c, i, row, cols, K, key, perm, chk, kc, wt);
  ROWOPS = 0; ROWDELS = 0; NNZ = 0;
  SR = vector(cap); AL = vector(cap); CR = vector(Q); W = vector(Q); STK = vector(4 * nr + 4000); SP = 0; NID = nr;
  for(r = 1, nr,
    row = ROWS[r]; SR[r] = row; AL[r] = 1; cols = row[1];
    for(i = 1, #cols, if(cols[i] < Q, kc = cols[i] + 1; CR[kc] = if(CR[kc] == 0, [r], concat(CR[kc], r)); W[kc]++));
    chk = s_chk(row, l);
    if(chk >= 0, return([1, chk, ROWOPS, ROWDELS, NNZ])));
  for(c = 1, Q, if(W[c] >= 1 && W[c] <= 2, s_push(c)));
  chk = s_peel(l); if(chk >= 0, return([1, chk, ROWOPS, ROWDELS, NNZ]));
  K = select((c) -> W[c] > 0, [1..Q]);
  while(#K > 0,
    key = vector(#K, i, W[K[i]] + if(orderlarge && FBIN(K[i] - 1), 1 << 20, 0));
    perm = vecsort(key, , 1);
    for(i = 1, #perm,
      c = K[perm[i]]; wt = W[c];
      if(wt < 1, next);
      if(wt > (key[perm[i]] % (1 << 20)) + 1, next);
      chk = s_pivot(c, l); if(chk >= 0, return([1, chk, ROWOPS, ROWDELS, NNZ]));
      chk = s_peel(l); if(chk >= 0, return([1, chk, ROWOPS, ROWDELS, NNZ])));
    K = select((c) -> W[c] > 0, K));
  [0, 0, ROWOPS, ROWDELS, NNZ];
}

\\ ---------- factor base membership ----------
FBIN(c) = (FBS * c % Q) < FBTH;

\\ ---------- one IC solve: arm in "G","S","D","F"; walk, decompositions, solve schedule ----------
\\ Gt, Ht in Jac of prime order l.  FBS (multiplier) must be set by the caller (global).  jzero() is called by the caller.
\\ Returns the stats vector (see ICFIELDS).
ICFIELDS = ["solved","xcand","trials","jops","cM","cS","cI","nfb","jsetup","nsplit","ndeg1","ndeg0","ndisc0","nFF","nFL","nLL","nrej","ndup","nrestart","nsolve","rowops","rowdels","nnz","nsq","nacc","wall"];
icrun(Gt, Ht, l, arm, sabotage) =
{
  my(q = Q, cf = icconf(arm, q), theta = cf[2], acc = cf[3], thr = cf[4], growth = cf[5], capT = cf[6],
     TM = vector(16), TAL = vector(16), TBE = vector(16), j, e, D, a, b, seen = Map(), ntr = 0, nsplit = 0, ndeg1 = 0, ndeg0 = 0, ndisc0 = 0,
     nFF = 0, nFL = 0, nLL = 0, nrej = 0, ndup = 0, nrestart = 0, nsolve = 0, rowops = 0, rowdels = 0, nnz = 0, nsq = 0, nacc = 0,
     jsetup, solved = 0, xcand = 0, t0 = getwalltime(), key, disc, sq, x1, x2, y1, y2, e1, e2, f1, f2, ty, ok, xc, cols, coefs, rw, res, maxr = 4096, i, x3, y3, u0, hv, h2 = lift(Mod(2, q)^-1));
  FBTH = if(theta >= 1, q, ceil(theta * q));
  ROWS = vector(maxr); 
  for(j = 1, 16, e = icmake(Gt, Ht, l); TM[j] = e[1]; TAL[j] = e[2]; TBE[j] = e[3]);
  e = icmake(Gt, Ht, l); D = e[1]; a = e[2]; b = e[3];
  jsetup = jops;
  while(ntr < capT && !solved,
    j = jconst(D) % 16 + 1;
    D = jadd(D, TM[j]); a = (a + TAL[j]) % l; b = (b + TBE[j]) % l; ntr++;
    key = jkey(D);
    if(mapisdefined(seen, key), ndup++; nrestart++; e = icmake(Gt, Ht, l); D = e[1]; a = e[2]; b = e[3]; next);
    mapput(seen, key, 1);
    xc = (-b) % l; cols = []; coefs = []; ty = -1;
    if(#D == 4,
      disc = (D[1] * D[1] - 4 * D[2]) % q;
      if(kronecker(disc, q) != 1, next);
      if(disc == 0, ndisc0++; next);
      nsplit++; sq = lift(sqrt(Mod(disc, q))); nsq++;
      x1 = ((-D[1] + sq) * h2) % q; x2 = ((-D[1] - sq) * h2) % q;
      y1 = (D[3] * x1 + D[4]) % q; y2 = (D[3] * x2 + D[4]) % q;
      f1 = if(theta >= 1, 1, FBIN(x1)); f2 = if(theta >= 1, 1, FBIN(x2));
      ty = if(f1 && f2, 1, if(f1 || f2, 2, 4));      \\ 1 = FF, 2 = FL, 4 = LL
      if(!bitand(acc, ty), nrej++; next);
      if(ty == 1, nFF++, if(ty == 2, nFL++, nLL++));
      e1 = if(y1 > QH, q - 1 % l, 1); e2 = if(y2 > QH, l - 1, 1);
      e1 = if(y1 > QH, l - 1, 1);
      cols = List(); coefs = List();
      if(x1 > x2, hv = x1; x1 = x2; x2 = hv; hv = y1; y1 = y2; y2 = hv; hv = e1; e1 = e2; e2 = hv);
      if(y1 != 0, listput(cols, x1); listput(coefs, e1));
      if(y2 != 0, listput(cols, x2); listput(coefs, e2));
      if(xc != 0, listput(cols, q); listput(coefs, xc));
      rw = [Vec(cols), Vec(coefs), a % l],
      \\ degree < 2 element (fallback representation)
      if(poldegree(D[1]) == 1,
        ndeg1++; u0 = lift(polcoeff(D[1], 0)); x1 = (q - u0) % q; y1 = lift(polcoeff(D[2], 0));
        f1 = if(theta >= 1, 1, FBIN(x1));
        if(!bitand(acc, if(f1, 1, 4)) || (arm == "S" && !f1), nrej++; next);
        e1 = if(y1 > QH, l - 1, 1); cols = List(); coefs = List();
        if(y1 != 0, listput(cols, x1); listput(coefs, e1));
        if(xc != 0, listput(cols, q); listput(coefs, xc));
        rw = [Vec(cols), Vec(coefs), a % l],
        ndeg0++; next));
    nacc++;
    if(nacc > #ROWS, ROWS = concat(ROWS, vector(#ROWS)));
    ROWS[nacc] = rw;
    if(nacc >= thr,
      nsolve++;
      res = icsolve(nacc, l, arm == "D");
      rowops += res[3]; rowdels += res[4]; nnz += res[5];
      if(res[1], solved = 1; xcand = res[2]);
      thr = max(thr + 1, ceil(thr * growth)));
  );
  [solved, xcand, ntr, jops, cM, cS, cI, nfb, jsetup, nsplit, ndeg1, ndeg0, ndisc0, nFF, nFL, nLL, nrej, ndup, nrestart, nsolve, rowops, rowdels, nnz, nsq, nacc, getwalltime() - t0];
}
