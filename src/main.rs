use add_two;

use itertools::Itertools;

fn main() {
    let res = add_two::run( 3.9);

    println!("Run exit code: {:?}", res);


    let m = add_two::Matrix::new(2,3,vec![1,2,3,4,5,6]).unwrap();

    println!("Matrix m = {}",m);

    let v = vec![1,2,3,4,5,6,7,8,9];

    //let l = v.len();
    let cl  = |(idx, el):(usize, &u32)| {
                    match idx {
                        0 => "| ".to_string() + &el.to_string() + ", ", 
                        i if i == v.len()-1 => el.to_string() + " |", 
                        _ => el.to_string() + ", "
                    }
               };

    let s = v.iter()
                    .enumerate()
                    .map( cl)
                    .collect::<Vec<String>>()
                    .concat();
    println!("{}", s);

    let z = v.chunks(3).map(|e| e[0]*e[1]).collect::<Vec<_>>();
    println!("{:?}", z[0]);
    let z = v.chunks(3).map(|e| prn(e)).collect::<Vec<_>>().concat();
    println!("{:?}",z);
}


//fn prn(v:&Vec<u32>) -> String{
fn prn(v:&[u32]) -> String{

    let cl  = |(idx, el):(usize, &u32)| {
                    match idx {
                        0 => "| ".to_string() + &el.to_string() + ", ", 
                        i if i == v.len()-1 => el.to_string() + " |\n", 
                        _ => el.to_string() + ", "
                    }
               };

    v.iter()
            .enumerate()
            .map( cl)
            .collect::<Vec<String>>()
            .concat()
}