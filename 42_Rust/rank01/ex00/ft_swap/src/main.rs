fn ft_swap(a: &mut i32, b: &mut i32)
{
    let c;

    c = *a;
    *a = *b;
    *b = c;
}

fn main()
{
    let mut x = 10;
    let mut y = 20;

    println!("{} e {}", x, y);

    ft_swap(&mut x, &mut y);

    println!("{} e {}", x, y);
}