use fnv::{FnvHashMap};

#[derive(Debug)]
struct TrieNode {
    tree: FnvHashMap<char, TrieNode>,
    points: Option<(Vec<usize>)>
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            tree: Default::default(),
            points: None,
        }
    }
    fn insert(&mut self, pattern: &[char], mut points: Vec<usize>, inc: bool) {
        if pattern.len() == 0 {
            self.points = Some(points);
            return;
        }
        let c = pattern[0];
        match c.to_digit(10) {
            Some(d) => {
                points.push(d as usize);
                self.insert(&pattern[1..], points, false);
            },
            None => {
                if inc {
                    points.push(0);
                }
                self.tree.entry(c)
                    .or_insert(TrieNode::new())
                    .insert(&pattern[1..], points, true);
            }
        }
    }
}

#[derive(Debug)]
pub struct Trie {
    tree: FnvHashMap<char, TrieNode>,
    count: usize
}

impl Trie {
    pub fn new() -> Self {
        Trie::default()
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn insert(&mut self, pattern: &str) {
        let chars = pattern.chars().collect::<Vec<_>>();
        let points = vec![0];
        let c = chars[0];
        self.count += 1;
        self.tree.entry(c)
            .or_insert(TrieNode::new())
            .insert(&chars[1..], points, true);
    }

    pub fn fetch(&self, chars: &[char]) -> Vec<usize> {
        use std::cmp::{max};
        //let chars = pattern.chars().collect::<Vec<_>>();
        let mut points = vec![0; chars.len()];
        for i in 0..chars.len() {
            let mut t = &self.tree;
            let _ = chars.iter().skip(i).all(|c| {
                if let Some(node) =  t.get(&c) {
                    t = &node.tree;
                    if let Some(ref ps) = node.points {
                        for j in 0..ps.len() {
                            points[i + j] = max(points[i + j], ps[j]);
                        }
                    }
                    return true;
                } else {
                    return false;
                }
            });
        }
        points
    }
}

impl Default for Trie {
    fn default() -> Self {
        Trie {
            tree: FnvHashMap::default(),
            count: 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Trie};

    #[test]
    fn single_creation() {
        let mut trie: Trie = Trie::new();
        trie.insert(".a1bc3d4");
        let val = trie.fetch(&".abcd.".chars().collect::<Vec<_>>());
        assert_eq!(val, vec![0, 0, 1, 0, 3, 4]);
    }

    #[test]
    fn create_same_prefix() {
        let mut trie: Trie = Trie::new();
        trie.insert(".ci2");
        trie.insert(".cit5r");
        let val = trie.fetch(&".citr".chars().collect::<Vec<_>>());
        assert_eq!(val, vec![0, 0, 0, 2, 5]);

        let val = trie.fetch(&".cia".chars().collect::<Vec<_>>());
        assert_eq!(val, vec![0, 0, 0, 2]);
    }

    #[test]
    fn priority_overwrite() {
        let mut trie: Trie = Trie::new();
        trie.insert(".a1bc");
        trie.insert(".ab7cde");
        trie.insert("e3f");
        let val = trie.fetch(&".abc".chars().collect::<Vec<_>>());
        assert_eq!(val, vec![0, 0, 1, 0]);

        let val = trie.fetch(&"abc".chars().collect::<Vec<_>>());
        assert_eq!(val, vec![0, 0, 0]);

        let val = trie.fetch(&".abcde".chars().collect::<Vec<_>>());
        assert_eq!(val, vec![0, 0, 1, 7, 0, 0]);

        let val = trie.fetch(&".eef".chars().collect::<Vec<_>>());
        assert_eq!(val, vec![0, 0, 0, 3]);
    }

    #[test]
    fn simple() {
        let mut trie: Trie = Trie::new();
        trie.insert(".a1bc");
        trie.insert(".ab7cde");
        trie.insert("e3f");
        let val = trie.fetch(&".abc".chars().collect::<Vec<_>>());
        assert_eq!(val, vec![0, 0, 1, 0]);

        let val = trie.fetch(&"abc".chars().collect::<Vec<_>>());
        assert_eq!(val, vec![0, 0, 0]);

        let val = trie.fetch(&".abcde".chars().collect::<Vec<_>>());
        assert_eq!(val, vec![0, 0, 1, 7, 0, 0]);

        let val = trie.fetch(&".eef".chars().collect::<Vec<_>>());
        assert_eq!(val, vec![0, 0, 0, 3]);
    }
}
