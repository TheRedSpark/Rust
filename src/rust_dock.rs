

pub(crate) fn dockumentation() {
    let mut counter = 0;
    loop {
        println!("again!");
    counter = counter + 1;
        println!("{counter}");
        if counter == 10000{
            break
        }
    }
}

pub(crate) fn looping(){
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}