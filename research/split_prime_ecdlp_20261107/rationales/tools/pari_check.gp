\\ Independent check of a class number N returned by the library's relation
\\ method, with PARI/GP's own form arithmetic: primality of the listed prime
\\ factors, N = product of the factorisation, D_K fundamental, f^N = 1 for every
\\ split prime form with l <= 300, and for each prime factor q of N how many of
\\ those forms are killed by N/q (0 means q divides every order; all means the
\\ q-part of the exponent is smaller than the q-part of N).
\\ Usage: echo 'check("label", D, N, [[q1,e1],[q2,e2],...])' | gp -q -f tools/pari_check.gp
default(parisizemax, 10^9);
check(label, D, N, fac) = {
  my(id = qfbred(qfbprimeform(D, 1)), nsplit = 0, bad = 0, prod = 1, allprime = 1, killed = vector(#fac), f);
  for (i = 1, #fac, prod *= fac[i][1]^fac[i][2]; if (!isprime(fac[i][1]), allprime = 0));
  forprime (l = 3, 300,
    if (kronecker(D, l) == 1,
      f = qfbprimeform(D, l); nsplit++;
      if (qfbred(qfbpow(f, N)) != id, bad++);
      for (i = 1, #fac, if (qfbred(qfbpow(f, N / fac[i][1])) == id, killed[i]++))));
  print("{\"label\": \"", label, "\", \"factors_all_prime\": ", allprime, ", \"N_eq_product\": ", N == prod,
        ", \"isfundamental\": ", isfundamental(D), ", \"split_primes_le_300\": ", nsplit,
        ", \"f_pow_N_not_identity\": ", bad,
        ", \"killed_by_N_over_q\": [", concat(vector(#fac, i, Str("[", fac[i][1], ", ", killed[i], "]", if (i < #fac, ", ", "")))), "]}");
}
