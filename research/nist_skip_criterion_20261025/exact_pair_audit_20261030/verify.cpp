#include "io.hpp"
#include <algorithm>
#include <cstdint>
#include <iostream>
#include <map>
#include <numeric>
#include <set>
using Count=uint64_t;
struct Value { std::vector<std::string> text;int bits,score;bool published; };
struct Result { int bits,k,h,pairs,wins,ties,losses,score;std::vector<Count> frequency; };
static std::vector<int> ranks(const std::vector<int>& values) {
    auto sorted=values;std::sort(sorted.begin(),sorted.end());std::vector<int> out;
    for(int v:values) {
        auto first=std::lower_bound(sorted.begin(),sorted.end(),v),last=std::upper_bound(sorted.begin(),sorted.end(),v);
        out.push_back(static_cast<int>(2*(first-sorted.begin())+(last-first)+1));
    }
    return out;
}
static std::vector<Count> dynamic(const std::vector<int>& values,int k) {
    int n=static_cast<int>(values.size()),limit=2*n*k;
    auto weights=ranks(values);std::vector<std::vector<Count>> dp(static_cast<size_t>(k+1),std::vector<Count>(static_cast<size_t>(limit+1)));
    dp[0][0]=1;
    for(int w:weights)for(int j=k;j>=1;--j)for(int sum=limit;sum>=w;--sum)
        dp[static_cast<size_t>(j)][static_cast<size_t>(sum)]+=dp[static_cast<size_t>(j-1)][static_cast<size_t>(sum-w)];
    std::vector<Count> histogram(static_cast<size_t>(2*k*(n-k)+1));
    for(int sum=0;sum<=limit;++sum)if(dp[static_cast<size_t>(k)][static_cast<size_t>(sum)]) {
        int score=2*k*(n-k)+k*(k+1)-sum;
        require(score>=0&&static_cast<size_t>(score)<histogram.size(),"rank identity range");
        histogram[static_cast<size_t>(score)]+=dp[static_cast<size_t>(k)][static_cast<size_t>(sum)];
    }
    return histogram;
}
static std::vector<Count> multiply(const std::vector<Count>& x,const std::vector<Count>& y) {
    std::vector<Count> out(x.size()+y.size()-1);
    for(size_t degree=0;degree<out.size();++degree)
        for(size_t i=0;i<x.size()&&i<=degree;++i)if(degree-i<y.size())out[degree]+=x[i]*y[degree-i];
    return out;
}
int main(int argc,char** argv) {
try {
    if(argc==2&&std::string(argv[1])=="--self-test") {
        auto a=dynamic({1,1,2},1);require(a==std::vector<Count>({1,0,0,2,0}),"known tied rank-DP");
        require(dynamic({1,2,3},1)==std::vector<Count>({1,0,1,0,1}),"known ordered rank-DP");
        auto square=multiply(a,a);require(square[0]==1&&square[3]==4&&square[6]==4&&std::accumulate(square.begin(),square.end(),Count(0))==9,"known convolution");
        std::cout << "PASS independent doubled-rank DP and explicit toy coefficients\n";return 0;
    }
    require(argc==5,"verify BRANCHES EXTERNAL RESULTS NEW_RECEIPT");
    auto data=table(argv[1],14),source=table(argv[2],6);require(data.size()==83&&source.size()==68,"source inventories");
    std::map<std::string,Value> seeds;std::vector<Value> ordered;
    for(size_t i=1;i<data.size();++i)if(data[i][6]=="1") {
        const auto& c=data[i];Value v{c,std::stoi(c[2]),std::stoi(c[7]),c[3]=="published"};
        require(v.score>=0&&v.score<=124&&seeds.emplace(c[1],v).second,"unique bounded seed scores");ordered.push_back(v);
    }
    require(seeds.size()==69,"69positions");
    std::ostringstream checks;checks << "source_id\tscope\tstatus\n";
    std::map<int,std::multiset<int>> local_pub,reported_pub;std::vector<std::string> unkeyed;int matched=0;
    for(const auto& s:source) {
        if(s[1]=="NA_no_seed_id_in_named_row") {
            require(s[3]=="published","unkeyed role");reported_pub[std::stoi(s[2])].insert(std::stoi(s[5]));unkeyed.push_back(s[0]);
        } else {
            auto found=seeds.find(s[1]);require(found!=seeds.end(),"external seed absent");const auto& a=found->second;
            require(a.bits==std::stoi(s[2])&&a.text[3]==s[3]&&a.text[4]==s[4]&&a.score==std::stoi(s[5]),"external score mismatch");
            ++matched;tsv(checks,{s[0],"seed_score","MATCH"});
        }
    }
    for(const auto& s:ordered)if(s.published&&s.bits<=256)local_pub[s.bits].insert(s.score);
    require(matched==60&&unkeyed.size()==8&&local_pub.size()==4&&reported_pub==local_pub,"source observations/multisets");
    for(const auto& s:unkeyed)tsv(checks,{s,"published_field_multiset_member","MATCH"});
    std::vector<Result> fields;std::vector<Count> combined{1};int pairs=0,observed=0,total=0;
    for(int bits:{192,224,239,256}) {
        std::vector<Value> pool;
        for(const auto& s:ordered)if(s.bits==bits&&(s.published||s.text[4]=="order_too_wide_only"||s.text[4]=="opposite_root_only"))pool.push_back(s);
        std::vector<int> scores;int k=0;
        for(const auto& s:pool){scores.push_back(s.score);k+=s.published;}
        int h=static_cast<int>(pool.size())-k;require(k>0&&h>0&&pool.size()<=24,"field inventory");
        auto rank=ranks(scores);int sum=0,tied=0;
        std::map<int,std::pair<int,int>> multiplicities;
        for(size_t i=0;i<pool.size();++i) {
            auto& q=multiplicities[pool[i].score];
            if(pool[i].published){sum+=rank[i];++q.first;}else ++q.second;
        }
        for(const auto& q:multiplicities)tied+=q.second.first*q.second.second;
        int score=2*k*h+k*(k+1)-sum;
        require(score>=tied&&(score-tied)%2==0,"rank/tie parity");
        int wins=(score-tied)/2,losses=k*h-wins-tied;require(losses>=0,"pair accounting");
        auto f=dynamic(scores,k);combined=multiply(combined,f);
        fields.push_back({bits,k,h,k*h,wins,tied,losses,score,std::move(f)});
        pairs+=k*h;observed+=score;total+=k+h;
    }
    Count assignments=0;for(Count n:combined)assignments+=n;
    require(pairs==102&&total==50&&assignments==20037600&&combined.size()==205,"allocation census");
    std::vector<Count> upper(combined.size());Count tail=0;
    for(size_t i=combined.size();i>0;--i){tail+=combined[i-1];upper[i-1]=tail;}
    Count exceed=upper.at(static_cast<size_t>(observed));
    std::ostringstream counts,hist,distribution,summary;
    counts << "bits\tpublished\tadditional\tpairs\twins\tties\tlosses\tscore_twice\tallocations\n";
    hist << "bits\tscore_twice\tallocations\n";
    for(const auto& f:fields) {
        Count n=0;for(Count c:f.frequency)n+=c;
        counts << f.bits << '\t' << f.k << '\t' << f.h << '\t' << f.pairs << '\t' << f.wins << '\t' << f.ties << '\t' << f.losses << '\t' << f.score << '\t' << n << '\n';
        for(size_t i=0;i<f.frequency.size();++i)hist << f.bits << '\t' << i << '\t' << f.frequency[i] << '\n';
    }
    distribution << "score_twice\tallocations\ttail_at_least\n";
    for(size_t i=0;i<combined.size();++i)distribution << i << '\t' << combined[i] << '\t' << upper[i] << '\n';
    summary << "{\"kind\":\"exact-existing-score-allocation-audit/v1\",\"input_seed_positions\":69,\"matched_external_rows\":68,\"seed_identified_matches\":60,\"published_field_multiset_rows\":8,\"primary_positions\":50,\"published\":8,\"additional\":42,\"pairs\":102,\"observed_score_twice\":" << observed << ",\"assignments\":" << assignments << ",\"tail_at_least_observed\":" << exceed << ",\"observed_fraction_exact\":\"" << observed << "/204\",\"allocation_tail_exact\":\"" << exceed << '/' << assignments << "\",\"external_procedural_deviations_repaired\":false,\"historical_use_established\":false,\"ECDLP_advantage_established\":false}\n";
    fs::path outputs=argv[3];
    for(const auto& entry:std::vector<std::pair<std::string,std::string>>{{"source-checks.tsv",checks.str()},{"fields.tsv",counts.str()},{"field-distributions.tsv",hist.str()},{"distribution.tsv",distribution.str()},{"summary.json",summary.str()}})
        require(read_file(outputs/entry.first)==entry.second,"independent mismatch: "+entry.first);
    require(!fs::exists(argv[4]),"fresh verifier receipt");
    std::ostringstream receipt;receipt << "{\"kind\":\"independent-rank-dp-allocation-verification/v1\",\"all_five_files_equal\":true,\"assignments\":" << assignments << ",\"matched_external_rows\":68,\"input_seed_positions\":69,\"method\":\"doubled-midranks and chosen-rank-sum dynamic programming\",\"shared\":\"same compiler and TSV I/O; no shared statistic or null arithmetic\",\"new_curve_measurements\":0,\"historical_use_established\":false}\n";
    put(argv[4],receipt.str());std::cout << "PASS independent rank-DP distribution, tails and source observations\n";return 0;
}catch(const std::exception& e){std::cerr << "FAIL: " << e.what() << '\n';return 1;}
}
