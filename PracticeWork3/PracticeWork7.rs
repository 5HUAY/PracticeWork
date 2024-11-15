fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return None; 
    }

    let target = total / n;
    let mut moves = 0;

    for &shipment in shipments {
        if shipment > target {
            moves += shipment - target;
        }
    }

    Some(moves)
}

fn gen_shipments(n: usize) -> Vec<u32> {

    let base: u32 = 10;
    let mut shipments = Vec::new();
    

    let total = base * n as u32;
    for _ in 0..n {
        shipments.push(base);
    }

    shipments
}

fn main() {
    let shipments = vec![20, 30, 40];
    match count_permutation(&shipments) {
        Some(moves) => println!("Мінімальна кількість переносу: {}", moves),
        None => println!("Не можливо рівномірно розподілити вантаж."),
    }

    let generated_shipments = gen_shipments(5);
    println!("Згенеровані вантажі: {:?}", generated_shipments);
}
