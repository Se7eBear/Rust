fn ft_strlen(s: &str) -> usize
{
    let mut i = 0;

    for _ in s.as_bytes()
    {
        i += 1;
    }
    i
}

// fn main()
// {
//     let texto = "Sete";
//     let len = ft_strlen(texto);

//     println!("({}) your len is: {}", texto, len);

//     println!("{}", ft_strlen(""))
// }

