struct SupportGuard {
 int prefix=0,rank=0,image_dimension=0;uint64_t tests=0,rejected=0,verified_rejected=0;
 std::array<E,83> pivots{};std::array<std::array<E,256>,11> tables{};
 E reduce(E v)const{for(int j=82;j>=0;--j)if(pivots[j]&&((v>>j)&1))v^=pivots[j];return v;}
 SupportGuard(int dimension,int cut,const std::vector<E>& scalars):prefix(cut){
  if(cut<1||cut>=dimension)throw std::runtime_error("support guard requires0<cut<dimension");
  std::vector<E> image;
  for(int j=cut;j<dimension;++j){E value=E(1)<<j;for(int i=0;i<cut;++i)value=square(value)^mul(scalars[i],value);image.push_back(value);}
  image_dimension=int(image.size());
  for(size_t i=0;i<image.size();++i)for(size_t j=i;j<image.size();++j){E v=mul(image[i],image[j]);while(v){int k=bit_index(v);if(pivots[k])v^=pivots[k];else{pivots[k]=v;++rank;break;}}}
  std::array<E,83> units{};for(int j=0;j<83;++j)units[j]=reduce(E(1)<<j);
  for(int byte=0;byte<11;++byte)for(int value=0;value<256;++value){E out=0;for(int bit=0;bit<8;++bit)if((value>>bit)&1)if(8*byte+bit<83)out^=units[8*byte+bit];tables[byte][value]=out;}
 }
 bool member(E v)const{E out=0;for(int i=0;i<11;++i)out^=tables[i][unsigned((v>>(8*i))&255)];return out==0;}
 bool accepts(const Cubic& h,const Remainder& q){++tests;E t=square(q[0])^mul(square(q[1]),h.e2)^mul(mul(q[1],q[2]),h.x4b)^mul(square(q[2]),square(h.e2));bool ok=member(t);if(!ok)++rejected;return ok;}
};
static int guard_prefix_requested=0;
static bool verify_support_guard=false;
static std::unique_ptr<SupportGuard> support_guard;
