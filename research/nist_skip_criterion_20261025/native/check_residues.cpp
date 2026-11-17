// Independent digit-stream arithmetic check; no PARI or bigint dependency.
#include <fstream>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
static void need(bool b,const char* s){if(!b)throw std::runtime_error(s);}
static unsigned residue(const std::string& s,unsigned modulus){
    unsigned base=10,r=0;size_t start=0;
    if(s.size()>2&&s[0]=='0'&&(s[1]=='x'||s[1]=='X')){base=16;start=2;}
    need(start<s.size(),"empty integer");
    for(size_t i=start;i<s.size();++i){unsigned digit;
        if(s[i]>='0'&&s[i]<='9')digit=s[i]-'0';
        else if(s[i]>='a'&&s[i]<='f')digit=s[i]-'a'+10;
        else if(s[i]>='A'&&s[i]<='F')digit=s[i]-'A'+10;
        else throw std::runtime_error("invalid digit");
        need(digit<base,"digit outside base");r=(r*base+digit)%modulus;}
    return r;
}
static std::vector<std::string> fields(const std::string& line){
    std::vector<std::string> out;std::string cell;std::istringstream s(line);while(std::getline(s,cell,'\t'))out.push_back(cell);return out;
}
int main(int argc,char** argv){
  try{
    need(argc==3,"usage: check_residues inputs.tsv truth-table.tsv");std::ifstream inputs(argv[1]),truth(argv[2]);
    need(bool(inputs)&&bool(truth),"missing input");std::string a,b;std::getline(truth,b);int count=0;
    while(std::getline(inputs,a)){
        if(a.empty()||a[0]=='#')continue;
        need(bool(std::getline(truth,b)),"truth table ended early");
        auto row=fields(a),t=fields(b);need(row.size()==9&&t.size()==10,"schema mismatch");
        need(row[0]==t[0]&&row[2]==t[1],"label mismatch");
        unsigned twist=(2*residue(row[5],8)+2+16-residue(row[7],8))%8;
        unsigned coeff=(residue(row[6],19)+1)%19;
        unsigned trace7=(residue(row[5],7)+8-residue(row[7],7))%7;
        unsigned trace13=(residue(row[5],13)+14-residue(row[7],13))%13;
        need(twist==std::stoul(t[2])&&coeff==std::stoul(t[3])&&trace7==std::stoul(t[4])&&trace13==std::stoul(t[5]),"independent residue mismatch");
        bool twist_gate=twist==5||twist==7;
        bool two=twist_gate&&coeff<=13,three=twist_gate&&trace7!=0&&trace13!=6;
        need(two==bool(std::stoi(t[8]))&&three==bool(std::stoi(t[9])),"independent rule mismatch");++count;
    }
    need(count==21,"wrong curve count");need(!std::getline(truth,b),"extra truth rows");
    std::cout<<"PASS: all 21 exact-parameter residues and both rule outputs agree using independent digit-stream arithmetic\n";return 0;
  }catch(const std::exception& e){std::cerr<<"ERROR: "<<e.what()<<'\n';return 1;}
}
