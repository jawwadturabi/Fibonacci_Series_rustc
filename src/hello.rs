fn main(){


      let a = 5;

    println!("Before Shadow{:p}", &a);

       let a = 6;

    println!("After Shadow{:p}", &a);
}