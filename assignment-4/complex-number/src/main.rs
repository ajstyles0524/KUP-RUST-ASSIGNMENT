// Complex number structure
pub struct Complex {
    real: f64,
    img: f64,
}
fn main() {
    // Complex number - 1
    let complex_1 = Complex {
        real: 2.0,
        img: 3.0,
    };
    // Complex number -2
    let complex_2 = Complex {
        real: 3.0,
        img: 2.0,
    };
    // Calling addition function to add two complex number
    addition(&complex_1, &complex_2);
    // Calling subtraction function to subtract two complex number
    subtraction(&complex_1, &complex_2);
    // Calling multiplication function to multiply two complex number
    multiplication(&complex_1, &complex_2);
}
// This function add two  complex numbers using structures to function
//
// #Arguments
//
// Reference of two complex number
//
// #Return
//
// No return
pub fn addition(complex_1: &Complex, complex_2: &Complex) {
    if complex_1.img + complex_2.img >= 0.0 {
        println!(
            "Sum of the complex numbers = {} + {}i",
            complex_1.real + complex_2.real,
            complex_1.img + complex_2.img
        );
    } else {
        println!(
            "Sum of the complex numbers = {} {}i",
            complex_1.real + complex_2.real,
            complex_1.img + complex_2.img
        );
    }
}
// This function subtract two complex numbers using structures to function
//
// #Arguments
//
// Reference of two complex number
//
// #Return
//
// No return
pub fn subtraction(complex_1: &Complex, complex_2: &Complex) {
    if complex_1.img + complex_2.img >= 0.0 {
        println!(
            "Difference of the complex numbers = {} + {}i",
            complex_1.real - complex_2.real,
            complex_1.img - complex_2.img
        );
    } else {
        println!(
            "Difference of the complex numbers = {} {}i",
            complex_1.real - complex_2.real,
            complex_1.img - complex_2.img
        );
    }
}
// This function multiply two complex numbers using structures to function
//
// #Arguments
//
// Reference of two complex number
//
// #Return
//
// No return
pub fn multiplication(complex_1: &Complex, complex_2: &Complex) {
    if complex_1.img + complex_2.img >= 0.0 {
        println!(
            "Multiplication of the complex numbers = {} + {}i",
            complex_1.real * complex_2.real,
            complex_1.img * complex_2.img
        );
    } else {
        println!(
            "Multiplication of the complex numbers = {} {}i",
            complex_1.real * complex_2.real,
            complex_1.img * complex_2.img
        );
    }
}
