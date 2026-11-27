#include "audit_arithmetic.hpp"

namespace signed_audit {
using namespace audit;
using Sparse=std::map<int,int>;
using Rows=std::set<Sparse>;
struct Joined{Rows rows;size_t trivial=0;};
Sparse equality(Pair a,Pair b0){
    Sparse row;
    auto add=[&](int label,int sign){if(label)row[(label-1)/2]+=((label%2)?1:-1)*sign;};
    for(int i:a)add(i,1);
    for(int i:b0)add(i,-1);
    for(auto it=row.begin();it!=row.end();){if(it->second==0)it=row.erase(it);else ++it;}
    if(!row.empty()&&row.begin()->second<0)for(auto& entry:row)entry.second=-entry.second;
    return row;
}
void collect(Joined& result,Pair a,Pair b0){auto row=equality(a,b0);if(row.empty())++result.trivial;else result.rows.insert(row);require(result.rows.size()<=4096,"signed nonzero candidate cap; not exhaustive");}
Joined grouped_signed(const std::vector<Item>& pairs){
    std::map<std::string,std::vector<Pair>> buckets;for(const auto& item:pairs)buckets[item.sum].push_back(item.indices);Joined out;
    for(const auto& item:buckets)for(size_t i=0;i<item.second.size();++i)for(size_t j=i+1;j<item.second.size();++j)collect(out,item.second[i],item.second[j]);
    return out;
}
Joined sorted_signed(const std::vector<Item>& pairs){
    std::vector<std::pair<std::string,Pair>> sorted;for(const auto& item:pairs)sorted.emplace_back(item.sum,item.indices);std::sort(sorted.begin(),sorted.end());Joined out;
    for(size_t i=0;i<sorted.size();){size_t end=i+1;while(end<sorted.size()&&sorted[end].first==sorted[i].first)++end;for(size_t j=i;j<end;++j)for(size_t k=j+1;k<end;++k)collect(out,sorted[j].second,sorted[k].second);i=end;}
    return out;
}
Row dense(const Sparse& sparse,size_t width){Row row(width);for(const auto& item:sparse)row.at(size_t(item.first))=item.second;return row;}
Point multiply(Point q,Big n){require(n>=0,"nonnegative known scalar");Point out;while(n!=0){if(mpz_odd_p(n.get_mpz_t()))out=plus(out,q);q=plus(q,q);n/=2;}return out;}
std::vector<Item> enumerate(const std::vector<Point>& base,bool openssl){
    std::vector<Point> symbols(1);for(const auto& q:base){require(!q.zero&&on_curve(q),"signed alphabet points");symbols.push_back(q);symbols.push_back(negative(q));}
    audit::SSL ssl;std::vector<SSLPoint> points;for(const auto& q:symbols)points.push_back(ssl.make(q));auto out_point=ssl.make({});std::vector<Item> out;
    for(size_t i=0;i<symbols.size();++i)for(size_t j=i;j<symbols.size();++j){std::string sum;if(openssl){require(EC_POINT_add(ssl.group.get(),out_point.get(),points[i].get(),points[j].get(),ssl.context.get())==1,"signed SSL addition");sum=ssl.encode(out_point.get());}else sum=key(plus(symbols[i],symbols[j]));out.push_back({{int(i),int(j)},sum,""});}
    require(out.size()==symbols.size()*(symbols.size()+1)/2,"complete signed pair count");return out;
}
void emit_point(std::ostream& out,const Point& q){if(q.zero)out<<"{\"infinity\":true}";else out<<"{\"infinity\":false,\"x\":\""<<q.x<<"\",\"y\":\""<<q.y<<"\"}";}
int run(int argc,char** argv){
    require(argc==3,"usage: signed_words FROZEN.json OUTPUT.json");auto data=read_json(argv[1]);std::vector<Point> base;for(const auto& item:data.at("projected_base").arr)base.push_back(read_point(item));require(base.size()==225,"frozen base225");
    auto first=enumerate(base,true),second=enumerate(base,false);auto digest=fingerprint(first);require(digest==fingerprint(second),"complete signed fingerprint match");auto a=grouped_signed(first),c=sorted_signed(second);require(a.rows==c.rows&&a.trivial==c.trivial,"independent signed joins agree");
    Matrix old;for(const auto& sparse:data.at("rows").arr){Row row(base.size());for(const auto& item:sparse.arr)row.at(size_t(item.arr[0].integer()))+=Big(item.arr[1].scalar);require(row_sum(row,base).zero,"baseline row sum");old.push_back(row);}require(old.size()==6&&matrix_rank(old)==6,"baseline six independent rows");Matrix all=old;for(const auto& row:a.rows){auto v=dense(row,base.size());require(row_sum(v,base).zero,"retained signed row elliptic sum");all.push_back(v);}
    audit::SSL ssl;auto generator=EC_GROUP_get0_generator(ssl.group.get());BN gx(BN_new()),gy(BN_new());require(EC_POINT_get_affine_coordinates(ssl.group.get(),generator,gx.get(),gy.get(),ssl.context.get())==1,"known generator");
    auto to_big=[](const BN& q){char* s=BN_bn2dec(q.get());require(s!=nullptr,"BN decimal output");Big out(s);OPENSSL_free(s);return out;};Point g{false,to_big(gx),to_big(gy)};std::vector<Point> known{g,plus(g,g)};auto control_pairs=enumerate(known,true),control_independent=enumerate(known,false);require(fingerprint(control_pairs)==fingerprint(control_independent),"control arithmetic fingerprint");auto control=grouped_signed(control_pairs),control2=sorted_signed(control_independent);require(control.rows==control2.rows&&control.trivial==control2.trivial&&control.trivial>0,"known-answer joins and cancellation");Sparse expected{{0,2},{1,-1}};require(control.rows.count(expected)==1,"retain nontrivial 2G minus2G relation");
    int controls=2;auto missing=first;missing.pop_back();rejects([&]{require(fingerprint(missing)==fingerprint(second),"missing signed pair");});++controls;rejects([&]{require(on_curve({false,0,0}),"off-curve signed point");});++controls;Row bad=dense(expected,2);++bad[0];rejects([&]{require(row_sum(bad,known).zero,"mutated known row");});++controls;
    Point kappa;Row canonical(base.size());const auto& reference=data.at("pool").at("split_fibers").arr[0].at("point_indices").arr;for(const auto& index:reference){const auto& image=data.at("images").arr.at(size_t(index.integer()));kappa=plus(kappa,read_point(image.at("point")));int j=image.at("projected_index").integer();if(j>=0)canonical.at(size_t(j))+=image.at("sign").integer();}
    const Big inverse_two=(subgroup_order+1)/2;Point half=multiply(kappa,inverse_two);require(plus(half,half)==kappa,"known halftrace double");auto sk=ssl.make(kappa),sh=ssl.make({});auto scalar=bn(inverse_two);require(EC_POINT_mul(ssl.group.get(),sh.get(),nullptr,sk.get(),scalar.get(),ssl.context.get())==1&&ssl.encode(sh.get())==key(half),"independent halftrace multiplication");Matrix aliases;for(size_t j=0;j<base.size();++j)for(int sign:{1,-1})if(half==(sign==1?base[j]:negative(base[j]))){Row row=canonical;for(auto& x:row)x=-x;row[j]+=2*sign;require(row_sum(row,base).zero,"halftrace alias norm");aliases.push_back(row);all.push_back(row);}
    require(a.rows.size()+aliases.size()<=256,"actual target check cap");std::ofstream out(argv[2]);require(bool(out),"fresh signed output");out<<"{\"standing\":\"producer-unverified\",\"scope\":\"frozen-canonical-base-signed-L1-at-most4\",\"alphabet_size\":451,\"unordered_pairs_with_diagonals\":"<<first.size()<<",\"pair_fingerprint_sha256\":\""<<digest<<"\",\"independent_pair_streams_match\":true,\"independent_join_sets_match\":true,\"formal_collision_aliases\":"<<a.trivial<<",\"known_answer_nonzero_rows\":"<<control.rows.size()<<",\"control_checks\":"<<controls<<",\"baseline_rank\":6,\"union_rank\":"<<matrix_rank(all)<<",\"new_curve_points\":0,\"signed_rows\":[";bool comma=false;for(const auto& row:a.rows){if(comma)out<<',';comma=true;emit_row(out,dense(row,base.size()));}out<<"],\"halftrace\":";emit_point(out,half);out<<",\"canonical_trace\":";emit_point(out,kappa);out<<",\"halftrace_alias_rows\":[";comma=false;for(const auto& row:aliases){if(comma)out<<',';comma=true;emit_row(out,row);}out<<"]}\n";require(bool(out),"complete signed result");return 0;
}
}
