// Exact finite feature-rule search and within-stream label calibration.
// Inputs are the native extractor's integer feature tables, not Python results.
#include <algorithm>
#include <array>
#include <cstdint>
#include <fstream>
#include <functional>
#include <iostream>
#include <map>
#include <numeric>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
using Mask=uint32_t;
static int count(Mask m){return __builtin_popcount(m);}
static void need(bool ok,const std::string& s){if(!ok)throw std::runtime_error(s);}
struct Predicate {Mask mask;std::string definition;int cost;};
struct Cover {int size=4;std::vector<int> witnesses;};
static Cover cover(Mask goal,const std::vector<Predicate>& predicates,Mask all,bool conjunction){
    std::vector<int> positions;for(int i=0;i<21;++i)if(goal&(Mask(1)<<i))positions.push_back(i);
    const int length=1<<positions.size(),full=length-1;
    std::vector<int> exact(length,-1),superset(length,-1),compressed(predicates.size());
    for(size_t i=0;i<predicates.size();++i){
        Mask bits=conjunction?(all^predicates[i].mask):predicates[i].mask;
        if(!bits||(bits&~goal))continue;
        int small=0;for(size_t j=0;j<positions.size();++j)if(bits&(Mask(1)<<positions[j]))small|=1<<j;
        compressed[i]=small;
        if(exact[small]<0||predicates[i].cost<predicates[exact[small]].cost)exact[small]=int(i);
    }
    superset=exact;
    for(size_t bit=0;bit<positions.size();++bit)for(int m=0;m<length;++m)if(!(m&(1<<bit))){
        int other=superset[m|(1<<bit)],mine=superset[m];
        if(other>=0&&(mine<0||count(compressed[other])>count(compressed[mine])||
          (count(compressed[other])==count(compressed[mine])&&predicates[other].cost<predicates[mine].cost)))superset[m]=other;
    }
    if(superset[full]>=0)return{1,{superset[full]}};
    std::vector<int> maximal;
    for(int m=1;m<length;++m)if(exact[m]>=0&&compressed[superset[m]]==m)maximal.push_back(exact[m]);
    std::sort(maximal.begin(),maximal.end(),[&](int a,int b){return predicates[a].cost<predicates[b].cost;});
    for(int a:maximal){int b=superset[full&~compressed[a]];if(b>=0)return{2,{a,b}};}
    for(size_t i=0;i<maximal.size();++i)for(size_t j=i+1;j<maximal.size();++j){
        int a=maximal[i],b=maximal[j],c=superset[full&~(compressed[a]|compressed[b])];
        if(c>=0)return{3,{a,b,c}};
    }
    return{};
}
static bool primary(const std::string& key){
    for(const auto& prefix:{"point_","secondary_","byte_","mr_model:rng_offset","b:","b-","b+","c:","c-","c+","seed","b_bit","c_bit"})
        if(key.rfind(prefix,0)==0)return false;
    return true;
}
static int cost(const std::string& key){
    if(key.rfind("secondary_",0)==0)return 100;
    if(key.rfind("point_",0)==0)return 20;
    if(key.rfind("division_",0)==0)return 12;
    if(key.find("mod")!=std::string::npos||key.find("chi")!=std::string::npos||key.find(":v")!=std::string::npos)return 1;
    if(key.find("hamming")!=std::string::npos||key.find("bits")!=std::string::npos)return 5;
    return 7;
}
static std::vector<Predicate> predicates_for(const std::vector<std::map<std::string,long>>& rows,int mode){
    const Mask all=(Mask(1)<<rows.size())-1;std::map<Mask,Predicate> unique;
    auto add=[&](Mask m,const std::string& description,int weight){
        if(!m||m==all)return;
        auto it=unique.find(m);if(it==unique.end()||weight<it->second.cost||
          (weight==it->second.cost&&description<it->second.definition))unique[m]={m,description,weight};
    };
    for(const auto& [key,_]:rows[0]){
        if(mode==0&&!primary(key))continue;
        if(mode<2&&key.rfind("secondary_",0)==0)continue;
        std::set<long> values;for(const auto& row:rows)values.insert(row.at(key));
        for(long value:values){
            Mask equal=0,less=0;
            for(size_t i=0;i<rows.size();++i){if(rows[i].at(key)==value)equal|=Mask(1)<<i;
                if(rows[i].at(key)<=value)less|=Mask(1)<<i;}
            std::string number=std::to_string(value);int weight=cost(key);
            add(equal,key+" == "+number,weight);add(all^equal,key+" != "+number,weight);
            add(less,key+" <= "+number,weight+1);add(all^less,key+" > "+number,weight+1);
        }
    }
    std::vector<Predicate> out;for(const auto& [_,p]:unique)out.push_back(p);return out;
}
static int best_single(Mask labels,const std::vector<Predicate>& p,int* witness=nullptr){
    int best=100;for(size_t i=0;i<p.size();++i){int error=count(labels^p[i].mask);
        if(error<best){best=error;if(witness)*witness=int(i);}}
    return best;
}
static std::vector<Mask> assignments(const std::vector<int>& groups,Mask actual){
    std::map<int,std::vector<int>> group_rows;for(size_t i=0;i<groups.size();++i)group_rows[groups[i]].push_back(i);
    std::vector<Mask> out{0};
    for(const auto& [_,positions]:group_rows){
        int selected=0;for(int i:positions)if(actual&(Mask(1)<<i))++selected;
        std::vector<Mask> pieces;
        std::function<void(size_t,int,Mask)> choose=[&](size_t at,int left,Mask mask){
            if(!left){pieces.push_back(mask);return;}if(positions.size()-at<size_t(left))return;
            choose(at+1,left-1,mask|(Mask(1)<<positions[at]));choose(at+1,left,mask);
        };choose(0,selected,0);
        std::vector<Mask> next;for(Mask a:out)for(Mask b:pieces)next.push_back(a|b);out.swap(next);
    }
    return out;
}
static void print_rule(std::ostream& out,const Cover& c,const std::vector<Predicate>& p,bool cnf){
    out<<"{\"clauses\":"<<(c.size<=3?std::to_string(c.size):"null")<<",\"operator\":\""<<(cnf?"AND":"OR")<<"\",\"predicates\":[";
    for(size_t i=0;i<c.witnesses.size();++i){if(i)out<<',';out<<'"'<<p[c.witnesses[i]].definition<<'"';}out<<"]}";
}
static void selftest(){
    Mask all=31,published=7,skipped=all^published;
    std::vector<Predicate> exact={{published,"planted",0}};
    need(cover(skipped,exact,all,true).size==1,"planted one-clause CNF not found");
    need(cover(published,exact,all,false).size==1,"planted one-clause DNF not found");
    std::vector<Predicate> pair={{15,"left",0},{23,"right",0}};
    auto c=cover(skipped,pair,all,true);need(c.size==2,"planted two-clause CNF not found");
    Mask recovered=all;for(int i:c.witnesses)recovered&=pair[i].mask;
    need(recovered==published,"recovered CNF does not match labels");
    std::vector<Predicate> constant={{0,"none",0},{all,"all",0}};
    need(cover(skipped,constant,all,true).size>3,"constant negative control unexpectedly separates");
    auto a=assignments({0,0,0,1,1},(Mask(1)<<0)|(Mask(1)<<3));
    need(a.size()==6,"grouped label enumeration wrong");
    std::cout<<"PASS: planted one- and two-clause rules, constant-feature negative control, grouped label enumeration\n";
}
int main(int argc,char** argv){
  try{
    if(argc==2&&std::string(argv[1])=="--self-test"){selftest();return 0;}
    need(argc==4,"usage: search inputs.tsv feature_directory output_directory");
    std::ifstream input(argv[1]);std::string line;std::vector<std::string> labels;std::vector<int> groups;
    Mask actual=0;std::vector<std::map<std::string,long>> rows;
    while(std::getline(input,line)){
        if(line.empty()||line[0]=='#')continue;
        std::vector<std::string> fields;
        std::istringstream stream(line);std::string field;while(std::getline(stream,field,'\t'))fields.push_back(field);
        need(fields.size()==9,"input row schema mismatch");int i=rows.size();labels.push_back(fields[0]);groups.push_back(std::stoi(fields[1]));
        if(fields[2]=="published")actual|=Mask(1)<<i;
        std::ifstream f(std::string(argv[2])+"/curve-"+std::to_string(i)+".features.tsv");need(bool(f),"missing feature table");
        std::map<std::string,long> data;std::string key;long value;
        while(f>>key>>value)need(data.emplace(key,value).second,"duplicate feature key");
        rows.push_back(std::move(data));
    }
    need((rows.size()==21&&count(actual)==10)||(rows.size()==19&&count(actual)==8),"expected 21/10 or 19/8 cohort");
    for(const auto& row:rows){need(row.size()==rows[0].size(),"feature count mismatch");for(const auto& [key,_]:rows[0])need(row.count(key),"feature keys mismatch");}
    const Mask all=(Mask(1)<<rows.size())-1;auto nulls=assignments(groups,actual);need(nulls.size()==4480,"expected 4480 fixed-count assignments");
    for(int mode=0;mode<3;++mode){
        std::string family=mode==0?"structural":mode==1?"generation":"all_with_seed_bits";
        auto predicates=predicates_for(rows,mode);int witness=-1,error=best_single(actual,predicates,&witness);
        Cover cnf=cover(all^actual,predicates,all,true),dnf=cover(actual,predicates,all,false);
        int observed=std::min(cnf.size,dnf.size),single_as_good=0,short_as_good=0;
        std::array<int,5> minimum_hist{};
        std::ofstream definitions(std::string(argv[3])+"/"+family+"-predicates.tsv");
        definitions<<"mask\tdefinition\n";for(const auto& p:predicates)definitions<<p.mask<<'\t'<<p.definition<<'\n';
        std::ofstream null_table(std::string(argv[3])+"/"+family+"-label-calibration.tsv");
        null_table<<"published_mask\tsingle_errors\tminimum_pure_clauses_capped_at_4\n";
        std::cout<<family<<": "<<predicates.size()<<" unique predicate masks; best single errors "<<error<<"; CNF "<<cnf.size<<"; DNF "<<dnf.size<<'\n'<<std::flush;
        int at=0;for(Mask lab:nulls){
            int single=best_single(lab,predicates);
            int minimum=std::min(cover(all^lab,predicates,all,true).size,cover(lab,predicates,all,false).size);
            ++minimum_hist[minimum];if(single<=error)++single_as_good;
            if(minimum<=observed)++short_as_good;
            null_table<<lab<<'\t'<<single<<'\t'<<minimum<<'\n';
            if(++at%1000==0)std::cerr<<family<<": calibrated "<<at<<" / "<<nulls.size()<<'\n';
        }
        std::ofstream report(std::string(argv[3])+"/"+family+"-summary.json");
        report<<"{\"family\":\""<<family<<"\",\"rows\":"<<rows.size()<<",\"features_per_row\":"<<rows[0].size()
          <<",\"unique_predicate_masks\":"<<predicates.size()<<",\"best_single_errors\":"<<error
          <<",\"best_single_predicate\":\""<<predicates[witness].definition<<"\",\"best_single_mismatches\":[";
        bool first=true;for(size_t i=0;i<labels.size();++i)if((actual^predicates[witness].mask)&(Mask(1)<<i)){if(!first)report<<',';first=false;report<<'"'<<labels[i]<<'"';}
        report<<"],\"exact_cnf\":";print_rule(report,cnf,predicates,true);report<<",\"exact_dnf\":";print_rule(report,dnf,predicates,false);
        report<<",\"fixed_count_label_assignments\":"<<nulls.size()<<",\"single_search_as_good_count\":"<<single_as_good
          <<",\"short_rule_search_as_good_count\":"<<short_as_good<<",\"minimum_clause_histogram\":[";
        for(int k=1;k<=4;++k){if(k>1)report<<',';report<<minimum_hist[k];}report<<"]}\n";
    }
    return 0;
  }catch(const std::exception& e){std::cerr<<"ERROR: "<<e.what()<<'\n';return 1;}
}
