const WIDTH: usize = 10;
const HEIGHT: usize = 5;

fn main() {
    let mut output = String::new();

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            if row == 0 || row == HEIGHT - 1 || col == 0 || col == WIDTH - 1 || row == col || col == WIDTH - row - 1 {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output);
}
