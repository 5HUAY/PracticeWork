fn is_palindrome(number: i32) -> bool {

    let num_str = number.to_string();

    num_str == num_str.chars().rev().collect::<String>()
}

fn main() {
    let number = 12321;
    if is_palindrome(number) {
        println!("{} є паліндромом.", number);
    } else {
        println!("{} не є паліндромом.", number);
    }
}
