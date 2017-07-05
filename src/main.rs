pub mod util
{
	pub fn always_true() -> bool
	{
		true 
	}
}

#[cfg(not(test))]
fn main() 
{
	if util::always_true()
	{
		println!("Hello, world!");
	}
}

#[cfg(test)]
mod test 
{
	use util;
	
    #[test]
    fn equals_test() 
    {
        assert!(1 == 1);
    }

    #[test]
    fn truth_test() 
    {
        assert!(util::always_true());
    }
}
