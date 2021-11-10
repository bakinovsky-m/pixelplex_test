use mygraphlib::{Graph,Node,Edge};
fn main() {
    let mut g = Graph::<u32>::new();
    g.add_node(Node::<u32>::new(1, 1));
    g.add_node(Node::<u32>::new(2, 2));
    g.add_node(Node::<u32>::new(3, 3));
    g.add_node(Node::<u32>::new(4, 4));
    g.add_edge(Edge::new(1,2));
    g.add_edge(Edge::new(1,4));
    g.add_edge(Edge::new(2,3));
    println!("{}", g.ser());
    g.traverse_from(&g.nodes[0], &|node| {
        println!("{:?}", node);
        for nbr in g.get_connected(node) {
            println!("\t{:?}", nbr);
        }
    });

    let s = "1 123\n2 321\n#\n1 2";
    let g = Graph::<u32>::deser(s);
    println!("{:?}", g);
}
