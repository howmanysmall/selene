use full_moon::parse;

fn main() {
    let code = "local function foo() end";
    let ast = parse(code).expect("Failed to parse");

    for stmt in ast.nodes().stmts() {
        if let full_moon::ast::Stmt::LocalFunction(local_func) = stmt {
            println!("Found local function: {:?}", local_func.name());
        }
    }

    let code2 = "local x = function() end";
    let ast2 = parse(code2).expect("Failed to parse");

    for stmt in ast2.nodes().stmts() {
        if let full_moon::ast::Stmt::LocalAssignment(local_assign) = stmt {
            for expr in local_assign.expressions() {
                if let full_moon::ast::Expression::Function(func) = expr {
                    println!("Anonymous function structure: {:?}", func);
                    println!("Function token: {:?}", func.function_token());
                    println!("Function body: {:?}", func.body());
                }
            }
        }
    }
}
