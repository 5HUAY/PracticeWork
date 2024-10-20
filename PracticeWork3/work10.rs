fn find_variables() {
    let mut solutions = Vec::new();

    for m in 1..=8 {
        for u in 1..=8 {
            if u == m { continue; }
            for x in 1..=8 {
                if x == m || x == u { continue; }
                for a in 1..=8 {
                    if a == m || a == u || a == x { continue; }
                    for s in 1..=8 {
                        if s == m || s == u || s == x || s == a { continue; }
                        for l in 1..=8 {
                            if l == m || l == u || l == x || l == a || l == s { continue; }
                            for o in 1..=8 {
                                if o == m || o == u || o == x || o == a || o == s || o == l { continue; }
                                for n in 1..=8 {
                                    if n == m || n == u || n == x || n == a || n == s || n == l || n == o { continue; }

                                    let muxa = m * 1000 + u * 100 + x * 10 + a;
                                    let x_a = x * 10 + a;
                                    let slon = s * 1000 + l * 100 + o * 10 + n;

                                    if muxa * x_a == slon {
                                        solutions.push((m, u, x, a, s, l, o, n));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    for (m, u, x, a, s, l, o, n) in &solutions {
        println!("  {}{}{}{}", m, u, x, a);
        println!("{}       {}", x, a);
        println!("---------");
        println!("  {}{}{}{}", s, l, o, n);
    }

    println!("Кількість рішень: {}", solutions.len());
}

fn main() {
    find_variables();
}
