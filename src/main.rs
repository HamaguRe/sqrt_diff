// 平方根の誤差を測定
// 参考
// 「Fast inverse square root」
//  (https://en.wikipedia.org/wiki/Fast_inverse_square_root)
// 「高速根号計算 (fast sqrt algorithm)」
//  (http://takashiijiri.com/study/miscs/fastsqrt.html)

extern crate gnuplot;
use gnuplot::*;

fn main() {
    let true_num = 1.414213_562373_095;
    let calc_num = 2.0f64;

    //let true_num = 1000000.0;
    //let calc_num = true_num * true_num;

    let mut x = vec![];
    let mut diff0_list = vec![];
    let mut diff1_list = vec![];
    let mut diff2_list = vec![];
    let mut diff3_list = vec![];
    for i in 1..30 {
        let tmp0 = babyronia(calc_num, i);
        let tmp1 = newton(calc_num, i);
        let tmp2 = fast_sqrt_f32(calc_num as f32, i) as f64;
        let tmp3 = fast_sqrt_f64(calc_num, i);
        let diff0 = (tmp0 - true_num).abs().log(10.0);
        let diff1 = (tmp1 - true_num).abs().log(10.0);
        let diff2 = (tmp2 - true_num).abs().log(10.0);
        let diff3 = (tmp3 - true_num).abs().log(10.0);
        diff0_list.push(diff0);
        diff1_list.push(diff1);
        diff2_list.push(diff2);
        diff3_list.push(diff3);
        x.push( i );
        
        println!("{} diff: {}", i, diff2);
    }

    let mut fg = Figure::new();
    let axes = fg.axes2d();
    axes.lines(&x, &diff0_list, &[Caption("babyronian")]);
    axes.lines(&x, &diff1_list, &[Caption("newton")]);
    axes.lines(&x, &diff2_list, &[Caption("fast sqrt f32")]);
    axes.lines(&x, &diff3_list, &[Caption("fast sqrt f64")]);
    fg.show();
}


// 収束速度はnewtonと同じ
fn babyronia(c: f64, n: usize) -> f64 {
    let mut x = c;
    for _ in 0..n {
        x = 0.5 * (x + c / x);
    }
    x
}

fn newton(c: f64, n: usize) -> f64 {
    let mut x = c;
    for _ in 0..n {
        x -= (0.5 * (x*x - c)) / x;
    }
    x
}

// 一度，逆平方根を求めている．
// IEEE-754
fn fast_sqrt_f32(c: f32, n: usize) -> f32 {
    let i = c.to_bits();
    let j = 0x5F3759DF - (i >> 1);
    let mut x = f32::from_bits(j);
    for _ in 0..n {
        x = x * (1.5 - 0.5 * x * x * c);
    }
    x * c
}

// 一度逆平方根を求めている．
// n = 5 まで繰り返せば，ほぼf64の限界の精度が出る
// f64のビットを直接弄ることで初期値を出している．
// この方法で初期値を取らないと，値がオーバーフローする．
// IEEE-754
fn fast_sqrt_f64(c: f64, n: usize) -> f64 {
    let i = c.to_bits();
    let j = 0x5FE6EB50C7B537AA - (i >> 1);
    let mut x = f64::from_bits(j);
    for _ in 0..n {
        x = x * (1.5 - 0.5 * x * x * c);
    }
    x * c
}
