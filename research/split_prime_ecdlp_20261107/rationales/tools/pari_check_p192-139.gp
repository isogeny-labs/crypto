default(parisizemax, 10^9);
D = -24109379060336110122544161233113975664949272517896865359515;
h = 20328816342330880337248986744;
q = 282344671421262226906235927;
id = qfbred(qfbprimeform(D, 1));
{
nsplit = 0; bad = 0; ann2 = 0; ann3 = 0; annq = 0; el = 0;
forprime(l = 3, 300,
  if (kronecker(D, l) == 1,
    f = qfbprimeform(D, l); nsplit++;
    if (qfbred(qfbpow(f, h)) != id, bad++);
    if (qfbred(qfbpow(f, h/2)) == id, ann2++);
    if (qfbred(qfbpow(f, h/3)) == id, ann3++);
    if (qfbred(qfbpow(f, h/q)) == id, annq++);
    g = qfbred(qfbpow(f, h/8)); if (qfbred(qfbpow(g, 2)) == id, el++)));
print("{\"isprime_q\": ", isprime(q), ", \"h_eq_8_9_q\": ", h == 8*9*q, ", \"isfundamental\": ", isfundamental(D),
      ", \"split_primes_le_300\": ", nsplit, ", \"f_pow_h_not_identity\": ", bad,
      ", \"killed_by_h_over_2\": ", ann2, ", \"killed_by_h_over_3\": ", ann3, ", \"killed_by_h_over_q\": ", annq,
      ", \"two_part_elements_of_order_le_2\": ", el, "}");
}
