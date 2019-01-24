use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::clone::Clone;

fn main() {}

pub fn balanced(str: String) -> bool {
    let mut map = HashMap::<String, String>::new();
    map.insert("[".to_string(), "]".to_string());
    map.insert("{".to_string(), "}".to_string());
    map.insert("(".to_string(), ")".to_string());
    let mut stack = Vec::<String>::new();
    for x in str.chars() {
        match map.get(&x.to_string()) {
            None => match stack.pop() {
                Some(p) => {
                    if p != x.to_string() {
                        return false;
                    }
                }
                _ => return false,
            },
            Some(c) => stack.push(c.to_string()),
        }
    }
    stack.len() == 0
}


pub fn bracket_balanced<T>(v: &[T],map: HashMap<T,T>) -> bool where
    T: Eq + Hash + Clone {
        false
    }

pub fn balanced_any<T>(v: &[T],map: HashMap<T,T>) -> bool where
    T: Eq + Hash + Clone {
   let mut stack = Vec::<T>::with_capacity(v.len());
   for x in v
       .iter()
       .map(|el: &T| el.to_owned()) {
       match map.get(&x) {
           None => match stack.pop() {
               Some(p) => {
                   if p != x {
                       return false;
                   }
               }
               _ => return false,
           },
           Some(c) => stack.push(c.clone()),
       }
   }
  stack.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced_passes() {
        assert!(balanced("{}".to_string()), "{}");
        assert!(balanced("{[]}".to_string()), "{[]}");
        assert!(balanced("{()[]}".to_string()), "{()[]}");
    }

    #[test]
    fn balanced_failures() {
        assert!(!balanced("{".to_string()), "{");
        assert!(!balanced("{)".to_string()), "{)");
        assert!(!balanced("{}}{{}{}".to_string()), "{}}{{}{}");
        assert!(!balanced("{()[]}hello".to_string()), "{()[]}hello");
    }
}
