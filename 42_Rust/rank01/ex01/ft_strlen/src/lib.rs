pub fn ft_strlen(s: &str) -> usize {
    let mut i = 0;

    for _ in s.as_bytes() {
        i += 1;
    }
    i
}

pub fn ft_str_is_alpha(s: &str) -> bool {
    for i in s.bytes() {
        let is_up = i >= b'A' && i <= b'Z';
        let is_lower = i >= b'a' && i <= b'z';

        if !is_up && !is_lower {
            return false;
        }
    }
    true
}

pub fn ft_strupcase(s: &str) -> String {
    let mut resultado = String::new();

    for i in s.bytes() {
        let is_lower = i >= b'a' && i <= b'z';

        if is_lower {
            let upper = (i - 32) as char;
            resultado.push(upper);
        } else {
            resultado.push(i as char);
        }
    }
    resultado
}

pub fn ft_strcapitalize(s: &str) -> String {
    let mut resultado = String::new();
    let mut interruptor = true;

    for i in s.bytes() {
        let is_up = i >= b'A' && i <= b'Z';
        let is_lower = i >= b'a' && i <= b'z';
        let is_numbers = i >= b'0' && i <= b'9';

        if is_lower || is_numbers || is_up {
            if interruptor == true {
                if is_lower {
                    resultado.push((i - 32) as char);
                } else {
                    resultado.push(i as char);
                }
                interruptor = false;
            } else {
                if is_up {
                    resultado.push((i + 32) as char);
                } else {
                    resultado.push(i as char);
                }
            }
        } else {
            resultado.push(i as char);
            interruptor = true;
        }
    }
    resultado
}

