\\ Independent enumeration of the first 512 small-x base abscissae and
\\ all signed two-point sum abscissae on Certicom ECCp-131.
p = 0x048E1D43F293469E33194C43186B3ABC0B;
a = 0x041CB121CE2B31F608A76FC8F23D73CB66;
b = 0x02F74F717E8DEC90991E5EA9B2FF03DA58;
n = 0x048E1D43F293469E317F7ED728F6B8E6F1;
qx = 0x03AA6F004FC62E2DA1ED0BFB62C3FFB568;
E = ellinit([a,b],p);
target_size = 512;
base = List();
x = 0;
while(#base < target_size,
  y = ellordinate(E,Mod(x,p));
  if(#y > 0, listput(base,[Mod(x,p),y[1]]));
  x++;
);
largest_x = x-1;
base = Vec(base);
keys = List();
for(i=1,target_size,
  s = elladd(E,base[i],base[i]);
  if(#s == 2, listput(keys,lift(s[1])));
  for(j=i+1,target_size,
    s = elladd(E,base[i],base[j]);
    if(#s == 2, listput(keys,lift(s[1])));
    s = ellsub(E,base[i],base[j]);
    if(#s == 2, listput(keys,lift(s[1])));
  );
);
raw_keys = #keys;
keys = Set(Vec(keys));
print("factor_base_abscissae=",#base);
print("largest_small_x=",largest_x);
print("raw_pair_sum_abscissae=",raw_keys);
print("distinct_pair_sum_abscissae=",#keys);
print("covered_nonzero_points=",2*#keys);
print("published_Q_direct_pair_sum=",setsearch(keys,qx)>0);
print("exact_uniform_hit_fraction=",2*#keys/(n-1));
