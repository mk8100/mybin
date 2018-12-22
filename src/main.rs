use add_two;

fn main() {
    let res = add_two::run( 3.9);

    println!("Run exit code: {:?}", res);


    let m = add_two::Matrix::new(2,3,vec![1,2,3,4,5,6]).unwrap();

    println!("Matrix m = {}",m);
}

