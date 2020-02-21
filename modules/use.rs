use deeply::nested::function as other_function;

fn function() {
    println!("called `functions()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    other_function();

    println!("Entering block");
    {
        use create::deeply::nested::function;
        function();
        println!("Leaving block");
    }
    function();
}
