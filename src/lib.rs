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

const MEM_SIZE: usize = 30_000;

pub fn run(source: &str, input: &str) -> Result<String, String> {
    let commands = parse(source);

    if let Err(e) = commands {
        return Err(e);
    }

    let commands = commands.unwrap();

    let mut input_chars = input.chars();
    let mut memory = [0u8; MEM_SIZE];
    let mut command_index: usize = 0;
    let mut memory_pointer: usize = 0;
    let mut out = String::from("");

    while let Some(command) = commands.get(command_index) {
        match *command {
            Command::MoveRight => {
                if memory_pointer + 1 == MEM_SIZE {
                    memory_pointer = 0;
                } else {
                    memory_pointer += 1;
                }
            }
            Command::MoveLeft => {
                if memory_pointer == 0 {
                    memory_pointer = MEM_SIZE - 1;
                } else {
                    memory_pointer -= 1;
                }
            }
            Command::Increment => memory[memory_pointer] += 1,
            Command::Decrement => memory[memory_pointer] -= 1,
            Command::Input => memory[memory_pointer] = input_chars.next().unwrap_or('\0') as u8,
            Command::Output => {
                let c = char::from_u32(memory[memory_pointer] as u32);
                if c.is_none() {
                    return Err(format!(
                        "Couldn't convert u32 {} to a char.",
                        memory[memory_pointer] as u32
                    ));
                }
                out.push(c.unwrap());
            }
            /*
             * If the current cell is zero,
             * we go to the end of the loop
             * by setting the command_index to index
             * which is the index of the matching bracket
             */
            Command::Open(index) => {
                if memory[memory_pointer] == 0 {
                    command_index = index;
                }
            }
            /*
             * If the current cell is not zero,
             * we go back to the beginning of the loop
             * by setting the command_index to index
             * which is the index of the matching bracket
             */
            Command::Close(index) => {
                if memory[memory_pointer] != 0 {
                    command_index = index;
                }
            }
        }

        command_index += 1;
    }

    Ok(out)
}

fn parse(source: &str) -> Result<Vec<Command>, String> {
    let tokens: Vec<char> = source
        .chars()
        .filter(|char| matches!(*char, '>' | '<' | '.' | ',' | '[' | ']' | '+' | '-'))
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
    // Fix soon™
    for i in 0..tokens.len() {
        commands.push(match tokens[i] {
            '>' => Command::MoveRight,
            '<' => Command::MoveLeft,
            '+' => Command::Increment,
            '-' => Command::Decrement,
            ',' => Command::Input,
            '.' => Command::Output,
            '[' => {
                // i + 1 causes index out of bounds if the tokens are ["[",]
                let result = find_matching_bracket('[', ']', i + 1, tokens.len());
                if let Err(e) = result {
                    return Err(e);
                }
                Command::Open(result.unwrap())
            }
            ']' => {
                // i - 1 causes subtract overflow if the tokens are ["]",]
                let result = find_matching_bracket(']', '[', i - 1, 0);
                if let Err(e) = result {
                    return Err(e);
                }
                Command::Close(result.unwrap())
            }
            _ => panic!(),
        })
    }

    Ok(commands)
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
