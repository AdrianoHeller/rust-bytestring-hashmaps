use std::collections;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read,ErrorKind};

fn main() {
    let name = "Straits".to_string();
    // Full substring
    let sub_string = &name[..];
    // Iterating over Splitted Scalar values
    for letter in sub_string.chars() {
        println!("{}",letter);
    }
    // Iterating over real bytes
    for letter in sub_string.bytes() {
        println!("{}",letter);
    }
    // This is the inner storing of a String
    let name:Vec<u8> = vec![83,116,114,97,105,116,115];
    // Associate and merge bytes as Utf-8 String
    let s = String::from_utf8(name).expect("Cannot convert into string.");

    println!("{}",s);

    let name = String::from("محمد");

    let num_bytes = name.bytes();

    let num_chars = name.chars();

    for i in num_bytes {
        println!("{}",i);
    }

    for c in num_chars {
        println!("{}",c);
    }

    let maome_in_bytes: Vec<u8> = vec![217,133,216,173,217,133,216,175];

    let mount_string = String::from_utf8(maome_in_bytes).unwrap();

    println!("{}",mount_string);

    // Hash Maps

    let mut team_scores = collections::HashMap::new();

    team_scores.insert(String::from("Yankees"),27);

    team_scores.insert(String::from("Jets"),21);

    println!("{:#?}",team_scores);

    let companies: Vec<String> = vec![String::from("Tesla"),String::from("Red Hat"),String::from("Mavics")];

    let valuation_in_billions: Vec<i32> = vec![200,8,2];

    // Joining and collecting iterated values from different vectors together
    let mut associated_valuation: HashMap<_,_> = companies.into_iter().zip(valuation_in_billions.into_iter()).collect();

    println!("{:#?}",associated_valuation);

    // Create new Heap String to query Hash keys
    let matched_value: String = String::from("Tesla");

    // Handle Return with Option type
    match associated_valuation.get(&matched_value) {
        Some(value) => println!("Value: {}", value),
        None => println!("There is no value to match search."),
    }

    // Check if key exists, if not, inserts it
    associated_valuation.entry(String::from("Mitsubishi")).or_insert(150);
    associated_valuation.entry(String::from("Tesla")).or_insert(150);

    // Iteration over hash map's entries -> key/val
    for (company,val) in &associated_valuation {
        println!("Company:{}\nValuation:{}\n",company,val);
    }

    let text_to_split: &str = "Say friend and you shall enter my good friend";

    let mut mellom: HashMap<_,_> = HashMap::new();

    for word in text_to_split.split_whitespace() {
        let counted_instances = mellom.entry(word).or_insert(0);
        *counted_instances += 1;
    }

    println!("{:#?}",mellom);

    let mut unordered_vec: Vec<i32> = vec![201,2,157,13,88];

    sort_vector_input(&mut unordered_vec);

    println!("{:?}",unordered_vec);

    let mut float_vec: Vec<f32> = vec![172.12,1.13,55.23,78.14];

    sort_float_vector_input(&mut float_vec);

    println!("{:?}",float_vec);

    // Avoid panicking with Result<T,E>
    match float_vec.get(100) {
        Some(item) => println!("{}",item),
        None => println!("No item available in that index range"),
    }

    let file_path: &str = "./src/props.json";

    let mut file_data = File::open(file_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_path).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}",error);
            })
        } else {
            panic!("Problem opening the file: {:?}",error);
        }
    });

    let mut contents: String = String::new();

    let f = File::open(file_path);

    let f = match f {
        Ok(mut file) => {
            file.read_to_string(&mut contents);
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create("./src/props.json") {
                    Ok(mut fc) => {
                        fc.read_to_string(&mut contents);
                    },
                    Err(e) => panic!("{:?}",e)
                }
            },
            err => {
                panic!("Problem opening file: {:?}",err)
            }
        },
    };

    println!("{:?}",contents);

    let vec_keys: Vec<String> = vec![String::from("item"),String::from("version")];
    let vec_values: Vec<String> = vec![String::from("config"),String::from("1.0.0")];

    let assert_data: HashMap<_,_> = vec_keys.into_iter().zip(vec_values.into_iter()).collect();

    println!("{:#?}",assert_data);

    assert_eq!(contents,"{\n  \"item\": \"config\",\n  \"version\": \"1.0.0\"\n}");

}

fn sort_vector_input(input_vec: &mut Vec<i32>) -> () {
    let sorted_vec = input_vec.sort();
    sorted_vec
}

fn sort_float_vector_input(float_vector_input: &mut Vec<f32>) -> () {
    let sorted_float_vec = float_vector_input.sort_by(|a,b| a.partial_cmp(b).unwrap());
    sorted_float_vec
}
