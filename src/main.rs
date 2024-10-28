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
    let mut prog = Foo::default();

    prog.run();

    println!("foo : {:?}", prog.foo);
    println!("bar : {:?}", prog.bar);
    println!("edge : {:?}", prog.edge);
    println!("path len: {:?}", prog.path_len);
}
