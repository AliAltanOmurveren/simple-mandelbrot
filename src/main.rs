use std::env;
use image::{self, RgbImage, Rgb};

struct Color(u8, u8, u8);

struct Complex {
    real: f32,
    imaginary: f32
}

impl Complex {
    fn new(real: f32, imaginary: f32) -> Complex{
        Complex {
            real,
            imaginary
        }
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let (size, iteration, color) = parse_arguments(args);

    let img_w = size;
    let img_h = size;

    // max iteration
    let mut max = 0;

    let mut img = RgbImage::new(img_w as u32, img_h as u32);

    // run mandelbrot for every pixel to get max iteration number
    // max iteration number will be used for mapping color range
    for (x, y, _) in img.enumerate_pixels(){
        // center image
        let x_cal = x as f32/ img_w * 3.0 - 2.0;
        let y_cal = y as f32/ img_h * 3.0 - 1.5;

        let m = mandelbrot(x_cal, y_cal, iteration);

        if m > max{
            max = m;
        }
    }

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // center image
        let x_cal = x as f32/ img_w * 3.0 - 2.0;
        let y_cal = y as f32/ img_h * 3.0 - 1.5;

        let m = mandelbrot(x_cal, y_cal, iteration);

        //println!("{}", m);
        if m != -1 {

            // map max range to color range
            let red = ((color.0 / max as u8) * m as u8) as u8;
            let green = ((color.1 / max as u8) * m as u8) as u8;
            let blue = ((color.2 / max as u8) * m as u8) as u8;
        
            *pixel = Rgb([red, green, blue]);
        }else {
            // inner color
            *pixel = Rgb([0,0,0]);
        }
        
    }

    img.save("image.png").unwrap();
}

fn mandelbrot(x: f32, y: f32, iter_num: i32) -> i32 {

    let mut comp_num = Complex::new(x, y);

    let mut iter: i32 = 0;

    // control
    while comp_num.real * comp_num.real + comp_num.imaginary * comp_num.imaginary <= 4.0{

        comp_num = Complex::new(x + comp_num.real * comp_num.real - comp_num.imaginary * comp_num.imaginary,
                                    y + 2.0 * comp_num.real * comp_num.imaginary);

        iter += 1;

        if iter >= iter_num {
            iter = -1;
            break
        }
    }

    iter
}

fn parse_arguments(args: Vec<String>) -> (f32, i32, Color){

    // default values
    let mut size = 2_000.0;

    let mut iteration = 100;

    let mut color = Color(0, 255, 255);


    if args.len() == 1{
        // no additional arguments

        println!("{} px x {} px, {} iterations, color({}, {}, {})", 
                    size, size, iteration, color.0, color.1, color.2);
    }else{
        for (i, arg) in args.iter().enumerate() {
            if i == 0{
                // first argument is always the executable's path
                // so pass 1st argument
                continue;
            }else{
                match &arg[..] {
                    // size argument
                    "-s" => {
                        // check the availability of next argument which is size value
                        match args.get(i + 1) {
                            // there is something next
                            Some(s) => {
                                // try to parse size to f32
                                match s.parse::<f32>() {
                                    Ok(num) => size = num,
                                    Err(_) => println!("Invalid Size: Using default value ({} px)!", size)
                                }
                            },
                            // there isn't next argument
                            None => println!("Invalid Size: Using default value ({} px)!", size)
                        }
                    }, 

                    // iteration argument
                    "-i" => {
                        // check the availability of next argument which is iteration value
                        match args.get(i + 1) {
                            Some(s) => {
                                // try to parse iteration to i32
                                match s.parse::<i32>() {
                                    Ok(num) => iteration = num,
                                    Err(_) => println!("Invalid iteration size: Using default value (100)!")
                                }
                            },
                            // there isn't next argument
                            None => println!("Invalid iteration size: Using default value (100)!")
                        }

                    },

                    // red argument
                    "-r" => {
                        // check the availability of next argument which is red value
                        match args.get(i + 1) {
                            Some(s) => {
                                // try to parse to u8
                                match s.parse::<u8>() {
                                    Ok(num) => color.0 = num,
                                    Err(_) => println!("Invalid red color value: Using default value (0)!")
                                }

                            }

                            None => println!("Invalid red color value: Using default value (0)!")
                        }
                    },

                    // green argument
                    "-g" => {
                        // check the availability of next argument which is green value
                        match args.get(i + 1) {
                            Some(s) => {
                                // try to parse to u8
                                match s.parse::<u8>() {
                                    Ok(num) => color.1 = num,
                                    Err(_) => println!("Invalid green color value: Using default value (255)!")
                                }

                            }

                            None => println!("Invalid green color value: Using default value (255)!")
                        }
                    },

                    // blue argument
                    "-b" => {
                        // check the availability of next argument which is blue value
                        match args.get(i + 1) {
                            Some(s) => {
                                // try to parse to u8
                                match s.parse::<u8>() {
                                    Ok(num) => color.2 = num,
                                    Err(_) => println!("Invalid blue color value: Using default value (255)!")
                                }

                            }

                            None => println!("Invalid blue color value: Using default value (255)!")
                        }
                    },

                    _ => ()
                }
            }
        }
    }

    println!("{} px x {} px, {} iterations, color({}, {}, {})", 
                    size, size, iteration, color.0, color.1, color.2);

    (size, iteration, color)
}

// example of complex number square
// 3 + 4i * 3 + 4i
// r^2 + 2ri - i^2
// 9 + 24 - 16