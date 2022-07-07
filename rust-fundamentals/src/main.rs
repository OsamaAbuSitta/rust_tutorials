fn main() {
  //first();
  //second();
  //match_expressions();
  //array();
  vectors();
}

fn vectors(){
    let items:[]
}

fn array(){
    let items:[i32;3] = [1,2,3];
    print!("{:?}",items);
}

fn match_expressions(){
    let mut total = 25;
    let mut free_shipping = false;

   total = match free_shipping {
       true =>  total + 0 ,
       false => total + 5
    };

    match total {
        1 => print!("1"),
        2 => print!("2"),
        _ => print!("default")
    }

    print!("Total: {:?}",total);

}

fn first() {
    let result = add(10, 10);
    println!("{result}");
    println!("{} {}", result, true);
    println!("{0} {0}", result);

    println!("{:?}", result);

    fn add(num_one: i32, num_two: i32) -> i32 {
        num_one + num_two
    }
}


fn second(){
    if 5 > 5 {
        print!("true");
    }
    else {
      print!("false");
    }
}