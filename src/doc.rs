use std::fs;
fn main() {
    if let Err(_) = fs::write("./libmysql_roaring.md", format!("* {}", list().join("\n* "))){
        std::process::exit(1);
    }
    if let Err(_) = fs::write("./libmysql_roaring.sql", sql().join("\n")){
        std::process::exit(1);
    }
}

fn list() -> Vec<String> {
    let mut doc: Vec<String> = Vec::new();
    for nullsafe in vec![false, true] {
        for bit_size in vec!["32", "64"] {
            let map_in_type = format!("{}BLOB", if bit_size == "64" { "LONG" } else { "" });
            let map_out_type = format!("{}BLOB{}", if bit_size == "64" { "LONG" } else { "" }, if nullsafe { "" } else { "|NULL" });
            let int_in_type = format!("{}INT", if bit_size == "64" { "BIG" } else { "" });
            let int_out_type = format!("{}INT{}", if bit_size == "64" { "BIG" } else { "" }, if nullsafe { "" } else { "|NULL" });
            let bool_out_type = format!("BOOL{}", if nullsafe { "" } else { "|NULL" });
            let text_out_type = format!("LONGTEXT{}", if nullsafe { "" } else { "|NULL" });
            let prefix = format!("roaring{}_{}", bit_size, if nullsafe { "nullsafe_" } else { "" });
            doc.push(format!("{}create(value {}, ...) {}", prefix, int_in_type, map_out_type));
            for op in vec!["remove", "insert"] {
                doc.push(format!("{}{}(map {}, value {}, ...) {}", prefix, op, map_in_type, int_in_type, map_out_type));
            }
            doc.push(format!("{}contains(map {}, value {}) {}", prefix, map_in_type, int_in_type, bool_out_type));
            doc.push(format!("{}count(map {}) {}", prefix, map_in_type, int_out_type));
            doc.push(format!("{}json(map {}) {}", prefix, map_in_type, text_out_type));
            for op in vec!["and", "or", "xor"] {
                doc.push(format!("{}{}(map {}, map {}, ...) {}", prefix, op, map_in_type, map_in_type, map_out_type));
            }
            for op in vec!["and", "or", "xor"] {
                doc.push(format!("{}{}_count(map {}, map {}, ...) {}", prefix, op, map_in_type, map_in_type, int_out_type));
            }
            doc.push(format!("{}group_create(value {}) {}", prefix, int_in_type, map_out_type));
            for op in vec!["and", "or", "xor"] {
                doc.push(format!("{}group_{}(map {}) {}", prefix, op, map_in_type, map_out_type));
            }
            for op in vec!["and", "or", "xor"] {
                doc.push(format!("{}group_{}_count(map {}) {}", prefix, op, map_in_type, int_out_type));
            }
        }
    }
    doc
}

fn sql() -> Vec<String> {
    let mut doc: Vec<String> = Vec::new();
    for nullsafe in vec![false, true] {
        for bit_size in vec!["32", "64"] {
            for drop in vec![true, false] {
                let map_out_type = if drop {";"} else {"RETURNS STRING SONAME 'libmysql_roaring.so';"};
                let int_out_type = if drop {";"} else {"RETURNS INTEGER SONAME 'libmysql_roaring.so';"};
                let bool_out_type = if drop {";"} else {"RETURNS INTEGER SONAME 'libmysql_roaring.so';"};
                let text_out_type = if drop {";"} else {"RETURNS STRING SONAME 'libmysql_roaring.so';"};
                let prefix = if drop {
                    format!("DROP FUNCTION IF EXISTS roaring{}_{}", bit_size, if nullsafe { "nullsafe_" } else { "" })
                } else {
                    format!("CREATE FUNCTION IF NOT EXISTS roaring{}_{}", bit_size, if nullsafe { "nullsafe_" } else { "" })
                };
                let prefix_group = if drop {
                    format!("DROP FUNCTION IF EXISTS roaring{}_{}", bit_size, if nullsafe { "nullsafe_" } else { "" })
                } else {
                    format!("CREATE AGGREGATE FUNCTION IF NOT EXISTS roaring{}_{}", bit_size, if nullsafe { "nullsafe_" } else { "" })
                };
                doc.push(format!("{}create {}", prefix, map_out_type));
                for op in vec!["remove", "insert"] {
                    doc.push(format!("{}{} {}", prefix, op, map_out_type));
                }
                doc.push(format!("{}contains {}", prefix, bool_out_type));
                doc.push(format!("{}count {}", prefix, int_out_type));
                doc.push(format!("{}json {}", prefix, text_out_type));
                for op in vec!["and", "or", "xor"] {
                    doc.push(format!("{}{} {}", prefix, op, map_out_type));
                }
                for op in vec!["and", "or", "xor"] {
                    doc.push(format!("{}{}_count {}", prefix, op, int_out_type));
                }
                doc.push(format!("{}group_create {}", prefix_group, map_out_type));
                for op in vec!["and", "or", "xor"] {
                    doc.push(format!("{}group_{} {}", prefix_group, op, map_out_type));
                }
                for op in vec!["and", "or", "xor"] {
                    doc.push(format!("{}group_{}_count {}", prefix_group, op, int_out_type));
                }
            }
        }
    }
    doc
}
