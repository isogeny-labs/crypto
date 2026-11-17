// Test plausible incomplete primality-test gates on proven-prime curve orders.
// These are models of bugs, not claims that any historical library had them.
#include "common.hpp"
#include <fstream>
#include <iostream>
#include <map>

static void gate(std::map<std::string,long>& out,const std::string& name,GEN raw,GEN n,GEN odd,long s){
    pari_sp saved=avma;GEN a=modii(raw,n);long k=-1;
    if(signe(a)){
        GEN x=Fp_pow(a,odd,n);k=0;
        while(!equali1(x)){x=Fp_sqr(x,n);++k;require(k<=s,"prime-order 2-adic order calculation failed");}
    }
    out["mr_model:"+name+":chi"]=kronecker(a,n);
    if(signe(a))require((k<s)==(kronecker(a,n)==1),"2-adic order and Euler character disagree");
    out["mr_model:"+name+":two_adic_order"]=k;avma=saved;
}
int main(int argc,char** argv){
  try{
    require(argc==4,"usage: primality_gates inputs.tsv base_features output_directory");
    pari_init(256000000,2000000);std::ifstream input(argv[1]);std::string line;int row=0;
    while(std::getline(input,line)){
        if(line.empty()||line[0]=='#')continue;pari_sp row_saved=avma;
        std::vector<std::string> f;std::istringstream fields(line);std::string field;while(std::getline(fields,field,'\t'))f.push_back(field);
        require(f.size()==9,"input schema mismatch");long bits=std::stol(f[1]);
        GEN n=integer(f[7]);require(isprime(n),"curve order is not proven prime");
        long s=gvaluation(subis(n,1),gen_2);GEN odd=shifti(subis(n,1),-s);
        std::ifstream base(std::string(argv[2])+"/curve-"+std::to_string(row)+".features.tsv");require(bool(base),"missing base features");
        std::map<std::string,long> out;std::string key;long value;while(base>>key>>value)out[key]=value;
        for(int a:primes_through(257))gate(out,"fixed_base_"+std::to_string(a),stoi(a),n,odd,s);
        uint64_t state=(uint64_t(std::stoul(f[3]))<<16)|0x330e;
        for(uint32_t i=0;i<20*std::stoul(f[4]);++i)state=(state*UINT64_C(0x5deece66d)+0xb)&((UINT64_C(1)<<48)-1);
        std::vector<uint64_t> states{state};
        int required=64+((bits+7)/8)+3;
        for(int i=0;i<required;++i)states.push_back((states.back()*UINT64_C(0x5deece66d)+0xb)&((UINT64_C(1)<<48)-1));
        for(int offset=0;offset<64;++offset){
            pari_sp saved=avma;std::map<std::string,GEN> bases;
            bases["state48"]=utoi(states[offset+1]);bases["word31"]=utoi(states[offset+1]>>17);bases["byte8"]=utoi(states[offset+1]>>40);
            for(int width:{8,31})for(int padding:{0,1}){
                GEN a=gen_0;int limbs=(bits+width-1)/width+padding;
                for(int k=0;k<limbs;++k)a=addii(shifti(a,width),utoi(states[offset+k+1]>>(48-width)));
                bases["full_field_"+std::to_string(width)+"_extra"+std::to_string(padding)]=a;
            }
            GEN block=gen_0;for(int k=0;k<20;++k)block=addii(shifti(block,8),utoi(states[offset+k+1]>>40));bases["block160"]=block;
            for(const auto& [shape,a]:bases){std::string name="rng_offset_"+std::to_string(offset)+":"+shape;
                gate(out,name+":raw",a,n,odd,s);gate(out,name+":plus1",addis(a,1),n,odd,s);
                gate(out,name+":range2",addis(modii(a,subis(n,3)),2),n,odd,s);}
            avma=saved;
        }
        std::string prefix=std::string(argv[3])+"/curve-"+std::to_string(row++);
        std::ofstream table(prefix+".features.tsv"),json(prefix+".json");bool first=true;
        json<<"{\"label\":\""<<f[0]<<"\",\"feature_count\":"<<out.size()<<",\"features\":{";
        for(const auto& [name,v]:out){table<<name<<'\t'<<v<<'\n';if(!first)json<<',';first=false;json<<'"'<<name<<"\":"<<v;}json<<"}}\n";
        std::cout<<f[0]<<": "<<out.size()<<" features including primality-gate models\n"<<std::flush;avma=row_saved;
    }
    require(row==21,"expected 21 rows");pari_close();return 0;
  }catch(const std::exception& e){std::cerr<<"ERROR: "<<e.what()<<'\n';return 1;}
}
