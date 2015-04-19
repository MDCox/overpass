use std::collections::HashMap;

pub fn eval(inp: String, db: &mut super::Graph) {
    parse(inp);
}

#[derive(Debug, Clone)]
enum TOKEN {
    //   Name    Type    Props
    NODE(String, String, HashMap<&'static str, &'static str>),
    EDGE(String, String, HashMap<&'static str, &'static str>),
    KEYW(String),
    NAME(String)
}

fn parse(inp: String) -> Vec<TOKEN> {
    let mut toks: Vec<TOKEN> = vec!();
    let mut pieces = inp.chars();

    let mut val = String::new();
    
    'parse: loop {
        let t = pieces.next().unwrap_or('~');

        match &*val {
            "MATCH"|"MERGE"|"CREATE"|"RETURN" => { 
                toks.push(TOKEN::KEYW(val.clone()));
                val.clear();
                continue;
            }, _ => ()
        }

        match t {
            // newlines mean nothing
            '\n'| ' ' => (),

            // Start an element
            '(' | '[' => (), 

            // End an element
            ')' => { toks.push(node_tok(val.clone())); val.clear() },
            ']' => { toks.push(edge_tok(val.clone())); val.clear() },
            
            // TODO: GET RID OF THIS STUPID TERRIBLE UGLY HACK
            '~' => break,

            // Deets
            'a'...'z' | 'A'...'Z' | '0'...'9' | 
            '{' | '}' | _ => val.push(t),
        }

    }

    println!("{:?}", toks);
    toks
}

fn keyw_tok(inp: String) -> TOKEN {
    TOKEN::KEYW(inp)
}

fn node_tok(inp: String) -> TOKEN {
    let mut props: HashMap<&'static str, &'static str> = HashMap::new();
    let name = String::new();
    let node_type = String::new();

    for c in inp.chars() {
        if name.len() > 0 {
            
        }
    }

    TOKEN::NODE(name, node_type, props)
}

fn edge_tok(inp: String) -> TOKEN {
    let mut props: HashMap<&'static str, &'static str> = HashMap::new();
    let name = String::new();
    let edge_type = String::new();


    TOKEN::EDGE(name, edge_type, props)
}
