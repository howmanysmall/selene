use full_moon::parse;

fn main() {
    // Test TypeFunction
    let code = r#"
        type function test()
            return {}
        end

        export type function exported_test()
            return {}
        end
    "#;

    let ast = parse(code).expect("Failed to parse");

    for stmt in ast.nodes().stmts() {
        match stmt {
            full_moon::ast::Stmt::TypeFunction(type_func) => {
                println!("TypeFunction methods:");
                println!("  - name: {:?}", type_func.name());
                println!("  - parameters: {:?}", type_func.parameters());
                println!("  - return_type: {:?}", type_func.return_type());
                println!(
                    "  - Available methods: {:?}",
                    std::any::type_name_of_val(type_func)
                );

                // Let's try to see what methods are available
                // by checking the debug representation
                println!("  - Full structure: {:#?}", type_func);
            }
            full_moon::ast::Stmt::ExportedTypeFunction(exported_type_func) => {
                println!("ExportedTypeFunction methods:");
                println!(
                    "  - type_function: {:?}",
                    exported_type_func.type_function()
                );
                println!(
                    "  - Available methods: {:?}",
                    std::any::type_name_of_val(exported_type_func)
                );

                println!("  - Full structure: {:#?}", exported_type_func);
            }
            _ => {}
        }
    }
}
