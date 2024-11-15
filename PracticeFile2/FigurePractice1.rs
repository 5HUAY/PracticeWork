fn main() 
{
    let SIZE: i32 = 5;

    for i in 0..SIZE 
    {
        for y in 0..SIZE - i - 1
        {
            print!(" ");
        }
        
        for z in 0..2 * i + 1
        {
            print!("*");
        }
        
        println!();
    }

    for i in (0..SIZE - 1).rev() 
    {
        for y in 0..SIZE - i - 1
        {
            print!(" ");
        }
        
        for z in 0..2 * i + 1
        {
            print!("*");
        }
        
        println!();
    }
}
