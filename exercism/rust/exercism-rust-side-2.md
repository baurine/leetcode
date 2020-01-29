# Exercism Rust Note

按网站上的指南安装 exercism 本地命令，使用 `exercism submit [files]` 提交作业。每执行一次测试，exercism 就会记录一次当前的实现并作为历史记录一并提交到网站。

## Side exercises

### Bob

考查点：chars 和 strings

问题：你朝 bob 说不同的话，它会进行不同的回复。

解答：

```rust
const REPLIES: [&'static str; 5] = [
  "Sure.",                             // question
  "Whoa, chill out!",                  // YELL
  "Calm down, I know what I'm doing!", // YELL and question, aka YELL?
  "Fine. Be that way!",                // empty
  "Whatever.",                         // others
];

pub fn reply(message: &str) -> &str {
  let msg = message.trim();
  let empty = msg.is_empty();
  let question = msg.chars().last() == Some('?');
  let alphas = msg
    .chars()
    .filter(|c| c.is_alphabetic())
    .collect::<String>();
  let yell = alphas.len() > 0 && alphas.chars().all(|c| c.is_uppercase());

  if empty {
    REPLIES[3]
  } else if yell && question {
    REPLIES[2]
  } else if yell {
    REPLIES[1]
  } else if question {
    REPLIES[0]
  } else {
    REPLIES[4]
  }
}
```

优化后：

```rust
const REPLY_QUESTION: &str = "Sure.";
const REPLY_YELL: &str = "Whoa, chill out!";
const REPLY_YELL_QUESTION: &str = "Calm down, I know what I'm doing!";
const REPLY_EMPTY: &str = "Fine. Be that way!";
const REPLY_OTHERS: &str = "Whatever.";

pub fn reply(message: &str) -> &str {
  let msg = message.trim();

  let empty = msg.is_empty();
  let question = msg.ends_with('?');
  let yell = msg.to_uppercase() == msg && msg.chars().any(|c| c.is_alphabetic());

  match (yell, question) {
    _ if empty => REPLY_EMPTY,
    (true, true) => REPLY_YELL_QUESTION,
    (true, _) => REPLY_YELL,
    (_, true) => REPLY_QUESTION,
    _ => REPLY_OTHERS,
  }
}
```

学习到了 `ends_with()`，判断全部为大写的简单办法：`msg.to_uppercase() == msg;`，用 match 替代 if...else。

### Matching Brackets

考查点：stack, recursion

问题：判断一个数学表达式括号是否匹配。

解答：

常规的栈的使用。暂时没想到用递归怎么解。看了一下社区解法也都是用栈实现的。我一开始写了个比较啰嗦的写法，用了 25 行，看了一下社区的解法，简洁写法可以达到 15 行，而且思路也巧妙一点。

```rust
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = vec![];
    for c in string.chars() {
        match c {
            '(' => brackets.push(')'),
            '[' => brackets.push(']'),
            '{' => brackets.push('}'),
            ')' | ']' | '}' if brackets.pop() != Some(c) => {
                return false;
            }
            _ => (),
        }
    }
    brackets.is_empty()
}
```

上述的巧妙在于，遇到 "( [ {" 不是把它们自身放到栈里，而是把它们的另一半放到栈里。

### DOT DSL

开始进入 Medium 级别难度了。一来就写个 DSL?? 我...

考查点：builder pattern, derive, modules ...

问题：其实并不是要写 DSL，只是实现 Graph/Node/Edge 三种 struct 而已。

实现：

```rust
pub mod graph {

    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    fn covert_attrs(attrs: &[(&str, &str)]) -> HashMap<String, String> {
        let mut new_attrs = HashMap::new();
        for item in attrs {
            new_attrs.insert(String::from(item.0), String::from(item.1));
        }
        new_attrs
    }

    pub mod graph_items {
        use std::collections::HashMap;

        pub mod node {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                attrs: super::HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: String::from(name),
                        attrs: super::HashMap::new(),
                    }
                }

                // pub fn with_attrs<'a>(&'a mut self, attrs: &[(&str, &str)]) -> &'a mut Self {
                //     for item in attrs {
                //         self.attrs
                //             .insert(String::from(item.0), String::from(item.1));
                //     }
                //     self
                // }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Node {
                        attrs: crate::graph::covert_attrs(attrs),
                        // must put it in the end
                        ..self
                    }
                }

                pub fn get_attr(&self, attr_key: &str) -> Option<&str> {
                    // self.attrs.get(attr_key).map(|x| (*x).as_str())
                    // self.attrs.get(attr_key).map(|x| x.as_str())
                    // self.attrs.get(attr_key).map(|x| (*x).as_ref())
                    self.attrs.get(attr_key).map(|x| x.as_ref())
                }
            }
        }

        pub mod edge {

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                node_start: String,
                node_end: String,
                attrs: super::HashMap<String, String>,
            }

            impl Edge {
                pub fn new(node_start: &str, node_end: &str) -> Self {
                    Edge {
                        node_start: String::from(node_start),
                        node_end: String::from(node_end),
                        attrs: super::HashMap::new(),
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Edge {
                        attrs: crate::graph::covert_attrs(attrs),
                        ..self
                    }
                }
            }
        }
    }

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(self, nodes: &Vec<Node>) -> Self {
            Graph {
                nodes: nodes.clone(),
                ..self
            }
        }

        pub fn with_edges(self, edges: &Vec<Edge>) -> Self {
            Graph {
                edges: edges.clone(),
                ..self
            }
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            Graph {
                attrs: crate::graph::covert_attrs(attrs),
                ..self
            }
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|&item| item.name == name)
        }
    }
}
```

学习到的：

- mod 的组织
- Vec 的 clone
- Rust 中的 spread 操作是两个点，且只能放到最后，不能放在别的位置
- `get_attr()` 方法的实现，有待进一步学习。&String 可以自动解引用为 &str，但 `Option<&String>` 并不能自动解引用为 `Option<&str>`，String 转成 &str 可以用 `as_str()` 或 `as_ref()`，再学习一下类型转换
- HashMap
- iterator find()

看了社区的一些解决方法，结构体的成员方法第一个参数还可以传 mut self，如下例所示：

```rust
pub fn with_nodes(mut self, nodes: &[graph_items::node::Node]) -> Self {
    nodes.iter().for_each(|node| {
        self.nodes.push(node.clone());
    });
    self
}
```

### Simple Linked List

- [Learn Rust With Entirely Too Many Linked Lists](https://github.com/rust-unofficial/too-many-lists)
- [如何在 Rust 中实现链表](https://mp.weixin.qq.com/s/wpqrvyLbt3SySFC5MQWvOA)

从上面两篇文章学习到了如何用 rust 写链表，相比其它语言确实麻烦，主要由所有权导致的。

关键知识点：

- Box
- Option
- 所有权
- std::mem::replace(&mut option, None) 可以用 `option.take()` 替代
- option.as_ref() 可以把 `Option<T>` 变成 `Option<&T>`
- push() 操作既可以插入在链表头，也可以插入在链表尾，插入在链表头使操作最方便

详细的笔记记录在 rust-study repo 中。

### Pascal's Triangle

考查点：math，index，vectors。

杨辉三角。vector 中嵌套 vector，简单的数学问题。
