use resast::{prelude::*, stmt::Stmt, ProgramPart};
use ressa::*;
use std::borrow::Cow;

#[cfg(not(feature = "hashbrown"))]
use std::collections::HashSet;

#[cfg(feature = "hashbrown")]
use hashbrown::HashSet;

/// extract all links that are created from a js script
#[cfg(not(feature = "tokio"))]
pub fn extract_links<T: PartialEq + Eq + std::hash::Hash + From<String>>(
    script: &str,
) -> HashSet<T> {
    let mut map = HashSet::new();

    match Parser::new(&Box::new(script)) {
        Ok(p) => {
            for part in p {
                _extract_links(part, &mut map);
            }
        }
        _ => (),
    };

    map
}

/// extract all links that are created from a js script
#[cfg(feature = "tokio")]
pub async fn extract_links<T: PartialEq + Eq + std::hash::Hash + From<String>>(
    script: &str,
) -> HashSet<T> {
    use tokio_stream::StreamExt;
    let mut map = HashSet::new();

    match Parser::new(&Box::new(script)) {
        Ok(p) => {
            let mut stream = tokio_stream::iter(Box::new(p));

            while let Some(part) = stream.next().await {
                _extract_links(part, &mut map);
            }
        }
        _ => (),
    };

    map
}

/// parse and extract links
fn _extract_links<T: PartialEq + Eq + std::hash::Hash + From<String>>(
    part: Result<ProgramPart<Cow<str>>, Error>,
    map: &mut HashSet<T>,
) {
    match part {
        Ok(program) => {
            // entry script program
            match program {
                ProgramPart::Stmt(stmt) => {
                    match stmt {
                        Stmt::Expr(expression) => {
                            match expression {
                                resast::expr::Expr::Assign(assign) => {
                                    match assign {
                                        AssignExpr {
                                            operator,
                                            left,
                                            right,
                                        } => {
                                            // assignment to create node
                                            if operator == AssignOp::Equal {
                                                match left {
                                                    AssignLeft::Expr(exp) => {
                                                        match *exp {
                                                            Expr::Member(mexp) => {
                                                                match mexp {
                                                                    MemberExpr {
                                                                        object: _,
                                                                        property,
                                                                        computed,
                                                                    } => {
                                                                        if !computed {
                                                                            match *property {
                                                                                Expr::Ident(
                                                                                    ident,
                                                                                ) => {
                                                                                    // links
                                                                                    if ident.name
                                                                                        == "href"
                                                                                    {
                                                                                        match *right {
                                                                                            Expr::Lit(value) => {
                                                                                                match value {
                                                                                                    Lit::String(string_lit) => {
                                                                                                        match string_lit {
                                                                                                            StringLit::Double(value) | StringLit::Single(value) => {
                                                                                                                map.insert(value.to_string().into());
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    _ => ()
                                                                                                }
                                                                                            },
                                                                                            _ => ()
                                                                                        }
                                                                                    }
                                                                                }
                                                                                _ => (),
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            _ => (),
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => (),
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        _ => (),
    };
}
