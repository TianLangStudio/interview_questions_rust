pub enum Fruit {
    Apple,
    Orange,
    Unknown
}

pub fn print_name(fruit: &Fruit) {
    let name = match fruit {
        Fruit::Apple => "Apple",
        Fruit::Orange => "Orange",
        _ => panic!("{} ","Unknown Fruit")
    };
    print!("Fruit Name: {}", name)
}

/*pub fn return_never() -> ! {
    //panic!("Never return")
}*/


pub fn result_with_never() -> Result<String, ()> {
    Ok(String::from("Ok"))
}
/*
#[cfg(test)]
mod tests {
    use crate::never::return_never;

    #[test]
    fn test_return_never() {
        return_never();
    }
}*/