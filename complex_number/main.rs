pub struct Complex{
    real: f64,
    img: f64,
}
fn main() {
    let complex_1 = Complex{
        real: 2.0,
        img : 3.0
    };
    let complex_2 = Complex{
        real: 3.0,
        img: 2.0,
    };
    addition(&complex_1,&complex_2);
    substraction(&complex_1,&complex_2);
    multiplication(&complex_1,&complex_2);

}
pub fn addition(complex_1:&Complex, complex_2:&Complex) {
    if complex_1.img + complex_2.img >= 0.0 {
        println!("Sum of the complex numbers = {} + {}i", complex_1.real + complex_2.real, complex_1.img + complex_2.img);
    }
    else{
        println!("Sum of the complex numbers = {} {}i", complex_1.real + complex_2.real, complex_1.img + complex_2.img);

    }
}

pub fn substraction(complex_1:&Complex, complex_2:&Complex){
    if complex_1.img + complex_2.img >= 0.0 {
        println!("Difference of the complex numbers = {} + {}i", complex_1.real - complex_2.real, complex_1.img - complex_2.img);
    }
    else{
        println!("Difference of the complex numbers = {} {}i", complex_1.real - complex_2.real, complex_1.img - complex_2.img);

    }
}

pub fn multiplication(complex_1:&Complex, complex_2:&Complex) {
    if complex_1.img + complex_2.img >= 0.0 {
        println!("Multiplication of the complex numbers = {} + {}i", complex_1.real * complex_2.real, complex_1.img * complex_2.img);
    }
    else{
        println!("Multiplication of the complex numbers = {} {}i", complex_1.real * complex_2.real, complex_1.img * complex_2.img);

    }
}