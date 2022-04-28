use dialoguer::Input; // This import allows for the creation of nice-looking input prompts

// This function puts all the strings from `arr` that contain the character `chr` into a vector which it returns
fn find_char(arr: Vec<String>, chr: char) -> Vec<String> {
  let mut outs = Vec::new();
  // Loop through the strings in `arr`
  for string in arr {
    // If the string contains `chr`, add it to the output vector
    if string.contains(chr.to_lowercase().next().unwrap()) {
      outs.push(string);
    }
  }
  // Return the output vector
  outs
}

// Main function, where the program starts
fn main() {
  // Get the user's input as a string
  let numstr: String = Input::new()
    .with_prompt("How many strings")
    .interact_text().unwrap();
  // Attempt to convert the user's input to an unsigned 32-bit integer
  let mut num = u32::from_str_radix(numstr.as_str(), 10);
  // If they don't enter a number, continue to ask for their input until they do
  while let Err(_) = num {
    let invalidnumstr: String = Input::new()
      .with_prompt("Please enter a number")
      .interact_text().unwrap();
    num = u32::from_str_radix(invalidnumstr.as_str(), 10);
  }
  // Clean up the parsing operation
  let num = num.unwrap();

  // Create the vector that will hold the strings the user inputs
  let mut inputs: Vec<String> = Vec::new();
  // Loop for the number of times the user inputted above
  for i in 0..num {
    // Get a string as input from the user
    let string: String = Input::new()
      .with_prompt(format!("String {}", i+1))
      .interact_text().unwrap();
    // Push that string to the `inputs` vector
    inputs.push(string);
  }

  // Get a string from the user
  let mut chrstring: String = Input::new()
    .with_prompt("What is the character you are searching for")
    .interact_text().unwrap();

  // While the string is not a length of 1 (a single char), get the user's input until it is
  while chrstring.len() != 1 {
    chrstring = Input::new()
      .with_prompt("Please enter a single character")
      .interact_text().unwrap();
  }

  // Change the string into a char by getting the first char
  let searchchr: char = chrstring.chars().next().unwrap();

  // Run the inputs through the `find_char` function
  let outputs = find_char(inputs, searchchr);
  // Print the `outputs` vector
  println!("The character '{}' was found in the strings {:?}", searchchr, outputs);
}
