pub mod processor;
pub mod resolver;
pub mod executor;

pub enum KodaType {
    Data,
    Variable,
    Pointer,
    Execution,
    Nest,
    Empty,
}

pub struct KodaNode {
    pub label: String,
    pub content: String,
    pub k_type: KodaType,
    pub indent: usize,
    pub children: Vec<KodaNode>,
}

pub fn tokenize(line: &str) -> KodaNode {
    let stripped = line.trim_start();
    let indent_level = line.len() - stripped.len();

    if stripped.is_empty() || stripped.starts_with('#') {
        return KodaNode {
            label: String::new(),
            content: String::new(),
            k_type: KodaType::Empty,
            indent: 0,
            children: Vec::new(),
        };
    }

    let mut parts = stripped.splitn(2, ':');
    let key = parts.next().unwrap_or("").trim();
    let val = parts.next().unwrap_or("").trim();

    let mut node = KodaNode {
        label: key.to_string(),
        content: val.to_string(),
        k_type: KodaType::Data,
        indent: indent_level,
        children: Vec::new(),
    };

    if key.starts_with('@') {
        node.k_type = KodaType::Variable;
        node.label = key[1..].to_string();
    }

    if val.starts_with('>') {
        node.k_type = KodaType::Pointer;
        node.content = val[1..].to_string();
    } else if val.starts_with('!') {
        node.k_type = KodaType::Execution;
        node.content = val[1..].to_string();
    } else if val.is_empty() {
        node.k_type = KodaType::Nest;
    }

    node
}

#[no_mangle]
pub extern "C" fn koda_parse(input: *const std::os::raw::c_char) -> *mut std::os::raw::c_char {
    // This allows C++, C#, Java to send a string and get a result
    std::ptr::null_mut() 
}
