// Direct evaluation of candidate rules from exact parameters, independent of
// feature-mask generation. This validates fit, not historical causation.
#include "common.hpp"
#include <fstream>
#include <iostream>

int main(int argc,char** argv){
  try{
    require(argc==3,"usage: verify_rules inputs.tsv output_prefix");pari_init(64000000,2000000);
    std::ifstream input(argv[1]);std::ofstream csv(std::string(argv[2])+".tsv"),summary(std::string(argv[2])+".json");
    csv<<"label\trole\ttwist_mod8\tb_plus_1_mod19\tt_mod7\tt_mod13\tgate1\tgate2\ttwo_test_accept\tthree_test_accept\n";
    std::string line;int rows=0,published=0,skipped=0,first_tp=0,first_tn=0,two_errors=0,three_errors=0,two_tp=0,two_tn=0;
    while(std::getline(input,line)){
        if(line.empty()||line[0]=='#')continue;pari_sp saved=avma;
        std::vector<std::string> f;std::istringstream stream(line);std::string s;while(std::getline(stream,s,'\t'))f.push_back(s);
        require(f.size()==9,"schema mismatch");GEN p=integer(f[5]),b=integer(f[6]),n=integer(f[7]);
        GEN twist=subii(shifti(addis(p,1),1),n),trace=subii(addis(p,1),n);
        long tr8=umodiu(twist,8),b19=umodiu(addis(b,1),19),t7=umodiu(trace,7),t13=umodiu(trace,13);
        bool pub=f[2]=="published",gate1=tr8>=5,gate2=b19<=13,two=gate1&&gate2,three=gate1&&t7!=0&&t13!=6;
        ++rows;published+=pub;skipped+=!pub;first_tp+=pub&&gate1;first_tn+=!pub&&!gate1;
        two_errors+=pub!=two;three_errors+=pub!=three;
        two_tp+=pub&&two;two_tn+=!pub&&!two;
        csv<<f[0]<<'\t'<<f[2]<<'\t'<<tr8<<'\t'<<b19<<'\t'<<t7<<'\t'<<t13<<'\t'<<gate1<<'\t'<<gate2<<'\t'<<two<<'\t'<<three<<'\n';
        avma=saved;
    }
    require(rows==21&&published==10&&skipped==11,"cohort mismatch");
    summary<<"{\"rows\":"<<rows<<",\"published\":"<<published<<",\"skipped\":"<<skipped
      <<",\"first_gate_published_accepted\":"<<first_tp<<",\"first_gate_skipped_rejected\":"<<first_tn
      <<",\"first_gate_published_rejected\":"<<(published-first_tp)<<",\"first_gate_skipped_accepted\":"<<(skipped-first_tn)
      <<",\"two_gate_published_accepted\":"<<two_tp<<",\"two_gate_skipped_rejected\":"<<two_tn
      <<",\"two_gate_published_rejected\":"<<(published-two_tp)<<",\"two_gate_skipped_accepted\":"<<(skipped-two_tn)
      <<",\"two_test_errors\":"<<two_errors<<",\"three_test_errors\":"<<three_errors<<"}\n";
    std::cout<<"rows="<<rows<<"; two-test errors="<<two_errors<<"; three-test errors="<<three_errors<<'\n';
    pari_close();return 0;
  }catch(const std::exception& e){std::cerr<<"ERROR: "<<e.what()<<'\n';return 1;}
}
