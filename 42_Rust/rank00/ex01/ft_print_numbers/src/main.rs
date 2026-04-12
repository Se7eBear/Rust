fn main()
{
    let mut i: u8 = 0;
    while i <= 9
    {
        let chara = (b'0' + i) as char;
        print!("{}", chara);
        i+=1;
    }
    print!("\n");
}
