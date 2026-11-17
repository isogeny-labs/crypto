#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>

int main(int argc,char** argv){
  try{
    if(argc<3)throw std::runtime_error("usage: calibration_union output.json calibration.tsv ...");
    std::set<unsigned> all_labels,exact_two_labels;std::map<unsigned,int> occurrences;
    for(int i=2;i<argc;++i){
        std::ifstream input(argv[i]);if(!input)throw std::runtime_error("missing calibration file");
        std::string line;std::getline(input,line);std::set<unsigned> current;
        while(std::getline(input,line)){std::istringstream fields(line);unsigned mask;int single,clauses;
            if(!(fields>>mask>>single>>clauses))throw std::runtime_error("malformed calibration row");
            if(!current.insert(mask).second)throw std::runtime_error("duplicate label assignment");
            if(clauses<=2)exact_two_labels.insert(mask);
            ++occurrences[mask];
        }
        if(current.size()!=4480)throw std::runtime_error("wrong permutation count");
        if(i==2)all_labels=current;else if(current!=all_labels)throw std::runtime_error("cohort assignment sets differ");
    }
    std::ofstream out(argv[1]);out<<"{\"families\":"<<(argc-2)<<",\"label_assignments\":4480,\"assignments_with_some_exact_rule_of_at_most_two_tests\":"<<exact_two_labels.size()
      <<",\"exact_fit_percent\":"<<(100.0*exact_two_labels.size()/4480.0)<<"}\n";
    std::cout<<exact_two_labels.size()<<" / 4480 assignments admit an exact one- or two-test fit across these families\n";
    return 0;
  }catch(const std::exception& e){std::cerr<<"ERROR: "<<e.what()<<'\n';return 1;}
}
