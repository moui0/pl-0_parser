#[derive(Debug)]
pub struct ASTNode {
    node_type: String,
    pub child: Vec<Box<ASTNode> >,
}

impl ASTNode {
    pub fn new(node_type: String) -> Self {
        ASTNode { 
            node_type,
            child: Vec::new(), 
        }
    }
    fn print_ast_dfs(&self, depth: usize, vis: &mut [bool], idx: usize, cnt: usize) {
        for i in 0..depth {
            if i == depth - 1 {
                print!("└───");
            } else {
                if vis[i] {
                    print!("│   ");
                } else {
                    print!("    ");
                }
            }
        }
        println!("{}", self.node_type);
        let mut flag = false;
        if idx == cnt - 1 && depth > 0 {
            vis[depth-1] = false;
            flag = true;
        }
        vis[depth + 1] = true;
        for (i, c) in self.child.iter().enumerate() {
            c.print_ast_dfs(depth + 1, vis, i, self.child.len());
        }
        vis[depth + 1] = false;
        if flag {
            vis[depth-1] = flag;
        }
    }
    pub fn print_ast(&self) {
        let mut vis = [false; 512];
        vis[0] = true;
        self.print_ast_dfs(0, &mut vis, 0, 1);
    }
}
