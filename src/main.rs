mod processor;
mod resolver;
use crate::processor::KodaTree;
use crate::resolver::Resolver;

fn main() {
    let raw_input = "
@api: https://v1.api.com
service:
    url: @api/users
    config: >./local_settings.json
";

    let mut tokens = Vec::new();
    for line in raw_input.lines() {
        tokens.push(crate::tokenize(line));
    }

    let mut tree = KodaTree::build(tokens);
    let mut res = Resolver::new();

    res.collect_variables(&tree.root);
    
    for node in &mut tree.root {
        res.resolve(node);
    }
}
