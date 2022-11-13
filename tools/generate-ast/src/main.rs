use std::{
    env::args,
    fs,
    io::{self, Write},
};

struct TreeType {
    base_name: String,
    struct_name: String,
    fields: Vec<String>,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        eprintln!("Usage: generate-ast [output_dir]");
        std::process::exit(64);
    }

    define_ast(
        &args[1],
        "Expr",
        vec![
            "Literal    : Object value",
            "Unary      : Token operator, Box<Expr> right",
            "Binary     : Box<Expr> left, Token operator, Box<Expr> right",
            "Grouping   : Box<Expr> exprs",
        ],
    )?;

    Ok(())
}

fn define_ast(output_dir: &str, base_name: &str, types: Vec<&str>) -> io::Result<()> {
    // Setting up what to write
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    let mut file = fs::File::create(path)?;
    let mut tree_types = Vec::new();

    for ttype in types {
        let (base_struct, args) = ttype.split_once(":").unwrap();
        let struct_name = format!("{}{}", base_struct.trim(), base_name);
        let args_split = args.trim().split(",");
        let mut fields = Vec::new();

        for arg in args_split {
            let (arg_type, arg) = arg.trim().split_once(" ").unwrap();
            fields.push(format!("{}: {}", arg, arg_type));
        }

        let tree_type = TreeType {
            base_name: base_name.to_string(),
            struct_name: struct_name.to_string(),
            fields,
        };
        tree_types.push(tree_type);
    }

    // Writing to the File
    write!(file, "use crate::error::Error;\n")?;
    write!(file, "use crate::token::Token;\n")?;
    write!(file, "use crate::object::Object;\n")?;
    write!(file, "\n")?;

    write!(file, "pub enum {} {{ \n", base_name)?;
    for t in &tree_types {
        write!(file, "\t{}({}),\n", t.struct_name, t.struct_name)?;
    }
    write!(file, "}}\n")?;
    write!(file, "\n")?;

    /*
    GOTTA GENERATE
    impl Expr {
        pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, Error> {
            match self {
                    Self::LiteralExpr(expr) => expr.accept(visitor),
                    Self::UnaryExpr(expr) => expr.accept(visitor),
                    Self::BinaryExpr(expr) => expr.accept(visitor),
                    Self::GroupingExpr(expr) => expr.accept(visitor),
                }
            }
        }
    */

    for t in &tree_types {
        write!(file, "pub struct {} {{\n", t.struct_name)?;
        for f in &t.fields {
            write!(file, "\t{},\n", f)?;
        }
        write!(file, "}}\n")?;
        write!(file, "\n")?;
    }

    write!(file, "pub trait {}Visitor<T> {{\n", base_name)?;
    for t in &tree_types {
        write!(
            file,
            "\tfn visit{}(&self, expr: &{}) -> Result<T, Error>;\n",
            t.struct_name, t.struct_name
        )?;
    }
    write!(file, "}}\n")?;
    write!(file, "\n")?;

    for t in &tree_types {
        write!(file, "impl {} {{\n", t.struct_name)?;
        write!(
            file,
            "\tfn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, Error> {{\n",
            t.base_name
        )?;
        write!(file, "\t\tvisitor.visit{}(self)\n", t.struct_name)?;
        write!(file, "\t}}\n")?;
        write!(file, "}}\n\n")?;
    }

    Ok(())
}
