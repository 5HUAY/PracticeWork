use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..99)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> Option<(i32, i32)> {
    if data.len() < 2 {
        return None;
    }

    let mut min_sum = i32::MAX;
    let mut min_pair = (0, 0);

    for window in data.windows(2) {
        let sum = window[0] + window[1];
        if sum < min_sum {
            min_sum = sum;
            min_pair = (window[0], window[1]);
        }
    }

    Some(min_pair)
}

fn print_vector(data: &[i32]) {
    println!("Вектор: {:?}", data);
    if let Some((a, b)) = min_adjacent_sum(data) {
        println!("Мінімальна пара сусідніх елементів: ({}, {})", a, b);
    } else {
        println!("Вектор занадто малий для знаходження пари.");
    }
}

fn main() {
    rand = "0.8" 
    let random_vector = gen_random_vector(20);
    print_vector(&random_vector);
}
