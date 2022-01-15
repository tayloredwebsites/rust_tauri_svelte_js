use rand::Rng; // for password generate

static CHARS: [char; 70] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9','a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's','t', 'u', 'v', 'w', 'x', 'y', 'z','A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S','T', 'U', 'V', 'W', 'X', 'Y', 'Z', '!','@','#','$','%','^','&','*'];

#[tauri::command]
fn my_custom_command(invoke_message: String) -> String{
	format!("Hey! I was invoked from JS, with this message: {}", invoke_message)
}

#[tauri::command]
fn generate_password(invoke_password: String) -> String{
    println!("length: {}", invoke_password);
    let mut rng = rand::thread_rng();
    let mut result = String::new();
    let length =  invoke_password.parse::<i32>().unwrap();
    for _x in 0..length {
        result.push(CHARS[rng.gen_range(0..70)]);
    }
    format!("Length is : {}", result);
    result
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn it_works() {
		let result = my_custom_command(String::from("test"));
        assert_eq!(result, "Hey! I was invoked from JS, with this message: test");
    }

    #[test]
    fn generates_string() {
        let result = generate_password(8);
        assert_eq!(result.len(), 8);
    }

}
