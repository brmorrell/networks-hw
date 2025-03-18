partb_raw = readtable('output/hw4_onephase_ls.csv');
partc_raw = readtable('output/hw4_samplels.csv');
partd_raw = readtable('output/hw4_ls_list.csv');


partb = table2array(partb_raw(1,2:end));

figure(1);
plot1 = plot(partb);
xlabel('Time Step');
ylabel('Log-Likelihood');
title('Log-Likelihood over One Phase');

partc = table2array(partc_raw(1,2:end));
n1 = 9;
p1 = length(partc)/(n1+1);
partc_phases = reshape(partc,n1+1,p1)';

figure(2);
hold on;
for i = 0:(p1-1)
    plot((n1*i):(n1*i+n1),partc_phases(i+1,:),"Color",'b');
    xline(n1*i);
end
xlabel('Time Step');
ylabel('Log-Likelihood');
title('Fit DC-SBM to sample graph');

partd = table2array(partd_raw(1,2:end));
n2 = 34;
p2 = length(partd)/(n2+1);
partd_phases = reshape(partd,n2+1,p2)';

figure(3);
hold on;
for i = 0:(p2-1)
    plot((n2*i):(n2*i+n2),partd_phases(i+1,:),"Color",'b');
    xline(n2*i);
end
xline(n2*p2);
xlabel('Time Step');
ylabel('Log-Likelihood');
title('Fit DC-SBM to Karate Club Graph');
