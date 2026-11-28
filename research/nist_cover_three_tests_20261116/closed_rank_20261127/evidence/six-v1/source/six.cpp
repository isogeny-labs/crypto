#define main frozen_norm_entrypoint
#include "norms.cpp"
#undef main

namespace six_test {
using namespace norm_test;
using Packed=std::array<unsigned char,32>;
static_assert(sizeof(Packed)==32,"fixed triple-table payload");
struct Witness {std::array<int,6> labels{};int second_sign=0,overall_sign=1;};
using Candidates=std::map<Sparse,Witness>;
struct Table {std::vector<Packed> rows;std::string digest;};
struct TripHash {
 Digest hash;
 void add(const std::array<int,3>&labels,const std::string&sum){
  std::array<unsigned char,6> bytes{};
  for(size_t i=0;i<3;++i){bytes[2*i]=static_cast<unsigned char>(labels[i]&255);bytes[2*i+1]=static_cast<unsigned char>((labels[i]>>8)&255);}
  unsigned char length=static_cast<unsigned char>(sum.size());
  require(EVP_DigestUpdate(hash.c.get(),bytes.data(),bytes.size())==1&&EVP_DigestUpdate(hash.c.get(),&length,1)==1&&EVP_DigestUpdate(hash.c.get(),sum.data(),sum.size())==1,"triple ordered-stream hash");
 }
};
int label(const Packed&r,size_t i){return int(r[25+2*i])+(int(r[26+2*i])<<8);}
bool same_key(const Packed&a,const Packed&b){return std::equal(a.begin(),a.begin()+25,b.begin());}
size_t expected_count(size_t symbols){return symbols*(symbols+1)*(symbols+2)/6;}
void complete(const Table&t,size_t symbols){require(t.rows.size()==expected_count(symbols),"complete unordered triple domain");}
Table enumerate_triples(const std::vector<Point>&base){
 std::vector<Point> symbols(1);for(const auto&p:base){require(!p.zero&&on_curve(p),"nonzero triple symbol");symbols.push_back(p);symbols.push_back(negative(p));}
 require(symbols.size()<1024,"bounded signed symbol domain");
 audit::SSL ssl;std::vector<SSLPoint> points;for(const auto&p:symbols)points.push_back(ssl.make(p));
 auto pair=ssl.make({}),sum=ssl.make({});Table table;table.rows.reserve(expected_count(symbols.size()));TripHash first,second;
 for(size_t i=0;i<symbols.size();++i){
  for(size_t j=i;j<symbols.size();++j){
   require(EC_POINT_add(ssl.group.get(),pair.get(),points[i].get(),points[j].get(),ssl.context.get())==1,"named triple pair");
   Point ab=plus(symbols[i],symbols[j]);
   for(size_t k=j;k<symbols.size();++k){
    require(EC_POINT_add(ssl.group.get(),sum.get(),pair.get(),points[k].get(),ssl.context.get())==1,"named triple sum");
    auto encoded=ssl.encode(sum.get()),independent=key(plus(ab,symbols[k]));require(encoded==independent,"every independent triple sum matches");
    std::array<int,3> labels{int(i),int(j),int(k)};first.add(labels,encoded);second.add(labels,independent);
    Packed record{};
    if(encoded.size()==25){require(encoded[0]==2||encoded[0]==3,"compressed triple orientation");record[0]=2;for(size_t a=1;a<25;++a)record[a]=static_cast<unsigned char>(encoded[a]);record[31]=encoded[0]==2?1:2;}
    else require(encoded.size()==1&&encoded[0]==0,"identity triple encoding");
    for(size_t a=0;a<3;++a){record[25+2*a]=static_cast<unsigned char>(labels[a]&255);record[26+2*a]=static_cast<unsigned char>((labels[a]>>8)&255);}
    table.rows.push_back(record);
   }
  }
  if(i%32==0)std::cerr<<"triple first-symbol "<<i<<" records "<<table.rows.size()<<'\n';
 }
 complete(table,symbols.size());table.digest=first.hash.finish();require(table.digest==second.hash.finish(),"full independent triple digest equality");return table;
}
Witness difference(const Packed&a,const Packed&b){
 require(same_key(a,b)&&a[0]!=0&&b[0]!=0,"nonidentity same-sum group");Witness w;
 for(size_t i=0;i<3;++i){w.labels[i]=label(a,i);w.labels[i+3]=label(b,i);}
 int oa=a[31]==1?1:-1,ob=b[31]==1?1:-1;w.second_sign=-oa*ob;return w;
}
Witness identity(const Packed&a){Witness w;for(size_t i=0;i<3;++i)w.labels[i]=label(a,i);return w;}
Sparse coefficients(const Witness&w){Sparse row;for(size_t i=0;i<6;++i){int multiplier=i<3?1:w.second_sign,label=w.labels[i];if(multiplier&&label)row[(label-1)/2]+=w.overall_sign*multiplier*((label%2)?1:-1);}for(auto i=row.begin();i!=row.end();)if(i->second==0)i=row.erase(i);else ++i;return row;}
void keep(Candidates&out,Witness w){auto row=coefficients(w);if(row.empty())return;if(row.begin()->second<0){w.overall_sign=-1;for(auto&[j,c]:row){(void)j;c=-c;}}out.emplace(std::move(row),w);require(out.size()<=4096,"six-word nonzero-row cap; completeness rejected");}
Candidates grouped(Table&t,size_t&groups,size_t&largest){
 std::sort(t.rows.begin(),t.rows.end());Candidates found;groups=0;largest=0;
 for(size_t i=0;i<t.rows.size();){size_t end=i+1;while(end<t.rows.size()&&same_key(t.rows[i],t.rows[end]))++end;++groups;largest=std::max(largest,end-i);
  if(t.rows[i][0]==0){for(size_t j=i;j<end;++j)keep(found,identity(t.rows[j]));}
  else{for(size_t j=i+1;j<end;++j)keep(found,difference(t.rows[j],t.rows[i]));}
  i=end;
 }
 return found;
}
void controls(){
 audit::SSL ssl;auto generator=EC_GROUP_get0_generator(ssl.group.get());BN gx(BN_new()),gy(BN_new());require(EC_POINT_get_affine_coordinates(ssl.group.get(),generator,gx.get(),gy.get(),ssl.context.get())==1,"fixture generator");
 auto integer=[](const BN&n){char*s=BN_bn2dec(n.get());require(s,"generator decimal");Big v(s);OPENSSL_free(s);return v;};Point g{false,integer(gx),integer(gy)};std::vector<Point> base{g,plus(g,g)};
 auto table=enumerate_triples(base);size_t groups=0,largest=0;auto found=grouped(table,groups,largest);require(found.count(Sparse{{0,2},{1,-1}}),"nonzero six-word fixture detected");
 Candidates brute;
 for(size_t i=0;i<table.rows.size();++i){if(table.rows[i][0]==0)keep(brute,identity(table.rows[i]));else for(size_t j=0;j<i;++j)if(same_key(table.rows[i],table.rows[j]))keep(brute,difference(table.rows[i],table.rows[j]));}
 SCMat a,b;for(const auto&[s,w]:found){require(coefficients(w)==s,"fixture witness row");a.push_back(dense(s,2));require(row_sum(a.back(),base).zero,"fixture group row");}for(const auto&[s,w]:brute){(void)w;b.push_back(dense(s,2));}
 size_t rank=sc_rank(a);a.insert(a.end(),b.begin(),b.end());require(rank==1&&sc_rank(b)==rank&&sc_rank(a)==rank,"representative and brute-force fixture row spans match");
 auto bad=dense(found.begin()->first,2);++bad[0];rejects_norm([&]{require(row_sum(bad,base).zero,"changed six-word witness coefficient");});table.rows.pop_back();rejects_norm([&]{complete(table,5);});
}
int run(int argc,char**argv){
 require(argc==4,"usage: six BASELINE TANGENTS 64|225");int count=std::stoi(argv[3]);require(count==64||count==225,"frozen six-word domain");auto baseline=read_json(argv[1]),tangents=read_json(argv[2]);
 std::vector<Point> base;for(const auto&q:baseline.at("projected_base").arr){auto p=read_point(q);require(!p.zero&&on_curve(p)&&p.y<=::audit::p-p.y,"fixed signed-canonical old point");base.push_back(p);}require(base.size()==225,"unchanged original columns");
 for(size_t i=0;i<225;++i){require(base[i]==read_point(tangents.at("projected_base").arr[i]),"original matrix prefix");}
 controls();std::vector<Point> selected(base.begin(),base.begin()+count);auto table=enumerate_triples(selected);size_t groups=0,largest=0;auto found=grouped(table,groups,largest);
 SCMat old(6,SCRow(225));for(size_t i=0;i<6;++i){for(const auto&t:baseline.at("rows").arr[i].arr)old[i].at(size_t(t.arr[0].integer()))+=SCBig(t.arr[1].scalar);require(row_sum(old[i],base).zero,"original baseline elliptic row");}
 require(sc_rank(old)==6,"original baseline rank");auto full=sc_read_matrix(tangents.at("matrix"));require(sc_rank(full)==108,"original tangent rank");
 for(const auto&[s,w]:found){require(coefficients(w)==s,"retained six-word witness");auto row=dense(s,225);require(row_sum(row,base).zero,"independent retained six-word elliptic row");old.push_back(row);row.resize(429);full.push_back(std::move(row));}
 size_t rank=sc_rank(old),frank=sc_rank(full);SCMat auxiliary;for(const auto&r:full)auxiliary.emplace_back(r.begin()+225,r.end());size_t arank=sc_rank(auxiliary);require(arank==102&&frank-arank==rank,"separate original/auxiliary closure equality");
 std::cout<<"{\"scope\":\"signed-old-base-length-at-most-six-norm-rows\",\"domain_columns\":"<<count<<",\"old_columns\":225,\"signed_alphabet\":"<<2*count+1<<",\"unordered_triples\":"<<table.rows.size()<<",\"packed_record_bytes\":32,\"table_payload_bytes\":"<<table.rows.size()*32<<",\"triple_stream_sha256\":\""<<table.digest<<"\",\"independent_streams_match\":true,\"sum_groups\":"<<groups<<",\"largest_sum_group\":"<<largest<<",\"distinct_nonzero_spanning_rows\":"<<found.size()<<",\"full_rank\":"<<frank<<",\"auxiliary_rank\":"<<arank<<",\"closed_rank\":"<<rank<<",\"candidate_gain\":"<<rank-6<<",\"controls_pass\":true,\"positive_gain_established\":false,\"rows\":[";
 bool comma=false;for(const auto&[s,w]:found){if(comma)std::cout<<',';comma=true;std::cout<<"{\"labels\":[";for(size_t i=0;i<6;++i)std::cout<<(i?",":"")<<w.labels[i];std::cout<<"],\"second_sign\":"<<w.second_sign<<",\"overall_sign\":"<<w.overall_sign<<",\"row\":";sc_emit_row(std::cout,dense(s,225));std::cout<<'}';}
 std::cout<<"],\"standing\":\"producer-unverified; positive norm candidates need actual cover/KM and independent replay\"}\n";return 0;
}
}
int main(int argc,char**argv){try{return six_test::run(argc,argv);}catch(const std::exception&e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
