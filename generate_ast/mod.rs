use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn generate_ast(output_dir: &str) -> io::Result<()> {
    define_ast(
        output_dir,
        "Expr",
        &["error", "token", "literal", "rc"],
        &[
            "Binary   : Rc<Expr> left, Token operator, Rc<Expr> right",
            "Grouping : Rc<Expr> expression",
            "Literal  : Option<Literal> value",
            "Unary    : Token operator, Rc<Expr> right",
        ],
    )?;
    Ok(())
}

fn define_ast(
    output_dir: &str,
    base_name: &str,
    imports: &[&str],
    types: &[&str],
) -> io::Result<()> {
    let path = format!("{output_dir}/{}.rs", base_name.to_lowercase());
    let mut file = File::create(path)?;
    let mut tree_types = Vec::new();

    for i in imports {
        if i == &"rc" {
            writeln!(file, "use std::rc::Rc;")?;
        } else {
            writeln!(file, "use crate::{}::*;", i)?;
        }
    }

    for ttype in types {
        let (base_class_name, args) = ttype.split_once(":").unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name);
        let arg_split = args.split(',');
        let mut fields = Vec::new();
        for arg in arg_split {
            let (t2type, name) = arg.trim().split_once(" ").unwrap();
            fields.push(format!("{}: {}", name, t2type));
        }
        tree_types.push(TreeType {
            base_class_name: base_class_name.trim().to_string(),
            class_name,
            fields,
        });
    }

    writeln!(file, "\npub enum {base_name} {{")?;
    for t in &tree_types {
        writeln!(file, "    {}(Rc<{}>),", t.base_class_name, t.class_name)?;
    }
    writeln!(file, "}}\n")?;

    writeln!(file, "impl PartialEq for {} {{", base_name)?;
    writeln!(file, "    fn eq(&self, other: &Self) -> bool {{")?;
    writeln!(file, "        match (self, other) {{")?;
    for t in &tree_types {
        writeln!(
            file,
            "            ({0}::{1}(a), {0}::{1}(b)) => Rc::ptr_eq(a, b),",
            base_name, t.base_class_name
        )?;
    }
    writeln!(file, "            _ => false,")?;
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}\n\nimpl Eq for {}{{}}\n", base_name)?;

    writeln!(file, "impl {} {{", base_name)?;
    writeln!(file, 
        "    pub fn accept<T>(&self, wrapper: Rc<{}>, {}_visitor: &dyn {base_name}Visitor<T>) -> Result<T, JialoxError> {{", 
        base_name, 
        base_name.to_lowercase()
    )?;
    writeln!(file, "        match self {{")?;
    for t in &tree_types {
        writeln!(
            file, 
            "            {0}::{1}(v) => {3}_visitor.visit_{2}_{3}(wrapper, v),",
            base_name,
            t.base_class_name,
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
        )?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}\n")?;

    for t in &tree_types {
        writeln!(file, "pub struct {} {{", t.class_name)?;
        for f in &t.fields {
            writeln!(file, "    pub {},", f)?;
        }
        writeln!(file, "}}\n")?;
    }

    writeln!(file, "pub trait {}Visitor<T> {{", base_name)?;
    for t in &tree_types {
        writeln!(
            file,
            "    fn visit_{0}_{1}(&self, wrapper: Rc<{3}>, {1}: &{2}) -> Result<T, JialoxError>;",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
            t.class_name,
            base_name
        )?;
    }
    writeln!(file, "}}\n")?;

    Ok(())
}