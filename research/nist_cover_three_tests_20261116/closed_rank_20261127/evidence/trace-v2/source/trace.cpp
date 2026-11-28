#define main frozen_norm_entrypoint
#include "norms.cpp"
#undef main

namespace trace_test {
using namespace norm_test;
struct Domain {int multiple;Streams stream;Found found;};
int run(int argc,char**argv){
 require(argc==4,"usage: trace BASELINE TANGENTS old|augmented");
 auto baseline=read_json(argv[1]),tangents=read_json(argv[2]);
 bool augmented=std::string(argv[3])=="augmented";
 require(augmented||std::string(argv[3])=="old","explicit trace domain");
 const auto&source=augmented?tangents.at("projected_base"):baseline.at("projected_base");
 std::vector<Point> base;
 for(const auto&q:source.arr){auto p=read_point(q);require(!p.zero&&on_curve(p)&&p.y<=::audit::p-p.y,"canonical fixed-domain point");base.push_back(p);}
 require(base.size()==(augmented?429u:225u),"fixed trace width");
 for(size_t i=0;i<225;++i){require(base[i]==read_point(baseline.at("projected_base").arr[i]),"fixed old-column prefix");}
 Sparse reference;Point kappa;
 for(const auto&i:baseline.at("pool").at("split_fibers").arr[0].at("point_indices").arr){const auto&image=baseline.at("images").arr[size_t(i.integer())];kappa=plus(kappa,read_point(image.at("point")));int j=image.at("projected_index").integer();if(j>=0)reference[j]+=image.at("sign").integer();}
 require(!kappa.zero,"nonidentity reference sum");
 SCMat original;
 if(augmented)original=sc_read_matrix(tangents.at("matrix"));
 else{original.assign(6,SCRow(225));for(size_t i=0;i<6;++i){for(const auto&t:baseline.at("rows").arr[i].arr)original[i].at(size_t(t.arr[0].integer()))+=SCBig(t.arr[1].scalar);}}
 require(sc_rank(original)==(augmented?108u:6u),"original matrix rank");
 for(const auto&r:original){require(r.size()==base.size()&&row_sum(r,base).zero,"original elliptic row");}
 SCMat all=original;std::vector<Domain> domains;
 for(int m=0;m<=2;++m){
  Sparse ref;for(auto[j,c]:reference){if(m!=0&&c!=0)ref[j]=m*c;}
  auto stream=enumerate(base,multiply(kappa,SCBig(m)));
  auto found=grouped(stream.pairs,ref),checked=sorted(stream.pairs,ref);
  require(found==checked,"complete trace-domain grouped/sorted witness equality");
  if(m==1)require_original(found,original);
  for(const auto&[s,q]:found){(void)q;auto row=dense(s,base.size());require(row_sum(row,base).zero,"independent actual trace-domain row sum");all.push_back(std::move(row));}
  domains.push_back({m,std::move(stream),std::move(found)});
 }
 size_t full=sc_rank(all);SCMat auxiliary;
 if(augmented){for(const auto&r:all)auxiliary.emplace_back(r.begin()+225,r.end());}
 size_t arank=sc_rank(auxiliary);require(full>=arank+6,"baseline closure retained");
 audit::SSL ssl;auto generator=EC_GROUP_get0_generator(ssl.group.get());BN gx(BN_new()),gy(BN_new());
 require(EC_POINT_get_affine_coordinates(ssl.group.get(),generator,gx.get(),gy.get(),ssl.context.get())==1,"public generator fixture");
 auto integer=[](const BN&n){char*s=BN_bn2dec(n.get());require(s,"generator decimal");Big b(s);OPENSSL_free(s);return b;};
 Point g{false,integer(gx),integer(gy)};std::vector<Point> fixture{g,plus(g,g)};
 for(int branch:{0,2}){
  Point target=multiply(g,SCBig(4*branch));Sparse ref;if(branch)ref[0]=4*branch;
  auto stream=enumerate(fixture,target);auto a=grouped(stream.pairs,ref),b=sorted(stream.pairs,ref);
  Sparse expected=branch?Sparse{{0,-8},{1,4}}:Sparse{{0,-2},{1,1}};
  require(a==b&&a.count(expected),"nonzero signed trace-branch fixture detected");
  auto bad=dense(expected,2);++bad[0];rejects_norm([&]{require(row_sum(bad,fixture).zero,"changed trace fixture row");});
 }
 std::cout<<"{\"scope\":\"fixed-domain-single-residual-trace-necessary-rank-bound\",\"augmented\":"<<(augmented?"true":"false")<<",\"old_columns\":225,\"total_columns\":"<<base.size()<<",\"full_rank\":"<<full<<",\"auxiliary_rank\":"<<arank<<",\"closed_rank\":"<<full-arank<<",\"candidate_gain\":"<<full-arank-6<<",\"controls_pass\":true,\"positive_gain_established\":false,\"domains\":[";
 bool comma=false;
 for(const auto&d:domains){if(comma)std::cout<<',';comma=true;std::cout<<"{\"target_multiple\":"<<d.multiple<<",\"pairs\":"<<d.stream.pairs.size()<<",\"pair_stream_sha256\":\""<<d.stream.digest<<"\",\"distinct_rows_including_zero\":"<<d.found.size()<<",\"rows\":[";bool sep=false;for(const auto&[s,q]:d.found){if(sep)std::cout<<',';sep=true;std::cout<<"{\"symbols\":[";for(size_t i=0;i<4;++i)std::cout<<(i?",":"")<<q[i];std::cout<<"],\"row\":";sc_emit_row(std::cout,dense(s,base.size()));std::cout<<'}';}std::cout<<"]}";}
 std::cout<<"],\"standing\":\"producer-unverified; positive norm candidates require actual geometric/KM and separate replay\"}\n";
 return 0;
}
}
int main(int argc,char**argv){try{return trace_test::run(argc,argv);}catch(const std::exception&e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
