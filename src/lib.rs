pub fn your_mom_is_gay_if_this_function_return_two(a: i32) -> i32 
{
    2
}

#[cfg(test)] 
mod tests {
    use super::*;
    #[test]
    fn test_if_your_mom_is_gay() {
        assert_eq!(2, your_mom_is_gay_if_this_function_return_two(1));
    }
}
