type Id = u32;

#[derive(Debug)]
pub struct Node<T> {
    pub id: Id,
    pub value: T,
}

impl<T> Node<T>
where T: std::fmt::Display + std::str::FromStr
{
    pub fn new(id: Id, value: T) -> Self {
        Self{id, value}
    }
    
    pub fn ser(&self) -> String {
        format!("{} {}", self.id, self.value)
    }

    pub fn deser(s: &str) -> Self {
        let spl = s.split(' ').collect::<Vec<&str>>();
        if spl.len() != 2 {
            panic!("parsing error of \"{}\"", s);
        } else {
            let id = spl[0].parse().expect(&format!("wrong id in \"{}\"", s).to_owned());
            match spl[1].parse() {
                Ok(value) => Self {id, value},
                Err(_) => panic!("can't parse value in \"{}\"", s)
            }
        }
    }
}

#[derive(Debug)]
pub struct Edge {
    begin: Id,
    end: Id,
}

impl Edge {
    pub fn new(begin: Id, end: Id) -> Self {
        Self{begin, end}
    }
    
    pub fn ser(&self) -> String {
        format!("{} {}", self.begin, self.end)
    }

    pub fn deser(s: &str) -> Self {
        let spl = s.split(' ').collect::<Vec::<&str>>();
        if spl.len() != 2 {
            panic!("parsing error of \"{}\"", s);
        }
        let err = format!("parsing error in \"{}\"", s);
        Self {begin: spl[0].parse().expect(&err), end: spl[1].parse().expect(&err)}
    }
}

#[derive(Debug)]
pub struct Graph<T> 
{
    nodes: Vec<Node<T>>,
    edges: Vec<Edge>,
}

impl<T> Graph<T>
where T: std::fmt::Display + std::str::FromStr
{
    pub fn new() -> Self {
        Self {nodes: vec![], edges: vec![]}
    }

    pub fn add_node_from(mut self, node: Node<T>) -> Self {
        if self.nodes.iter().any(|n| n.id == node.id) {
            panic!("duplicate node with id {}", node.id);
        }
        self.nodes.push(node);
        self
    }

    pub fn add_node(self, id: Id, value: T) -> Self {
        self.add_node_from(Node::new(id, value))
    }

    pub fn remove_node_by_id(mut self, node_id: Id) -> Self {
        self.edges.retain(|e| !(node_id == e.begin || node_id == e.end));
        self.nodes.retain(|n| n.id != node_id);
        self
    }

    pub fn add_edge_from(mut self, edge: Edge) -> Self {
        if let None = self.nodes.iter().find(|n| edge.begin == n.id).and(self.nodes.iter().find(|n| edge.end == n.id)) {
            panic!("adding edge between non-existent nodes ({}->{})", edge.begin, edge.end);
        }
        if !self.edges.iter().any(|e| e.begin == edge.begin && e.end == edge.end) {
            self.edges.push(edge);
        }
        self
    }

    pub fn add_edge(self, begin: Id, end: Id) -> Self {
        self.add_edge_from(Edge::new(begin, end))
    }

    pub fn remove_edge(mut self, edge: &Edge) -> Self {
        self.edges.retain(|e| e.begin != edge.begin && e.end == edge.end);
        self
    }

    pub fn ser(&self) -> String {
        let mut res = String::new();

        for node in &self.nodes {
            res.push_str(&node.ser());
            res.push('\n');
        }
        res.push_str("#\n");
        for edge in &self.edges {
            res.push_str(&edge.ser());
            res.push('\n');
        }
        res
    }

    pub fn deser(s: &str) -> Self {
        let mut res = Self::new();
        let mut b: bool = true; // now parsing nodes

        for line in s.lines() {
            if line == "#" {
                b = false;
                continue;
            }
            if b {
                res = res.add_node_from(Node::deser(&line));
            } else {
                res = res.add_edge_from(Edge::deser(&line));
            }
        }
        
        res
    }

    pub fn get_connected<'a>(&'a self, node: &'a Node<T>) -> Vec::<&'a Node<T>> {
        let mut res = vec![];

        for edge in &self.edges {
            if edge.begin == node.id {
                if let Some(n) = self.nodes.iter().find(|nn| nn.id == edge.end) {
                    res.push(n);
                }
            }
        }
        
        res
    }
    
    /// Depth-first search
    pub fn traverse_from<F>(&self, root: &Node<T>, f: &mut F)
    where F: FnMut(&Node<T>)
    {
        self.traverse_from_with_memory(root, f, &vec![]);
    }

    /// DFS with cycles safety
    fn traverse_from_with_memory<'a, F>(&'a self, root: &'a Node<T>, f: &mut F, memory: &'a Vec<&'a Node<T>>)
    where F: FnMut(&Node<T>)
    {
        if let Some(_) = memory.iter().find(|n| n.id == root.id) {
            return
        }
        f(root);
        let mut memory = memory.clone();
        memory.push(root);
        let nexts = self.get_connected(root);
        for next in nexts {
            self.traverse_from_with_memory(next, f, &memory);
        }
    }

    pub fn get_all_nodes(&self) -> Vec::<&Node<T>> {
        self.nodes.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Graph, Node};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn ser() {
        let g = Graph::<u32>::new()
            .add_node(1, 1)
            .add_node(2, 2)
            .add_node(3, 3)
            .add_node(4, 4)
            .add_node(5, 5)
            .add_edge(1,2)
            .add_edge(1,4)
            .add_edge(2,3)
            .add_edge(5,3)
            ;
        let s = g.ser();
        assert_eq!(s, r#"1 1
2 2
3 3
4 4
5 5
#
1 2
1 4
2 3
5 3
"#);
        let g = g.remove_node_by_id(5);
        let s = g.ser();
        assert_eq!(s, r#"1 1
2 2
3 3
4 4
#
1 2
1 4
2 3
"#);
    }

    #[test]
    fn deser() {
        let s = r#"1 123
2 321
#
1 2"#;
        let g = Graph::<u32>::deser(s);
        assert!(g.get_all_nodes()[0].id == 1);
        assert!(g.get_all_nodes()[0].value == 123);
        assert!(g.get_all_nodes()[1].id == 2);
        assert!(g.get_all_nodes()[1].value == 321);
        assert!(g.get_connected(g.get_all_nodes()[0])[0].id == 2);
        assert!(g.get_connected(g.get_all_nodes()[0])[0].value == 321);
        assert!(g.get_connected(g.get_all_nodes()[1]).is_empty());
    }

    #[test]
    fn traverse() {
        let g = Graph::<u32>::new()
            .add_node(1, 1)
            .add_node(2, 2)
            .add_node(3, 3)
            .add_node(4, 4)
            .add_node(5, 5)
            .add_edge(1, 2)
            .add_edge(2, 1)
            .add_edge(1, 3)
            .add_edge(3, 4)
            .add_edge(4, 3)
            .add_edge(4, 5)
            ;
        let mut q = vec![];
        g.traverse_from(g.get_all_nodes()[0], & mut |node: &Node::<u32>| {
            q.push(node.id);
        });
        for n_id in q {
            if let None = g.get_all_nodes().iter().find(|n| n.id == n_id) {
                panic!();
            }
        }
    }

    #[test]
    fn traverse_cycle_safety() {
        let g = Graph::<u32>::new()
            .add_node(1, 1)
            .add_node(2, 2) // 1---\
            .add_edge(1, 2) // \    \
            .add_edge(2, 1) //  \--- 2
            ;
        g.traverse_from(g.get_all_nodes()[0], &mut |_| {});
    }

    #[test]
    #[should_panic]
    fn panic_on_dupl_node() {
        Graph::<u32>::new()
            .add_node(1, 1)
            .add_node(1, 2);
    }

    #[test]
    #[should_panic]
    fn panic_on_invalid_edge() {
        Graph::<u32>::new()
            .add_node(1, 1)
            .add_node(2, 2)
            .add_edge(1, 2)
            .add_edge(1, 3)
            ;
    }
}
