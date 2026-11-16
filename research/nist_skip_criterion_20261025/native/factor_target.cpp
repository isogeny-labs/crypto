#include "common.hpp"
#include <fstream>
#include <iostream>

int main(int argc,char** argv){
  try{
    require(argc==4,"usage: factor_target inputs.tsv row target");
    pari_init(256000000,2000000);setrand(gen_1);std::ifstream input(argv[1]);std::string line;
    int row=std::stoi(argv[2]),at=0;
    while(std::getline(input,line)){
        if(line.empty()||line[0]=='#')continue;
        if(at++!=row)continue;
        std::vector<std::string> fields;std::istringstream stream(line);std::string s;
        while(std::getline(stream,s,'\t'))fields.push_back(s);require(fields.size()==9,"schema mismatch");
        GEN p=integer(fields[5]),n=integer(fields[7]),target=nullptr;std::string type=argv[3];
        if(type=="minus")target=subis(n,1);else if(type=="plus")target=addis(n,1);
        else if(type=="twist")target=subii(shifti(addis(p,1),1),n);
        else if(type=="disc")target=absi(subii(sqri(subii(addis(p,1),n)),shifti(p,2)));
        else throw std::runtime_error("unknown target");
        std::cerr<<fields[0]<<' '<<type<<" target="<<decimal(target)<<'\n';
        GEN fact=factorint(target,0),product=gen_1,largest=gen_1,square=gen_1;long count=0;
        std::ostringstream factors;factors<<'[';
        for(long i=1;i<lg(gel(fact,1));++i){GEN prime=gcoeff(fact,i,1);long exponent=itos(gcoeff(fact,i,2));
            require(isprime(prime),"unproved factor");product=mulii(product,powiu(prime,exponent));count+=exponent;
            square=mulii(square,powiu(prime,exponent/2));if(cmpii(prime,largest)>0)largest=prime;
            if(i>1)factors<<',';factors<<"[\""<<decimal(prime)<<"\","<<exponent<<']';}
        factors<<']';require(equalii(product,target),"factor product failed");
        std::cout<<"{\"label\":\""<<fields[0]<<"\",\"target\":\""<<type<<"\",\"integer\":\""<<decimal(target)
            <<"\",\"factors\":"<<factors.str()<<",\"largest_prime_bits\":"<<bit_length(largest)
            <<",\"factor_count_with_multiplicity\":"<<count<<",\"square_part_root\":\""<<decimal(square)
            <<"\",\"complete_and_proven\":true}\n";
        pari_close();return 0;
    }
    throw std::runtime_error("row not found");
  }catch(const std::exception& e){std::cerr<<"ERROR: "<<e.what()<<'\n';return 1;}
}
