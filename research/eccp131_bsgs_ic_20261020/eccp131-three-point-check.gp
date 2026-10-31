\\ Independent PARI/GP check of a three-point relation outside the
\\ two-point support of the first 16 small-x ECCp-131 base abscissae.
p = 0x048E1D43F293469E33194C43186B3ABC0B;
a = 0x041CB121CE2B31F608A76FC8F23D73CB66;
b = 0x02F74F717E8DEC90991E5EA9B2FF03DA58;
E = ellinit([a,b],p);
run() = {
  base = List();
  x = 0;
  while(#base < 16,
    y = ellordinate(E,Mod(x,p));
    if(#y > 0, listput(base,[Mod(x,p),y[1]]));
    x++;
  );
  base = Vec(base);
  keys = List();
  for(i=1,16,
    s = elladd(E,base[i],base[i]);
    if(#s == 2, listput(keys,lift(s[1])));
    for(j=i+1,16,
      s = elladd(E,base[i],base[j]);
      if(#s == 2, listput(keys,lift(s[1])));
      s = ellsub(E,base[i],base[j]);
      if(#s == 2, listput(keys,lift(s[1])));
    );
  );
  keys = Set(Vec(keys));
  q = elladd(E,elladd(E,base[1],base[2]),base[3]);
  print("field_prime=",p);
  print("factor_base_abscissae=",#base);
  print("distinct_pair_sum_abscissae=",#keys);
  print("three_point_x=",lift(q[1]));
  print("three_point_y=",lift(q[2]));
  print("three_point_in_two_point_support=",setsearch(keys,lift(q[1]))>0);
  print("three_point_identity=",q==elladd(E,elladd(E,base[1],base[2]),base[3]));
};
run();
