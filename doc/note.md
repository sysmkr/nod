# Introduction

The project (nod) name refers to the exisiting english word, meaning 'short, quick and even sometimes
discret acknowledgment'.

The idea is to build a **Raft distributed concensus engine**, in which an undetermined number of nodes
works together towards a goal. A node is an entity, such as a physical/virtual machine or a container, 
able to send/recieve data, and handle it. It can be storing, computing, and more...

A group of node constitute a *network*, where they can interact with each other, as every computer and
phone nowdays. In this project, a more approriate term would be a *cluster*, which differs from a
simple *network* because nodes work together and has defined roles, instead of simply interacting.
In short, a *network* is a crowd, a *cluster* is a team.

Pairing nodes as in a cluster enables a different range/scale of capapilities. The limit is no longer the
power of a single node, but rather the sum of each. It also eliminate single-point failure problems: in 
the case where a node is in charge of absolutely everything, if it fails, everything is down. In a cluster,
nodes can rely on each other, if one fails, the others will close the gap, and everything will still work
as intended.

(Rework everything bellow)

A *concensus engine* mentioned earlier encompass everything related to communication, role attribution
and workload distribution. The **Raft consensus algorithm** is used here to ensure all nodes agree on the 
same state transitions. It operates by electing a leader, replicating logs, and committing entries only
when a majority of nodes confirm storage. This approach guarantees fault tolerance—up to (N-1)/2 nodes 
can fail without disrupting the system—and provides strong consistency. Raft simplifies distributed 
coordination by centralizing decision-making in the leader, who handles client requests and synchronizes 
logs across followers. The algorithm is designed for clarity, reliability, and scalability, making it suitable 
for systems requiring coordinated, fault-tolerant operation.
