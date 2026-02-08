use regex::Regex;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct Parser<'a> {
    vec: Vec<&'a str>,
    pos: usize,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Ident is not vailed, only ^[a-z0-9]+$ is vailed ident.: {0}")]
    UnvailedIdent(String),
    #[error("Tokens are not enough")]
    UnToken,
    #[error("There must be {0}")]
    Token(String),
    #[error("brackets are not matched")]
    UnMatchedBrackets,
}

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub struct File<'a> {
    def: Vec<(Ident<'a>, Expr<'a>)>,
    expr: Expr<'a>,
}

#[derive(Debug)] // かっこも分ける必要がある?
enum Expr<'a> {
    Fun {
        arg: Ident<'a>,
        expr: Box<Expr<'a>>,
    },
    Apply {
        fun: Box<Expr<'a>>,
        arg: Box<Expr<'a>>,
    },
    Ident(Ident<'a>),
}

#[derive(Debug)]
struct Ident<'a>(&'a str);

impl<'a> Ident<'a> {
    fn try_new(ident: &'a str) -> Result<Self> {
        let regex = Regex::new("^([a-z0-9]+)|(_)$").unwrap();
        if !regex.is_match(ident) {
            Err(ParseError::UnvailedIdent(ident.into()))
        } else {
            Ok(Self(ident))
        }
    }
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&'a str> {
        self.vec.get(self.pos).map(|s| &**s)
    }

    fn peekn(&self, n: usize) -> Option<&'a str> {
        self.vec.get(self.pos + n).map(|s| &**s)
    }

    fn next(&mut self) -> Option<&'a str> {
        self.pos += 1;
        self.vec.get(self.pos - 1).map(|s| &**s)
    }

    fn nexte(&mut self) -> Result<&'a str> {
        self.next().ok_or(ParseError::UnToken)
    }

    fn nextee(&mut self, s: &str) -> Result<()> {
        if self.nexte()? != s {
            Err(ParseError::Token(s.into()))
        } else {
            Ok(())
        }
    }

    pub fn parse(ts: Vec<&'a str>) -> Result<File<'a>> {
        let mut parser = Parser { vec: ts, pos: 0 };

        let mut def = vec![];

        while let Some(d) = parser.parse_def()? {
            def.push(d);
        }

        Ok(File {
            def,
            expr: parser.parse_expr()?,
        })
    }

    fn parse_expr(&mut self) -> Result<Expr<'a>> {
        Ok(match self.peek() {
            Some("\\") => {
                // これは確定だから
                let (arg, expr) = self.parse_fun()?;
                Expr::Fun {
                    arg,
                    expr: Box::new(expr),
                }
            }
            Some("(") => {
                // かっこの次がexprならfunc applyにつなげる
                todo!()
            }
            // func applyの次もexpr?ならまたfunc apply
            _ => {
                todo!()
            }
        })
    }

    fn parse_def(&mut self) -> Result<Option<(Ident<'a>, Expr<'a>)>> {
        // defの終わりに終端記号をつけるべき?
        if self.peekn(1) != Some("=") {
            Ok(None)
        } else {
            let ident = Ident::try_new(self.nexte()?)?;
            self.nextee("=")?;
            let expr = self.parse_expr()?;
            Ok(Some((ident, expr)))
        }
    }

    fn parse_fun(&mut self) -> Result<(Ident<'a>, Expr<'a>)> {
        self.nextee("\\")?;
        let ident = Ident::try_new(self.nexte()?)?;
        let expr = self.parse_expr()?;
        Ok((ident, expr))
    }

    fn parse_bracket(&mut self) -> Result<Expr<'a>> {
        let mut level: usize = 0;
        let mut ts = vec![];

        loop {
            match self.peek() {
                Some("(") => {
                    if level != 0 {
                        ts.push("(");
                    }

                    level += 1;
                }
                Some(")") => {
                    let Some(levelt) = level.checked_sub(1) else {
                        return Err(ParseError::UnMatchedBrackets);
                    };
                    level = levelt;

                    if level == 0 {
                        break;
                    } else {
                        ts.push(")")
                    }
                }
                Some(t) => ts.push(t),
                _ => {
                    return Err(ParseError::UnMatchedBrackets);
                }
            }
        }

        Self { pos: 0, vec: ts }.parse_expr()
    }
}
