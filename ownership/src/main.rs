fn main() {
    //1. Cada valor no Rust tem uma variavel que é chamada de dono
    //2. Apenas um dono por vez
    //3. Quando o dono sai de scopo, o valor também é dropado

    /*
    {
        // s não é valido, pois não foi declarado
        let s = String::from("hello"); // s é valido pois foi declarado
        //fazer algo com s
    } //fim do scopo, s não é valido

     */
    /*
    let x = 5;
    let y = x; //copy
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}, world", s1);
    */
    /*
    let s = String::from("hello");
    takes_ownership(s);
    println!("{}", s);

    let x = 5;
    makes_copy(x);
    println!("{}", x); */

    /*
     let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("O tamanho de '{}' é  {}", s1, len);
    */

    /*
    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("{}", s1);
    */

    /*
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
    */

    /*
    let reference_to_nothing = dangle();
    */
    /*let mut s = String::from("hello world");
    let s2 = "hello world";

    let word = first_word(s2);
    */

    let a = [1, 2, 3, 4, 5];
    let slice =&a[0..2];
    println!("{:?}", slice);
}

/*fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
} */

/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
 */

/*fn change(some_string: &mut String) {
    some_string.push_str(", world");
} */
/*
fn calculate_length(s: &String) -> usize {

    let length = s.len(); //lens() retorna o tamanho da string
    length
} */

/*
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
*/
/*
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} */
