# Introduction

The project (nod) name refers to the exisiting english word, meaning 'short, quick and even sometimes
discret acknowledgment'.

The idea is to build a **Raft distributed concensus engine**, in which an undetermined number of nodes
works together towards a goal. A *node* is an entity, such as a physical/virtual machine or a container, 
able to send/recieve data, and handle it. It can be storing, computing, and more...

A group of node constitute a *network*, where they can interact with each other, as every computer and
phone nowdays. In this case, a more approriate term would be a *cluster*, which differs from a
simple network by making nodes work together and have defined roles, instead of simply interacting.
In short, a network is a crowd, a cluster is a team.

Pairing nodes as in a cluster enables a different range/scale of capapilities, where thier computing/storage
capacity is shared/combined. It enables compounded computing power by dividing/dispathcing tasks
in smaller ones to multiple nodes, and increase storage safety by creating smart-repartition of stored data
to never lose anything.
