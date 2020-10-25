use billboard::{Alignment, BorderStyle, Config};

fn main() {
    let sigma_x = 150_f32;
    let sigma_y = 100_f32;
    let tau_xy = 75_f32;
    let theta = -60_f32;
    stress_transformation(sigma_x, sigma_y, tau_xy, theta);
    // principal_stress_planes(sigma_x, sigma_y, tau_xy);
    // principal_stresses(sigma_x, sigma_y, tau_xy);
    // maximum_in_plane_shear_stress(sigma_x, sigma_y, tau_xy);
}

#[allow(dead_code)]
fn stress_transformation(sigma_x: f32, sigma_y: f32, tau_xy: f32, theta: f32) {
    let billboard = Config::default()
        .box_alignment(Alignment::Left)
        .text_alignment(Alignment::Left)
        .border_style(BorderStyle::Round)
        .build();
    let mut message1 = "INITIAL STRESS STATE\n\n".to_string();
    message1 += &format!("𝜎_x  = {:?}\n", sigma_x);
    message1 += &format!("𝜎_y  = {:?}\n", sigma_y);
    message1 += &format!("𝜏_xy = {:?}\n", tau_xy);
    let mut message3 = "TRANSFORMATION\n\n".to_string();
    message3 += &format!("𝜃 = {:?}", theta);
    let mut message2 = "FINAL STRESS STATE\n\n".to_string();
    message2 += &format!(
        "𝜎_x'   = {:?}\n",
        sigma_x_prime(sigma_x, sigma_y, tau_xy, theta)
    );
    message2 += &format!(
        "𝜎_y'   = {:?}\n",
        sigma_y_prime(sigma_x, sigma_y, tau_xy, theta)
    );
    message2 += &format!(
        "𝜏_x'y' = {:?}\n",
        tau_x_prime_y_prime(sigma_x, sigma_y, tau_xy, theta)
    );
    billboard.display(&message1);
    billboard.display(&message3);
    billboard.display(&message2);
}

#[allow(dead_code)]
fn sigma_x_prime(sigma_x: f32, sigma_y: f32, tau_xy: f32, theta: f32) -> f32 {
    let a = (sigma_x + sigma_y) / 2_f32;
    let b = ((sigma_x - sigma_y) / 2_f32) * (2_f32 * theta.to_radians()).cos();
    let c = tau_xy * (2_f32 * theta.to_radians()).sin();
    a + b + c
}

#[allow(dead_code)]
fn sigma_y_prime(sigma_x: f32, sigma_y: f32, tau_xy: f32, theta: f32) -> f32 {
    let a = (sigma_x + sigma_y) / 2_f32;
    let b = ((sigma_x - sigma_y) / 2_f32) * (2_f32 * theta.to_radians()).cos();
    let c = tau_xy * (2_f32 * theta.to_radians()).sin();
    a - b - c
}

#[allow(dead_code)]
fn tau_x_prime_y_prime(sigma_x: f32, sigma_y: f32, tau_xy: f32, theta: f32) -> f32 {
    let a = (sigma_x - sigma_y) / 2_f32 * (2_f32 * theta).to_radians().sin();
    let b = tau_xy * (2_f32 * theta).to_radians().cos();
    b - a
}

#[allow(dead_code)]
fn principal_stress_planes(sigma_x: f32, sigma_y: f32, tau_xy: f32) {
    let theta_1 = ((2_f32 * tau_xy) / (sigma_x - sigma_y)).atan().to_degrees() / 2_f32;
    let theta_2 = theta_1 + 90_f32;
    println!("=======PRINCIPAL STRESS PLANES=====");
    println!(
        "[Option 1]\t𝜃: {:?}\t𝜎: {:?}",
        theta_1,
        sigma_x_prime(sigma_x, sigma_y, tau_xy, theta_1)
    );
    println!(
        "[Option 2]\t𝜃: {:?}\t𝜎: {:?}",
        theta_2,
        sigma_x_prime(sigma_x, sigma_y, tau_xy, theta_2)
    );
}

#[allow(dead_code)]
fn principal_stresses(sigma_x: f32, sigma_y: f32, tau_xy: f32) {
    let a = (sigma_x + sigma_y) / 2_f32;
    let b = (((sigma_x - sigma_y) / 2_f32).powi(2) + tau_xy.powi(2)).sqrt();
    let sigma_1 = a + b;
    let sigma_2 = a - b;
    println!("=======PRINCIPAL STRESSES=====");
    println!("sigma_1: {:?}", sigma_1);
    println!("sigma_2: {:?}", sigma_2);
}

#[allow(dead_code)]
fn maximum_in_plane_shear_stress(sigma_x: f32, sigma_y: f32, tau_xy: f32) {
    let theta = ((sigma_y - sigma_x) / (2_f32 * tau_xy)).atan().to_degrees() / 2_f32;
    let sigma = (sigma_x + sigma_y) / 2_f32;
    println!("=======MAXIMUM IN-PLANE SHEAR STRESS=====");
    println!("𝜃: {:?}", theta);
    println!("𝜎: {:?}", sigma);
    println!(
        "𝜏: {:?}",
        tau_x_prime_y_prime(sigma_x, sigma_y, tau_xy, theta)
    );
}
