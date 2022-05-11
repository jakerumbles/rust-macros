macro_rules! foo {
    (3) => {
        println!("It worked");
    };
    ( $($e:expr),* ) => {
        $(
            for i in 0..$e {
                println!("i: {}", i);
            }
        )*
    };
    ( ,$($e:expr),* ) => {
        $(
            println!("{}", $e);
        )*
    };
    (x => $e:expr) => (println!("Expression: {}", $e));
}

macro_rules! square {
    ($e:expr) => {
        $e * $e
    };
}

macro_rules! o_0 {
    (
        $(
            $x:expr; [ $( $y:expr ),* ]
        );*
    ) => {
        &[ $($( $x + $y ),*),* ]
    };
}

macro_rules! match_anything {
    () => {
        println!("This macro matches anything!");
    };
}

macro_rules! variable {
    ($v:ident) => {
        let $v = 5;
    };
}

macro_rules! write_html {
    ($w:expr, ) => {
        ()
    };

    ($w:expr, $e:tt) => {
        println!("e token tree: {}", $e);
        write!($w, "{}", $e)
    };

    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        write!($w, "<{}>", stringify!($tag));
        write_html!($w, $($inner)*);

        write!($w, "</{}>", stringify!($tag));
        write_html!($w, $($rest)*);
    }}
}

fn main() {
    use std::fmt::Write;

    // Testing pattern matching
    foo!(3);
    foo!(4, 5, 6);
    foo!(,1,2,3);
    foo!(x => 12);

    // Returning values
    let x = 40;
    let square = square!(x);
    println!("Square of {} is {}", x, square);

    // Nested loop in macro
    let x: &[i32] = o_0!(1; [1, 2, 3]; 2; [2, 3, 4]);
    assert_eq!(x, [2, 3, 4, 4, 5, 6]);

    match_anything!();
    match_anything!();

    // Access variable that was declared in macro
    variable!(z);
    println!("z: {}", z);

    // Recursive macro
    let mut out = String::new();
    write_html!(&mut out,
    html[
        head[title["Macros guide"]]
        body[h1["Macros are the best"]]
    ]);
}
