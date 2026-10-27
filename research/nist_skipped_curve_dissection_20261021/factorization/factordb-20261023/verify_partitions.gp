\\ Offline exact arithmetic only; inputs are decimal literals validated by the shell reader.
partition_check(r) = {
  my(n=r[2], api=r[3], known=r[4]);
  my(ap=prod(i=1,#api,api[i][1]^api[i][2]));
  my(kp=prod(i=1,#known,known[i][1]^known[i][2]));
  my(left=vector(#known,i,known[i][2]), coarsening=1);
  foreach(api,pair,
    for(k=1,pair[2],
      my(v=pair[1]);
      for(i=1,#known,
        while(left[i]>0 && v%known[i][1]==0,
          v/=known[i][1];left[i]--;
        );
      );
      if(v!=1,coarsening=0);
    );
  );
  [ap==n,kp==n,coarsening && vecsum(left)==0];
};
boolstr(v) = if(v,"true","false");
{
  read(getenv("FDB_INPUTS"));
  foreach(DATA,r,
    my(v=partition_check(r));
    print("{\"target\":\"",r[1],"\",\"api_product_verified\":",boolstr(v[1]),
      ",\"local_product_verified\":",boolstr(v[2]),
      ",\"api_partition_is_coarsening_of_local\":",boolstr(v[3]),"}");
  );
  my(bad=Vec(DATA)[1]);bad[2]++;
  my(v=partition_check(bad));
  print("{\"control\":\"wrong_integer\",\"rejected\":",boolstr(!v[1] && !v[2]),"}");
}
quit;
