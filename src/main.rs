use std::{collections::HashSet, hash::Hash};

pub struct Value {
    data: f32,
    grad: f32,
    _backward: Box<dyn FnMut()>,
    // _prev: HashSet<*const Value>,
    _prev: Vec<*mut Value>,
    _op: String,
    label: String,
}

impl Value {
    pub fn new(data: f32, children: Vec<*mut Value>, op: &str, label: &str) {
        let v = Value {
            data: data,
            grad: 0.0,
            _backward: Box::new(|| {}),
            _prev: children,
            _op: op.to_string(),
            label: label.to_string()
        };
        v;
    }

    pub fn repr(&self) -> String {
        format!("Value(data={}", self.data)
    }

    pub fn backward(&mut self) {
        let mut topo: Vec<&mut Value> = Vec::new();
        let mut visited: HashSet<*mut Value> = HashSet::new();

        fn build_topo(v: &mut Value, topo: &mut Vec<&mut Value>, visited: &mut HashSet<*mut Value>) {
            if !visited.contains(&(v as *mut Value)) {
                visited.insert(v as *mut Value);
                for child in &mut v._prev {
                    build_topo(child, topo, visited);
                }
                topo.push(v);
            }
        }

        build_topo(self, &mut topo, &mut visited);

        self.grad = 1.0;
        for node in topo.iter().rev() {
            node._backward;
        }
    }
}

impl Value {
    pub fn add(&mut self, other: &mut Value) -> Value {
        let mut out = Value{
            data: self.data + other.data, 
            grad: 0.0, 
            _backward: Box::new(|| ()), 
            _prev: vec![self, other],
            _op: "+".to_string(), 
            label: "+".to_string(),
        };

        let _backward = || {
            self.grad += 1.0 * out.grad;
            other.grad += 1.0 * out.grad;
        };
        out._backward = Box::new(_backward);

        return out;
    }
}

// impl Value {
//     pub fn new(data: f32, children: Vec<&Value>, op: &str, label: &str) -> Self {
//         let mut v = Value {
//             data: data,
//             grad: 0.0,
//             _backward: Box::new(|| {}),
//             _prev: children.iter().map(|&child| child as *const _).collect(),
//             _op: op.to_string(),
//             label: label.to_string()
//         };

//         return v;
//     }

//     pub fn repr(&self) -> String {
//         format!("Value(data={})", self.data)
//     }

//     pub fn backward(&mut self) {
//         self.grad = 1.0;

//         let mut topo = Vec::new();
//         let mut visited = HashSet::new();

//         fn build_topo<'a>(v: &'a mut Value, topo: &mut Vec<&'a mut Value>, visited: &mut HashSet<*const Value>) {
//             if !visited.contains(&(v as *const _)) {
//                 visited.insert(v as *const _ );
//                 for child in v._prev.iter().map(|&child| unsafe { &mut *(child as *mut Value) }) {
//                     build_topo(child, topo, visited);
//                 }
//                 topo.push(v);
//             }
//         }

//         build_topo(self, &mut topo, &mut visited);

//         for node in topo.iter_mut().rev() {
//             (node._backward)();
//         }
//     }
// }

// impl Value {
//     pub fn add(&mut self, other: &Value) -> Value {
//         let mut result = Value::new(self.data + other.data, vec![self, other], "+", "");
        
//         let backward = {
//             let grad = &mut self.grad;
//             Box::new(move || {
//                 *grad += 1.0;
//             })
//         };

//         result._backward = backward;

//         return result;
//     }
// }

fn main() {
    println!("Hello, world!");
}
