#[derive(Debug)]
enum Command {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Input,
    Output,
    // Looping
    Open(usize),
    Close(usize),
}

const MEM_SIZE: usize = 3_0_0_0_0;

pub fn run(source: &str, input: &str) -> Result<String, String> {
    let commands = parse(source);

    if commands.is_err() {
        return Err(commands.unwrap_err());
    }

    let commands = commands.unwrap();

    let mut input_chars = input.chars();
    let mut memory = [0u8; MEM_SIZE];
    let mut command_counter: usize = 0;
    let mut memory_counter: usize = 0;
    let mut out = String::from("");

    while let Some(command) = commands.get(command_counter) {
        match *command {
            Command::MoveRight => {
                if memory_counter + 1 == MEM_SIZE {
                    memory_counter = 0;
                } else {
                    memory_counter += 1;
                }
            }
            Command::MoveLeft => {
                if memory_counter == 0 {
                    memory_counter = MEM_SIZE - 1;
                } else {
                    memory_counter -= 1;
                }
            }
            Command::Increment => memory[memory_counter] += 1,
            Command::Decrement => memory[memory_counter] -= 1,
            Command::Input => memory[memory_counter] = input_chars.next().unwrap_or('\0') as u8,
            Command::Output => {
                let c = char::from_u32(memory[memory_counter] as u32);
                if c.is_none() {
                    return Err(format!(
                        "Couldn't convert u32 {} to a char.",
                        memory[memory_counter] as u32
                    ));
                }
                out.push(c.unwrap());
            }
            Command::Open(index) => {
                if memory[memory_counter] == 0 {
                    command_counter = index;
                }
            }
            Command::Close(index) => {
                if memory[memory_counter] != 0 {
                    command_counter = index;
                }
            }
        }

        command_counter += 1;
    }

    Ok(out)
}

fn parse(source: &str) -> Result<Vec<Command>, String> {
    let tokens: Vec<char> = source
        .chars()
        .filter(|char| match *char {
            '>' | '<' | '.' | ',' | '[' | ']' | '+' | '-' => true,
            _ => false,
        })
        .collect();

    let mut commands: Vec<Command> = vec![];

    let find_matching_bracket = |open, close, start_index, stop_index| {
        // Not the best way but it works
        let mut i = start_index;
        let mut open_loops = 1;

        loop {
            if tokens[i] == open {
                open_loops += 1;
            } else if tokens[i] == close {
                open_loops -= 1;
            }

            if open_loops == 0 {
                return Ok(i);
            }

            if i == stop_index {
                return Err(format!("Unmatched brackets at index {}", i));
            } else if start_index < stop_index {
                i += 1;
            } else {
                i -= 1;
            }
        }
    };

    for i in 0..tokens.len() {
        commands.push(match tokens[i] {
            '>' => Command::MoveRight,
            '<' => Command::MoveLeft,
            '+' => Command::Increment,
            '-' => Command::Decrement,
            ',' => Command::Input,
            '.' => Command::Output,
            '[' => {
                let result = find_matching_bracket('[', ']', i + 1, tokens.len());
                if result.is_err() {
                    return Err(result.unwrap_err());
                }
                Command::Open(result.unwrap())
            }
            ']' => {
                let result = find_matching_bracket(']', '[', i - 1, 0);
                if result.is_err() {
                    return Err(result.unwrap_err());
                }
                Command::Close(result.unwrap())
            }
            _ => panic!(),
        })
    }

    return Ok(commands);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test hello world program.
    fn hello_world() {
        assert_eq!(
            run(
                r#"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+."#, // Simple BrainF*ck program which outputs "Hello World!"
                "" // Input is empty cause the program doesn't take any input.
            ),
            Ok(String::from("Hello World!"))
        )
    }

    #[test]
    /// Test echoing input.
    fn echo() {
        const INPUT: &str = "Hello, I am working correctly."; // Example input
        assert_eq!(run(r#",[.,]"#, INPUT), Ok(String::from(INPUT)))
    }
}
