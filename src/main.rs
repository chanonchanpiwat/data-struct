use std::collections::{HashSet, BTreeSet};

enum Element {
    Number(i32),
    Boolean(bool),
}

// set
/*
{
    fast find
    fast remove
    slow insert 
}: compared to vec

non duplicate
sorted ??
*/
fn get_set() {
    // two type of imp detail
    let mut set = HashSet::new();
    let mut _b_set: BTreeSet<i32> = BTreeSet::new();
    
    // insert
    set.insert(2);
    set.insert(6);

    // get
    set.get(&1);

    // remove
    let _is_removed = set.remove(&2);
}


// static array
fn get_array() -> [i32; 2] {
    let a = [1, 3];
    a
}

// dynamic array
fn get_vector() -> Vec<Element> {
    // three way initialize
    let mut vec1: Vec<Element> = Vec::new();

    //push
    vec1.push(Element::Number(1));
    vec1.push(Element::Boolean(true));

    let mut _vec2 = vec![1,2];

    let mut vec3 = Vec::with_capacity(2);
    vec3.push(1);
    vec3.push(2);


    // resize
    _vec2.resize(5, 89);
    vec1.resize_with(5, || {
        return  Element::Number(89)
    });
    
    //pop
    vec1.pop();
    
    //insert
    _vec2.insert(1, 5); 

    //erase
    _vec2.remove(2);

    //iter
    let _found = vec1.iter().find(|e| {
        return  match e {
            Element::Number(v) => v == &1,
            Element::Boolean(_) => false,
        };
    });

    _vec2.sort();

    vec1
}

fn get_tuple() -> (i32, i32) {
    (1, 2)
}

fn main() {
    let arr = get_array();
    let a1 = arr.get(1).unwrap();
    let a0 = match arr.get(0) {
        Some(v) => v,
        None => panic!(),
    };

    let vec = get_vector();
    let vec1 = match vec.get(1) {
        Some(v) => match v {
            Element::Number(v) => v,
            Element::Boolean(_) => &0,
        },
        None => panic!(),
    };
    let size = vec.len();

    let (t1, t2) = get_tuple();

   let aa = Element::Number(2);

   get_set();

    println!("array: a0:{a0} a1:{a1}");
    println!("vector: vec1:{vec1} size:{size}");
    println!("tuple t1:{t1} t2:{t2}");


  
}
