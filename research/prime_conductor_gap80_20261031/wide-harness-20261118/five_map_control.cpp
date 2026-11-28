// Fixed degree-5 correctness control on the frozen 96-bit pilot field.
// Uses the actual complete native evaluator, not the large pilot/goal kernel.
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
static void point_json(std::ostream&out,const Point&P){require(!P.inf,"finite packet point");out<<"[\""<<P.x<<"\",\""<<P.y<<"\"]";}
int main(int argc,char**argv){try{
 require(argc==2&&!std::filesystem::exists(argv[1]),"fresh PARI-map output path required");
 Z original_pilot_degree,r,N;Point unused_original_kernel,G;unused_original_kernel.inf=G.inf=false;
 require(bool(std::cin>>p>>a>>b>>original_pilot_degree>>unused_original_kernel.x>>unused_original_kernel.y>>r>>G.x>>G.y>>N),"frozen pilot input plus supplied order");
 a=mod(a);b=mod(b);
 require(p==Z("78125000378223625457769953627")&&r==Z("125000000302579")&&N==5*r*r,"fixed pilot family");
 pari_init(256000000,2000000);
 require(isprime(gen(p))&&isprime(gen(r))&&mod(p+1-N)!=0&&on(G,a,b)&&mul(G,r).inf,"source prime/subgroup/ordinary control");
 GEN E=ellinit(mkvec5(gen_0,gen_0,gen_0,gen(a),gen(b)),gen(p),DEFAULTPREC);
 Z clearing=r*r;Point T;unsigned attempts=0;
 for(unsigned x=0;x<64;x++){
   attempts++;GEN ordinates=ellordinate(E,utoi(x),DEFAULTPREC);
   if(lg(ordinates)<=1)continue;
   GEN P=mkvec2(utoi(x),gel(ordinates,1)),candidate=ellmul(E,P,gen(clearing));
   if(lg(candidate)==3){T={integer(gel(candidate,1)),integer(gel(candidate,2)),false};break;}
 }
 require(!T.inf&&on(T,a,b)&&mul(T,5).inf,"bounded order-5 kernel search");
 std::vector<unsigned long> scalars;std::vector<Point> inputs{G};
 for(unsigned i=0;i<32;i++){auto s=shared(r.get_ui(),i);scalars.push_back(s);inputs.push_back(mul(G,Z(s)));}
 inputs.push_back(T);inputs.push_back({});
 fmpz_t prime;fmpz_init(prime);fmpz_set_mpz(prime,p.get_mpz_t());fmpz_mod_ctx_init(ctx,prime);
 Result result=construct(Z(5),T,inputs,4,8,0,7);
 fmpz_mod_ctx_clear(ctx);fmpz_clear(prime);
 GEN iso=ellisogeny(E,gp(T),0,0,1),coefficients=gel(iso,1),map=gel(iso,2);
 require(result.ca==integer(gel(coefficients,4))&&result.cb==integer(gel(coefficients,5))&&result.covered==2,"native/PARI complete quotient");
 for(size_t i=0;i<inputs.size();i++)require(equal(result.images[i],ellisogenyapply(map,gp(inputs[i]))),"native/PARI image");
 GEN F=ellinit(coefficients,gen(p),DEFAULTPREC);Point H=result.images[0];
 require(!H.inf&&on(H,result.ca,result.cb)&&lg(ellmul(F,gp(H),gen(r)))==2,"image generator prime order");
 require(result.images[33].inf&&result.images[34].inf,"exceptional maps");
 Z ia=mod(16*a),ib=mod(64*b);Point I{mod(4*G.x),mod(8*G.y),false};
 GEN C=ellinit(mkvec5(gen_0,gen_0,gen_0,gen(ia),gen(ib)),gen(p),DEFAULTPREC);
 std::ostringstream out;
 out<<"{\"scope\":\"degree-5 full native evaluator control on the frozen 96-bit pilot field\",\"original_pilot_degree\":\""<<original_pilot_degree<<"\",\"goal_minimum_degree_bits\":80,\"control_degree\":5,\"p\":\""<<p<<"\",\"r\":\""<<r<<"\",\"group_order\":\""<<N<<"\",\"ambient_order_status\":\"source supplied; quotient inherited by rational isogeny, not recounted\",\"source\":[\""<<a<<"\",\""<<b<<"\"],\"destination\":[\""<<result.ca<<"\",\""<<result.cb<<"\"],\"coordinate_control\":{\"u\":\"2\",\"a\":\""<<ia<<"\",\"b\":\""<<ib<<"\"},\"kernel\":";point_json(out,T);
 out<<",\"kernel_lift_attempts\":"<<attempts<<",\"generators\":{\"source\":";point_json(out,G);out<<",\"destination\":";point_json(out,H);out<<",\"coordinate_control\":";point_json(out,I);out<<"},\"targets\":[";
 for(unsigned i=0;i<32;i++){
   const Point&Q=inputs[i+1];const Point&R=result.images[i+1];Point J{mod(4*Q.x),mod(8*Q.y),false};
   require(equal(Q,ellmul(E,gp(G),utoi(scalars[i])))&&equal(R,ellmul(F,gp(H),utoi(scalars[i]))),"transported scalar identity");
   require(on(J,ia,ib)&&equal(J,ellmul(C,gp(I),utoi(scalars[i])))&&mod(J.x*inverse(4))==Q.x&&mod(J.y*inverse(8))==Q.y,"coordinate control and inverse");
   if(i)out<<',';out<<"{\"scalar\":\""<<scalars[i]<<"\",\"source\":";point_json(out,Q);out<<",\"destination\":";point_json(out,R);out<<",\"coordinate_control\":";point_json(out,J);out<<"}";
 }
 out<<"],\"native_map_inputs_checked\":35,\"kernel_pairs_covered\":\""<<result.covered<<"\",\"conductor_gap\":null,\"high_fidelity_admitted\":false,\"full_target_constructed\":false,\"runtime_measurements\":null}\n";
 std::ofstream mapfile(argv[1]);require(bool(mapfile),"map output creation");char*description=GENtostr(map);mapfile<<description<<'\n';pari_free(description);mapfile.close();require(bool(mapfile),"map output write");
 std::cout<<out.str();pari_close();flint_cleanup();return 0;
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 2;}}
