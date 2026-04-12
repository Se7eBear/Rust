fn main()
{
    let mut i: u8 = b'a';
    while i <= b'z'
    {
        let charac = (i) as char;
        print!("{}", charac);
        i+=1;
    }
    print!("\n");
}
