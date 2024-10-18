fn draw_tree(levels: usize) {
    (0..levels).for_each(|i| {
        let height = i + 3;
        let max_width = height * 2 - 1;

        (0..height).for_each(|j| {
            let stars = j * 2 + 1;
            let spaces = (max_width - stars) / 2;
            println!("{}{}", " ".repeat(spaces), "*".repeat(stars));
        });
    });


    let trunk_width = if levels % 2 == 0 { levels + 1 } else { levels };
    let trunk_height = levels;
    let max_width = (levels + 2) * 2 - 1;

    (0..trunk_height).for_each(|_| {
        let spaces = (max_width - trunk_width) / 2;
        println!("{}{}", " ".repeat(spaces), "*".repeat(trunk_width));
    });
}

fn main() {
    let triangles = 4;
    draw_tree(triangles);
}
