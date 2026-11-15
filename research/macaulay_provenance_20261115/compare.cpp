#include "common.hpp"
int main(int argc,char**argv){
 try{
  mca::require(argc==4,"usage: compare FIXTURE INPUT BASIS");const std::string fixture=argv[1];const auto s=canonical::read_system(argv[2]);
  const auto basis=quotient::read_basis(argv[3],s);const auto result=quotient::Extractor(s,basis,128).run();
  std::set<std::vector<canonical::U>> expected;std::size_t dimension=0;
  if(fixture=="grid"){expected={{0,0},{0,1},{1,0},{1,1}};dimension=4;}
  else if(fixture=="nonreduced"){expected={{0,0}};dimension=4;}
  else if(fixture=="extension"){dimension=2;}
  else if(fixture=="unit"){dimension=0;}
  else if(fixture=="x"){expected={{0}};dimension=1;}
  else if(fixture=="xy"){expected={{0,0}};dimension=1;}
  else {
   const auto ref=quotient::Extractor(s,quotient::read_basis(fixture,s),128).run();
   expected.insert(ref.points.begin(),ref.points.end());dimension=ref.dimension;
  }
  const std::set<std::vector<canonical::U>> actual(result.points.begin(),result.points.end());
  mca::require(actual==expected&&actual.size()==result.points.size()&&result.dimension==dimension,"complete coordinate-set or dimension mismatch");
  std::cout<<"PASS exact coordinate set points="<<actual.size()<<" quotient="<<dimension<<'\n';return 0;
 }catch(const std::exception&e){std::cerr<<"FAIL/UNKNOWN: "<<e.what()<<'\n';return 2;}
}
