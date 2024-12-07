use std::io;

fn main() {

    println!("This convert temperature to celcius or fahrenheit");
    println!("===================================================");
    println!("Please enter FTOC or CTOF");
    println!("FTOC: mean fahrenheit to celcius, CTOF mean celcius to fahrenhit");

    let mut picker = String::new();

    io::stdin()
        .read_line(&mut picker)
        .expect("Failed to read line");

    let picker: String = picker.trim().parse().expect("Please input the choice FTOC or CTOF");

    if picker == "FTOC" {
        let mut fahrenheit_temperature = String::new();
        println!("===================================================");
        println!("Please input temperature on fahrenheit");

        io::stdin()
            .read_line(&mut fahrenheit_temperature)
            .expect("Failed to read line");

        let fahrenheit_temperature: i32 = fahrenheit_temperature.trim().parse().expect("please input the number");  

        let result_caculation_on_celcius: i32;     

        if fahrenheit_temperature >= 32 && fahrenheit_temperature <= 212 {
            result_caculation_on_celcius = calculate_fahrenheit_to_celcius(fahrenheit_temperature);

            println!("Result: {fahrenheit_temperature}°F To {result_caculation_on_celcius}°C");
        } else {
            println!("Please input the number on just nominal of the fahrenheit temperature");
        }        
    }

    if picker == "CTOF" {
        let mut celcius_temperature = String::new();
        println!("===================================================");
        println!("Please input temperature on celcius");

        io::stdin()
            .read_line(&mut celcius_temperature)
            .expect("Failed to read line");

        let celcius_temperature: i32 =  celcius_temperature.trim().parse().expect("please input the number");

        let result_caculation_on_fahrenheit: i32;   

        if celcius_temperature >= 0 && celcius_temperature <= 100 {
            result_caculation_on_fahrenheit = calculate_celcius_to_fahrenheit(celcius_temperature);
            println!("Result: {celcius_temperature}°C To {result_caculation_on_fahrenheit}°C");
        } else {
            println!("Please input the number on just nominal of the celcius temperature");
        }
    }
}

fn calculate_fahrenheit_to_celcius(temperature_fahrenheit: i32) -> i32 {
    let celcius: i32 = (temperature_fahrenheit - 32)*5/9;
    return celcius;
}
fn calculate_celcius_to_fahrenheit(temperatur_celcius: i32) -> i32 {
    let fahrenheit: i32 = (temperatur_celcius * 9/5) + 32;
    return fahrenheit;
}
