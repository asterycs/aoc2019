use std::collections::{VecDeque};

use common::*;
use intcode::*;

struct Computer {
    vm: IntcodeVM,
    input_queue: VecDeque<i64>,
    output_queue: VecDeque<i64>
}

impl Computer {
    fn new(program: &Vec<i64>) -> Computer {
        Computer{vm: IntcodeVM::new(program), input_queue: VecDeque::new(), output_queue: VecDeque::new()}
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Packet {
    x: i64,
    y: i64
}

fn run_computers(computers: &mut Vec<Computer>) {
    for computer in computers.iter_mut() {
        run(&mut computer.vm, &mut computer.input_queue, &mut computer.output_queue);
    }
}

fn initialize_computers(program: &Vec<i64>) -> Vec<Computer> {
    let mut computers = Vec::new();
    computers.resize_with(50, || Computer::new(program));

    for (address, computer) in computers.iter_mut().enumerate() {
        computer.input_queue.push_back(address as i64);
    }

    computers
}

fn direct_packages(packet_queues: &mut Vec<Vec<Packet>>, computers: &mut Vec<Computer>) {
    for (address, computer) in computers.iter_mut().enumerate() {
        if let Some(packets) = packet_queues.get_mut(address) {
            for packet in packets.drain(..).into_iter() {
                computer.input_queue.push_back(packet.x);
                computer.input_queue.push_back(packet.y);
            }

            if packets.is_empty() {
                computer.input_queue.push_back(-1);
            }
        }
    }
}

fn part1(program: &Vec<i64>) -> Result<i64,()> {
    let mut computers = initialize_computers(program);

    let mut packet_queues: Vec<Vec<Packet>> = Vec::new();
    packet_queues.resize_with(50, Vec::new);

    loop {
        run_computers(&mut computers);

        for computer in computers.iter_mut() {
            for c in computer.output_queue.drain(..).into_iter().collect::<Vec<_>>().chunks_exact(3).into_iter() {
                let address = c[0];
                let x = c[1];
                let y = c[2];

                if let Some(packet_queue) = packet_queues.get_mut(address as usize) {
                    packet_queue.push(Packet{x, y});
                } else {
                    if address == 255 {
                        return Ok(y);
                    }
                }
            }
        }

        direct_packages(&mut packet_queues, &mut computers);
    }
}

fn is_idle(packet_queues: &Vec<Vec<Packet>>) -> bool {
    packet_queues.iter().fold(true, |acc, q| acc && q.is_empty())
}

fn part2(program: &Vec<i64>) -> Result<i64,()> {
    let mut computers = initialize_computers(program);

    let mut packet_queues: Vec<Vec<Packet>> = Vec::new();
    packet_queues.resize_with(50, Vec::new);

    let mut nat_packet = None;
    let mut relayed_nat_packet = None;

    loop {
        run_computers(&mut computers);

        for computer in computers.iter_mut() {
            for c in computer.output_queue.drain(..).into_iter().collect::<Vec<_>>().chunks_exact(3).into_iter() {
                let address = c[0];
                let x = c[1];
                let y = c[2];

                let packet = Packet{x, y};

                if let Some(packet_queue) = packet_queues.get_mut(address as usize) {
                    packet_queue.push(packet);
                } else {
                    if address == 255 {
                        nat_packet = Some(packet);
                    }
                }
            }
        }

        if is_idle(&packet_queues) {
            if let Some(packet) = nat_packet {
                packet_queues[0].push(packet);
                if relayed_nat_packet == Some(packet) {
                    return Ok(packet.y);
                }else{
                    relayed_nat_packet = Some(packet);
                }
            }
        }

        direct_packages(&mut packet_queues, &mut computers);
    }
}

intcode_task!(23.txt, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_is_idle() {
        let packet_queues = vec![vec![], vec![], vec![]];

        assert!(is_idle(&packet_queues));

        let packet_queues = vec![vec![], vec![Packet{x: 123, y: 123}], vec![]];

        assert!(!is_idle(&packet_queues));
    }
}