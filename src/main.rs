enum Element {
    Number(i32),
    Boolean(bool),
}

// static array
fn get_array() -> [i32; 2] {
    let a = [1, 3];
    a
}

// dynamic array
fn get_vector() -> Vec<Element> {
    let mut vec: Vec<Element> = Vec::new();
    vec.push(Element::Number(1));
    vec.push(Element::Number(2));
    vec.push(Element::Boolean(true));
    vec
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

    let (t1, t2) = get_tuple();

    println!("array: a0:{a0} a1:{a1}");
    println!("vector: vec1:{vec1}");
    println!("tuple t1:{t1} t2:{t2}");
}
