type Id = u32;

#[derive(Debug)]
pub struct Node<T> {
    id: Id,
    value: T,
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
    pub nodes: Vec<Node<T>>,
    edges: Vec<Edge>,
}

impl<T> Graph<T>
where T: std::fmt::Display + std::str::FromStr
{
    pub fn new() -> Self {
        Self {nodes: vec![], edges: vec![]}
    }

    pub fn add_node(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
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
                res.add_node(Node::deser(&line));
            } else {
                res.add_edge(Edge::deser(&line));
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
    
    pub fn traverse_from(&self, root: &Node<T>, f: &dyn Fn(&Node<T>) -> ()) {
        f(root);
        let nexts = self.get_connected(root);
        for next in nexts {
            self.traverse_from(next, f);
        }
    }

    pub fn get_all_nodes(&self) -> Vec::<&Node<T>> {
        self.nodes.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
