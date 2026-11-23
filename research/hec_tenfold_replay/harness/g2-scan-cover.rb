s=File.read('certificates.cpp')
key='  int used = 0, missed_classes = 0;'
raise unless s.scan(key).size==1
puts '#include <functional>'
s=s.sub(key,<<'CPP')
  // A lossless one-root scan may omit no entire triple. A lossless
  // two-root scan may omit no pair occurring together in any triple.
  assert(n<63);
  using Mask=unsigned long long;
  std::vector<Mask> adj(n);
  std::vector<std::vector<Mask>> third(n,std::vector<Mask>(n));
  long long signed_rows=0,unique_triples=0,one_prefix=0,two_prefix=0;
  std::vector<bool> square(p);for(int y=1;y<p;y++)square[m(y*y)]=true;
  if(p==127)assert(m(81LL*81*81*81*81+3LL*81*81*81+2*81*81+81+c)==0);
  else {
    int a=0,b=0;
    for(int coefficient:{1,0,3,2,1,c}){
      int constant=m(coefficient-97*b);b=m(a-222*b);a=constant;
    }
    assert(!a && !b && m(222*222-4*97) && !square[m(222*222-4*97)]);
  }
  Mask odd=0;
  for(int j=0;j<n;j++){
    int x=xs[j],g=p==127?m(x-81):m(x*x+222*x+97);assert(g);
    if(!square[g])odd|=1ULL<<j;
  }
  int positive=n-__builtin_popcountll(odd);
  long long pure=0,matched_prefix=0;
  for(const auto &[id,k]:subgroup){
    auto tmp=id/(1LL*p*p);
    int u=tmp/p,v=tmp%p;
    for(auto tr:table[k]){
      int ix[3];bool admissible=true;
      for(int j=0;j<3;j++){
        ix[j]=std::abs(tr[j])-1;int x=xs[ix[j]];
        if(!m(x*x+v*x+u))admissible=false;
      }
      if(!admissible)continue;
      ++signed_rows;
      Mask support=(1ULL<<ix[0])|(1ULL<<ix[1])|(1ULL<<ix[2]);
      int negatives=__builtin_popcountll(odd & support);
      assert(negatives==0 || negatives==2);
      pure+=negatives==0;matched_prefix+=ix[0]<positive && ix[1]<positive && ix[2]<positive;
      int hits1=0,hits2=0;
      for(int j:ix){hits1+=j<(n+3)/4;hits2+=j<(3*n+3)/4;}
      one_prefix+=hits1>=1;two_prefix+=hits2>=2;
      if(!(third[ix[0]][ix[1]]&(1ULL<<ix[2])))++unique_triples;
      for(int i=0;i<3;i++)for(int j=0;j<3;j++)if(i!=j){
        adj[ix[i]] |= 1ULL<<ix[j];
        third[ix[i]][ix[j]] |= 1ULL<<ix[3-i-j];
      }
    }
  }
  assert(signed_rows==eligible-overlaps);
  for(int need:{1,2}){
    Mask best=0;
    long long visited=0;
    std::function<void(Mask,Mask)> search=[&](Mask chosen,Mask candidates){
      ++visited;
      if(__builtin_popcountll(chosen)>__builtin_popcountll(best))best=chosen;
      while(candidates){
        if(__builtin_popcountll(chosen)+__builtin_popcountll(candidates)<=__builtin_popcountll(best))break;
        int i=__builtin_ctzll(candidates);candidates&=candidates-1;
        Mask next=candidates;
        if(need==2)next&=~adj[i];
        else for(Mask prior=chosen;prior;prior&=prior-1)next&=~third[i][__builtin_ctzll(prior)];
        search(chosen|(1ULL<<i),next);
      }
    };
    search(0,(1ULL<<n)-1);
    for(int i=0;i<n;i++)if(best&(1ULL<<i)){
      if(need==2)assert(!(adj[i]&best));
      for(int j=0;j<n;j++)if(best&(1ULL<<j))assert(!(third[i][j]&best));
    }
    if(need==1)assert(__builtin_popcountll(best)==__builtin_popcountll(odd));
    printf("%d %d %d %lld %lld %lld %d %llu %lld %d %lld %lld\n",p,n,need,signed_rows,unique_triples,
           need==1?one_prefix:two_prefix,n-__builtin_popcountll(best),best,visited,positive,pure,matched_prefix);
  }
  return;
  int used = 0, missed_classes = 0;
CPP
if ARGV.include?('expanded')
  s=s.sub('        xs.push_back(x);', <<'CPP')
        int factor=p==127?m(x-81):m(x*x+222*x+97);
        bool positive=false;
        for(int t=1;t<=p/2;t++)positive|=m(t*t)==factor;
        if(!positive)break;
        xs.push_back(x);
CPP
  s=s.sub('  assert(n<63);', '  assert(n==limit); printf("%d %d %lld\n",p,n,eligible-overlaps); return;')
end
puts s
