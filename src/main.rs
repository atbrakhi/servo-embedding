extern crate simpleservo;

fn main() {
    println!("Servo version: {}", simpleservo::servo_version());
    println!("Is url valid: {}", simpleservo::is_uri_valid("https://servo.org/"));
}
