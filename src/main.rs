use dialoguer::Input;

fn find_char(arr: Vec<String>, chr: char) -> Vec<String> {
  let mut outs = Vec::new();
  for string in arr {
    if string.contains(chr.to_lowercase().next().unwrap()) {
      outs.push(string);
    }
  }
  outs
}

fn main() {
  let numstr: String = Input::new()
    .with_prompt("How many strings")
    .interact_text().unwrap();
  let mut num = u32::from_str_radix(numstr.as_str(), 10);
  while let Err(_) = num {
    let invalidnumstr: String = Input::new()
      .with_prompt("Please enter a number")
      .interact_text().unwrap();
    num = u32::from_str_radix(invalidnumstr.as_str(), 10);
  }
  let num = num.unwrap();

  let mut inputs: Vec<String> = Vec::new();
  for i in 0..num {
    let string: String = Input::new()
      .with_prompt(format!("String {}", i+1))
      .interact_text().unwrap();
    inputs.push(string);
  }

  let mut chrstring: String = Input::new()
    .with_prompt("What is the character you are searching for")
    .interact_text().unwrap();

  while chrstring.len() != 1 {
    chrstring = Input::new()
      .with_prompt("Please enter a single character")
      .interact_text().unwrap();
  }

  let searchchr: char = chrstring.chars().next().unwrap();

  let outputs = find_char(inputs, searchchr);
  println!("The character '{}' was found in the strings {:?}", searchchr, outputs);
}
