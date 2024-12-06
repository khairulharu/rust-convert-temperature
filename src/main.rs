fn main() {

    let actual_temperature = 77;
    let temperature_on_fahrenheit: i32;
    let temperature_on_celcius: i32;

    if actual_temperature >= 0 && actual_temperature <= 100 {

       temperature_on_celcius = actual_temperature;
       temperature_on_fahrenheit =  calculate_celcius_to_fahrenheit(temperature_on_celcius);

       println!("temperature on fahrenheit is: {}^F and celcius is: {}^C", temperature_on_fahrenheit, temperature_on_celcius);
    } else if actual_temperature >= 32 && actual_temperature <= 212 {

        temperature_on_fahrenheit =  actual_temperature;
        temperature_on_celcius = calculate_fahrenheit_to_celcius(actual_temperature);

        println!("temperature on fahrenheit is: {}^F and celcius is: {}^C", temperature_on_fahrenheit, temperature_on_celcius);
    } else {
        println!("Out of Bound temperature please input jus actual celcius or fahrenheit");
    }
}

fn calculate_fahrenheit_to_celcius(temperature_fahrenheit: i32) -> i32 {
    let celcius: i32 = (temperature_fahrenheit - 32)*(5/9);
    return celcius;
}
fn calculate_celcius_to_fahrenheit(temperatur_celcius: i32) -> i32 {
    let fahrenheit: i32 = (temperatur_celcius * 9/5) + 32;
    return fahrenheit;
}
