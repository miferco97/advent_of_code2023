use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::exit;

fn create_number(v_num: &Vec<u32>) -> u32 {
    if v_num.len() == 1 {
        return 10 * v_num[0] + v_num[0];
    }
    return 10 * v_num[0] + v_num[v_num.len() - 1];
}

fn parse_numbers(str: &str) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    let lines: Vec<String> = str.split("\n").map(|x| String::from(x)).collect();
    print!("{:?}", lines);
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let mut v_num: Vec<u32> = Vec::new();
        for char in line.chars() {
            if let Some(t) = char.to_digit(10) {
                v_num.push(t);
            }
        }
        if v_num.len() == 0 {
            continue;
        }
        v.push(create_number(&v_num))
    }
    v
}


#[derive(Debug)]
struct IndexValue {
    value: u32,
    index: usize,
}
impl IndexValue {
    fn new() -> IndexValue {
       IndexValue { value:0, index:0 }
    }
}


fn get_min_and_max_index(v1:&Vec<(u32,usize)>)->(IndexValue,IndexValue){
    let mut min_index = usize::MAX;
    let mut max_index:usize = 0;
    let mut min:IndexValue= IndexValue::new();
    let mut max:IndexValue = IndexValue::new();

    for (value,index) in v1 {
        // println!("value : {}",value);
        if index < &min_index {
            min_index = *index;
            min = IndexValue{value:*value,index:*index};
            // println!("changing min value : {:?}",min);
        }
        if index > &max_index || max_index == 0{
            max_index = *index;
            max = IndexValue{value:*value,index:*index};
            // println!("changing max value : {:?}",max);
        }
    }
    (min,max)

}

fn generate_number_from_vectors(v1:&Vec<(u32,usize)>, v2:&Vec<(u32,usize)>) -> u32 {
    let mut v = v1.clone();
    v.extend(v2);
    println!("v is {:?}",v);
    let (min_1,max_1) = get_min_and_max_index(&v);
    // println!("min_index : {:?}, max_index : {:?}", min_1,max_1);

    return 10*min_1.value+max_1.value;
}

fn find_all<'a>(slice:&'a str, key:&'a str)->Option<Vec<usize>>{
    let mut slice_ref = slice;
    let mut values:Vec<usize> = Vec::new();

    while slice_ref.len() > 0 {
        match slice_ref.find(key) {
            Some(index)=> {
                values.push(slice.len()-slice_ref.len()+index);
                // println!("{}",slice_ref);
                // if slice_ref.len()<(index-2){
                //     return Some(values);
                // }
                slice_ref=&slice_ref[index+key.len()..];
            }
            None => {break;}
        }
    } 
    Some(values)
}

fn parse_number_from_letters(str: &str) -> u32 {
    let local_string = String::from(str);
    let mut numbers_map: Vec<(String, u32)> = Vec::new();
    numbers_map.push((String::from("one"), 1));
    numbers_map.push((String::from("two"), 2));
    numbers_map.push((String::from("three"), 3));
    numbers_map.push((String::from("four"), 4));
    numbers_map.push((String::from("five"), 5));
    numbers_map.push((String::from("six"), 6));
    numbers_map.push((String::from("seven"), 7));
    numbers_map.push((String::from("eight"), 8));
    numbers_map.push((String::from("nine"), 9));

    let mut number_letter: Vec<(u32, usize)> = Vec::new();

    for key_value in numbers_map {
        let (key, value) = key_value;
        if let Some(occurrences) = find_all(local_string.as_str(),key.as_str()){
            // println!("found {} at index {}",value,index);
            for index in occurrences{
                number_letter.push((value, index))}
        }
    }
    // println!("letter numbers are {:?}", number_letter);
    // let num = parse_numbers(str);
    let mut v_num:Vec<(u32,usize)> = Vec::new();
    // println!("line numbers are {:?}", num);
    for (index,char) in str.chars().enumerate() {
        if let Some(t) = char.to_digit(10) {
            v_num.push((t,index));
        }
    }
    // println!("line numbers are {:?}", v_num);

    let number = generate_number_from_vectors(&number_letter, &v_num);
    println!("number of this line is : {}", number);

    number
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Filename not provided");
        exit(0);
    }
    let filename: &str = args[1].as_str();
    let content = fs::read_to_string(filename).expect("Error reading the file");
    let lines: Vec<&str> = content.split("\n").collect();

    let mut sum:u32 =0 ;
    for line in lines {
        sum += parse_number_from_letters(&line);
    }

    println!("The sum is {}",sum);

}
