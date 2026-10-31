#include "common.hpp"
#include <fstream>
#include <iostream>

// Deterministic textbook Tonelli-Shanks, with the least positive nonresidue.
static GEN tonelli(GEN value,GEN p){
    long s=gvaluation(subis(p,1),gen_2);GEN q=shifti(subis(p,1),-s),z=stoi(2);
    while(kronecker(z,p)!=-1)z=addis(z,1);
    GEN c=Fp_pow(z,q,p),r=Fp_pow(value,shifti(addis(q,1),-1),p),t=Fp_pow(value,q,p);
    long m=s;
    while(!equali1(t)){
        long i=0;GEN u=t;while(!equali1(u)){u=Fp_sqr(u,p);++i;require(i<m,"Tonelli order failure");}
        GEN factor=Fp_pow(c,int2n(m-i-1),p);r=Fp_mul(r,factor,p);c=Fp_sqr(factor,p);t=Fp_mul(t,c,p);m=i;
    }
    require(equalii(Fp_sqr(r,p),value),"Tonelli root failed");return r;
}
int main(int argc,char** argv){
  try{
    require(argc==3,"usage: root_convention inputs.tsv historical|minimal");
    pari_init(128000000,2000000);std::ifstream input(argv[1]);std::string line;
    bool historical=std::string(argv[2])=="historical";
    require(historical||std::string(argv[2])=="minimal","invalid mode");
    while(std::getline(input,line)){
        if(line.empty()||line[0]=='#'){std::cout<<line<<'\n';continue;}
        pari_sp saved=avma;std::vector<std::string> fields;std::istringstream in(line);std::string f;
        while(std::getline(in,f,'\t'))fields.push_back(f);require(fields.size()==9,"schema mismatch");
        GEN p=integer(fields[5]),b=integer(fields[6]);
        if(umodiu(p,4)==3){require(equalii(Fp_pow(Fp_sqr(b,p),shifti(addis(p,1),-2),p),b),"p=3 mod 4 root convention mismatch");}
        else{
            require(std::stol(fields[1])==224,"unexpected p=1 mod 4 field");
            GEN desired=historical?subii(p,tonelli(Fp_sqr(b,p),p)):gmin(b,subii(p,b));
            if(historical&&fields[2]=="published")require(equalii(desired,b),"historical root does not reproduce published model");
            fields[6]=decimal(desired);
        }
        for(size_t i=0;i<fields.size();++i){if(i)std::cout<<'\t';std::cout<<fields[i];}std::cout<<'\n';avma=saved;
    }
    pari_close();return 0;
  }catch(const std::exception& e){std::cerr<<"ERROR: "<<e.what()<<'\n';return 1;}
}
