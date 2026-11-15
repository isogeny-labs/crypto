\\ Independent arithmetic comes from PARI/GP, not the candidate's Rust modules.
\\ The shell entry point restricts input to literal integer/vector syntax first.
check(v) = {
  if(type(v) != "t_VEC" || #v != 11, error("input shape"));
  for(i=1,2, if(type(v[i]) != "t_INT", error("scalar type")));
  for(i=10,11, if(type(v[i]) != "t_INT", error("scalar type")));
  my(p=v[1], c=v[2], r=v[10], k=v[11]);
  if(p < 7 || p >= 2^31 || p % 12 != 7 || !isprime(p), error("field prime"));
  if(c <= 0 || c >= p, error("extension constant"));
  if(r < 2 || r > 2^64-1 || !isprime(r) || k <= 0 || k >= r, error("subgroup or scalar"));
  for(i=3,9,
    if(type(v[i]) != "t_VEC" || #v[i] != 3, error("coordinate shape"));
    for(j=1,3,
      if(type(v[i][j]) != "t_INT" || v[i][j] < 0 || v[i][j] >= p,
        error("noncanonical coordinate"))));
  my(T=Mod(1,p)*'z^3-Mod(c,p));
  if(!polisirreducible(T), error("reducible extension"));
  my(u=ffgen(T), w=vector(7,i,v[i+2][1]+v[i+2][2]*u+v[i+2][3]*u^2));
  my(E=ellinit([0,w[1],0,w[2],w[3]]));
  if(#E == 0 || E.disc == 0, error("singular curve"));
  my(G=[w[4],w[5]], Q=[w[6],w[7]]);
  if(!ellisoncurve(E,G) || !ellisoncurve(E,Q), error("point off curve"));
  if(ellmul(E,G,r) != [0] || ellmul(E,Q,r) != [0], error("subgroup"));
  if(ellmul(E,G,k) != Q, error("scalar mismatch"));
  1
};
iferr(if(check(read(getenv("PARI_REPLAY_INPUT"))), print("PARI_REPLAY_OK"), quit(1)), E, print("PARI_REPLAY_REJECT"); quit(1));
quit(0);
