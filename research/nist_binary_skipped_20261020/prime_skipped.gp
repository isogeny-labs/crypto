\\ Check the curve models of the prime-run candidates (Rust) against the
\\ 2026-10-16 census. Inputs (prepended by prime_skipped.sh): MODELS, ORDERS,
\\ RUN. Output: model <run> <i> <p> <b> <N> <recount = census> <N prime> <[N]P = O>
{
my(L=externstr(Str("cat ",MODELS)), O=externstr(Str("cat ",ORDERS)), rec=Map());
for(k=1,#O, my(w=strsplit(O[k]," ")); mapput(rec, eval(w[1]), eval(w[2])));
for(l=1,#L,
  my(w=strsplit(L[l]," "),i=eval(w[1]),p=eval(w[3]),b=eval(w[4]),E,N);
  E=ellinit([-3,b],p); N=ellcard(E);
  print("model ",RUN," ",i," ",p," ",b," ",N," ",(mapget(rec,i)==N)," ",ispseudoprime(N)," ",prod(j=1,3,ellmul(E,random(E),N)==[0])));
}
