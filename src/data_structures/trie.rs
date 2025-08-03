pub struct TrieNode {
    value: char,
    nodes: Vec<TrieNode>,
    is_end: bool,
}

impl TrieNode {
    pub fn new(strs: Vec<&str>) -> Self {
        let mut trie_tree = TrieNode { value: ' ', nodes: vec![], is_end: false };
        for str in strs {
            let mut current = &mut trie_tree;
            for c in str.chars() {
                let mut index = -1;
                for (i, node) in current.nodes.iter().enumerate() {
                    if node.value == c {
                        index = i as isize;
                    }
                }
                if index == -1 {
                    let new_node = TrieNode { value: c, nodes: vec![], is_end: false };
                    current.nodes.push(new_node);
                    index = current.nodes.len() as isize - 1;
                    current = &mut current.nodes[index as usize];
                } else {
                    current = &mut current.nodes[index as usize];
                }
            }
            current.is_end = true;
        }

        trie_tree
    }

    pub fn find(&self, str: &str) -> bool {
        let mut current = self;
        let mut found = true;
        for c in str.chars() {
            let mut index = -1;
            for (i, node) in current.nodes.iter().enumerate() {
                if node.value == c {
                    index = i as isize;
                }
            }
            if index == -1 {
                found = false;
                break
            }
            current = &current.nodes[index as usize];
        }
        if !current.is_end {
            found = false;
        }
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_tree_new() {
        let strs = vec!["hello", "world", "hi", "how", "are", "you"];
        let trie_tree = TrieNode::new(strs);
        assert_eq!(trie_tree.nodes[0].value, 'h');
        assert_eq!(trie_tree.nodes[0].nodes.len(), 3);
        assert_eq!(trie_tree.nodes[0].nodes[0].value, 'e');
        assert_eq!(trie_tree.nodes[0].nodes[0].nodes.len(), 1);
        assert_eq!(trie_tree.nodes[0].nodes[0].nodes[0].value, 'l');
    }

    #[test]
    fn test_trie_tree_find() {
        let strs = vec!["hello", "world", "hi", "how", "are", "you"];
        let trie_tree = TrieNode::new(strs);
        assert_eq!(trie_tree.find("hello"), true);
        assert_eq!(trie_tree.find("world"), true);
        assert_eq!(trie_tree.find("hi"), true);
        assert_eq!(trie_tree.find("how"), true);
        assert_eq!(trie_tree.find("are"), true);
        assert_eq!(trie_tree.find("you"), true);
        assert_eq!(trie_tree.find("hello world"), false);
        assert_eq!(trie_tree.find("hell"), false);
        assert_eq!(trie_tree.find("wor"), false);
        assert_eq!(trie_tree.find("h"), false);
    }
}