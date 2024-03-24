use std::{array, io};

fn main() {
    //println!("Hello, world!");
    //println!("hhhhuuuii rast11111hvdd hdd");
    let mut age: u32 = 25;
    age = 11;
    println!("My age is {}", age);

    // string types examples
    let mut name: &str = "John"; // not garantee
    let mut name1 = String::from("John123");
    println!("{}", name);
    println!("{}", name1);

    // char bool types
    let symbol = 's';
    println!("symbol --> {}",symbol);

    let logic = true;
    println!("logic --> {}",logic);


    // if operator
    let age_if = 16;
    if age_if >= 18 {
        println!("YES!");
    } else {
        println!("nonononnnno!");
    }

    // variable & if variable
    let b_var = true;
    let num = if b_var {
        1 
    } else {
        0 
    };
    println!("num if variable --> {}",num);

    //================================================================
    // loops in rust
    println!("for loop");
    for i in 0..10 {
        println!("i --> {}",i);
    }

    // while loop
    println!("while loop");
    let mut i = 0;
    while i < 10 {
        println!("i --> {}",i);
        i += 1;
    }

    // loop
    println!("loop loop");
    i = 0;
    loop {
        println!("loop ....");
        //i = i + 1;
        i += 1; // no this increment i++ !!!!!!!!

        if i == 10 {
            break;
        }
    }

    i = 8;
    match i {
        10 => println!("10 match"),
        0..=9 => println!("hui loop"),
        _ => println!("nothing")
    };


    /*// quadratic equaluation
    let mut a_str: String = String::new();
    let mut b_str: String = String::new();
    let mut c_str: String = String::new();

    println!("evaluete equaluation ->");
    println!("enter A:");
    match io::stdin().read_line(&mut a_str) {
        Ok(_) => println!("A: {}", a_str),
        Err(e) => println!("Error: {:?}", e)
    }
    println!("enter B:");
    match io::stdin().read_line(&mut b_str) {
        Ok(_) => println!("B: {}", b_str),
        Err(e) => println!("Error: {:?}", e)
    }
    println!("enter C:");
    match io::stdin().read_line(&mut c_str) {
        Ok(_) => println!("C: {}", c_str),
        Err(e) => println!("Error: {:?}", e)
    }

    let a:f32 = a_str.trim().parse().unwrap();
    let b:f32 = b_str.trim().parse().unwrap();
    let c:f32 = c_str.trim().parse().unwrap();

    println!("A = {}, B = {}, C = {}", a,b,c);

    let d: f32 = b*b - 4.0 * (a*c);
    println!("descriminant {}", d);

    if d > 0.0 {
        let x1: f32 = (-b + d.sqrt()) / (2.0 * a);
        let x2: f32 = (-b - d.sqrt()) / (2.0 * a);
        println!("x1 = {}, x2 = {}", x1, x2);
    } else {
        println!("no solution");
    }*/


    // arrays i RUST
    let array = [3,5,2,7,9,0,3,1,4,5,2,9,88,4,5,76,4,];
    println!("{:?}", array);

    let mut i: usize = 0;
    while i < array.len() {
        println!("i --> {}",array[i]);
        i += 1;
    }

    for i in array.iter()  {
        println!("i --> {}", i);
    }

    // constants
    const GG: u8 = 6;

    //tules
    let pupil = (String::from("Ruslan"), 11, 12);
    println!("pupil --> {:?}", pupil);

    let (name, grade1, grade2) = pupil;
    println!("name = {}, grade1 = {}, grade2 = {}", name, grade1, grade2);

    //function =================================================================
    let (sum, sub, mul, stringgg)= math(1,2);
    println!("sum = {}, sub = {}, mul ={}, String = {}",sum,sub, mul, stringgg);

    //slice =================================================================
    let str_sl = String::from("TEXT STRING");
    let arr_sl = [2,4,6,8,10];
    let slice = &str_sl[..4];
    let arr_slice = &arr_sl[1..3];
    println!("slice str = {}, slice arr = {:?}", slice, arr_slice);

    // structures =================================================================
    #[derive(Debug)]
    struct Person{
        name: String,
        age: u32,
        grade: u32
    };

    #[derive(Debug)]
    struct Person123 (u32, u64, String);

    let person = Person{
        name: String::from("Ruslan"),
        age: 11,
        grade: 12
    };
    let person_tuple = Person123(11,12,"helll".to_string());
    println!("person1 --> {:?}, person2 -->{:?}", person, person_tuple);

}


fn math(a: i32, b: i32) -> (i32, i32, i32, String) {
    (a+b, a-b, a*b, String::from("hui"))
}
