// Constant rank block followed by polynomial Gray updates. Physical fields are GF83.
struct GrayMatrix {
 std::array<E,83> high{};std::array<E,8> gold{};
 void add(const GrayMatrix& b){for(int i=0;i<83;++i)high[i]^=b.high[i];for(int j=0;j<8;++j)gold[j]^=b.gold[j];}
};
struct ConstantGray {
 E z,q,A;int w2,w3,kcount=0,outerdim=0;const GoldProjection& gold;
 std::array<E,83> piv{},coord{},usq{};std::vector<E> U,H,rdelta;E r0=0;
 GrayMatrix matrix;std::vector<GrayMatrix> singles;std::vector<std::vector<GrayMatrix>> pairs;
 std::vector<Row> rhs_table;uint64_t assignment=0;
 E right(E rhs)const{E out=0;while(rhs){int j=bit_index(rhs);if(!piv[j])throw std::runtime_error("constant right inverse missed pivot");rhs^=piv[j];out^=coord[j];}return out;}
 E base_r(E h)const{return right((mul(square(h),q)^mul(square(z),z))>>w3);}
 Row rhs(E h)const{
  E h2=square(h),e1=h2^h^z,e20=mul(A,h2)^mul(z,e1),e30=mul(h2,q)^mul(square(z),z),r=base_r(h);
  E e2=square(r)^mul(h,r)^e20,e3=mul(z,square(r))^e30;
  return {e2>>w2,gold.project(mul(e1,e2)^e3)&((E(1)<<gold.free_count)-1)};
 }
 GrayMatrix at(E h)const{
  GrayMatrix out;E e1=square(h)^h^z,e1h=mul(e1,h),mask=(E(1)<<gold.free_count)-1;
  for(int i=0;i<kcount;++i){out.high[i]=(usq[i]^mul(h,U[i]))>>w2;E g=gold.project(mul(e1^z,usq[i])^mul(e1h,U[i]))&mask;for(int j=0;j<gold.free_count;++j)if((g>>j)&1)out.gold[j]^=E(1)<<i;}
  return out;
 }
 ConstantGray(const Family& outer,E zz,E qq,E aa,int width2,int width3,const GoldProjection& gg):z(zz),q(qq),A(aa),w2(width2),w3(width3),gold(gg){
  if(!z||w3>83||outer.kernel.size()>20)throw std::runtime_error("constant backend field/outer cap");
  std::array<Row,83> columns{};
  for(int i=0;i<83;++i){E v=mul(z,square(E(1)<<i))>>w3,c=E(1)<<i;columns[i]={v,0};while(v){int j=bit_index(v);if(piv[j]){v^=piv[j];c^=coord[j];}else{piv[j]=v;coord[j]=c;break;}}}
  Family constant=solve(columns,{0,0});if(constant.rank!=83-w3)throw std::runtime_error("constant automorphism rank");U=constant.kernel;kcount=int(U.size());for(int i=0;i<kcount;++i)usq[i]=square(U[i]);H=outer.kernel;outerdim=int(H.size());E h0=outer.particular;r0=base_r(h0);matrix=at(h0);singles.resize(outerdim);pairs.resize(outerdim,std::vector<GrayMatrix>(outerdim));
  rhs_table.resize(uint64_t(1)<<outerdim);Row b0=rhs(h0);rhs_table[0]=b0;
  for(int i=0;i<outerdim;++i){rdelta.push_back(base_r(h0^H[i])^r0);singles[i]=at(h0^H[i]);singles[i].add(matrix);Row bi=rhs(h0^H[i]);bi^=b0;rhs_table[uint64_t(1)<<i]=bi;}
  for(int i=0;i<outerdim;++i)for(int j=i+1;j<outerdim;++j){GrayMatrix v=at(h0^H[i]^H[j]);v.add(matrix);v.add(singles[i]);v.add(singles[j]);for(int k=0;k<kcount;++k)if(v.high[k])throw std::runtime_error("non-affine high matrix");pairs[i][j]=pairs[j][i]=v;Row b=rhs(h0^H[i]^H[j]);b^=b0;b^=rhs_table[uint64_t(1)<<i];b^=rhs_table[uint64_t(1)<<j];rhs_table[(uint64_t(1)<<i)|(uint64_t(1)<<j)]=b;}
  for(int i=0;i<outerdim;++i)for(int j=i+1;j<outerdim;++j)for(int k=j+1;k<outerdim;++k){uint64_t x=(uint64_t(1)<<i)|(uint64_t(1)<<j)|(uint64_t(1)<<k);Row b=rhs(h0^H[i]^H[j]^H[k]);for(uint64_t sub=(x-1)&x;;sub=(sub-1)&x){b^=rhs_table[sub];if(!sub)break;}rhs_table[x]=b;}
  for(int j=0;j<outerdim;++j)for(uint64_t x=0;x<rhs_table.size();++x)if((x>>j)&1)rhs_table[x]^=rhs_table[x^(uint64_t(1)<<j)];
 }
 Family next(uint64_t index){
  if(index){int j=__builtin_ctzll(index);for(int i=0;i<kcount;++i)matrix.high[i]^=singles[j].high[i];for(int g=0;g<gold.free_count;++g)matrix.gold[g]^=singles[j].gold[g];uint64_t others=assignment&~(uint64_t(1)<<j);while(others){int k=__builtin_ctzll(others);others&=others-1;for(int g=0;g<gold.free_count;++g)matrix.gold[g]^=pairs[j][k].gold[g];}assignment^=uint64_t(1)<<j;r0^=rdelta[j];}
  std::array<Row,83> columns{};for(int i=0;i<kcount;++i){columns[i].lo=matrix.high[i];for(int g=0;g<gold.free_count;++g)columns[i].hi|=((matrix.gold[g]>>i)&1)<<g;}
  Family reduced=solve(columns,rhs_table[assignment],kcount);auto physical=[&](E c){E out=0;while(c){int j=bit_index(c);out^=U[j];c^=E(1)<<j;}return out;};Family result;result.consistent=reduced.consistent;result.particular=r0^physical(reduced.particular);result.reduced_rank=reduced.rank;result.coordinate_count=kcount;for(E v:reduced.kernel)result.kernel.push_back(physical(v));result.rank=83-int(result.kernel.size());return result;
 }
};
