pub fn find_end_of_text(data: &[u8], is_unicode: bool) -> Result<i32, &'static str> {
    let mut found_eot = (false, 0);
    for (i, val) in data.iter().enumerate() {
        match is_unicode {
            true => {
                if *val == 0 {
                    if found_eot.0 == true && found_eot.1 == i - 1 {
                        return Ok((i + 1) as i32);
                    } else {
                        found_eot = (true, i);
                    }
                } else {
                    found_eot = (false, i);
                }
            },
            false => {
                if *val == 0 {
                    return Ok((i + 1) as i32);
                }
            }
        }
    }
    Err("Not a text field")
}

pub fn read_text(data: &[u8], is_unicode: bool) -> (String, usize) {
    let eot = find_end_of_text(data, is_unicode).unwrap() as usize;
    let value = String::from_utf8_lossy(&data[0..eot-1]).to_string(); // Ignoring \0
    (value, eot + 1)
}