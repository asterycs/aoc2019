use std::collections::VecDeque;

use common::*;
use intcode::*;

fn part1(program: &Vec<i64>) -> Result<(),()> {
    let mut output_queue = VecDeque::new();
    let mut vm = IntcodeVM::new(&program);

    let springscript =
r#"
OR D J
NOT C T
AND T J
NOT A T
OR T J
WALK
"#.to_owned();

    let mut input_queue: VecDeque<_> = encode_ascii(&springscript).into_iter().collect();

    run(&mut vm, &mut input_queue, &mut output_queue);

    println!("output (ascii): {}", decode_ascii(&output_queue.clone().into_iter().collect()));
    println!("last output (raw): {:?}", output_queue.back());
    
    Ok(())
}

fn part2(program: &Vec<i64>) -> Result<(),()> {
    let mut output_queue = VecDeque::new();
    let mut vm = IntcodeVM::new(&program);

    let springscript =
r#"
NOT E T
AND H T
AND D T
NOT I J
AND T J
NOT C J
AND D J
AND H J
NOT A T
OR T J
NOT B T
AND D T
OR T J
RUN
"#.to_owned();

    let mut input_queue: VecDeque<_> = encode_ascii(&springscript).into_iter().collect();

    run(&mut vm, &mut input_queue, &mut output_queue);

    println!("output (ascii): {}", decode_ascii(&output_queue.clone().into_iter().collect()));
    println!("output (raw): {:?}", output_queue.back());
    
    Ok(())
}

intcode_task!(21.txt, part1, part2);
