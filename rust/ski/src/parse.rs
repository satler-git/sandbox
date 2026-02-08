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
    #[error("There is no expr even though there must be")]
    MustBeExpr,
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
            expr: parser.parse_fun_apply()?,
        })
    }

    fn parse_fun_apply(&mut self) -> Result<Expr<'a>> {
        let mut prev = self.parse_expr()?.ok_or(ParseError::MustBeExpr)?;

        while let Some(next) = self.parse_expr()? {
            prev = Expr::Apply {
                fun: Box::new(prev),
                arg: Box::new(next),
            };
        }

        Ok(prev)
    }

    fn parse_expr(&mut self) -> Result<Option<Expr<'a>>> {
        Ok(Some(match dbg!(self.peek()) {
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
                self.parse_bracket()?
            }
            Some("$") => self.parse_daller()?,
            Some(_) => Expr::Ident(self.parse_ident()?),
            // func applyの次もexpr?ならまたfunc apply
            None => return Ok(None),
        }))
    }

    fn parse_def(&mut self) -> Result<Option<(Ident<'a>, Expr<'a>)>> {
        if self.peekn(1) != Some("=") {
            // このときexpr
            Ok(None)
        } else {
            let mut parser = {
                let mut vec = vec![];

                while self.peek().is_some() && self.peek() != Some(";") {
                    vec.push(self.next().unwrap());
                }

                if self.peek() == Some(";") {
                    let _ = self.next();
                }

                Parser { vec, pos: 0 }
            };

            let ident = parser.parse_ident()?;
            parser.nextee("=")?;
            let expr = parser.parse_fun_apply()?;
            Ok(Some((ident, expr)))
        }
    }

    fn parse_fun(&mut self) -> Result<(Ident<'a>, Expr<'a>)> {
        self.nextee("\\")?;
        let ident = self.parse_ident()?;
        let expr = self.parse_fun_apply()?;
        Ok((ident, expr))
    }

    fn parse_ident(&mut self) -> Result<Ident<'a>> {
        Ident::try_new(self.nexte()?)
    }

    fn parse_bracket(&mut self) -> Result<Expr<'a>> {
        let mut level = 0;
        let mut ts = vec![];

        loop {
            match self.next() {
                Some("(") => {
                    if level != 0 {
                        ts.push("(");
                    }

                    level += 1;
                }
                Some(")") => {
                    level -= 1;

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

        Parser { pos: 0, vec: ts }.parse_fun_apply()
    }

    fn parse_daller(&mut self) -> Result<Expr<'a>> {
        self.nextee("$")?;
        let mut vec = vec![];
        while let Some(n) = self.next() {
            vec.push(n);
        }
        Parser { pos: 0, vec }.parse_fun_apply()
    }
}
