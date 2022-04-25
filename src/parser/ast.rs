use crate::lexer::Symbol;

pub struct ASTNode {
    sym: Symbol,
    child: Vec<Box<ASTNode> >,
}

impl ASTNode {
    fn print_tree_dfs(&self, depth: usize, vis: &mut [bool]) {
        // TODO: if self == NULL return;
        for i in 0..depth {
            if i == depth - 1 {
                print!("+---");
            } else {
                if vis[i] {
                    print!("|   ");
                } else {
                    print!("    ");
                }
            }
        }
        println!("{:?}", self.sym);
        vis[depth + 1] = true;
        // TODO:
        // self.child.iter().map(|c| {
        //     c.print_tree_dfs(depth + 1, vis);
        // });
        vis[depth + 1] = false;

    }
    pub fn print_tree(&self) {

    }
}
