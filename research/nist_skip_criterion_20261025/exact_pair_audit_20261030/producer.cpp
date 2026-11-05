#include "io.hpp"
#include <algorithm>
#include <cstdint>
#include <functional>
#include <iostream>
#include <map>
#include <numeric>
#include <set>
using Count=uint64_t;
struct Row { std::string id,role,category; int bits,score; };
struct Field { int bits,k,h,pairs,wins=0,ties=0,losses=0,observed=0;std::vector<Count> histogram; };
static std::vector<Count> enumerate(const std::vector<int>& s,int k) {
    int n=static_cast<int>(s.size());std::vector<Count> histogram(static_cast<size_t>(2*k*(n-k)+1));
    std::function<void(int,int,uint32_t)> choose=[&](int at,int left,uint32_t mask) {
        if(!left) {
            int score=0;
            for(int i=0;i<n;++i)if(mask&(uint32_t(1)<<i))for(int j=0;j<n;++j)if(!(mask&(uint32_t(1)<<j)))
                score+=s[j]>s[i]?2:s[j]==s[i]?1:0;
            ++histogram[static_cast<size_t>(score)];return;
        }
        if(n-at<left)return;
        choose(at+1,left-1,mask|(uint32_t(1)<<at));choose(at+1,left,mask);
    };
    choose(0,k,0);return histogram;
}
static std::vector<Count> combine(const std::vector<Count>& a,const std::vector<Count>& b) {
    std::vector<Count> c(a.size()+b.size()-1);
    for(size_t i=0;i<a.size();++i)for(size_t j=0;j<b.size();++j)c[i+j]+=a[i]*b[j];
    return c;
}
static void control() {
    auto a=enumerate({1,1,2},1);require(a==std::vector<Count>({1,0,0,2,0}),"tied toy inventory");
    require(enumerate({1,2,3},1)==std::vector<Count>({1,0,1,0,1}),"ordered toy inventory");
    auto b=combine(a,a);require(b[0]==1&&b[3]==4&&b[6]==4&&std::accumulate(b.begin(),b.end(),Count(0))==9,"toy convolution");
    std::cout << "PASS explicit tied/ordered allocations and convolution\n";
}
int main(int argc,char** argv) {
try {
    if(argc==2&&std::string(argv[1])=="--self-test"){control();return 0;}
    require(argc==5&&std::string(argv[4])=="ACK_EXISTING_RESULT_AUDIT","producer BRANCHES EXTERNAL FRESH_OUT ACK_EXISTING_RESULT_AUDIT");
    auto raw=table(argv[1],14),external=table(argv[2],6);require(raw.size()==83&&external.size()==68,"complete source inventories");
    std::vector<Row> rows;std::map<std::string,Row> by_id;
    for(size_t i=1;i<raw.size();++i)if(raw[i][6]=="1") {
        const auto& c=raw[i];Row r{c[1],c[3],c[4],std::stoi(c[2]),std::stoi(c[7])};
        require(r.score>=0&&r.score<=124&&by_id.emplace(r.id,r).second,"unique bounded seed score");rows.push_back(r);
    }
    require(rows.size()==69,"all69 positions retained");
    std::ostringstream source_checks;source_checks << "source_id\tscope\tstatus\n";
    std::map<int,std::vector<int>> ext_pub,local_pub;std::vector<std::string> unkeyed;
    int exact=0;
    for(const auto& c:external) {
        if(c[1]=="NA_no_seed_id_in_named_row") {
            require(c[3]=="published","unkeyed publication only");ext_pub[std::stoi(c[2])].push_back(std::stoi(c[5]));unkeyed.push_back(c[0]);
        } else {
            require(by_id.count(c[1]),"external identity absent");const auto& r=by_id.at(c[1]);
            require(r.bits==std::stoi(c[2])&&r.role==c[3]&&r.category==c[4]&&r.score==std::stoi(c[5]),"external seed score mismatch");
            ++exact;tsv(source_checks,{c[0],"seed_score","MATCH"});
        }
    }
    for(const auto& r:rows)if(r.role=="published"&&r.bits<=256)local_pub[r.bits].push_back(r.score);
    require(exact==60&&unkeyed.size()==8&&local_pub.size()==4&&ext_pub.size()==4,"audit source coverage");
    for(auto& entry:local_pub) {
        auto a=entry.second,b=ext_pub.at(entry.first);std::sort(a.begin(),a.end());std::sort(b.begin(),b.end());
        require(a==b,"published field-score multiset");
    }
    for(const auto& id:unkeyed)tsv(source_checks,{id,"published_field_multiset_member","MATCH"});
    std::vector<Field> fields;std::vector<Count> joint{1};int total_pairs=0,observed=0,primary_rows=0;
    for(int bits:{192,224,239,256}) {
        std::vector<Row> pool;
        for(const auto& r:rows)if(r.bits==bits&&(r.role=="published"||r.category=="order_too_wide_only"||r.category=="opposite_root_only"))pool.push_back(r);
        int k=0;std::vector<int> scores;for(const auto& r:pool){k+=r.role=="published";scores.push_back(r.score);}
        int h=static_cast<int>(pool.size())-k;require(k>0&&h>0&&pool.size()<=24,"paired field inventory");
        Field f{bits,k,h,k*h,0,0,0,0,{}};
        for(const auto& p:pool)if(p.role=="published")for(const auto& u:pool)if(u.role!="published") {
            f.wins+=u.score>p.score;f.ties+=u.score==p.score;f.losses+=u.score<p.score;
        }
        f.observed=2*f.wins+f.ties;require(f.wins+f.ties+f.losses==f.pairs,"pair conservation");
        f.histogram=enumerate(scores,k);joint=combine(joint,f.histogram);
        total_pairs+=f.pairs;observed+=f.observed;primary_rows+=k+h;fields.push_back(std::move(f));
    }
    Count allocations=std::accumulate(joint.begin(),joint.end(),Count(0)),tail=0;
    require(total_pairs==102&&primary_rows==50&&allocations==20037600&&joint.size()==205,"fixed allocation inventory");
    for(size_t i=static_cast<size_t>(observed);i<joint.size();++i)tail+=joint[i];
    std::ostringstream counts,field_hist,distribution,summary;
    counts << "bits\tpublished\tadditional\tpairs\twins\tties\tlosses\tscore_twice\tallocations\n";
    field_hist << "bits\tscore_twice\tallocations\n";
    for(const auto& f:fields) {
        counts << f.bits << '\t' << f.k << '\t' << f.h << '\t' << f.pairs << '\t' << f.wins << '\t' << f.ties << '\t' << f.losses << '\t' << f.observed << '\t' << std::accumulate(f.histogram.begin(),f.histogram.end(),Count(0)) << '\n';
        for(size_t i=0;i<f.histogram.size();++i)field_hist << f.bits << '\t' << i << '\t' << f.histogram[i] << '\n';
    }
    distribution << "score_twice\tallocations\ttail_at_least\n";Count running=allocations;
    for(size_t i=0;i<joint.size();++i){distribution << i << '\t' << joint[i] << '\t' << running << '\n';running-=joint[i];}
    require(running==0,"tail conservation");
    summary << "{\"kind\":\"exact-existing-score-allocation-audit/v1\",\"input_seed_positions\":69,\"matched_external_rows\":68,\"seed_identified_matches\":60,\"published_field_multiset_rows\":8,\"primary_positions\":50,\"published\":8,\"additional\":42,\"pairs\":102,\"observed_score_twice\":" << observed << ",\"assignments\":" << allocations << ",\"tail_at_least_observed\":" << tail << ",\"observed_fraction_exact\":\"" << observed << "/204\",\"allocation_tail_exact\":\"" << tail << '/' << allocations << "\",\"external_procedural_deviations_repaired\":false,\"historical_use_established\":false,\"ECDLP_advantage_established\":false}\n";
    fs::path out=argv[3];require(!fs::exists(out),"fresh output");fs::create_directories(out);
    put(out/"source-checks.tsv",source_checks.str());put(out/"fields.tsv",counts.str());
    put(out/"field-distributions.tsv",field_hist.str());put(out/"distribution.tsv",distribution.str());put(out/"summary.json",summary.str());
    std::cout << "Exact allocation audit written; independent rank-DP verification pending\n";return 0;
}catch(const std::exception& e){std::cerr << "FAIL: " << e.what() << '\n';return 1;}
}
