#include <nlohmann/json.hpp>
#include <algorithm>
#include <fstream>
#include <iostream>
#include <map>
#include <stdexcept>
#include <string>
#include <vector>
using J=nlohmann::json;
J stats(std::vector<double> v){
    if(v.empty()) throw std::runtime_error("empty group");
    std::sort(v.begin(),v.end());
    const auto n=v.size();double sum=0;for(double x:v)sum+=x;
    return {{"n",n},{"median",n%2?v[n/2]:(v[n/2-1]+v[n/2])/2},{"min",v.front()},{"max",v.back()},{"mean",sum/n}};
}
struct Values{std::vector<double> wall,cpu,rss;};
int main(int argc,char**argv){try{
    if(argc!=3) throw std::runtime_error("analyze RAW_JSONL SUMMARY");
    std::ifstream in(argv[1]);std::string line;unsigned raw=0,warmup=0;
    std::map<std::string,Values> groups;
    std::map<std::string,std::map<std::string,J>> paired;
    while(std::getline(in,line)){
        auto outer=J::parse(line);auto r=outer.at("row");++raw;
        if(outer.at("exit")!=0||r.at("status")!="passed")throw std::runtime_error("failed row");
        if(outer.at("round").get<int>()<0){++warmup;continue;}
        const std::string arm=outer.at("arm"),experiment=r.at("experiment");
        const bool root=experiment=="paper-root";
        const double repetitions=root?r.at("repetitions").get<double>():1;
        const auto t=r.at(root?"batch":"complete");
        const double wall=t.at("wall_seconds").get<double>()/repetitions;
        const double cpu=t.at("cpu_seconds").get<double>()/repetitions;
        const auto degree=root?10:r.at("degree").get<int>();
        const std::string kind=root?(arm.find("isogeny")!=std::string::npos?"isogeny":"given"):r.at("case").get<std::string>();
        for(const auto& k:std::vector<std::string>{experiment+"/"+std::to_string(degree)+"/"+kind+"/"+arm,experiment+"/"+std::to_string(degree)+"/all/"+arm}){
            auto& g=groups[k];g.wall.push_back(wall);g.cpu.push_back(cpu);g.rss.push_back(r.at("peak_rss_kib").get<double>());
        }
        const std::string fixture=root?kind:r.at("fixture_id").get<std::string>();
        const std::string key=experiment+"/"+fixture+"/"+std::to_string(outer.at("round").get<int>());
        if(paired[key].count(arm))throw std::runtime_error("duplicate unit");
        paired[key][arm]={{"wall",wall},{"cpu",cpu},{"degree",degree},{"kind",kind},{"experiment",experiment}};
    }
    if(raw!=1512||warmup!=252||paired.size()!=250)throw std::runtime_error("incomplete planned matrix: "+std::to_string(raw)+" "+std::to_string(warmup)+" "+std::to_string(paired.size()));
    J out={{"schema","root-recurrence-summary/v1"},{"raw_rows",raw},{"warmup_rows",warmup},{"measured_rows",raw-warmup},{"standing","unverified"}};
    for(auto&[key,g]:groups)out["groups"][key]={{"wall_seconds",stats(g.wall)},{"cpu_seconds",stats(g.cpu)},{"peak_rss_kib",stats(g.rss)}};
    std::map<std::string,Values> ratios;
    for(const auto&[key,arms]:paired){(void)key;
        for(const auto& a:std::vector<std::string>{"0:dense","0:trace","1:trace","2:trace","0:flint","0:dense-isogeny","0:trace-isogeny","3:trace","4:trace","5:trace","6:trace","7:trace"})
        for(const auto& b:std::vector<std::string>{"0:pullback","0:flint","1:trace","2:trace","3:trace","1:trace-isogeny","2:trace-isogeny","3:trace-isogeny","4:trace","5:trace","4:trace-isogeny","5:trace-isogeny","6:trace","7:trace","6:trace-isogeny","7:trace-isogeny"}){
            if(a==b||!arms.count(a)||!arms.count(b))continue;
            const auto&x=arms.at(a);const auto&y=arms.at(b);
            const std::string prefix=x.at("experiment").get<std::string>()+"/"+std::to_string(x.at("degree").get<int>());
            for(const auto& kind:std::vector<std::string>{x.at("kind").get<std::string>(),"all"}){
                auto& g=ratios[prefix+"/"+kind+"/"+a+"_over_"+b];
                g.wall.push_back(x.at("wall").get<double>()/y.at("wall").get<double>());
                g.cpu.push_back(x.at("cpu").get<double>()/y.at("cpu").get<double>());
            }
        }
    }
    for(auto&[k,g]:ratios)out["paired_ratios"][k]={{"wall",stats(g.wall)},{"cpu",stats(g.cpu)}};
    for(int degree: {8,16,32,64,128,256}) {
        const std::string prefix="translation/"+std::to_string(degree)+"/all/";
        std::vector<std::string> present;
        for(const auto& arm:std::vector<std::string>{"0:dense","0:trace","1:trace","2:trace","3:trace","4:trace","5:trace","6:trace","7:trace"})
            if(out["groups"].contains(prefix+arm))present.push_back(arm);
        for(const auto& arm:present){
            const auto& a=out["groups"][prefix+arm];bool dominated=false;
            const double aw=a["wall_seconds"]["median"],am=a["peak_rss_kib"]["median"];
            for(const auto& other:present){const auto&b=out["groups"][prefix+other];
                const double bw=b["wall_seconds"]["median"],bm=b["peak_rss_kib"]["median"];
                if(bw<=aw&&bm<=am&&(bw<aw||bm<am))dominated=true;
            }
            if(!dominated)out["observed_time_rss_frontier"][std::to_string(degree)].push_back(arm);
        }
    }
    std::ofstream file(argv[2]);file<<out.dump(2)<<'\n';
    std::cout<<J({{"raw",raw},{"warmup",warmup},{"measured",raw-warmup},{"paired_units",paired.size()},{"checks","passed"}}).dump()<<'\n';
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
