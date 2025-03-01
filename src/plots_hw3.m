p1_raw = readtable('output/hw3_p1.csv');
p2_raw = readtable('output/hw3_p2.csv');
jroc_raw = readtable('output/hw3_jroc.csv');
droc_raw = readtable('output/hw3_droc.csv');
sroc_raw = readtable('output/hw3_sroc.csv');



p1_labels = convertCharsToStrings(table2array(p1_raw(:,1)));
p1_data = table2array(p1_raw(:,2:3));

ind1 = p1_labels == "HVR_5";
ind2 = p1_labels == "nor";

hvr_acc = p1_data(ind1,:);
nor_acc = p1_data(ind2,:);

hvr_avg = splitapply(@mean,hvr_acc(:,2),findgroups(hvr_acc(:,1)));
nor_avg = splitapply(@mean,nor_acc(:,2),findgroups(nor_acc(:,1)));


figure(1);
plot1 = plot((0:49)*0.02,hvr_avg);
hold on;
plot2 = plot((0:49)*0.02,nor_avg);
xlabel('Fraction of Attributes Observed');
ylabel('Accuracy');
title('Local Smoothing of Node Atributes');
yline(1/6, 'Color','k');
yline(0.5, 'Color','g');
legend('HVR\_5','Directors','HVR\_5 baseline','Directors baseline','Location','northeastoutside');


p2_labels = convertCharsToStrings(table2array(p2_raw(:,1)));
p2_data = table2array(p2_raw(:,2:5));

ind12 = p2_labels == "HVR_5";
ind22 = p2_labels == "nor";

hvr_auc = p2_data(ind12,:);
nor_auc = p2_data(ind22,:);

hvr_auc_avg = splitapply(@mean,hvr_auc(:,2:4),findgroups(hvr_auc(:,1)));
nor_auc_avg = splitapply(@mean,nor_auc(:,2:4),findgroups(nor_auc(:,1)));

figure(2);
plot3 = plot((0:19)*0.05,hvr_auc_avg);
yline(0.5);
xlabel('Fraction of Edges Observed');
ylabel('AUC');
title('Edge Prediction Accuracy (HVR\_5)');
legend('Jaccard','Degree Product', 'Shortest Path','Location','northeastoutside');

figure(3);
plot4 = plot((0:19)*0.05,nor_auc_avg);
yline(0.5);
xlabel('Fraction of Edges Observed');
ylabel('AUC');
title('Edge Prediction Accuracy (Directors)');
legend('Jaccard','Degree Product', 'Shortest Path','Location','northeastoutside');

p3_labels = convertCharsToStrings(table2array(jroc_raw(:,1)));
p3_data = table2array(jroc_raw(:,2:3));

ind13 = p3_labels == "HVR_5";
ind23 = p3_labels == "nor";

hvr_jroc = p3_data(ind13,:);
nor_jroc = p3_data(ind23,:);

p4_labels = convertCharsToStrings(table2array(droc_raw(:,1)));
p4_data = table2array(droc_raw(:,2:3));

ind14 = p4_labels == "HVR_5";
ind24 = p4_labels == "nor";

hvr_droc = p4_data(ind14,:);
nor_droc = p4_data(ind24,:);

p5_labels = convertCharsToStrings(table2array(sroc_raw(:,1)));
p5_data = table2array(sroc_raw(:,2:3));

ind15 = p5_labels == "HVR_5";
ind25 = p5_labels == "nor";

hvr_sroc = p5_data(ind15,:);
nor_sroc = p5_data(ind25,:);

figure(4);
hold on;
plot8 = plot(hvr_jroc(:,2),hvr_jroc(:,1));
plot9 = plot(hvr_droc(:,2),hvr_droc(:,1));
plot0 = plot(hvr_sroc(:,2),hvr_sroc(:,1));
plot(xlim,ylim,'Color','k','LineStyle','--');
xlabel('FPR');
ylabel('TPR');
title('ROC (HVR\_5)');
legend('Jaccard','Degree Product', 'Shortest Path', 'Baseline','Location','northeastoutside');

figure(5);
hold on;
plot5 = plot(nor_jroc(:,2),nor_jroc(:,1));
plot6 = plot(nor_droc(:,2),nor_droc(:,1));
plot7 = plot(nor_sroc(:,2),nor_sroc(:,1));
plot(xlim,ylim,'Color','k','LineStyle','--');
xlabel('FPR');
ylabel('TPR');
title('ROC (Directors)');
legend('Jaccard','Degree Product', 'Shortest Path', 'Baseline','Location','northeastoutside');

