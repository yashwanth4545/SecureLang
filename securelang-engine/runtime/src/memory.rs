use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    String(String),
    Bool(bool),
    SecureString(String), // Specialized for memory safe secrets
    ObjectRef(usize),     // Pointer to the GC heap
}

pub struct GCHeap {
    objects: HashMap<usize, Object>,
    next_id: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Instance { class_name: String, fields: HashMap<String, Value> },
    Array(Vec<Value>),
}

impl GCHeap {
    pub fn new() -> Self {
        GCHeap {
            objects: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn allocate(&mut self, obj: Object) -> usize {
        let id = self.next_id;
        self.objects.insert(id, obj);
        self.next_id += 1;
        id
    }

    pub fn get(&self, id: usize) -> Option<&Object> {
        self.objects.get(&id)
    }

    pub fn mark_and_sweep(&mut self, roots: &[Value]) {
        println!("[GC] Running Mark and Sweep phase...");
        let mut marked = std::collections::HashSet::new();

        for root in roots {
            if let Value::ObjectRef(id) = root {
                self.mark(*id, &mut marked);
            }
        }

        let mut to_remove = Vec::new();
        for key in self.objects.keys() {
            if !marked.contains(key) {
                to_remove.push(*key);
            }
        }

        for key in to_remove {
            println!("[GC] Sweeping object ID {}", key);
            self.objects.remove(&key);
        }
    }

    fn mark(&self, id: usize, marked: &mut std::collections::HashSet<usize>) {
        if marked.insert(id) {
            if let Some(obj) = self.objects.get(&id) {
                match obj {
                    Object::Instance { fields, .. } => {
                        for val in fields.values() {
                            if let Value::ObjectRef(child_id) = val {
                                self.mark(*child_id, marked);
                            }
                        }
                    }
                    Object::Array(items) => {
                        for val in items {
                            if let Value::ObjectRef(child_id) = val {
                                self.mark(*child_id, marked);
                            }
                        }
                    }
                }
            }
        }
    }
}
