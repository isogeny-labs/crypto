#include "common.hpp"
#include <fstream>
#include <iostream>
#include <regex>

int main(int argc,char** argv){
  try{
    require(argc==5,"usage: verify_factor_hint inputs.tsv row target hint.json");
    pari_init(128000000,2000000);std::ifstream input(argv[1]);std::string line;int wanted=std::stoi(argv[2]),at=0;
    while(std::getline(input,line)){
        if(line.empty()||line[0]=='#')continue;if(at++!=wanted)continue;
        std::vector<std::string> fields;std::istringstream in(line);std::string s;
        while(std::getline(in,s,'\t'))fields.push_back(s);require(fields.size()==9,"schema mismatch");
        GEN p=integer(fields[5]),n=integer(fields[7]),target=nullptr;std::string type=argv[3];
        if(type=="minus")target=subis(n,1);else if(type=="plus")target=addis(n,1);
        else if(type=="twist")target=subii(shifti(addis(p,1),1),n);
        else if(type=="disc")target=absi(subii(sqri(subii(addis(p,1),n)),shifti(p,2)));
        else throw std::runtime_error("unknown target");
        std::ifstream hint(argv[4]);require(bool(hint),"missing hint");
        std::string text((std::istreambuf_iterator<char>(hint)),{});require(text.size()<1000000,"hint too large");
        std::regex pair(R"rx(\[\s*"([0-9]+)"\s*,\s*([0-9]+)\s*\])rx");
        GEN product=gen_1,largest=gen_1,square=gen_1;long count=0,entries=0;std::ostringstream factors;factors<<'[';
        for(auto i=std::sregex_iterator(text.begin(),text.end(),pair);i!=std::sregex_iterator();++i){
            require((*i)[1].length()<1000,"factor too large");GEN prime=integer((*i)[1]);long exponent=std::stol((*i)[2]);
            require(exponent>0&&exponent<1024&&++entries<64,"invalid factor multiplicity");
            require(isprime(prime),"hint contains a factor that is not proven prime");
            product=mulii(product,powiu(prime,exponent));square=mulii(square,powiu(prime,exponent/2));count+=exponent;
            if(cmpii(prime,largest)>0)largest=prime;if(entries>1)factors<<',';
            factors<<"[\""<<decimal(prime)<<"\","<<exponent<<']';
        }
        require(entries&&equalii(product,target),"hint product differs from exact target");factors<<']';
        std::cout<<"{\"label\":\""<<fields[0]<<"\",\"target\":\""<<type<<"\",\"integer\":\""<<decimal(target)
            <<"\",\"factors\":"<<factors.str()<<",\"largest_prime_bits\":"<<bit_length(largest)
            <<",\"factor_count_with_multiplicity\":"<<count<<",\"square_part_root\":\""<<decimal(square)
            <<"\",\"complete_and_proven\":true,\"hint_independently_checked_by_native_prime_and_product_tests\":true}\n";
        pari_close();return 0;
    }
    throw std::runtime_error("row not found");
  }catch(const std::exception& e){std::cerr<<"ERROR: "<<e.what()<<'\n';return 1;}
}
