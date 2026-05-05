use crate::tokenize;
use crate::processor::KodaTree;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn koda_parse_to_json(input: *const c_char) -> *mut c_char {
    if input.is_null() { return std::ptr::null_mut(); }
    
    let c_str = unsafe { CStr::from_ptr(input) };
    let r_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let mut tokens = Vec::new();
    for line in r_str.lines() {
        tokens.push(tokenize(line));
    }

    let tree = KodaTree::build(tokens);
    
    let json_output = serialize_to_json(&tree);
    
    let c_string = std::ffi::CString::new(json_output).unwrap();
    c_string.into_raw()
}

fn serialize_to_json(tree: &KodaTree) -> String {
    let mut out = String::from("{");
    for (i, node) in tree.root.iter().enumerate() {
        out.push_str(&format!("\"{}\": \"{}\"", node.label, node.content));
        if i < tree.root.len() - 1 { out.push_str(","); }
    }
    out.push_str("}");
    out
}

#[no_mangle]
pub extern "C" fn koda_free_string(p: *mut c_char) {
    unsafe {
        if p.is_null() { return; }
        std::ffi::CString::from_raw(p);
    }
}
