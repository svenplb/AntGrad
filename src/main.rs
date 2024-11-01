#[derive(Clone)]
pub struct Value {
    pub data: f64,
    pub _prev: Vec<Value>,
    pub _op: String,
    pub label: String,
}

impl Value {
    fn init(data: f64, label: String, children: Vec<Value>) -> Value {
        Value {
            data,
            _prev: children,
            _op: "".to_string(),
            label: "".to_string(),
        }
    }

    fn repr(&self) -> String {
        format!("Value(data={})", self.data)
    }

    fn add(&self, other: &Value) -> Value {
        Value {
            data: self.data + other.data,
            _prev: vec![(*self).clone(), (*other).clone()],
            _op: "+".to_string(),
            label: String::new(),
        }
    }

    fn sub(&self, other: &Value) -> Value {
        Value {
            data: self.data - other.data,
            _prev: vec![(*self).clone(), (*other).clone()],
            _op: "-".to_string(),
            label: String::new(),
        }
    }

    fn mult(&self, other: &Value) -> Value {
        Value {
            data: self.data * other.data,
            _prev: vec![(*self).clone(), (*other).clone()],
            _op: "*".to_string(),
            label: String::new(),
        }
    }

    fn div(&self, other: &Value) -> Value {
        Value {
            data: self.data / other.data,
            _prev: vec![(*self).clone(), (*other).clone()],
            _op: "/".to_string(),
            label: String::new(),
        }
    }
}

pub fn draw_dot(root: &Value) -> Result<(), std::io::Error> {
    print_tree(root, "", true);
    Ok(())
}

fn print_tree(node: &Value, prefix: &str, is_last: bool) {
    let marker = if is_last { "└── " } else { "├── " };
    let node_str = if node._op.is_empty() {
        format!("{} = {:.2}", node.label, node.data)
    } else {
        format!("{} = {:.2} ({})", node.label, node.data, node._op)
    };
    println!("{}{}{}", prefix, marker, node_str);

    let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

    for (i, child) in node._prev.iter().enumerate() {
        let is_last_child = i == node._prev.len() - 1;
        print_tree(child, &child_prefix, is_last_child);
    }
}

fn main() {
    let a = Value::init(2.0, "a".to_string(), vec![]);
    let b = Value::init(3.0, "b".to_string(), vec![]);
    let c = Value::init(3.0, "b".to_string(), vec![]);
    let d = Value::init(3.0, "b".to_string(), vec![]);
    let e = Value::init(3.0, "b".to_string(), vec![]);

    let f = a.add(&b);
    let g = c.add(&d);
    let h = f.mult(&g);
    let i = h.div(&e);
    let j = i.sub(&a);
    println!("{}", j.repr());

    draw_dot(&c).unwrap();
}
