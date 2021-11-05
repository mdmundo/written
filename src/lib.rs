pub fn caller() {
    let numbers = "654312".chars().collect::<Vec<char>>();
    let chunks = numbers.rchunks(3).enumerate();
    for_loop(chunks);
}

pub fn for_loop(chunks: std::iter::Enumerate<std::slice::RChunks<char>>) {
    for number in chunks {
        match number {
            (pow, values) => println!("{} {:#?}", pow, values),
        }
    }
}

pub fn call_gen(pow: usize, values: &[char]) {
    let append_thousands = get_thousands(pow);
    let hundreds = get_hundreds(values[0]);
    let is_teen = values[1] == '1';
    let (tens, units) = if is_teen {
        get_teens(values[2])
    } else {
        (get_tens(values[1]), get_units(values[1]))
    };
    // return hundreds and tens and units thousands...
}

pub fn get_thousands(number: usize) -> () {
    ()
}

pub fn get_hundreds(number: char) -> () {
    ()
}

pub fn get_teens(number: char) -> ((), ()) {
    ((), ())
}

pub fn get_tens(number: char) -> () {
    ()
}

pub fn get_units(number: char) -> () {
    ()
}

#[cfg(test)]
mod tests {
    #[test]
    fn correct_approach() {
        let numbers = "654312".chars().collect::<Vec<char>>();
        let chunks = numbers.rchunks(3).enumerate();

        for number in chunks {
            match number {
                (pow, values) => println!("{} {:#?}", pow, values),
            }
        }
    }

    #[test]
    fn gen_approach() {
        fn unit(val: char) -> Option<&'static str> {
            match val {
                '1' => Some("Um"),
                _ => None,
            }
        }
        fn gen(pow: usize, val: char) -> Option<&'static str> {
            match pow {
                0 => unit(val),
                _ => None,
            }
        }
        assert_eq!(gen(0, '1'), Some("Um"));
    }
}
