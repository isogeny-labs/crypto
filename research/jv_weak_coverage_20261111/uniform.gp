\\ U: uniform curves over F_{p^6}; prints one line per curve: p i t even admitted
\\ usage: echo 'P=1009; N=2000; read("uniform.gp")' | gp -q
{
  setrand(20261111 + P);
  g = ffgen(P^6, 'w); q3 = P^6;
  for (i = 0, N - 1,
    my(a, b, E, t, ad);
    until (4*a^3 + 27*b^2 != 0, a = random(g); b = random(g));
    E = ellinit([a, b], g);
    t = q3 + 1 - ellcard(E);
    ad = (t % 16 == (q3 + 1) % 16) || ((-t) % 16 == (q3 + 1) % 16);
    print(P, " ", i, " ", t, " ", (t + q3 + 1) % 2 == 0, " ", ad));
}
