pub fn ft_atoi(s: &str) -> Result<i64, String> {
    let mut resultado: i64 = 0;
    let mut i = 0;
    let mut sinal: i64 = 1;
    let byte = s.as_bytes();

    if byte.is_empty(){
        return Err("invalido".to_string());
    }
    while i < byte.len() && (byte[i] == b' ' || (byte[i] >= 9 && byte[i] <= 13)) {
        i += 1;             
        }
    while i < byte.len() && (byte[i] == b'-' || byte[i] == b'+'){
        if byte[i] == b'-'{
            sinal = -1;
        }
        i += 1;
    }
    while i < byte.len() && (byte[i] >= b'0' && byte[i] <= b'9'){
        resultado = resultado * 10 + (byte[i] - b'0') as i64;
        i += 1;
    }
    Ok(resultado * sinal)
}
// fn ft_itoa(n: i64) -> String {

// }
