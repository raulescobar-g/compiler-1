

"start" -> { "fn main() -> i32 {", "body", "}" };
"body" -> { \*"stmts" | "return", "stmts" } 
"stmts" -> { "expr", ";" }
"expr" -> { "(", "expr", ")" |  "expr" + "expr" }
