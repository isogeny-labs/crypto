#include <gmpxx.h>
#include <fstream>
#include <iostream>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
using Z=mpz_class;
std::vector<std::string> split(const std::string&s){std::vector<std::string> r;std::istringstream in(s);std::string v;while(std::getline(in,v,'\t'))r.push_back(v);return r;}
void require(bool b,const std::string&m){if(!b)throw std::runtime_error(m);}
int main(int argc,char**argv){try{
 require(argc==3,"usage: analyze RUN0.tsv RUN1.tsv");std::ifstream f0(argv[1],std::ios::binary),f1(argv[2],std::ios::binary);require(f0.good()&&f1.good(),"missing raw input");std::string a((std::istreambuf_iterator<char>(f0)),{}),b((std::istreambuf_iterator<char>(f1)),{});require(a==b,"A/A bytes differ");std::istringstream lines(a);std::string line;std::getline(lines,line);require(line=="kind\ta\tu\tM\tweights\tplacement\tp\tlift\tdefect\ttarget\tnumerator\tdenominator\taccepted","header");
 std::set<std::string> expected,seen;for(auto c:std::vector<std::vector<int>>{{3,3,3},{3,3,4},{3,3,5},{4,3,4},{4,4,4}})for(int w=0;w<3;++w)for(int place=0;place<2;++place)for(int p:{1,4,9})for(int lift=0;lift<2;++lift){std::ostringstream key;key<<"case"<<'\t'<<c[0]<<'\t'<<c[1]<<'\t'<<c[2]<<'\t'<<w<<'\t'<<place<<'\t'<<p<<'\t'<<lift<<"\t0";expected.insert(key.str());}
 expected.insert("control\t3\t3\t3\t0\t0\t4\t0\t1");expected.insert("control\t3\t3\t3\t0\t0\t4\t0\t2");expected.insert("affine\t3\t3\t3\t0\t0\t4\t0\t3");int rows=0;
 while(std::getline(lines,line)){auto v=split(line);require(v.size()==13,"row width");std::string key;for(int i=0;i<9;++i){if(i)key+='\t';key+=v[i];}require(expected.count(key),"unregistered key");require(seen.insert(key).second,"duplicate key");int w=std::stoi(v[4]),p=std::stoi(v[6]),defect=std::stoi(v[8]);Z target(v[9]),num(v[10]),den(v[11]);require(target==(w==0?1:w==1?-11:0),"wrong known target");require(den>0,"nonpositive denominator");Z gcd;mpz_gcd(gcd.get_mpz_t(),num.get_mpz_t(),den.get_mpz_t());require(gcd==1,"unreduced rational");bool integral=mpz_odd_p(den.get_mpz_t());Z error=num-target*den,modulus=Z(1)<<p;bool correct=integral&&mpz_divisible_p(error.get_mpz_t(),modulus.get_mpz_t());bool accepted=defect==0?correct:defect==1?!correct:defect==2?!integral:(num==-1&&den==1&&target==1);require(accepted,"recomputed acceptance failure");require(v[12]=="1","producer rejected row");++rows;}
 require(seen==expected&&rows==183,"incomplete census");std::cout<<"{\"schema\":\"hermite-native-recount/v1\",\"physical_executions\":2,\"positive_cases_each\":180,\"defective_controls_each\":3,\"rows_recounted\":366,\"byte_identical\":true,\"standing\":\"unsigned producer evidence\"}\n";
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 2;}return 0;}
