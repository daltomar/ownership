fn main() {
    let s = String::from("hello");
    
    let s1 = gives_ownership();

    let s2 = String::from("meloww");

    let s3 = takes_and_gives_back(s2);

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

}

fn gives_ownership() -> String {

    let some_string = String::from("yours");

    some_string

}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}