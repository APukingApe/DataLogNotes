
use std::any::Any;
use std::rc;
use std::collections::HashMap;

use ascent::*;
use ascent::aggregators::*;

ascent! {
    struct Foo;

    // relation
    relation foo(i32, String);
    relation bar(i32, i32);
    relation foobar(String);

    // fact
    // Extensional datbase
    foo(1, "a".to_string());
    foo(2, "b".to_string());

    // rule

    // project, selction
    bar(x, x+1) <-- foo(x, y), if *x == 1;

    // join
    foobar(y) <-- foo(x, y), bar(x, _);

    // Horn Clause
    // ∀x, y, if foo(x, y) ∧ bar(x, _) then foobar(x).
    // logic programming

    // there is a edge `x:i32`, `y:i32` in the graph, where x, y are the node of the graph
    relation edge(i32, i32);
    //  there a path from node `x:i32` and node `y:i32` in graph stored in database
    relation path(i32, i32);
    relation path_len(i32, i32, i32);

    edge(1,2);
    edge(1,3);
    edge(2,4);
    edge(3,4);

    // `x:i32` is a node in graph
    relation node(i32);
    node(x), node(y) <-- edge(x, y);

    path(x, y), path_len(x, y, 1) <-- node(x), edge(x, y);
    path(x, z), path_len(x, z, m + 1) <-- path(x, y), edge(y, z), path_len(x , y, m);

    relation mini_column(i32);

    mini_column(min_m) <-- node(x), node(y), agg min_m = max(m) in path_len(x, y, m);
}

fn main() {
    // let mut prog = Foo::default();

    // prog.run();

    // println!("foo : {:?}", prog.foo);
    // println!("bar : {:?}", prog.bar);
    // println!("edge : {:?}", prog.edge);
    // println!("path len: {:?}", prog.path_len);
    // Vector forll type

    
    let x = 267;
    //x = 2;
    let mut y = 1;
    y = 2;
    y = x;
    let i:String = String::from("5");
    println!("{:?}", i);
    let mut z:usize = 23;
    z = x;

    let mut vec = Vec::new();
    vec.push(x);
    println!("{:?}", vec);
    let vec1 = vec; // vec is empty
    let vec2 = vec![1];
    println!("{:?}", vec2);
    //vec.push(y);
    //vec1.push(z);
    //let vec2 = vec;


    println!("{}, {}", z, x);
    //println!("{}", vec1[1]);
    println!("{:?}\n", return_zero());
    println!("{:?}", partial_add(1234)(3));
}

// Unit type () return empty 
fn compute_path() -> (){
    // Statement do not ends with ; means it returns
    ()
}

// i32, i64
fn return_zero() -> Nat {
    Nat::Z()
}

// Sum type, union type
// ::=  (Ty + ...)
enum EitherType<T1, T2> {
    Left(T1),
    Right(T2),
}

// Product type
// ::= (Ty × ...)
// (i32, i32)

// add: i32 -> i32 -> i32
fn add(x: i32, y: i32) -> i32 {
    x + y
}
// Arrow / Closure type
// add: i32 -> (i32 -> i32)
fn partial_add(x: i32) -> impl Fn(i32) -> i32   {
    let f = move |y: i32| {
        x + y
    };
    f
}

// Procedure macro
#[derive(Debug)]
enum Nat {
    Z(),
}

fn pointer() {
    
}

#[derive(Debug)]
enum List<T> {
    None(),
    Cons(T, Box<List<T>>), // 1, (2, (3, 4))
}

#[test]
fn test_list() {
    let none: List<i32> = List::None();
    let x: List<i32> = List::Cons(1, Box::new(none));
    let y: List<i32> = List::Cons(3, Box::new(x)); // To do: Use macro to overwrite
    println!("{:?}", y);
}
// fn list_length<T>(input: &List<T>) -> usize {
//     match input {
//         List::None() => 0,
//         List::Cons(_, tail) => 1 + list_length(tail) 
//     }
// }

#[test]
fn test_list_length(){
    let none: List<i32> = List::None();
    let x: List<i32> = List::Cons(1, Box::new(none));
    let y: List<i32> = List::Cons(3, Box::new(x)); // To do: Use macro to overwrite}
    y.length();
    //list_length(y);
    //assert_eq!(LengthMeasure::length(&y), 2);
    assert_eq!(y.length(), 2);
}

trait LengthMeasure {
    // fn length(x: &Self) -> usize;
    fn length(&self) -> usize;
}

impl<T> LengthMeasure for List<T> {
    // fn length(x: &Self) -> usize {
    //     list_length(x)
    // }
    fn length(&self) -> usize {
        match self {
            List::None() => 0,
            List::Cons(_, tail) => 1 + tail.length()
        }
    }
} 