p=43;r=11;w=ffgen(Mod(1,p)*(t^2+1),'w);E=ellinit([w^0,0*w]);P=[4*w^0,5*w^0];Q=[-P[1],w*P[2]];
g=elltatepairing(E,P,Q,r)^168;if(g==1 || g^r!=1,error("degenerate pairing"));
targets=[[4,5],[13,24],[14,36],[23,35],[31,18],[31,25],[23,8],[14,7],[13,19],[4,38]];logs=List();
for(j=1,#targets,T=[targets[j][1]*w^0,targets[j][2]*w^0];v=elltatepairing(E,T,Q,r)^168;k=fflog(v,g,r);if(ellmul(E,P,k)!=T,error("wrong source logarithm"));listput(logs,k));
jstr(z)=Str(Strchr(34),z,Strchr(34));
print("{",jstr("recovered_logs"),":",Vec(logs),",",jstr("nondegenerate_order"),":11,",jstr("source_logarithms_checked"),":10}");quit;
