#include "io.hpp"
#include <iostream>
int main(int argc,char** argv) {
try {
    require(argc==4,"tamper KIND RESULTS FRESH_COPY");fs::path out=argv[3];require(!fs::exists(out),"fresh copy");
    fs::copy(argv[2],out,fs::copy_options::recursive);std::string kind=argv[1];
    if(kind=="summary") {
        auto s=read_file(out/"summary.json");std::string needle="\"tail_at_least_observed\":";
        auto begin=s.find(needle);require(begin!=std::string::npos,"summary target");begin+=needle.size();
        auto end=s.find_first_not_of("0123456789",begin);require(end>begin,"summary integer");
        s.replace(begin,end-begin,std::to_string(std::stoull(s.substr(begin,end-begin))+1));put(out/"summary.json",s);
    } else {
        std::string name;size_t column;
        if(kind=="field"){name="fields.tsv";column=8;}
        else {require(kind=="frequency","corruption kind");name="distribution.tsv";column=1;}
        std::istringstream input(read_file(out/name));std::ostringstream output;std::string line;int row=0;
        while(std::getline(input,line)) {
            if(row++==1){auto c=cells(line);require(column<c.size(),"corruption column");c[column]=std::to_string(std::stoull(c[column])+1);tsv(output,c);}
            else output << line << '\n';
        }
        require(row>1,"corruption row");put(out/name,output.str());
    }
    std::cout << "Prepared one deliberate " << kind << " corruption\n";return 0;
}catch(const std::exception& e){std::cerr << "FAIL: " << e.what() << '\n';return 1;}
}
