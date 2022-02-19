use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path of the first file to compare
    #[clap(short, long)]
    first_path: String,

    #[clap(short, long)]
    /// Path of the second file to compare
    second_path: String,

    #[clap(short, long)]
    /// Key to use for the comparison (sorting and comparison).
    key: String
}

fn get_sorted_array(path: &str, key: &str) -> std::collections::HashMap<String, serde_json::Value> {
    let file_content = std::fs::read_to_string(path).unwrap();
    
    let val: serde_json::Value = serde_json::from_str(&file_content).unwrap();

    if let serde_json::Value::Array(arr) = val {
        let mut vals: std::collections::HashMap<String, serde_json::Value> = std::collections::HashMap::new();
        for val in arr.iter() {
            vals.insert(val.as_object().unwrap().get(key).unwrap().to_string(), val.clone());
        }
        
        vals
    } else {
        panic!("Json file is not an array([{{}}, {{}}, ...]).");
    }
}

fn main() {
    let args: Args = Args::parse();
    let fst = get_sorted_array(&args.first_path, &args.key);
    let snd = get_sorted_array(&args.second_path, &args.key);


    for fkey in fst.keys() {
        if !snd.contains_key(fkey) {
            println!("{} does not contain {}", &args.second_path, fkey);
        } else if fst[fkey] != snd[fkey] {
            println!("Files differ on {}: {}", &args.key, fkey);
        }
    }

    for skey in snd.keys() {
        if !fst.contains_key(skey) {
            println!("{} does not contain {}", &args.first_path, skey);
        }
    }
}
