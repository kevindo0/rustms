#[macro_export]
macro_rules! repeat_two {
    ($($i:expr)*; $($j:expr)*) => {
    {   
        $(  
        println!("{}, {}", $i, $j);
        )*  
    }}  
}

#[macro_export]
macro_rules! repeat_two2 {
    ($($i:expr)*, $($j:expr)*) => {
    {   
        $(  
        println!("{}, {}", $i, $j);
        )*  
    }}  
}

// use:
//use dcmacro::repeat_two;
//repeat_two!(1 3; 2 4);

//use dcmacro::repeat_two2;
//repeat_two2!(1 3, 2 4);

