// Necessary visitation/point-gate tests for explicitly defined generation artifacts.
// An unaligned model that fits these tests still needs a complete order census.
#include "common.hpp"
#include <algorithm>
#include <fstream>
#include <functional>
#include <iostream>
#include <map>
#include <set>

using Mask=uint32_t;
struct Row{int position;int bits;int index;bool published;std::string b,seed;};
struct Group{int bits;uint32_t run;std::string p;int last=0;std::vector<Row> rows;};
static GEN tonelli(GEN value,GEN p){
    long s=gvaluation(subis(p,1),gen_2);GEN q=shifti(subis(p,1),-s),z=stoi(2);
    while(kronecker(z,p)!=-1)z=addis(z,1);
    GEN c=Fp_pow(z,q,p),r=Fp_pow(value,shifti(addis(q,1),-1),p),t=Fp_pow(value,q,p);long m=s;
    while(!equali1(t)){long i=0;GEN u=t;while(!equali1(u)){u=Fp_sqr(u,p);++i;require(i<m,"Tonelli failure");}
        GEN factor=Fp_pow(c,int2n(m-i-1),p);r=Fp_mul(r,factor,p);c=Fp_sqr(factor,p);t=Fp_mul(t,c,p);m=i;}
    return r;
}
struct Event{bool ready=false,points=false,square=false,x0=false,x1=false,xseed=false,nul=false,hashnul=false;std::string b;};
struct Source{
    const Group& group;GEN p;long minus27_character;std::vector<uint64_t> states;std::vector<Event> events;
    explicit Source(const Group& g):group(g),p(integer(g.p)),minus27_character(kronecker(stoi(-27),p)),events(20*g.last){
        states.push_back((uint64_t(g.run)<<16)|0x330e);
    }
    uint64_t state(int after){
        require(after>=0&&after<20*group.last+1000000,"RNG expansion limit");
        while(int(states.size())<=after)states.push_back((states.back()*UINT64_C(0x5deece66d)+0xb)&((UINT64_C(1)<<48)-1));
        return states[after];
    }
    GEN seed_at(int start){unsigned char bytes[20];for(int j=0;j<20;++j)bytes[j]=state(start+j+1)>>40;return from_bytes(bytes,20);}
    static bool zero_byte(GEN value){auto b=bytes20(value);return std::find(b.begin(),b.end(),0)!=b.end();}
    long character(GEN x,GEN b){return kronecker(modii(addii(subii(powiu(x,3),mulsi(3,x)),b),p),p);}
    const Event& event(int start,bool points){
        Event& e=events.at(start);if(e.ready&&(!points||e.points))return e;pari_sp saved=avma;
        GEN seed=seed_at(start),c=x962(seed,group.bits);
        if(!e.ready){e.nul=zero_byte(seed);e.hashnul=zero_byte(sha1_integer(seed));
            e.square=signe(c)&&signe(modii(addis(mulsi(4,c),27),p))&&kronecker(c,p)*minus27_character==1;e.ready=true;}
        if(points&&!e.points){
            if(e.square){GEN ratio=Fp_div(stoi(-27),c,p);
                GEN b=umodiu(p,4)==3?Fp_pow(ratio,shifti(addis(p,1),-2),p):subii(p,tonelli(ratio,p));
                require(equalii(Fp_sqr(b,p),ratio),"root convention failure");e.b=decimal(b);
                e.x0=character(gen_0,b)>=0;e.x1=character(gen_1,b)>=0;e.xseed=character(seed,b)>=0;}
            e.points=true;
        }
        avma=saved;return e;
    }
    bool trigger(int type,int start){
        if(type==0)return true;const Event& e=event(start,type>=3&&type<=8);
        switch(type){case 1:return e.square;case 2:return !e.square;
            case 3:return e.square&&e.x0;case 4:return e.square&&!e.x0;
            case 5:return e.square&&e.x1;case 6:return e.square&&!e.x1;
            case 7:return e.square&&e.xseed;case 8:return e.square&&!e.xseed;
            case 9:return e.nul;case 10:return e.hashnul;}
        throw std::runtime_error("bad trigger");
    }
    bool point(int start,int& cursor,int width,int padding,bool retry,int sign_draw){
        const Event& e=event(start,true);if(!e.square)return true;
        pari_sp saved=avma;GEN b=integer(e.b);int limbs=(group.bits+width-1)/width+padding;
        bool success=false;
        for(int attempt=0;attempt<1000;++attempt){GEN x=gen_0;
            for(int k=0;k<limbs;++k)x=addii(shifti(x,width),utoi(state(++cursor)>>(48-width)));
            success=character(modii(x,p),b)>=0;if(success||!retry)break;
        }
        require(success||!retry,"point retry bound exhausted");if(success)cursor+=sign_draw;
        avma=saved;return success;
    }
};
struct Model{int warmup,type,draws,draw_kind;bool point=false,retry=false;int width=8,padding=0,sign=0;Mask mask=0;bool live=true;};
static std::string name(const Model& m){
    std::ostringstream out;out<<"warmup="<<m.warmup;
    if(m.point)out<<";point="<<(m.retry?"retry":"once")<<";word_bits="<<m.width<<";extra_limbs="<<m.padding<<";sign_draw="<<m.sign;
    else out<<";trigger="<<m.type<<";draw_kind="<<m.draw_kind<<";draws="<<m.draws;
    return out.str();
}
static int draws(const Model& m,const Group& g){
    switch(m.draw_kind){case 0:return m.draws;case 1:return(g.bits+7)/8;case 2:return(g.bits+30)/31;
      case 3:return(g.bits+25)/26;case 4:return 7*((g.bits+30)/31);case 5:return 8*((g.bits+30)/31);
      case 6:return(g.bits+30)/31+1;case 7:return(g.bits+7)/8+1;}
    throw std::runtime_error("bad draw kind");
}
static Mask simulate(Model& m,Source& source){
    const Group& group=source.group;Mask visited=0;std::map<int,int> known;
    for(const auto& row:group.rows)known[20*(row.index-1)]=row.position;
    int end=20*group.last,start=m.warmup;
    while(start<end){
        int cursor=start+20;bool accept=true;
        if(m.point)accept=source.point(start,cursor,m.width,m.padding,m.retry,m.sign);
        auto row=known.find(start);if(row!=known.end()&&accept)visited|=Mask(1)<<row->second;
        if(!m.point){int amount=draws(m,group);if(amount&&source.trigger(m.type,start))cursor+=amount;}
        require(cursor>start,"RNG walk failed to advance");start=cursor;
    }
    return visited;
}
static std::set<Mask> assignments(const std::map<int,Group>& groups){
    std::set<Mask> out{0};
    for(const auto& [_,g]:groups){int wanted=0;for(const auto& r:g.rows)wanted+=r.published;
        std::vector<Mask> pieces;
        std::function<void(size_t,int,Mask)> choose=[&](size_t at,int left,Mask mask){
            if(!left){pieces.push_back(mask);return;}if(g.rows.size()-at<size_t(left))return;
            choose(at+1,left-1,mask|(Mask(1)<<g.rows[at].position));choose(at+1,left,mask);
        };choose(0,wanted,0);std::set<Mask> next;for(auto a:out)for(auto b:pieces)next.insert(a|b);out.swap(next);
    }return out;
}
int main(int argc,char** argv){
  try{
    if(argc==2&&std::string(argv[1])=="--self-test"){
        pari_init(32000000,100000);Group g{384,123,"101",4,{{0,384,1,true,"1","1"},{1,384,2,false,"1","1"},{2,384,3,true,"1","1"},{3,384,4,false,"1","1"}}};
        Source source(g);Model model{0,0,20,0};require(simulate(model,source)==5,"planted alternating-block visitation failed");
        model.warmup=20;require(simulate(model,source)==10,"warmup visitation control failed");
        model.warmup=1;model.draws=0;require(simulate(model,source)==0,"unaligned negative control failed");
        pari_close();std::cout<<"PASS: planted block-consumption pattern, warmup control, unaligned negative control\n";return 0;
    }
    require(argc==3,"usage: rng_models inputs.tsv output_directory");pari_init(256000000,2000000);
    std::ifstream input(argv[1]);std::map<int,Group> groups;std::vector<std::string> labels;std::string line;Mask actual=0;
    while(std::getline(input,line)){
        if(line.empty()||line[0]=='#')continue;std::vector<std::string> f;std::istringstream stream(line);std::string s;
        while(std::getline(stream,s,'\t'))f.push_back(s);require(f.size()==9,"schema mismatch");
        int bits=std::stoi(f[1]),index=std::stoi(f[4]),position=labels.size();bool published=f[2]=="published";
        Group& g=groups[bits];g.bits=bits;g.run=std::stoul(f[3]);g.p=f[5];g.last=std::max(g.last,index);
        g.rows.push_back({position,bits,index,published,f[6],f[8]});labels.push_back(f[0]);if(published)actual|=Mask(1)<<position;
    }
    require(labels.size()==21,"expected 21 rows");std::vector<Model> models;
    for(int warmup=0;warmup<=64;++warmup){
        for(int type=0;type<=10;++type){
            for(int d=0;d<=64;++d)models.push_back({warmup,type,d,0});
            for(int kind=1;kind<=7;++kind)models.push_back({warmup,type,0,kind});
        }
        for(int width:{8,31})for(int padding:{0,1})for(int sign:{0,1})for(bool retry:{false,true}){
            Model m{warmup,1,0,0};m.point=true;m.width=width;m.padding=padding;m.sign=sign;m.retry=retry;models.push_back(m);}
    }
    std::ofstream trace(std::string(argv[2])+"/stage-counts.tsv");trace<<"field_bits\tmodels_compatible_with_forced_published_controls\n";
    for(int bits:{384,521,192,224,239,256}){
        const Group& group=groups.at(bits);pari_sp saved=avma;Source source(group);
        for(const auto& r:group.rows){GEN seed=source.seed_at(20*(r.index-1));require(equalii(seed,integer(r.seed)),"raw seed position mismatch");
            if(r.published){const Event& e=source.event(20*(r.index-1),true);require(e.square&&equalii(integer(e.b),integer(r.b)),"published root/model mismatch");}}
        int live=0;for(auto& model:models)if(model.live){Mask hit=simulate(model,source);model.mask|=hit;
            if(group.rows.size()==1&&!(hit&(Mask(1)<<group.rows[0].position)))model.live=false;
            live+=model.live;}
        trace<<bits<<'\t'<<live<<'\n'<<std::flush;std::cerr<<"field "<<bits<<": "<<live<<" compatible models\n";
        avma=saved;
    }
    auto nulls=assignments(groups);require(nulls.size()==4480,"null enumeration failed");
    std::set<Mask> masks,compatible_nulls;int best=100,exact=0,live=0;
    std::ofstream table(std::string(argv[2])+"/models.tsv"),matches(std::string(argv[2])+"/exact-matches.tsv");
    table<<"mask\terrors\tmodel\n";matches<<"mask\tmodel\n";std::vector<int> best_models;
    for(size_t i=0;i<models.size();++i){const auto& m=models[i];if(!m.live)continue;++live;masks.insert(m.mask);
        int error=__builtin_popcount(actual^m.mask);table<<m.mask<<'\t'<<error<<'\t'<<name(m)<<'\n';
        if(nulls.count(m.mask))compatible_nulls.insert(m.mask);
        if(error<best){best=error;best_models.clear();}if(error==best)best_models.push_back(i);
        if(!error){++exact;matches<<m.mask<<'\t'<<name(m)<<'\n';}
    }
    std::ofstream report(std::string(argv[2])+"/summary.json");
    report<<"{\"models_tested\":"<<models.size()<<",\"compatible_with_forced_controls\":"<<live
        <<",\"unique_compatible_masks\":"<<masks.size()<<",\"exact_matches\":"<<exact
        <<",\"minimum_errors_compatible_models\":"<<best<<",\"null_exact_fit_count\":"<<compatible_nulls.size()
        <<",\"null_label_assignments\":4480,\"best_models\":[";
    for(size_t k=0;k<best_models.size()&&k<20;++k){if(k)report<<',';const auto& m=models[best_models[k]];
        report<<"{\"model\":\""<<name(m)<<"\",\"mismatches\":[";bool first=true;
        for(size_t i=0;i<labels.size();++i)if((actual^m.mask)&(Mask(1)<<i)){if(!first)report<<',';first=false;report<<'"'<<labels[i]<<'"';}report<<"]}";}
    report<<"]}\n";std::cout<<"models="<<models.size()<<" live="<<live<<" exact="<<exact<<" best_errors="<<best<<'\n';
    pari_close();return 0;
  }catch(const std::exception& e){std::cerr<<"ERROR: "<<e.what()<<'\n';return 1;}
}
