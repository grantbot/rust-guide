pub fn add_three_times_four(x: int) -> int {
    times_four(add_three(x))
}

//We don't want to make these public just for testing. They're internal helper
//methods and thus should be kept private. So we need to test them in here.
fn add_three(x: int) -> int { x + 3 }

fn times_four(x: int) -> int { x * 4 }


//Only compile when we run cargo test, otherwise ignore. Won't be part of a
//normal build
#[cfg(test)]
//We do internal unit tests here because these functions are private, so we
//can't use external integration tests. These will be run as tests for this
//particular file when 'cargo test' is run. Integration tests should go in
//tests/lib.rs
mod tests {
    //super imports functions from parent module. Sub-modules can see private
    //funcs in parent.
    //We could also use super::*;, and then #![feature(globs)] at the top
    use super::add_three;
    use super::times_four;

    #[test]
    fn test_add_three() {
        let result = add_three(5i);
        
        assert_eq!(result, 8i);
    }

    #[test]
    fn test_times_four() {
        let result = times_four(5i);

        assert_eq!(result, 20i);
    }
}
