use ndarray::Array2;
use std::io;

fn build_data(width: usize, height: usize) -> Array2<f64> {
    let mut arr = Array2::<f64>::zeros((width, height));

    for i in 1..height {
        arr[[0, i]] = (i as f64) * 2.0;
        arr[[1, i]] = arr[[0, i]] * 2.0; // Getting the double
    }
    arr
}

fn main() {
    let lenght_data = 12;
    let data = build_data(2, lenght_data);

    let epochs = 15;
    let lr = 0.01;

    let mut m = 0.1;
    let mut b = 0.1;

    for _epoch in 0..epochs {
        for i in 0..lenght_data {
            let x = data[[0, i]];
            let y_target = data[[1, i]];

            let y_hat = m * x + b;

            let error = y_hat - y_target;

            m = m - (lr * error.round() * x);
            b = b - (lr * error.round());
        }
    }

    loop {
        println!("\nType a number: ");
        let mut xipt = String::new();

        io::stdin().read_line(&mut xipt).expect("Input Error.");

        let xipt: f64 = xipt.trim().parse().expect("Digite a number");

        let res = m * xipt + b;

        println!("{:?}", res.round());
    }
}
