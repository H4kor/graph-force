# Graph Force

A python/rust library for embedding graphs in 2D space, using force-directed layouts.

## Development

### Setup
```
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

### Build
```
maturin develop
```

## Usage
```python
import graph_force

edges = [(0, 1), (1, 2), (2, 3), (3, 0)]
pos = graph_force.layout_from_edge_list(4, edges)
```