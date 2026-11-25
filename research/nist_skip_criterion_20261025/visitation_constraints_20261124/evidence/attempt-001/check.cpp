#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <map>
#include <numeric>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
using I=std::int64_t;
void need(bool v,const std::string& why){if(!v)throw std::runtime_error(why);}
std::vector<std::string> fields(const std::string& line){
    std::vector<std::string> out;std::istringstream in(line);std::string s;
    while(std::getline(in,s,'\t'))out.push_back(s);
    return out;
}
struct Row{int bits;I index;bool kept;};
struct Gap{int bits;I start,finish,low,high;int attempts;};
I spacing_gcd(const std::vector<I>& starts){
    need(starts.size()>=2,"need two retained positions");I g=0;
    for(std::size_t j=1;j<starts.size();++j)g=std::gcd(g,20*(starts[j]-starts[j-1]));
    return g;
}
bool monotone_witness(const Gap& large_r,const Gap& small_r){
    return large_r.attempts>=small_r.attempts&&large_r.high<small_r.low;
}
int main(int argc,char** argv){
 try{
    need(argc==3,"usage: check historical.tsv point-attempts.tsv");
    // Positive controls prevent a detector that declares every model impossible.
    need(spacing_gcd({2,5,8})==60,"planted nonzero-stride control");
    Gap a{0,0,0,2,6,3},b{0,0,0,1,4,2};
    need(!monotone_witness(a,b),"feasible monotone control");
    a.high=0;need(monotone_witness(a,b),"infeasible monotone control");
    std::ifstream in(argv[1]);need(bool(in),"input unavailable");
    std::map<int,std::vector<Row>> groups;std::string line;int count=0;
    std::set<std::pair<int,I>> keys;
    while(std::getline(in,line)){
      if(line.empty()||line[0]=='#')continue;auto f=fields(line);
      need(f.size()==9,"historical schema");int bits=std::stoi(f[1]);I index=std::stoll(f[4]);
      need(f[2]=="published"||f[2]=="skipped","label schema");
      need(keys.insert({bits,index}).second,"duplicate position");
      groups[bits].push_back({bits,index,f[2]=="published"});++count;
    }
    need(count==21&&groups.size()==6,"complete original cohort required");
    std::map<std::pair<int,I>,int> attempts;
    std::ifstream points(argv[2]);need(bool(points),"point inputs unavailable");
    while(std::getline(points,line)){
      if(line.empty()||line[0]=='#')continue;auto f=fields(line);need(f.size()==5,"point schema");
      auto key=std::make_pair(std::stoi(f[1]),std::stoll(f[2]));
      int r=std::stoi(f[4]);need(r>=1&&keys.count(key),"point observation must join cohort");
      need(attempts.emplace(key,r).second,"duplicate point control");
    }
    need(attempts.size()==6,"all six point controls required");
    std::vector<Gap> gaps;
    I startup_low=0,startup_high=INT64_MAX;
    int gcd_witnesses=0;
    std::cout<<"kind\tbits\tstart\tfinish\tmin_extra_seed_blocks\tmax_extra_seed_blocks\tattempts_or_gcd\n";
    for(auto& item:groups){int bits=item.first;auto& rows=item.second;
      std::sort(rows.begin(),rows.end(),[](auto x,auto y){return x.index<y.index;});
      std::vector<I> kept;for(auto r:rows)if(r.kept)kept.push_back(r.index);
      need(!kept.empty(),"missing retained control");
      I lower=0;for(auto r:rows)if(!r.kept&&r.index<kept.front())lower=std::max(lower,r.index);
      I upper=kept.front()-1;need(lower<=upper,"source initial interval invalid");
      startup_low=std::max(startup_low,lower);startup_high=std::min(startup_high,upper);
      std::cout<<"startup\t"<<bits<<"\t0\t"<<kept.front()<<'\t'<<lower<<'\t'<<upper<<"\t0\n";
      if(kept.size()>1){
        I g=spacing_gcd(kept);bool skipped_inside=false;
        for(auto r:rows)if(!r.kept&&r.index>kept.front()&&r.index<kept.back())skipped_inside=true;
        if(g==20&&skipped_inside)++gcd_witnesses;
        std::cout<<"spacing_gcd\t"<<bits<<'\t'<<kept.front()<<'\t'<<kept.back()<<"\t0\t0\t"<<g<<'\n';
      }
      for(std::size_t j=1;j<kept.size();++j){
        I lo=0,hi=kept[j]-kept[j-1]-1;
        for(auto r:rows)if(!r.kept&&r.index>kept[j-1]&&r.index<kept[j])lo=std::max(lo,r.index-kept[j-1]);
        auto r=attempts.find({bits,kept[j-1]});need(r!=attempts.end(),"missing point-attempt source");
        gaps.push_back({bits,kept[j-1],kept[j],lo,hi,r->second});
        std::cout<<"gap\t"<<bits<<'\t'<<kept[j-1]<<'\t'<<kept[j]<<'\t'<<lo<<'\t'<<hi<<'\t'<<r->second<<'\n';
      }
    }
    need(gcd_witnesses==2,"expected two full fixed-stride contradictions");
    int monotone=0,affine=0;
    for(auto x:gaps)for(auto y:gaps)if(x.bits==y.bits&&x.start!=y.start){
      if(monotone_witness(x,y)){
        ++monotone;
        std::cerr<<"monotone contradiction: bits="<<x.bits<<" r="<<x.attempts
                 <<" cost<="<<20*x.high<<"; r="<<y.attempts<<" cost>="<<20*y.low<<'\n';
      }
      if(x.attempts==2&&y.attempts==1&&x.low>2*y.high){
        ++affine;
        std::cerr<<"affine contradiction: bits="<<x.bits<<" f(2)>="<<20*x.low
                 <<" > 2*f(1)<="<<40*y.high<<'\n';
      }
    }
    need(monotone>=1&&affine>=1,"expected point-cost witnesses absent");
    need(startup_low>startup_high,"common warmup contradiction absent");
    std::cerr<<"common warmup impossible: lower="<<startup_low<<" > upper="<<startup_high<<" seed blocks\n";
    std::cerr<<"21 existing rows and six existing point observations retained; three controls pass.\n";
    return 0;
 }catch(const std::exception& e){std::cerr<<"ERROR: "<<e.what()<<'\n';return 1;}
}
