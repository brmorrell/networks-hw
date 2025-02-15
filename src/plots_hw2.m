medici_raw = readtable('output/hw2_medici_data.csv');
medici_raw_rand = readtable('output/hw2_medici_dist.csv');
three_nets_raw = readtable('output/hw2_p4_data.csv');
wstrogatz_raw = readtable('output/hw2_ws_data.csv');
berkeley_raw = readtable('output/hw2_berkeley_data.csv');


three_nets_labels = convertCharsToStrings(table2array(three_nets_raw(:,1)));
three_nets_data = table2array(three_nets_raw(:,2:3));

ind1 = three_nets_labels == "sanak_intertidal";
ind2 = three_nets_labels == "bfmaier";
ind3 = three_nets_labels == "cat_brain";

sanak_data = three_nets_data(ind1,:);
bfmaier = three_nets_data(ind2,:);
cat_brain = three_nets_data(ind3,:);

figure(1);
sanak_plot_c = histogram(sanak_data(2:1001,1));
xlabel('Clustering Coefficient');
ylabel('Frequency');
title('Sanak Intertidal Food Web (Configuration Model Distribution)');
xline(sanak_data(1,1),'Color','red');

figure(2);
sanak_plot_l = histogram(sanak_data(2:1001,2));
xlabel('Mean Path Length');
ylabel('Frequency');
title('Sanak Intertidal Food Web (Configuration Model Distribution)');
xline(sanak_data(1,2),'Color','red');

figure(3);
bfm_plot_c = histogram(bfmaier(2:1001,1), 'BinWidth',0.001);
xlabel('Clustering Coefficient');
ylabel('Frequency');
title('BFMaier Facebook Network (Configuration Model Distribution)');
xline(bfmaier(1,1),'Color','red');

figure(4);
bfm_plot_l = histogram(bfmaier(2:1001,2));
xlabel('Mean Path Length');
ylabel('Frequency');
title('BFMaier Facebook Network (Configuration Model Distribution)');
xline(bfmaier(1,2),'Color','red');

figure(5);
cat_plot_c = histogram(cat_brain(2:1001,1));
xlabel('Clustering Coefficient');
ylabel('Frequency');
title('Cat Brain Network (Configuration Model Distribution)');
xline(cat_brain(1,1),'Color','red');

figure(6);
cat_plot_l = histogram(cat_brain(2:1001,2));
xlabel('Mean Path Length');
ylabel('Frequency');
title('Cat Brain Network (Configuration Model Distribution)');
xline(cat_brain(1,2),'Color','red');


medici_labels = convertCharsToStrings(table2array(medici_raw(:,1)));
medici_centrality = table2array(medici_raw(:,2));
medici_rand_draws = table2array(medici_raw_rand(:,2:1001));


figure(7);
h1 = histogram(medici_centrality, 'BinWidth',0.05);
xline(medici_centrality(8),'Color','red');
xline(medici_centrality(11),'Color','red');
text(medici_centrality(8),4, "Medici", 'Vert','top', 'Horiz','left', 'FontSize',12,'Rotation',60);
text(medici_centrality(11),4, "Guadagni", 'Vert','top', 'Horiz','left', 'FontSize',12,'Rotation',60);
xlabel('Harmonic Centrality');
ylabel('Frequency');
title('Padgett Florentine Families Centrality');

figure(8);
medici_cent_dists = boxplot((medici_rand_draws-medici_centrality)');
hold on;
yline(0,'Color','black');
xlabel('Family');
ylabel('Harmonic Centrality (Random - Observed)');
title('Padgett Florentine Families Centrality');



ws_ps = table2array(wstrogatz_raw(:,2));
ws_data = table2array(wstrogatz_raw(:,3:102));

for i = [9 11 12 14 15 30 70 109]
    figure(i);
    hn=histogram(ws_data(i-8,:),20);
    xlim([0,300]);
    xlabel('Centrality');
    ylabel('Frequency');
    legend(strcat('p=',num2str((i-9)/100)))
    title('Betweenness Centrality');
end
%}


berkeley_labels = convertCharsToStrings(table2array(berkeley_raw(:,1)));
berkeley_data = table2array(berkeley_raw(:,2:4));

ind1 = berkeley_labels == "Berkeley13";

m=852444/2;

berkeley_only = berkeley_data(ind1,:);

figure(200);
p4 = scatter(berkeley_only(:,3)/m,berkeley_only(:,1), 'b.');
set(gca,'xscale','log');
xlabel('Swaps performed (units of m)');
ylabel('Clustering Coefficient');
legend('Berkeley C');
title('Berkeley Clustering Coeficient');

figure(201);
p5 = scatter(berkeley_only(:,3)/m,berkeley_only(:,2), 'b.');
set(gca,'xscale','log');
xlabel('Swaps performed (units of m)');
ylabel('Mean path length');
legend('Berkeley l');
title('Berkeley Mean Path Length');