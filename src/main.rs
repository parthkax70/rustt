#[derive(Debug)]
struct HelloMess<T> {
    name: T,
    age: i32,
}

fn main() {
    let p1 = HelloMess {
        name: 4267,
        age: 745
    };

    let name = p(format!("{:?}", p1));

    let hello_name = format!("Hello, {}!", name);

    println!("{}", hello_name);

}

fn p<T>(n: T) -> T
where
    T: ToString,
{
    let p: String = n.to_string();

    println!("{}", p);

    n
}
