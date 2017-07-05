#[cfg(not(test))]
fn main() 
{
    println!("Hello, world!");
}

#[cfg(test)]
mod test 
{
    #[test]
    fn equals_test() 
    {
        assert!(1 == 2);
    }
}
