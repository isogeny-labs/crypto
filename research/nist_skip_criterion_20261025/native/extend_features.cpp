#include "common.hpp"
#include <fstream>
#include <iostream>
#include <map>
#include <regex>
#include <set>

struct Row{std::vector<std::string> f;};
struct FactorData{bool complete=false;GEN largest=nullptr,second=nullptr,square=nullptr;long distinct=0,total=0,odd=0;};
static FactorData factor_record(const std::string& original,const std::string& hints,int row,const std::string& type,GEN expected){
    std::string name="/curve-"+std::to_string(row)+"-"+type+".json",text;
    for(const auto& prefix:{original,hints}){std::ifstream f(prefix+name);if(!f)continue;
        std::string candidate((std::istreambuf_iterator<char>(f)),{});if(!candidate.empty()){text=candidate;break;}}
    if(text.empty())return{};
    FactorData out;out.largest=gen_1;out.second=gen_1;out.square=gen_1;GEN product=gen_1;
    std::regex pair(R"rx(\[\s*"([0-9]+)"\s*,\s*([0-9]+)\s*\])rx");
    for(auto it=std::sregex_iterator(text.begin(),text.end(),pair);it!=std::sregex_iterator();++it){
        GEN prime=integer((*it)[1]);long e=std::stol((*it)[2]);require(e>0&&e<1024&&isprime(prime),"invalid certified factor");
        product=mulii(product,powiu(prime,e));out.square=mulii(out.square,powiu(prime,e/2));++out.distinct;out.total+=e;out.odd+=e%2;
        if(cmpii(prime,out.largest)>0){out.second=out.largest;out.largest=prime;}
        else if(cmpii(prime,out.second)>0)out.second=prime;
    }
    require(out.distinct&&equalii(product,expected),"factor record product mismatch");out.complete=true;return out;
}
static void byte_features(std::map<std::string,long>& data,const std::string& name,GEN integer_value,int length){
    std::vector<unsigned long> bytes(length);GEN value=absi(integer_value);
    for(int i=length-1;i>=0;--i){bytes[i]=umodiu(value,256);value=shifti(value,-8);}
    require(signe(value)==0,"byte encoding too short");
    long zero=0,ff=0,high=0,space=0,first_zero=length,adjacent=0;std::set<unsigned long> distinct;
    for(int i=0;i<length;++i){auto b=bytes[i];zero+=b==0;ff+=b==255;high+=b>=128;
        space+=(b==9||b==10||b==11||b==12||b==13||b==32);if(b==0&&first_zero==length)first_zero=i;
        if(i&&bytes[i-1]==b)++adjacent;distinct.insert(b);}
    data["byte_"+name+":zero_count"]=zero;data["byte_"+name+":ff_count"]=ff;
    data["byte_"+name+":high_count"]=high;data["byte_"+name+":space_count"]=space;
    data["byte_"+name+":first_zero"]=first_zero;data["byte_"+name+":adjacent_equal"]=adjacent;
    data["byte_"+name+":distinct_count"]=distinct.size();
}
int main(int argc,char** argv){
  try{
    require(argc==7,"usage: extend_features inputs.tsv base_features original_factors verified_hints outdir row_count");
    pari_init(512000000,2000000);std::ifstream input(argv[1]);std::vector<Row> rows;std::string line;int limit=std::stoi(argv[6]);
    while(std::getline(input,line)){
        if(line.empty()||line[0]=='#')continue;std::istringstream in(line);std::string s;Row r;
        while(std::getline(in,s,'\t'))r.f.push_back(s);require(r.f.size()==9,"input schema mismatch");rows.push_back(r);
        if(int(rows.size())==limit)break;
    }
    require(int(rows.size())==limit,"row count mismatch");
    std::vector<std::map<std::string,FactorData>> factors(limit);std::map<std::string,bool> complete;
    for(const std::string type:{"minus","plus","twist","disc"}){
        bool all=true;for(int i=0;i<limit;++i){GEN p=integer(rows[i].f[5]),n=integer(rows[i].f[7]),target=nullptr;
            if(type=="minus")target=subis(n,1);else if(type=="plus")target=addis(n,1);
            else if(type=="twist")target=subii(shifti(addis(p,1),1),n);else target=absi(subii(sqri(subii(addis(p,1),n)),shifti(p,2)));
            factors[i][type]=factor_record(argv[3],argv[4],i,type,target);all&=factors[i][type].complete;
        }complete[type]=all;std::cerr<<type<<": complete for entire cohort = "<<all<<'\n';
    }
    for(int i=0;i<limit;++i){
        std::ifstream base(std::string(argv[2])+"/curve-"+std::to_string(i)+".features.tsv");require(bool(base),"missing base features");
        std::map<std::string,long> data;std::string key;long value;while(base>>key>>value)data[key]=value;
        GEN n=integer(rows[i].f[7]),seed=integer(rows[i].f[8]),b=integer(rows[i].f[6]);long bits=std::stol(rows[i].f[1]);
        byte_features(data,"seed",seed,20);byte_features(data,"sha1",sha1_integer(seed),20);
        byte_features(data,"b",b,(bits+7)/8);byte_features(data,"n",n,(bits+7)/8);
        int chunks=(bits-1)/160;long hash_zero_total=0;
        for(int k=0;k<=chunks;++k){auto bytes=bytes20(sha1_integer(modii(addis(seed,k),int2n(160))));for(auto c:bytes)hash_zero_total+=c==0;}
        data["byte_hash_expansion_zero_total"]=hash_zero_total;
        for(const auto& [type,available]:complete)if(available){const auto& f=factors[i].at(type);std::string prefix="full_factor_"+type+":";
            data[prefix+"largest_bits"]=bit_length(f.largest);data[prefix+"second_bits"]=equali1(f.second)?0:bit_length(f.second);
            data[prefix+"largest_per_field_permille"]=1000*bit_length(f.largest)/bits;
            data[prefix+"largest_second_permille"]=equali1(f.second)?-1:1000*bit_length(f.largest)/bit_length(f.second);
            data[prefix+"distinct_count"]=f.distinct;data[prefix+"total_count"]=f.total;data[prefix+"odd_exponent_count"]=f.odd;
            data[prefix+"square_root_bits"]=bit_length(f.square);
            if(bit_length(f.square)<31)data[prefix+"square_root"]=itos(f.square);
        }
        std::string prefix=std::string(argv[5])+"/curve-"+std::to_string(i);
        std::ofstream table(prefix+".features.tsv"),json(prefix+".json");
        json<<"{\"label\":\""<<rows[i].f[0]<<"\",\"feature_count\":"<<data.size()<<",\"features\":{";bool first=true;
        for(const auto& [name,val]:data){table<<name<<'\t'<<val<<'\n';if(!first)json<<',';first=false;json<<'"'<<name<<"\":"<<val;}
        json<<"}}\n";
    }
    pari_close();return 0;
  }catch(const std::exception& e){std::cerr<<"ERROR: "<<e.what()<<'\n';return 1;}
}
