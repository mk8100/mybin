use matrix;
use matrix::MatrixErr;

fn main() {

    let m = matrix::Matrix::new(3,3,vec![1,2,3,4,5,6,7,8,9]).unwrap();
    println!("m:Matrix<i32> = {}",m);
    
    let n = matrix::Matrix::new(3,3,vec![1.1,2.2,3.3,4.4,5.5,6.6,7.7,8.8,9.9]).unwrap();
    println!("n:Matrix<f64> = {}",n);

    println!("{}", MatrixErr::NotEnoughDataInVector);
}

