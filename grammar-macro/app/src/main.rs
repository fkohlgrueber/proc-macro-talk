use grammar_macro::grammar;

grammar!{
    Expr = If(Expr, BlockType, Expr?)
         | Block_(BlockType)

    BlockType = Block(Stmt*)

    Lit = Char(char)
        | Bool(bool)

    Stmt = Expr(Expr)
         | Semi(Expr)
}


grammar!{
    Expr2 = Wohoo(&'static str*) 
          | Blub(std::collections::HashMap<String, usize>?)
}

fn main() {
}
