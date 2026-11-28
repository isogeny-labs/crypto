// Native audit of imported scalar observations. No curve arithmetic is performed.
#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <map>
#include <numeric>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

using Row = std::vector<std::string>;
void require(bool b, const std::string& s) { if (!b) throw std::runtime_error(s); }
std::vector<Row> csv(const char* file) {
    std::ifstream in(file); require(bool(in), "cannot open input");
    std::vector<Row> out; std::string line;
    while (std::getline(in, line)) {
        if (!line.empty() && line.back() == '\r') line.pop_back();
        if (line.empty()) continue;
        Row row; std::stringstream ss(line); std::string value;
        while (std::getline(ss, value, ',')) row.push_back(value);
        out.push_back(row);
    }
    require(!out.empty(), "empty CSV"); return out;
}
size_t column(const Row& h, const std::string& s) {
    auto i = std::find(h.begin(), h.end(), s);
    require(i != h.end(), "missing column " + s); return size_t(i - h.begin());
}
struct Curve {
    int family = 0, candidate = 0; bool published = false;
    std::array<std::vector<int>, 16> metrics;
    std::array<bool, 16> seen{};
};

int main(int argc, char** argv) try {
    require(argc == 3, "usage: audit scores.csv cover-metrics.csv");
    auto scores = csv(argv[1]); const auto sh = scores.front();
    std::map<int, std::vector<int>> selected, skipped;
    size_t published = 0;
    for (size_t i = 1; i < scores.size(); ++i) {
        auto& r = scores[i]; int sum = 0;
        for (int d = 0; d < 8; ++d) sum += std::stoi(r.at(column(sh, "d" + std::to_string(d))));
        require(sum == std::stoi(r.at(column(sh, "S"))), "score sum mismatch");
        bool pub = r.at(column(sh, "status")) == "published";
        (pub ? selected : skipped)[std::stoi(r[0])].push_back(sum);
        published += pub;
    }
    require(scores.size() == 22 && published == 10, "wrong score population");
    for (int f : {192,224,239,256,384,521}) {
        auto& a = selected[f]; auto& b = skipped[f];
        require(!a.empty(), "missing published family");
        if (!b.empty()) require(*std::max_element(a.begin(),a.end()) < *std::min_element(b.begin(),b.end()), "within-stream score separation fails");
        std::cout << "family " << f << ": published=" << a.size() << " eligible_omissions=" << b.size()
                  << (b.empty() ? " no separation control\n" : " strict published-lower ranking\n");
    }
    auto data = csv(argv[2]); const auto h = data.front();
    require(h.size() == 37 && data.size() == 305, "wrong metric-table shape");
    std::map<std::pair<int,int>, Curve> by;
    for (size_t i = 1; i < data.size(); ++i) {
        const auto& r = data[i]; require(r.size() == h.size(), "ragged metric table");
        int f=std::stoi(r[0]), c=std::stoi(r[1]), d=std::stoi(r[3]); bool pub=std::stoi(r[2]) != 0;
        require(d>=0 && d<16, "cover parameter outside fixed grid");
        auto& v=by[{f,c}]; require(!v.seen[d], "duplicate cover");
        if (std::any_of(v.seen.begin(),v.seen.end(),[](bool x){return x;})) require(v.published==pub, "inconsistent labels");
        v.family=f; v.candidate=c; v.published=pub; v.seen[d]=true;
        for (size_t j=4;j<r.size();++j) v.metrics[d].push_back(std::stoi(r[j]));
        // Derive the two cyclic-subgroup metrics natively from the recorded ranks.
        for (auto side : {std::string("Prym"),std::string("whole_genus3_Jacobian")}) {
            int plus=std::stoi(r[column(h,side+"_rational_three_rank_extension_1")]);
            int both=std::stoi(r[column(h,side+"_rational_three_rank_extension_2")]);
            const int powers[]={1,3,9,27,81,243,729};
            require(plus>=0 && plus<=both && both<=6,"invalid eigenspace ranks");
            v.metrics[d].push_back((powers[plus]+powers[both-plus]-2)/2);
        }
        require(v.metrics[d].size()==35,"wrong effective metric count");
    }
    require(by.size()==19,"wrong controlled curve count");
    std::vector<std::string> feature_names(h.begin()+4,h.end());
    feature_names.push_back("Prym_rational_cyclic_three_subgroup_count");
    feature_names.push_back("whole_genus3_Jacobian_rational_cyclic_three_subgroup_count");
    std::vector<Curve> curves; std::map<int,std::vector<int>> families;
    unsigned actual=0; int idx=0;
    for (auto& item:by) {
        auto& c=item.second; require(std::all_of(c.seen.begin(),c.seen.end(),[](bool x){return x;}),"incomplete cover grid");
        families[c.family].push_back(idx); if(c.published) actual |= 1u<<idx;
        curves.push_back(c); ++idx;
    }
    std::set<unsigned> attainable; int tests=0,hits=0,global_hits=0;
    for(int feature=0;feature<35;++feature) for(int lo=0;lo<16;++lo) for(int hi=lo;hi<16;++hi)
    for(int aggregation=0;aggregation<3;++aggregation) for(int direction=0;direction<2;++direction) {
        ++tests; std::vector<int> value(19);
        for(int i=0;i<19;++i) {
            int v=curves[i].metrics[lo][feature];
            for(int d=lo+1;d<=hi;++d) {
                int a=curves[i].metrics[d][feature];
                v=aggregation==0?v+a:aggregation==1?std::min(v,a):std::max(v,a);
            }
            value[i]=v;
        }
        unsigned mask=0; bool valid=true;
        for(auto& family:families) {
            auto order=family.second; size_t k=0;
            for(int i:order) k+=curves[i].published;
            std::sort(order.begin(),order.end(),[&](int a,int b){return direction==0?value[a]<value[b]:value[a]>value[b];});
            require(k>0 && k<order.size(),"uncontrolled family in ranking search");
            if(value[order[k-1]]==value[order[k]]) { valid=false; break; }
            for(size_t i=0;i<k;++i) mask |= 1u<<order[i];
        }
        if(!valid) continue;
        attainable.insert(mask);
        if(mask!=actual) continue;
        ++hits;
        int maxpub=-1,minpub=1000000000,maxskip=-1,minskip=1000000000;
        for(int i=0;i<19;++i) if(curves[i].published) {maxpub=std::max(maxpub,value[i]);minpub=std::min(minpub,value[i]);}
        else {maxskip=std::max(maxskip,value[i]);minskip=std::min(minskip,value[i]);}
        global_hits += direction==0 ? maxpub<minskip : minpub>maxskip;
        std::cout << "hit feature=" << feature_names.at(feature) << " d=" << lo << ".." << hi
                  << " aggregation=" << aggregation << " direction=" << direction << '\n';
    }
    require(tests==28560 && attainable.size()==684 && hits==1 && global_hits==0,"search replay differs from imported summary");
    std::cout << "rows=304 effective_metrics=35 rules=" << tests << " separators=" << hits
              << " global_cutoffs=" << global_hits << " attainable_labelings=" << attainable.size() << "/4480\n";
    std::cout << "Observation-table audit passed; curve arithmetic and PCC standing remain unverified.\n";
    return 0;
} catch(const std::exception& e) { std::cerr << "audit error: " << e.what() << '\n'; return 1; }
