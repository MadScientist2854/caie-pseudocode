import sys

def define_ast(out_dir, base_name, types):
    path  = out_dir + "/" + base_name.lower() + ".rs"
    with open(path, "w") as f:
        if base_name == "Stmt":
            f.write("use super::expr::Expr;\n")
            f.write("use super::token::Token;\n")
        elif base_name == "Expr":
            f.write("use super::token::{Token, Literal};\n")
        f.write("use std::fmt::Debug;\n\n#[derive(Clone, Debug)]\npub enum " + base_name + " {\n")

        for _type in types:
            type_name = _type.split("|")[0].strip()
            f.write("    " + type_name + "(")
            field_list = _type.split("|")[1].strip()
            fields = field_list.split(", ")
            if fields[0] == base_name:
                fields[0] = "Box<"+ base_name +">"
            f.write(fields[0])
            for field in fields[1:]:
                f.write(", ")
                if field == base_name:
                    field = "Box<" + base_name + ">"
                f.write(field)
            f.write("),\n")

        f.write("}")

        #

        # f.write("impl " + base_name + " {\n")
        # for _type in types:
        #     type_name = _type.split("|")[0].strip()
        #     field_list = _type.split("|")[1].strip()
        #     fields = field_list.split(", ")

        #     # f.write("pub struct " + type_name + " {\n")
        #     # for field in fields:
        #     #     f.write("    "+field + ",\n")
        #     # f.write("}\n\n")

        #     # f.write("impl " + type_name + " {\n")
        #     f.write("    pub fn new_" + type_name.lower() + "(")
        #     f.write("_0: " + fields[0])
        #     for i in range(1, len(fields)):
        #         f.write(", _" + str(i) + ": " + fields[i])
        #     f.write(") -> " + base_name + " {\n")
        #     f.write("        " + base_name + "::" + type_name + "(")
        #     if fields[0] == "Expr":
        #         f.write("Box::new(_0)")
        #     else:
        #         f.write("_0")
        #     for i in range(1, len(fields)):
        #         if fields[i] == "Expr":
        #             f.write(", Box::new(_" + str(i) + ")")
        #         else:
        #             f.write(", _" + str(i))
        #     f.write(")\n")
        #     # for field in fields:
        #     #     f.write("            " + field + ",\n")
        #     f.write("    }\n")
        # f.write("}\n")
    

if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise Exception("Usage: generate_ast <output directory>")

    out_dir: str = sys.argv[1]
    define_ast(out_dir, "Expr", [
        "Unary | Token, Expr",
        "Binary | Expr, Token, Expr",
        "Grouping | Expr",
        "IdentExpr | Token",
        "FnCall | Token, Vec<Expr>",
        "ArrIdx | Token, Expr, Option<Box<Expr>>",
        "ArrType | (Box<Expr>, Box<Expr>), Option<(Box<Expr>, Box<Expr>)>, Expr",
        "Literal | Literal"
    ])
    define_ast(out_dir, "Stmt", [
        "Block | Vec<Stmt>",
        "ExprStmt | Expr",
        "Declare | Token, Expr",
        "Constant | Token, Expr",
        "Assign | Expr, Expr",
        "ProcCall | Token, Vec<Expr>",
        "Input | Expr",
        "Output | Vec<Expr>",
        "Ret | Expr",
        "Procedure | Token, Vec<(Token, Expr, bool)>, Stmt",
        "Function | Token, Vec<(Token, Expr, bool)>, Expr, Stmt",
        "ForTo | Token, Expr, Expr, Option<Expr>, Stmt",
        "IfThen | Expr, Stmt, Option<Box<Stmt>>",
        "Case | Expr, Vec<(Expr, Stmt)>, Option<Box<Stmt>>",
        "Repeat | Expr, Stmt",
        "WhileDo | Expr, Stmt",
    ])