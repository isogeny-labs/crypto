// Post-run descriptive analysis and public-group replay; no timing is collected.
#include <bits/stdc++.h>
#include <nlohmann/json.hpp>
namespace reference {
bool control_check_only=true;
#include "../runpod/pair_control_callable.cpp"
}
using J=nlohmann::json;
using U=std::uint64_t;
struct Unit {
    unsigned B=0,round=0,attempts=0;std::string phase,fixture,arm,last;
    double cpu=0,wall=0,rss=0;bool failed=false,is_rho=false;
    std::map<std::string,double> phase_cpu,phase_wall;
};
using Key=std::tuple<unsigned,std::string,std::string>;
void need(bool b,const std::string& s){if(!b)throw std::runtime_error(s);}
double median(std::vector<double> a){need(!a.empty(),"empty median");std::sort(a.begin(),a.end());return a.size()%2?a[a.size()/2]:(a[a.size()/2-1]+a[a.size()/2])*0.5;}
J dist(const std::vector<double>& a){if(a.empty())return nullptr;return {{"n",a.size()},{"median",median(a)},{"mean",std::accumulate(a.begin(),a.end(),0.0)/a.size()},{"min",*std::min_element(a.begin(),a.end())},{"max",*std::max_element(a.begin(),a.end())}};}
bool completed(const Unit& u){return !u.failed&&(u.last=="relation"||(u.is_rho&&u.last=="solved"));}
int main(int argc,char** argv){try{
    need(argc==3,"usage: postprocess RUN_DIRECTORY FROZEN_STUDY");const std::filesystem::path run=argv[1],study=argv[2];
    std::map<std::string,J> fixtures;
    for(const auto& entry:std::filesystem::directory_iterator(study/"historical/prime_fixtures")){
        const auto name=entry.path().filename().string();
        if(name.find("_natural_s")==std::string::npos||name.find(".validation.")!=std::string::npos||entry.path().extension()!=".json")continue;
        std::ifstream f(entry.path());J j;f>>j;fixtures[name]=j;
    }
    need(fixtures.size()==60,"expected sixty public fixtures");
    std::map<U,double> process_rss;std::ifstream resources(run/"process-times.jsonl");
    while(true){resources>>std::ws;if(resources.peek()==std::char_traits<char>::eof())break;J j;resources>>j;need(process_rss.emplace(j.at("row_index"),j.at("peak_rss_kib")).second,"duplicate process resource row");}
    std::map<Key,Unit> units;std::map<std::tuple<unsigned,std::string,unsigned>,J> generations,supports;
    std::map<std::tuple<unsigned,std::string,unsigned>,bool> truth;
    std::map<std::string,U> scalars;
    std::set<std::string> arms={"known","pairs","straus-pairs","window-pairs","window-horner","compose","support","support-bloom","first","first-bloom","first-bloom-aa","rho"};
    unsigned rows=0,scalar_replays=0,support_checks=0,generation_checks=0,existence_checks=0,failed_rows=0;
    std::ifstream raw(run/"rows.jsonl");need(bool(raw),"missing rows");
    while(true){raw>>std::ws;if(raw.peek()==std::char_traits<char>::eof())break;J j;raw>>j;++rows;
        const std::string fixture=j.at("source_fixture"),arm=j.at("arm_label"),phase=j.at("phase"),status=j.at("status");
        const unsigned round=j.at("round"),attempt=j.at("attempt");
        need(fixtures.count(fixture)&&arms.count(arm)&&round<=5&&phase==(round==0?"warmup":"measured"),"unexpected panel identity");
        need(j.at("row_index")==rows,"raw row sequence changed");
        const auto& input=fixtures.at(fixture);const unsigned B=input.at("B");
        auto& u=units[{round,fixture,arm}];u.B=B;u.round=round;u.fixture=fixture;u.arm=arm;u.phase=phase;u.is_rho=arm=="rho";
        need(attempt==u.attempts&&!completed(u)&&!u.failed,"invalid target attempt sequence");
        need(attempt<(u.is_rho?1u:4u),"attempt bound exceeded");++u.attempts;u.last=status;
        if(status=="failed"){u.failed=true;++failed_rows;continue;}
        need(j.at("wrapper_exit")==0,"nonzero wrapper reported successful row");
        need(process_rss.count(rows),"missing process resource record");u.rss=std::max(u.rss,process_rss.at(rows));
        if(j.contains("recovered_scalar")&&!j.at("recovered_scalar").is_null()){
            const U k=j.at("recovered_scalar");const auto& c=input.at("curve");const U p=c.at("p"),a=c.at("a"),r=c.at("r");
            need(k<r,"noncanonical recovered scalar");reference::Field f{p};
            need(reference::equal(reference::scalar(f,a,k,{c.at("P")[0],c.at("P")[1],false}),{c.at("Q")[0],c.at("Q")[1],false}),"independent public group replay failed");++scalar_replays;
            if(scalars.count(fixture))need(scalars.at(fixture)==k,"cross-mechanism scalar mismatch");else scalars[fixture]=k;
        }
        if(u.is_rho){need(status=="solved"||status=="censored","invalid rho status");if(status=="solved")need(j.at("public_scalar_check")==true,"rho public check absent");u.wall+=j.at("complete_adapter_wall_seconds").get<double>();continue;}
        need(status=="relation"||status=="none","invalid incidence status");need(j.at("timings_collected")==true,"untimed row in timed matrix");
        u.cpu+=j.at("complete").at("cpu_seconds").get<double>();u.wall+=j.at("complete").at("wall_seconds").get<double>();
        for(auto it=j.at("phases").begin();it!=j.at("phases").end();++it){u.phase_cpu[it.key()]+=it.value().at("cpu_seconds").get<double>();u.phase_wall[it.key()]+=it.value().at("wall_seconds").get<double>();}
        const auto key=std::make_tuple(round,fixture,attempt);
        if(generations.count(key)){need(generations.at(key)==j.at("generation"),"generation/rejection mismatch");++generation_checks;}else generations[key]=j.at("generation");
        if(truth.count(key)){need(truth.at(key)==(status=="relation"),"relation existence mismatch");++existence_checks;}else truth[key]=status=="relation";
        if(j.value("full_support",false)){
            const auto& g=j.at("gcd_coefficients");need(!g.empty()&&g.back()==1,"nonmonic support");
            if(supports.count(key)){need(supports.at(key)==g,"full support mismatch");++support_checks;}else supports[key]=g;
        }
    }
    need(units.size()==6*60*arms.size(),"incomplete target/round/arm matrix");
    for(unsigned round=0;round<=5;++round)for(const auto& fixture:fixtures)for(const auto& arm:arms)need(units.count({round,fixture.first,arm}),"missing planned target unit");
    J output={{"schema","trace-local-postcheck/v1"},{"standing","unverified"},{"raw_rows",rows},{"target_round_arm_units",units.size()},{"expected_units",6*60*arms.size()},{"public_scalar_replays",scalar_replays},{"full_support_checks",support_checks},{"generation_accounting_checks",generation_checks},{"attempt_existence_checks",existence_checks},{"failed_rows",failed_rows},{"interpretation","Descriptive local timings; repeated rounds are not independent targets. Bounded work includes all attempted pools and censored units. Completed-only paired comparisons condition on both targets completing. Rho CPU/phases are unavailable; process overhead is separate."}};
    std::map<std::pair<unsigned,std::string>,std::vector<const Unit*>> groups;
    for(const auto& pair:units)if(pair.second.phase=="measured")groups[{pair.second.B,pair.second.arm}].push_back(&pair.second);
    for(const auto& group:groups){std::vector<double> cpu,wall,rss;std::map<std::string,std::vector<double>> phases_cpu,phases_wall;unsigned solved=0,censored=0,failed=0,incomplete=0,attempts=0;
        for(const auto* u:group.second){attempts+=u->attempts;if(completed(*u))++solved;else if(u->failed)++failed;else if(u->is_rho||u->attempts==4)++censored;else ++incomplete;
            if(!u->failed){wall.push_back(u->wall);rss.push_back(u->rss);if(!u->is_rho)cpu.push_back(u->cpu);for(const auto& p:u->phase_cpu)phases_cpu[p.first].push_back(p.second);for(const auto& p:u->phase_wall)phases_wall[p.first].push_back(p.second);}}
        J row={{"B",group.first.first},{"arm",group.first.second},{"units",group.second.size()},{"completed",solved},{"censored",censored},{"failed",failed},{"incomplete",incomplete},{"attempts_charged",attempts},{"cpu_seconds",dist(cpu)},{"wall_seconds",dist(wall)},{"peak_rss_kib",dist(rss)}};
        for(const auto& p:phases_cpu)row["phase_cpu_seconds"][p.first]=dist(p.second);
        for(const auto& p:phases_wall)row["phase_wall_seconds"][p.first]=dist(p.second);
        output["table"].push_back(row);
    }
    const std::vector<std::pair<std::string,std::string>> comparisons={{"known","compose"},{"window-horner","compose"},{"pairs","straus-pairs"},{"straus-pairs","window-pairs"},{"pairs","window-pairs"},{"pairs","first"},{"pairs","first-bloom"},{"compose","support"},{"support","support-bloom"},{"first","first-bloom"},{"first-bloom","first-bloom-aa"},{"first","rho"},{"first-bloom","rho"}};
    for(const auto& comparison:comparisons)for(const unsigned B:{8,16,32,64,128}){std::vector<double> cpu,wall;
        for(const auto& pair:units){const auto& a=pair.second;if(a.phase!="measured"||a.B!=B||a.arm!=comparison.first||!completed(a))continue;const auto it=units.find({a.round,a.fixture,comparison.second});if(it==units.end()||!completed(it->second))continue;const auto& b=it->second;if(a.wall>0&&b.wall>0)wall.push_back(a.wall/b.wall);if(!a.is_rho&&!b.is_rho&&a.cpu>0&&b.cpu>0)cpu.push_back(a.cpu/b.cpu);}
        output["paired_completed_only"][comparison.first+"/"+comparison.second][std::to_string(B)]={{"cpu_ratio",dist(cpu)},{"wall_ratio",dist(wall)}};
    }
    std::cout<<output.dump(2)<<'\n';return 0;
}catch(const std::exception& e){std::cerr<<e.what()<<'\n';return 1;}}
