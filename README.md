# Graph Force

A python/rust library for embedding graphs in 2D space, using force-directed layouts.

## Installation

```bash
pip install graph_force
```

## Usage

The first parameter defines the number of nodes in graph.
The second parameter is an iterable of edges, where each edge is a tuple of two integers representing the nodes it connects. Node ids start at 0. 


```python
import graph_force

edges = [(0, 1), (1, 2), (2, 3), (3, 0)]
pos = graph_force.layout_from_edge_list(4, edges)
```

### Example with networkx

This library does not have a function to consume a networkx graph directly, but it is easy to convert it to an edge list.

```python
import networkx as nx
import graph_force

G = nx.grid_2d_graph(10, 10)
# we have to map the names to integers
# as graph_force only supports integers as node ids at the moment
edges = []
mapping = {n: i for i, n in enumerate(G.nodes)}
i = 0
for edge in G.edges:
    edges.append((mapping[edge[0]], mapping[edge[1]]))

pos = graph_force.layout_from_edge_list(len(G.nodes), edges, iter=1000)
nx.draw(G, {n: pos[i] for n, i in mapping.items()}, node_size=2, width=0.1)
```

### Example with edge file

This methods can be used with large graphs, where the edge list does not fit into memory.

Format of the file:
- Little endian
- 4 bytes: number of nodes(int)
- 12 bytes: nodeA(int), nodeB(int), weight(float)

```python
import graph_force
import struct

with open("edges.bin", "rb") as f:
    n = 10
    f.write(struct.pack("i", n))
    for x in range(n-1):
        f.write(struct.pack("iif", x, x+1, 1))

pos = graph_force.layout_from_edge_file("edges.bin", iter=50)
```


### Options

`iter`, `threads` and `model` are optional parameters, supported by `layout_from_edge_list` and `layout_from_edge_file`.

```python
pos = graph_force.layout_from_edge_list(
    number_of_nodes,
    edges,
    iter=500,  # number of iterations, default 500
    threads=0,  # number of threads, default 0 (all available)
    model="spring_model", # model to use, default "spring_model", other option is "networkx_model"
)
```
#### Available models

- `spring_model`: A simple spring model (my own implementation)
- `networkx_model`: Reimplementation of the [spring model from networkx](https://networkx.org/documentation/stable/reference/generated/networkx.drawing.layout.spring_layout.html)

## Contributing

- [Development](DEVELOPMENT.md)