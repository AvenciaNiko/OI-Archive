use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct SnakePart {
    pos: (u32, u32),
    color: u32,
}

impl SnakePart {
    fn new(a: (u32, u32), c: u32) -> Self {
        Self { pos: a, color: c }
    }
}

fn get_input(s: &mut String) {
    s.clear();
    io::stdin().read_line(s).expect("Failed to read");
}

fn main() {
    let mut input_line: String = String::new();

    let mut snake: Vec<SnakePart> = vec![SnakePart::new((1, 1), 0)];
    let mut fruit: HashMap<(u32, u32), u32> = HashMap::new();

    get_input(&mut input_line);

    let input: Vec<u32> = input_line
        .split_whitespace()
        .map(|x| x.parse::<u32>().expect("Parse Error"))
        .collect();

    let (fruit_count, instr_count): (u32, u32) = (input[1], input[2]);

    for _ in 0..fruit_count {
        get_input(&mut input_line);

        let input: Vec<u32> = input_line
            .split_whitespace()
            .map(|x| x.parse().expect("Parse error"))
            .collect();

        fruit.insert((input[0], input[1]), input[2]);
    }

    for _ in 0..instr_count {
        get_input(&mut input_line);

        let input: Vec<&str> = input_line.split_whitespace().collect();
        let mut head = snake[0].pos;

        /*
        println!("\n{:?}", snake);
        println!("{:?}", input);
        */
        match input[0] {
            "G" => {
                head.0 -= 1;
            }
            "D" => {
                head.0 += 1;
            }
            "L" => {
                head.1 -= 1;
            }
            "P" => {
                head.1 += 1;
            }
            "Z" => {
                let pos: (u32, u32) = (
                    input[1].parse::<u32>().expect("Failed"),
                    input[2].parse::<u32>().expect("Failed"),
                );

                let mut found: bool = false;
                for i in snake.iter() {
                    if i.pos == pos {
                        found = true;
                        println!("{}", i.color);
                        break;
                    }
                }

                if !found {
                    println!("-1");
                }

                continue;
            }
            _ => {}
        }

        check_for_fruit(&mut fruit, &mut snake, head);
        move_snake(&mut snake, head, 0);
    }
}

fn move_snake(snake: &mut Vec<SnakePart>, carry: (u32, u32), ind: usize) {
    if ind < snake.len() {
        let next_carry = snake[ind].pos;
        snake[ind].pos = carry;
        move_snake(snake, next_carry, ind + 1);
    }
}

fn move_color(snake: &mut Vec<SnakePart>, carry: u32, ind: usize) {
    if ind < snake.len() {
        let next_carry = snake[ind].color;
        snake[ind].color = carry;
        move_color(snake, next_carry, ind + 1);
    }
}

fn check_for_fruit(
    fruits: &mut HashMap<(u32, u32), u32>,
    snake: &mut Vec<SnakePart>,
    head: (u32, u32),
) {
    match fruits.get(&head) {
        Some(color) => {
            snake.push(SnakePart::new((0, 0), 0));
            move_color(snake, *color, 0);
            fruits.remove(&head);
        }
        None => {}
    }
}
