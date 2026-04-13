use std::env;

fn main()
{
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 
    {
        println!("No arguments provided.");
        return;
    }
    for (i, arg) in args.iter().skip(1).enumerate()
    {
        println!("{}: {}", i + 1, arg);
    }
}
