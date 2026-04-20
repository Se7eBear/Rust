use ft_strlen::*;

fn main() {
    let frase = "oi, bom dia seNHOR";
    let texto = "Sete";
    let len = ft_strlen(texto);
    let alpha = ft_str_is_alpha(texto);
    let upper = ft_strupcase(texto);
    let capitalize = ft_strcapitalize(frase);

    println!("({}) your len is: {}", texto, len);
    println!("{}", ft_strlen(""));
    println!("{} is a: {}", texto, alpha);
    println!("{} = lower, {} = upper", texto, upper);
    println!("{}", capitalize);
}