import networkx as nx
import numpy as np
import matplotlib
import matplotlib.pylab as plt
#matplotlib inline
import random as rnd
rnd.seed()
import copy
import csv

#blatantly stole the draw function from the in-class lab because I'm lazy
def drawGz(G,z):
    # DO NOT MODIFY THIS FUNCTION
    # This function draws G with node labels from partition z
    #
    # input  : G is a networkx graph
    #        : z is a dictionary of group labels for G's nodes
    # output : none
    # 
    # WARNING: function is optimistic: assumes inputs are properly formatted

    colors = ['#d61111','#11d646','#11c6d6','#d67711','#1b11d6','#d611cc'] # map node labels to colors (for the visualization)
    
    node_colors = []
    for i in G.nodes():
        node_colors.append(colors[z[i]])
    nsize  = 600
    flabel = True

    if G.order() > 50:
        nsize  = 100
        flabel = False
    
    nx.draw_kamada_kawai(G,with_labels=flabel,node_size=nsize,width=2,node_color=node_colors) # draw it prettier
    #nx.draw_networkx(G,with_labels=flabel,node_size=nsize,width=2,node_color=node_colors) # draw it pretty
    limits=plt.axis('off')                                      # turn off axes
    plt.show() 

    return

G  = nx.read_edgelist("data/ps4.txt")

n  = G.order()
z1 = {}
z2 = {}
with open('output/hw4_onemove.csv', mode ='r')as file:
    csvFile = csv.reader(file)
    for line in csvFile:
        if line[0] == "Node":
            z1[line[2]] = int(line[3])
            z2[line[2]] = int(line[3])
        else:
            z2[line[2]] = int(line[3])

drawGz(G,z1)
drawGz(G,z2)

z3 = {}
z4 = {}
with open('output/hw4_onephase.csv', mode ='r')as file:
    csvFile = csv.reader(file)
    for line in csvFile:
        z3[line[1]] = int(line[2])
        z4[line[1]] = int(line[3])

drawGz(G,z3)
drawGz(G,z4)

z5 = {}
with open('output/hw4_samplegraph.csv', mode ='r')as file:
    csvFile = csv.reader(file)
    for line in csvFile:
        z5[line[1]] = int(line[2])

drawGz(G,z5)

G2 = nx.read_edgelist("data/zkc.txt")

n2  = G2.order()
z6 = {}
with open('output/hw4_partition_graph.csv', mode ='r')as file:
    csvFile = csv.reader(file)
    for line in csvFile:
        z6[line[2]] = int(line[3])

drawGz(G2,z6)
