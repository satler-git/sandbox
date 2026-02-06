#[derive(Debug, Default)]
pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize(script: &str) -> Vec<&str> {
        script
            .split_whitespace()
            .flat_map(split_delm)
            .flat_map(|x| x.split('.'))
            .collect()
    }
}

struct SplitDelm<'a> {
    s: &'a str,
    next: usize,
}

const SPLIT_AT: [&str; 3] = ["(", ")", "\\"];

impl<'a> Iterator for SplitDelm<'a> {
    type Item = &'a str;

    // TODO?: UTF-8があるとバグる
    fn next(&mut self) -> Option<Self::Item> {
        let now_next = self.next;
        if self.next == self.s.len() {
            None
        } else if SPLIT_AT.contains(&&self.s[now_next..=now_next]) {
            self.next += 1;
            Some(&&self.s[now_next..=now_next])
        } else {
            while self.next < self.s.len() && !SPLIT_AT.contains(&&self.s[self.next..=self.next]) {
                self.next += 1;
            }
            Some(&&self.s[now_next..self.next])
        }
    }
}

fn split_delm(from: &str) -> SplitDelm<'_> {
    SplitDelm { s: from, next: 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_delm() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            vec!["(", "str", ")"],
            split_delm("(str)").collect::<Vec<&str>>()
        );
        assert_eq!(
            vec!["\\", "\\", "q"],
            split_delm("\\\\q").collect::<Vec<&str>>()
        );
        assert_eq!(
            vec!["(", "(", "\\", "(", "\\", "str", ")", ")", ")"],
            split_delm("((\\(\\str)))").collect::<Vec<&str>>()
        );
        Ok(())
    }
}
