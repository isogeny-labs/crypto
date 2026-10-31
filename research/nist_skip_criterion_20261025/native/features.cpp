// Exploratory feature extraction on fixed, validated public curve models.
// Native PARI arithmetic; no Python process, solver timing or DLP recovery.
#include "common.hpp"
#include <algorithm>
#include <cmath>
#include <fstream>
#include <iostream>
#include <map>
#include <set>

struct Features {
    std::map<std::string,long> data;
    void add(const std::string& key,long value) {
        require(!data.count(key),"duplicate feature: "+key);data[key]=value;
    }
};
static int valuation(GEN value,int prime) {
    return signe(value)==0?-1:int(gvaluation(value,utoipos(prime)));
}
static void properties(Features& f,const std::string& name,GEN value,GEN p,GEN n) {
    pari_sp saved=avma;
    f.add(name+":sign",signe(value)); f.add(name+":bits",bit_length(value));
    f.add(name+":hamming",popcount(value));
    for(int modulus:{3,4,5,7,8,11,13,16,17,19,23,31,32})
        f.add(name+":mod"+std::to_string(modulus),umodiu(value,modulus));
    for(int prime:{2,3,5,7,11,13,17,19,23,31})
        f.add(name+":v"+std::to_string(prime),valuation(value,prime));
    f.add(name+":chi_p",kronecker(value,p)); f.add(name+":chi_n",kronecker(value,n));
    avma=saved;
}
static long point_character(GEN x,GEN b,GEN p) {
    return kronecker(modii(addii(subii(powiu(x,3),mulsi(3,x)),b),p),p);
}
static GEN freelip_first_x(unsigned long seed,long discard,GEN p) {
    GEN modulus=subis(int2n(107),1), state=utoipos(seed);
    GEN multiplier=Fp_pow(stoi(3),stoi(121),modulus);
    for(long i=0;i<discard;++i)state=modii(mulii(state,multiplier),modulus);
    GEN x=gen_0;long limbs=(bit_length(p)+25)/26+1;
    for(long i=0;i<limbs;++i){state=modii(mulii(state,multiplier),modulus);
        x=addii(shifti(x,26),modii(state,int2n(26)));}
    return modii(x,p);
}
static void small_factors(Features& f,const std::string& name,GEN value,const std::vector<int>& primes) {
    pari_sp saved=avma; GEN remaining=absi(value), smooth=gen_1;
    int count=0,last=0;
    for(int prime:primes){
        int e=valuation(remaining,prime);
        if(e>0){GEN power=powuu(prime,e);smooth=mulii(smooth,power);
            remaining=diviiexact(remaining,power);count+=e;last=prime;}
        if(prime==97||prime==997||prime==9973||prime==99991){
            std::string suffix=":B"+std::to_string(prime);
            f.add(name+suffix+":smooth_bits",bit_length(smooth));
            f.add(name+suffix+":remaining_bits",bit_length(remaining));
            f.add(name+suffix+":factor_count",count);
            f.add(name+suffix+":largest_small_prime",last);
            f.add(name+suffix+":remaining_is_proven_prime",isprime(remaining));
        }
    }
    avma=saved;
}
int main(int argc,char** argv) {
  try {
    require(argc==4,"usage: features inputs.tsv zero_based_row output_prefix");
    pari_init(512000000,2000000);setrand(gen_1);
    std::ifstream input(argv[1]);require(bool(input),"cannot open input");
    int wanted=std::stoi(argv[2]), current=0;std::string line;
    while(std::getline(input,line)) {
      if(line.empty()||line[0]=='#')continue;
      if(current++!=wanted)continue;
      std::vector<std::string> row;std::istringstream stream(line);std::string field;
      while(std::getline(stream,field,'\t'))row.push_back(field);
      require(row.size()==9,"expected nine input columns");
      long bits=std::stol(row[1]);uint32_t run=std::stoul(row[3]),index=std::stoul(row[4]);
      GEN p=integer(row[5]), b=integer(row[6]), n=integer(row[7]), seed=integer(row[8]);
      GEN a=subis(p,3), c=x962(seed,bits), t=subii(addis(p,1),n);
      GEN disc=subii(sqri(t),shifti(p,2)), twist=subii(shifti(addis(p,1),1),n);
      std::cerr<<row[0]<<": input certificates\n";
      require(isprime(p)&&isprime(n),"p or n is not proven prime");
      require(bit_length(p)==bits&&bit_length(n)<=bits,"bit-length eligibility failed");
      require(equalii(seed,stream_seed(run,index)),"stream seed mismatch");
      require(equalii(modii(mulii(c,sqri(b)),p),modii(powiu(a,3),p)),"seed coefficient relation failed");
      require(signe(modii(addii(mulsi(4,powiu(a,3)),mulsi(27,sqri(b))),p))!=0,"singular curve");
      GEN curve=ellinit(mkvec2(a,b),p,DEFAULTPREC), x=gen_0, y=nullptr;
      for(long j=0;j<1000;++j){x=stoi(j);GEN rhs=modii(addii(subii(powiu(x,3),mulsi(3,x)),b),p);
          if(kronecker(rhs,p)>=0){y=Fp_sqrt(rhs,p);break;}}
      require(y,"no diagnostic witness in first 1000 x values");
      GEN point=mkvec2(x,y), killed=ellmul(curve,point,n);
      require(!gequal0(ellisoncurve(curve,point)),"witness off curve");
      require(lg(killed)==2&&gequal0(gel(killed,1)),"witness not killed by n");
      GEN bound=sqrtint(shifti(p,2)), center=addis(p,1);
      require(equali1(addis(divii(subis(subii(center,bound),1),n),1))&&
              equali1(divii(addii(center,bound),n)),"Hasse uniqueness failed");
      require(!equalii(n,p),"anomalous order");GEN pk=gen_1;
      for(long k=1;k<1024;++k){pk=modii(mulii(pk,p),n);require(!equali1(pk),"low embedding degree");}
      GEN numerator=mulsi(4,powiu(a,3));
      GEN jinv=Fp_div(mulsi(1728,numerator),addii(numerator,mulsi(27,sqri(b))),p);
      std::map<std::string,GEN> values={{"n",n},{"t",t},{"b",b},{"c",c},{"seed",seed},
        {"j",jinv},{"n-1",subis(n,1)},{"n+1",addis(n,1)},{"t-1",subis(t,1)},
        {"t+1",addis(t,1)},{"b-1",subis(b,1)},{"b+1",addis(b,1)},
        {"c-1",subis(c,1)},{"c+1",addis(c,1)},{"disc",disc},{"twist",twist}};
      Features features;
      std::cerr<<row[0]<<": arithmetic and point gates\n";
      for(const auto& [name,value]:values)properties(features,name,value,p,n);
      features.add("trace_hasse_abs_ppm",itos(divii(mulsi(1000000,absi(t)),bound)));
      features.add("b_bit_deficit",bits-bit_length(b));features.add("c_bit_deficit",bits-1-bit_length(c));
      features.add("seed_hamming",popcount(seed));
      for(long k=-32;k<=32;++k){pari_sp saved=avma;
          features.add("point_small_x:"+std::to_string(k),point_character(stoi(k),b,p));avma=saved;}
      for(long k=0;k<64;++k){pari_sp saved=avma;
          features.add("point_power2_x:"+std::to_string(k),point_character(int2n(k),b,p));avma=saved;}
      for(const auto& [name,value]:values){pari_sp saved=avma;
          features.add("point_derived_x:"+name,point_character(modii(value,p),b,p));avma=saved;}
      for(int offset=-4;offset<=4;++offset){pari_sp saved=avma;
          features.add("point_rng_block:"+std::to_string(offset),point_character(stream_seed(run,index+offset),b,p));avma=saved;}
      for(int offset=0;offset<=4;++offset){pari_sp saved=avma;
          features.add("point_sha1_seed_offset:"+std::to_string(offset),point_character(sha1_integer(modii(addis(seed,offset),int2n(160))),b,p));avma=saved;}
      for(unsigned long lip_seed:{20000,20420,20507,20509,20512,20517})
        for(long discard:{0,7}){pari_sp saved=avma;
          features.add("point_freelip:"+std::to_string(lip_seed)+":"+std::to_string(discard),
                       point_character(freelip_first_x(lip_seed,discard,p),b,p));avma=saved;}
      for(long bit=0;bit<160;++bit)features.add("secondary_seed_bit:"+std::to_string(bit),bittest(seed,bit));
      auto primes=primes_through(100000);
      std::cerr<<row[0]<<": Frobenius and SEA features\n";
      GEN coverage=gen_1;long elkies=0,atkin=0,ramified=0,lmax=0;
      for(int ell:primes){if(ell==2)continue;if(ell>1000)break;pari_sp saved=avma;
          long symbol=kronecker(disc,utoipos(ell));
          if(symbol>=0){coverage=mulsi(ell,coverage);++elkies;}else ++atkin;
          if(symbol==0)++ramified;
          if(!lmax&&cmpii(coverage,shifti(bound,1))>0)lmax=ell;
          if(ell<=251){features.add("disc_chi_l:"+std::to_string(ell),symbol);
              features.add("disc_v_l:"+std::to_string(ell),valuation(disc,ell));}
          if(ell==47||ell==97||ell==149||ell==199||ell==251||ell==293||ell==499){
              auto suffix=":"+std::to_string(ell);features.add("sea_elkies_count"+suffix,elkies);
              features.add("sea_atkin_count"+suffix,atkin);features.add("sea_ramified_count"+suffix,ramified);}
          // Keep coverage alive while reclaiming temporaries in this loop.
          coverage=gerepilecopy(saved,coverage);
      }
      require(lmax>0,"SEA coverage bound not reached through 1000");features.add("sea_elkies_only_lmax",lmax);
      std::cerr<<row[0]<<": extension order features\n";
      GEN previous=gen_2,trace=t,power=p;
      for(long degree=2;degree<=24;++degree){
          GEN next=subii(mulii(t,trace),mulii(p,previous));previous=trace;trace=next;power=mulii(power,p);
          if(std::set<long>{2,3,4,6,8,12,24}.count(degree)){
              GEN order=subii(addis(power,1),trace);
              for(int ell:{2,3,5,7,11,13,17,19})features.add("extension_order_v:"+std::to_string(degree)+":"+std::to_string(ell),valuation(order,ell));}
      }
      std::cerr<<row[0]<<": division polynomials\n";
      for(long ell:{3,5,7,11,13,17,19}){pari_sp saved=avma;
          GEN polynomial=liftall(elldivpol(curve,ell,0)),factors=FpX_factor(polynomial,p);
          long total=0,least=100000,largest=0,linear=0,negative_y=0,count=lg(gel(factors,1))-1;
          for(long k=1;k<=count;++k){GEN factor=gcoeff(factors,k,1);long degree=degpol(factor);
              GEN exponents=gel(factors,2);long multiplicity=typ(exponents)==t_VECSMALL?exponents[k]:itos(gel(exponents,k));
              require(multiplicity==1,"division polynomial has repeated factors");
              total+=degree;least=std::min(least,degree);largest=std::max(largest,degree);
              if(degree==1){++linear;GEN root=Fp_div(negi(gel(factor,2)),gel(factor,3),p);
                  if(point_character(root,b,p)==-1)++negative_y;}
          }
          require(total==(ell*ell-1)/2,"division polynomial degree mismatch");
          auto suffix=":"+std::to_string(ell);features.add("division_factor_count"+suffix,count);
          features.add("division_least_degree"+suffix,least);features.add("division_largest_degree"+suffix,largest);
          features.add("division_linear_count"+suffix,linear);features.add("division_linear_nonsquare_y_count"+suffix,negative_y);
          avma=saved;
      }
      std::cerr<<row[0]<<": bounded small-prime decomposition\n";
      for(const auto& name:{"n-1","n+1","twist"})small_factors(features,name,values.at(name),primes);
      std::ofstream out(std::string(argv[3])+".json"), table(std::string(argv[3])+".features.tsv");
      require(bool(out)&&bool(table),"cannot open outputs");
      out<<"{\"label\":\""<<row[0]<<"\",\"bits\":"<<bits<<",\"role\":\""<<row[2]
         <<"\",\"run\":"<<run<<",\"candidate\":"<<index<<",\"p\":\""<<decimal(p)
         <<"\",\"a\":\""<<decimal(a)<<"\",\"b\":\""<<decimal(b)<<"\",\"n\":\""<<decimal(n)
         <<"\",\"seed\":\""<<decimal(seed)<<"\",\"trace\":\""<<decimal(t)
         <<"\",\"disc\":\""<<decimal(disc)<<"\",\"j\":\""<<decimal(jinv)
         <<"\",\"witness_x\":\""<<decimal(x)<<"\",\"witness_y\":\""<<decimal(y)
         <<"\",\"certificates_pass\":true,\"features\":{";
      bool first=true;for(const auto& [key,value]:features.data){if(!first)out<<',';first=false;
          out<<'"'<<key<<"\":"<<value;table<<key<<'\t'<<value<<'\n';}
      out<<"}}\n";
      std::cout<<row[0]<<": "<<features.data.size()<<" features; certificates passed\n";
      pari_close();return 0;
    }
    throw std::runtime_error("row not found");
  }catch(const std::exception& e){std::cerr<<"ERROR: "<<e.what()<<'\n';return 1;}
}
