#![deny(missing_docs)]

pub fn add_two(a: i32) -> i32 {
    a + 4
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
