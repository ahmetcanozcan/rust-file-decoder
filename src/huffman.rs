use std::collections::HashMap;

pub fn encode(s: &str) -> (Vec<u8>, u8, HashMap<String, char>) {
  let root = {
    let mut h = frequency(s);
    let mut p = get_node_list(&mut h);
    generate_tree(&mut p)
  };
  let mut code_table = HashMap::<char, String>::new();
  assign_codes(&root, &mut code_table, "".to_string());
  let mut encoded_string = String::new();
  for c in s.chars() {
    let t = code_table.get(&c);
    encoded_string.push_str(t.unwrap());
  }
  let mut encoded_numbers = Vec::<u8>::new();
  println!("{} -> ({})", encoded_string, encoded_string.len());
  while encoded_string.len() > 7 {
    let str_ = &encoded_string[..8];
    print!("str : {} ", str_);
    let number = binary_to_u8(&str_);
    println!("--- number : {}", number);
    encoded_numbers.push(number);
    encoded_string = String::from(&encoded_string[8..]);
  }
  let last_items_size: u8 = encoded_string.len() as u8;
  println!(
    "LAST ITEM : encoded string : {} --- encoded number : {}",
    encoded_string,
    binary_to_u8(&encoded_string)
  );
  encoded_numbers.push(binary_to_u8(&encoded_string));
  (encoded_numbers, last_items_size, reverse_map(code_table))
}

fn reverse_map(m: HashMap<char, String>) -> HashMap<String, char> {
  let mut result = HashMap::<String, char>::new();
  for (k, v) in m {
    result.insert(v, k);
  }
  result
}

fn binary_to_u8(s: &str) -> u8 {
  // let mut result: u8 = 0;
  // let mut i = 0;
  // for c in s.chars() {
  //   if c == '1' {
  //     result += u8::pow(2, i);
  //   }
  //   i += 1;
  // }
  // result

  u8::from_str_radix(s, 2).unwrap()
}

fn generate_tree(p: &mut Vec<Box<TreeNode>>) -> Box<TreeNode> {
  while p.len() > 1 {
    p.sort_by(|x, y| (&(x.f)).cmp(&(y.f)));
    let first = p.pop().unwrap();
    let second = p.pop().unwrap();
    let mut parent = Box::new(TreeNode::new(None, first.f + second.f));
    parent.left = Some(first);
    parent.right = Some(second);
    p.push(parent);
  }
  p.pop().unwrap()
}

fn frequency(s: &str) -> HashMap<char, i32> {
  let mut h = HashMap::new();
  for c in s.chars() {
    let counter = h.entry(c).or_insert(0);
    *counter += 1;
  }
  h
}

fn get_node_list(map: &mut HashMap<char, i32>) -> Vec<Box<TreeNode>> {
  map
    .iter()
    .map(|x| Box::new(TreeNode::new(Some(*(x.0)), *(x.1))))
    .collect()
}

fn assign_codes(parent: &Box<TreeNode>, map: &mut HashMap<char, String>, tag: String) {
  if let Some(c) = parent.c {
    map.insert(c, tag);
  } else {
    if let Some(ref node) = parent.left {
      assign_codes(node, map, tag.clone() + "0");
    }
    if let Some(ref node) = parent.right {
      assign_codes(node, map, tag.clone() + "1");
    }
  }
}

pub fn decode(vec: &mut Vec<u8>, lsize: u8, table: HashMap<String, char>) -> String {
  let mut s = String::new();
  vec.reverse();
  while vec.len() > 1 {
    let temp = vec.pop().unwrap();
    let t = u8_to_string(temp, None);
    println!("str : {}  number : {}", t, temp);
    s = format!("{}{}", s, t);
  }
  let last = vec.pop().unwrap();
  let last_str = u8_to_string(last, Some(lsize));
  s = format!("{}{}", s, last_str);
  println!("LAST str : {} num : {}", last_str, last);
  println!("{} -> {}", s, s.len());
  println!("table: {:?}", table);

  let mut result = String::new();
  let mut i = 0;
  while s.len() > i {
    let temp_str = &s[i..];
    for (k, v) in &table {
      if temp_str.starts_with(k.as_str()) {
        result.push(*v);
        i += k.len();
        break;
      }
    }
  }

  result
}

fn u8_to_string(num: u8, lim: Option<u8>) -> String {
  let mut result = String::new();
  let n = match lim {
    Some(number) => number,
    None => 8,
  };

  for i in 0..n {
    let i = n - i - 1;
    if num & u8::pow(2, i as u32) == u8::pow(2, i as u32) {
      result.push_str("1");
    } else {
      result.push_str("0");
    }
  }

  result
}

struct TreeNode {
  c: Option<char>,
  f: i32,
  left: Option<Box<TreeNode>>,
  right: Option<Box<TreeNode>>,
}

impl TreeNode {
  fn new(c: Option<char>, f: i32) -> TreeNode {
    TreeNode {
      c,
      f,
      left: None,
      right: None,
    }
  }
}
