#define main aggregate_fixture_main
#include "local_torsion.cpp"
#undef main

int main(int argc,char** argv) {
    std::cout.setf(std::ios::unitbuf);
    try {
        check(argc==2 && std::string(argv[1]).size()==1 && argv[1][0]>='0'
              && argv[1][0]<='3',"choose public point0..3");
        unsigned index=unsigned(argv[1][0]-'0');
        Model model(Z("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",16),
                    Z("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",16),
                    {0,0,0,0,7});
        Point generator{false,
            Z("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",16),
            Z("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",16)};
        Source source(model);Point input=public_point(model,index);
        check(source.times(generator,model.r).infinity && source.times(input,model.r).infinity,
              "OpenSSL source order");
        check(model.at(model.p)==model.source,"source model projection");
        std::cout<<"{\"kind\":\"public_input\",\"curve\":\"secp256k1\",\"index\":"<<index
                 <<",\"x\":\""<<input.x<<"\",\"y\":\""<<input.y<<"\"}\n";
        for (unsigned precision : {1U,2U,4U,8U,16U,32U,64U}) {
            Z modulus=power(model.p,precision);Coeff a=model.at(modulus);Group group(a,modulus);
            Lift out=lift(model,input,precision);Jac q=group.from(out.q);
            check(lift(model,{},precision).q.infinity,"identity lift");
            check(group.valid(q) && group.times(q,model.r).z==0,"independent torsion");
            Division div(a,out.q.x,modulus);
            for (unsigned n : {1U,2U,3U,4U}) {
                Z fn=div.f(Z(n)).value,product=div.f(Z(n-1)).value*div.f(Z(n+1)).value;
                Z den=mod(fn*fn,modulus);
                if (n%2==0) den=mod(den*div.D.value,modulus);
                else product=mod(product*div.D.value,modulus);
                check(mod(out.q.x-product*inv(den,modulus),modulus)==
                      group.to(group.times(q,Z(n))).x,"division recurrence/group mismatch");
            }
            unsigned controls=1,relations=0;
            Point negative{false,out.q.x,mod(-out.q.y-a[0]*out.q.x-a[2],modulus)};
            check(mod(negative.y,model.p)!=input.y,"wrong ordinate control accepted");
            if (precision>=2) {
                Division wrong(a,mod(out.q.x+model.p,modulus),modulus);
                check(wrong.f(model.r).value!=0,"perturbed x control accepted");++controls;
            }
            Point bad=input;bad.y=mod(bad.y+1,model.p);
            check(equation(model.source,bad,model.p)!=0,"malformed fixture valid");
            bool rejected=false;try { (void)lift(model,bad,precision); }
            catch(const std::runtime_error&) { rejected=true; }
            check(rejected,"malformed input accepted");++controls;
            if (precision==64) {
                Point negsource{false,input.x,mod(-input.y,model.p)};
                check(equal(lift(model,negsource,precision).q,negative),"negation consistency");
                Lift gp=lift(model,generator,precision);
                check(group.times(group.from(gp.q),model.r).z==0,"generator lift torsion");
                check(equal(lift(model,source.add(input,generator),precision).q,
                            group.to(group.add(q,group.from(gp.q)))),"addition consistency");
                relations=2;
            }
            std::cout<<"{\"kind\":\"public_local_lift\",\"curve\":\"secp256k1\",\"index\":"
                     <<index<<",\"order_bits\":256,\"precision_digits\":"<<precision
                     <<",\"inputs\":1,\"torsion_checks\":1,\"scalar_or_additive_relations\":"
                     <<relations<<",\"controls_rejected\":"<<controls
                     <<",\"recurrence_checks\":4,\"max_recurrence_nodes\":"<<out.nodes
                     <<",\"newton_rounds\":"<<out.rounds<<",\"coordinate_sha256\":\""
                     <<digest(encoded(out.q))<<"\"}\n";
        }
        return 0;
    } catch(const std::exception& e) { std::cerr<<e.what()<<'\n';return 1; }
}
