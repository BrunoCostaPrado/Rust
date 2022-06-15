fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = get_largest(number_list);
    println!("O maior numero é {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let largest = get_largest(number_list);

    println!("O maior numero é {}", largest);
}

fn get_largest(number_list: Vec<i32>) -> i32 {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
