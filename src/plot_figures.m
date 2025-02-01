degrees = readtable('output/hw1_degree_data.csv');
dists = readtable('output/hw1_distance_data.csv');

mean_k=table2array(degrees(:,2));
mean_k2=table2array(degrees(:,3));
edges = table2array(degrees(:,4));
nodes = table2array(degrees(:,5));
labels=convertCharsToStrings(table2array(degrees(:,1)));


mgd=table2array(dists(:,2));
l_max=table2array(dists(:,3));
lcc=table2array(dists(:,4));
labels2=convertCharsToStrings(table2array(dists(:,1)));

mnd=mean_k2./mean_k;
mnd_ratio = mnd./mean_k;

figure(1);
h1 = histogram(mean_k,'BinWidth',3);
xlabel('Mean degree');
ylabel('Number of Universities');
title('Mean Degree Histogram');

figure(2);
p2 = scatter(mean_k,mnd_ratio);
partclabels = ["Reed98","Berkeley13","Colgate88","Mississippi66","Virginia63"];
for i = 1:length(labels)
    keep = 0;
    for filter = partclabels
        if strcmp(filter,labels(i))
            keep=1;
        end
    end
    if ~keep
        labels(i)="";
    end
end
text(mean_k,mnd_ratio, labels, 'Vert','bottom', 'Horiz','left', 'FontSize',7);
yline(1,'Color','red');
xlabel('Mean Degree');
ylabel('MND/Mean Degree ratio');
title('MND-Mean degree ratio vs Mean Degree');
legend('facebook100 data');


figure(3);
p3 = scatter(nodes,l_max);
xlabel('Network size');
ylabel('l_{max}');
title('Diameter vs Network size');
legend('facebook100 data');
ylim([4 12])

figure(4);
p4 = scatter(lcc,mgd);
xlabel('Size of Largest Component');
ylabel('Mean Geodesic Distance');
title('MGD vs LCC size');
legend('facebook100 data');
ylim([2 3.5])


figure(5);
p5 = scatter(lcc,mgd);
xlabel('Size of Largest Component');
ylabel('Mean Geodesic Distance');
title('MGD vs LCC size');
legend('facebook100 data');
set(gca,'xscale','log');
ylim([1.5 3.5])
