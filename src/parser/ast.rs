#[derive(Debug)]
pub struct ASTNode {
    node_type: String,
    pub child: Vec<Box<ASTNode>>,
}

impl ASTNode {
    pub fn new(node_type: String) -> Self {
        ASTNode {
            node_type,
            child: Vec::new(),
        }
    }
    fn print_ast_dfs(&self, depth: usize, vis: &mut [bool], last_child: bool) {
        for i in 0..depth {
            let prelude = match (i == depth - 1, last_child, vis[i]) {
                (true, true, _) => "└── ",
                (true, false, _) => "├── ",
                (false, _, true) => "│   ",
                (false, _, false) => "    ",
            };
            print!("{}", prelude);
        }
        println!("{}", self.node_type);
        let mut flag = false;
        if last_child && depth > 0 {
            vis[depth - 1] = false;
            flag = true;
        }
        vis[depth + 1] = true;
        for (i, child) in self.child.iter().enumerate() {
            child.print_ast_dfs(depth + 1, vis, i == self.child.len() - 1);
        }
        vis[depth + 1] = false;
        if flag {
            vis[depth - 1] = flag;
        }
    }
    pub fn print_ast(&self) {
        let mut vis = [false; 512];
        vis[0] = true;
        self.print_ast_dfs(0, &mut vis, false);
    }
}
