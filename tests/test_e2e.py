import graph_force
import struct

def test_list_of_edges():
    edges = [(1, 2), (2, 3), (3, 4), (4, 5), (5, 6)]
    pos = graph_force.layout_from_edge_list(7, edges)
    assert pos is not None
    assert len(pos) == 7


def test_iterator_of_edges():
    pos = graph_force.layout_from_edge_list(
        7,
        ((0, i + 1) for i in range(6))
    )
    assert pos is not None
    assert len(pos) == 7

def test_tuple_of_edges():
    pos = graph_force.layout_from_edge_list(
        7,
        ((0,1), (1,2), (2,3), (3,4), (4,5), (5,6))
    )
    assert pos is not None
    assert len(pos) == 7

def test_model_selection():
    edges = [(1, 2), (2, 3), (3, 4), (4, 5), (5, 6)]
    pos = graph_force.layout_from_edge_list(7, edges, model='networkx_model')
    assert pos is not None
    assert len(pos) == 7

def test_from_file():
    with open("/tmp/edges.bin", "wb") as f:
        n = 10
        f.write(struct.pack("i", n))
        for x in range(n-1):
            f.write(struct.pack("iif", x, x+1, 1))

    pos = graph_force.layout_from_edge_file('/tmp/edges.bin')
    assert pos is not None
    assert len(pos) == 10