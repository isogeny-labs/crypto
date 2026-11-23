require 'open3'
rows=File.readlines(ARGV.fetch(0)).map{|s|s.split.map(&:to_i)}
raise unless rows.size==8 && rows.map{|r|r.take(2)}.uniq.size==8
orig=File.read('variants/g3/kernel.cpp').split('extern "C" int quartic').first
fused,status=Open3.capture2('ruby','harness/g3-cubic-pencil.rb')
raise unless status.success?
puts orig.lines.select{|line|line.start_with?('#include')}.uniq
puts 'namespace control {'
puts orig
puts '}'
puts fused.split('int go(int p,int*y').first
puts '}'
puts 'static int coefficient,active;extern "C" void set_curve(int c){coefficient=c;}'
puts 'extern "C" void set_arm(int i,bool audit){active=i;fused::dual=i==2;fused::validation=audit;}'
puts 'extern "C" int quartic(int p,int*y,int*v,int,unsigned long long*i,int*r,unsigned long long*x,int*n){int a=-1;switch(p*512+coefficient){'
rows.each do |p,c,l,_|
 args="run<#{p},#{c},#{l},true,true,true>(nullptr,y,v,&a,true,i,r,x,n)"
 puts "case #{p*512+c}: if(active){fused::#{args};}else{control::#{args};}break;"
end
puts '}return a;}'
puts <<'CPP'
extern "C" int audit_catalogue(int p,int c,int l){
  set_curve(c);
  int ref[383]{},base=-1,success=0;
  for(int mode=0;mode<3;mode++){
    set_arm(mode,true);
    fused::last_poles=0;fused::used_rows[0]=fused::used_rows[1]=0;
    int y[383]{},v[383]{},rt[383]{},len=0;unsigned long long inv[383]{},xs[383]{};
    int a=quartic(p,y,v,1,inv,rt,xs,&len);
    if(a>=0){
      if(base<0){base=a;std::copy_n(v,p,ref);}
      assert(v[base] && ref[a]);
      for(int j=0;j<p;j++)assert(1LL*v[j]*ref[base]%l==1LL*ref[j]*v[base]%l);
      success|=1<<mode;
    }
    printf("CAT %d %d %d %d %d %d %d\n",p,c,mode,a>=0,fused::last_poles,fused::used_rows[0],fused::used_rows[1]);
  }
  return success;
}
CPP
