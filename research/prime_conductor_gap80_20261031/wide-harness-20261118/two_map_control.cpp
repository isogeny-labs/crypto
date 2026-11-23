// Bounded degree-2 public control on the frozen source field, not the target
// 80-bit-degree construction. Standard normalized Velu formulas, PARI replay.
#define main complete_pipeline_fixture_entry
#include "../native/complete_pipeline.cpp"
#undef main
#include <openssl/sha.h>
#include <filesystem>
#include <fstream>
static unsigned long shared(unsigned long r,unsigned long i){
 std::string bytes="{\"domain\":\"uniform_scalar_sha256_shared_v1\",\"parts\":["+std::to_string(r)+",20261118,"+std::to_string(i)+"]}";
 unsigned char digest[SHA256_DIGEST_LENGTH];SHA256(reinterpret_cast<const unsigned char*>(bytes.data()),bytes.size(),digest);
 uint64_t v=0;for(unsigned j=0;j<8;j++)v=(v<<8)|digest[j];return 1+v%(r-1);
}
static Point phi2(Point P){
 if(P.inf||P.x==mod(-14))return {};
 Z v=inverse(mod(P.x+14));
 return {mod(P.x+448*v),mod(P.y*(1-448*v*v)),false};
}
static void point_json(std::ostream&out,const Point&P){
 require(!P.inf,"finite packet point");out<<"[\""<<P.x<<"\",\""<<P.y<<"\"]";
}
int main(int argc,char**argv){try{
 require(argc==2&&!std::filesystem::exists(argv[1]),"fresh PARI-map output path required");
 Z requested,r,N;Point unused_requested_kernel,G;unused_requested_kernel.inf=G.inf=false;
 require(bool(std::cin>>p>>a>>b>>requested>>unused_requested_kernel.x>>unused_requested_kernel.y>>r>>G.x>>G.y>>N),"frozen source input plus supplied order");
 a=mod(a);b=mod(b);
 require(p==Z("4794168233664648458988367110514316682522236917173341621535751")&&a==mod(-140)&&b==784&&r==Z("1125023250121"),"exact bounded source family");
 pari_init(256000000,2000000);
 require(isprime(gen(p))&&isprime(gen(r))&&mpz_odd_p(r.get_mpz_t())&&N%r==0&&mod(p+1-N)!=0,"source prime/subgroup/ordinary metadata");
 Point T{mod(-14),0,false};
 require(on(T,a,b)&&mul(T,2).inf&&on(G,a,b)&&mul(G,r).inf,"source and kernel control");
 GEN E=ellinit(mkvec5(gen_0,gen_0,gen_0,gen(a),gen(b)),gen(p),DEFAULTPREC);
 GEN iso=ellisogeny(E,gp(T),0,0,1),coefficients=gel(iso,1),map=gel(iso,2);
 Z ca=integer(gel(coefficients,4)),cb=integer(gel(coefficients,5));
 require(ca==mod(-2380)&&cb==44688,"PARI quotient coefficients");
 require(gequal0(gp_read_str("(x^3-140*x+784)*((x+14)^2-448)^2-(x+14)*(x*(x+14)+448)^3+2380*(x+14)^3*(x*(x+14)+448)-44688*(x+14)^4")),"cleared integer curve equation identity");
 GEN F=ellinit(coefficients,gen(p),DEFAULTPREC);
 Point H=phi2(G);
 require(equal(H,ellisogenyapply(map,gp(G)))&&on(H,ca,cb)&&lg(ellmul(F,gp(H),gen(r)))==2,"generator-image and exact prime subgroup control");
 require(equal(phi2(T),ellisogenyapply(map,gp(T)))&&equal(phi2(Point{}),ellisogenyapply(map,gp(Point{}))),"kernel and infinity control");
 Z ia=mod(16*a),ib=mod(64*b);Point I{mod(4*G.x),mod(8*G.y),false};
 GEN C=ellinit(mkvec5(gen_0,gen_0,gen_0,gen(ia),gen(ib)),gen(p),DEFAULTPREC);
 require(on(I,ia,ib)&&lg(ellmul(C,gp(I),gen(r)))==2,"coordinate-control generator");
 std::ostringstream out;
 out<<"{\"scope\":\"degree-2 map/input control on the actual 202-bit source field\",\"requested_target_degree\":\""<<requested<<"\",\"control_degree\":2,\"p\":\""<<p<<"\",\"r\":\""<<r<<"\",\"group_order\":\""<<N<<"\",\"ambient_order_status\":\"source supplied; quotient inherited by rational isogeny, not recounted\",\"source\":[\""<<a<<"\",\""<<b<<"\"],\"destination\":[\""<<ca<<"\",\""<<cb<<"\"],\"coordinate_control\":{\"u\":\"2\",\"a\":\""<<ia<<"\",\"b\":\""<<ib<<"\"},\"kernel\":";point_json(out,T);
 out<<",\"map\":{\"x\":\"x+448/(x+14)\",\"y\":\"y*(1-448/(x+14)^2)\",\"exceptions\":[\"O\",\"(-14,0)\"],\"cleared_integer_equation\":\"0\"},\"generators\":{\"source\":";point_json(out,G);out<<",\"destination\":";point_json(out,H);out<<",\"coordinate_control\":";point_json(out,I);out<<"},\"targets\":[";
 for(unsigned long i=0;i<32;i++){
   auto s=shared(r.get_ui(),i);Point Q=mul(G,Z(s)),R=phi2(Q),J{mod(4*Q.x),mod(8*Q.y),false};
   require(equal(Q,ellmul(E,gp(G),utoi(s)))&&equal(R,ellisogenyapply(map,gp(Q)))&&equal(R,ellmul(F,gp(H),utoi(s)))&&on(R,ca,cb),"native/PARI transported target");
   require(on(J,ia,ib)&&equal(J,ellmul(C,gp(I),utoi(s)))&&mod(J.x*inverse(4))==Q.x&&mod(J.y*inverse(8))==Q.y,"coordinate-control target and inverse");
   if(i)out<<',';out<<"{\"scalar\":\""<<s<<"\",\"source\":";point_json(out,Q);out<<",\"destination\":";point_json(out,R);out<<",\"coordinate_control\":";point_json(out,J);out<<"}";
 }
 out<<"],\"native_map_inputs_checked\":35,\"conductor_gap\":null,\"high_fidelity_admitted\":false,\"full_target_constructed\":false,\"runtime_measurements\":null}\n";
 std::ofstream mapfile(argv[1]);require(bool(mapfile),"map output creation");char*description=GENtostr(map);mapfile<<description<<'\n';pari_free(description);mapfile.close();require(bool(mapfile),"map output write");
 std::cout<<out.str();pari_close();flint_cleanup();return 0;
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 2;}}
