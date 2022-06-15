#[derive(Debug, Clone)]
struct TrieNode {
    nodes: Vec<Option<TrieNode>>,
    end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            nodes: vec![None; 26],
            end: false,
        }
    }
}

#[derive(Debug)]
struct Trie {
    nodes: Vec<Option<TrieNode>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Trie {
            nodes: vec![None; 26],
        }
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let v: Vec<char> = word.chars().collect();
        if self.nodes[get_alpha_index(v[0])].is_none() {
            self.nodes[get_alpha_index(v[0])] = Some(TrieNode::new());
        }
        let mut head = self.nodes[get_alpha_index(v[0])].as_mut();
        for i in 1..v.len() {
            if let Some(node) = head {
                if node.nodes[get_alpha_index(v[i])].is_none() {
                    node.nodes[get_alpha_index(v[i])] = Some(TrieNode::new());
                }
                head = node.nodes[get_alpha_index(v[i])].as_mut();
            }
        }
        if let Some(node) = head {
            node.end = true;
        }
    }

    /** Returns if the word is in the trie. */
    fn search(&mut self, word: String) -> bool {
        let v: Vec<char> = word.chars().collect();
        let mut head = &self.nodes[get_alpha_index(v[0])];
        for i in 1..v.len() {
            if let Some(node) = head.as_ref() {
                let next = &node.nodes[get_alpha_index(v[i])];
                head = next;
            } else {
                return false;
            }
        }

        if let Some(node) = head.as_ref() {
            node.end
        } else {
            false
        }
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&mut self, prefix: String) -> bool {
        let v: Vec<char> = prefix.chars().collect();
        let mut head = &self.nodes[get_alpha_index(v[0])];
        for i in 1..v.len() {
            if let Some(node) = head.as_ref() {
                let next = &node.nodes[get_alpha_index(v[i])];
                head = next;
            } else {
                return false;
            }
        }

        head.is_some()
    }
}

fn get_alpha_index(c: char) -> usize {
    ((c as u8) - ('a' as u8)) as usize
}
#[derive(Debug,Clone)]
struct TrieNode {
    nodes : Vec<Option<TrieNode>>,
    end : bool
}

impl TrieNode {
    fn new() -> Self {
        TrieNode { nodes: vec![None;26], end: false }
    }
}

#[derive(Debug)]
struct Trie {
     nodes : Vec<Option<TrieNode>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Trie { nodes: vec![None;26] }
    }
    
    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let v : Vec<char> = word.chars().collect();
        if self.nodes[get_alpha_index(v[0])].is_none() {
            self.nodes[get_alpha_index(v[0])] = Some(TrieNode::new());
        }
        let mut head = self.nodes[get_alpha_index(v[0])].as_mut();
        for i in 1..v.len() {
            if let Some(node) = head {
                if node.nodes[get_alpha_index(v[i])].is_none() {
                    node.nodes[get_alpha_index(v[i])] = Some(TrieNode::new());
                }
                head = node.nodes[get_alpha_index(v[i])].as_mut();
            }
        }
        if let Some(node) = head {
            node.end = true;
        }
    }
    
    /** Returns if the word is in the trie. */
    fn search(&mut self, word: String) -> bool {
        let v : Vec<char> = word.chars().collect();
        let mut head = &self.nodes[get_alpha_index(v[0])];
        for i in 1..v.len() {
            if let Some(node) = head.as_ref() {
                let next = &node.nodes[get_alpha_index(v[i])];
                head = next;
            }
            else {
                return false;
            }
        }
        
        if let Some(node) = head.as_ref() {
            node.end
        }
        else {
            false
        }
        
    }
    
    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&mut self, prefix: String) -> bool {
        let v : Vec<char> = prefix.chars().collect();
        let mut head = &self.nodes[get_alpha_index(v[0])];
        for i in 1..v.len() {
            if let Some(node) = head.as_ref() {
                let next = &node.nodes[get_alpha_index(v[i])];
                head = next;
            }
            else {
                return false;
            }
        }
        
        head.is_some()
    }
}


fn get_alpha_index(c : char) -> usize {
    ((c as u8) - ('a' as u8)) as usize 
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */