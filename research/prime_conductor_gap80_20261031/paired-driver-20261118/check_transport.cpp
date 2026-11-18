// Tiny public correctness controls only. Never a timing or large-case solver.
#define main complete_pipeline_fixture_entry
#include "../native/complete_pipeline.cpp"
#undef main
#include <fstream>
int main(int argc,char**argv){try{
 require(argc==3,"expected case input and prepared-point TSV");std::ifstream in(argv[1]),rows(argv[2]);
 Z ell,r,ca,cb;Point K,G,H;K.inf=G.inf=H.inf=false;unsigned long count;
 require(bool(in>>p>>a>>b>>ell>>K.x>>K.y>>r>>G.x>>G.y),"case header");
 require(p>3&&p<(Z(1)<<32)&&ell>=5&&ell<=10009&&r>=5&&r<=100000,"development verifier bounds");
 require(bool(rows>>ca>>cb>>H.x>>H.y>>count)&&count<=32,"prepared header");
 pari_init(256000000,2000000);
 GEN E=ellinit(mkvec5(gen_0,gen_0,gen_0,gen(a),gen(b)),gen(p),DEFAULTPREC),iso=ellisogeny(E,gp(K),0,0,1);
 GEN coefficients=gel(iso,1),map=gel(iso,2);
 require(integer(gel(coefficients,4))==ca&&integer(gel(coefficients,5))==cb,"independent PARI quotient mismatch");
 require(equal(H,ellisogenyapply(map,gp(G))),"independent PARI generator-image mismatch");
 require(lg(ellisogenyapply(map,gp(K)))==2&&lg(ellisogenyapply(map,gp(Point{})))==2,"independent PARI exceptional maps");
 GEN F=ellinit(coefficients,gen(p),DEFAULTPREC);
 unsigned long checked=0;
 for(unsigned long i=0;i<count;i++){
   Z s;Point Q,R;Q.inf=R.inf=false;require(bool(rows>>s>>Q.x>>Q.y>>R.x>>R.y),"truncated prepared row");
   require(s>0&&s<r&&equal(Q,ellmul(E,gp(G),gen(s))),"independent source target mismatch");
   require(equal(R,ellisogenyapply(map,gp(Q))),"independent PARI target-image mismatch");
   require(integer(elllog(E,gp(Q),gp(G),gen(r)))==s,"source known-answer recovery");
   require(integer(elllog(F,gp(R),gp(H),gen(r)))==s,"destination known-answer recovery");
   checked++;
 }
 std::string trailing;require(!(rows>>trailing),"trailing prepared data");
 std::cout<<"{\"scope\":\"tiny public map and known-answer recovery controls\",\"degree\":\""<<ell<<"\",\"target_maps_checked\":"<<checked<<",\"generator_maps_checked\":1,\"exceptional_maps_checked\":2,\"source_recoveries\":"<<checked<<",\"destination_recoveries\":"<<checked<<",\"runtime_measurements\":null}\n";
 pari_close();flint_cleanup();return 0;
}catch(const std::exception&e){std::cerr<<"check-transport: "<<e.what()<<'\n';return 2;}}
