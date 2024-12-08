use std::{fs::File, io::{BufRead, BufReader}, collections::VecDeque};

#[derive(Clone)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

struct Calibration {
    result: u64,
    operands: Vec<u64>,
}

impl Calibration {
    fn from_str(s: String) -> Self {
        let parts: Vec<&str> = s.split(":").collect();
        // println!("{:?}", parts);
        let operands: Vec<u64> = parts[1].trim().split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect();
        Self {
            result: parts[0].parse::<u64>().unwrap(),
            operands
        }
    }

    fn result(self) -> Option<u64> {
        // first num -> first op -> second num -> first op -> third num
        //                                     -> 2nd op   -> third num
        //           -> 2nd op   -> second num -> first op -> third num
        //                                     -> 2nd op   -> third num
        let mut op_queue: VecDeque<(Vec<Operator>, &Vec<u64>, u64)> = VecDeque::new();
        op_queue.push_back((Vec::new(), &self.operands, 0));

        while !op_queue.is_empty() {
            let item = op_queue.pop_front().unwrap();
            if item.0.len() == item.1.len() - 1 {
                // we should have n-1 operators to operands, then we try computing the result
                
                // set acc to the first num to start
                let mut acc: u64 = item.1[0];

                for n in 0..item.0.len() {
                    acc = match item.0[n] {
                        Operator::Add => acc + item.1[n+1],
                        Operator::Multiply => acc * item.1[n+1],
                        Operator::Concat => {
                            let mut acc_str = acc.to_string();
                            acc_str.push_str(&item.1[n+1].to_string());
                            acc_str.parse::<u64>().unwrap()
                        }
                    }
                }

                if acc == self.result {
                    return Some(acc);
                }
            } else {
                // we haven't built a full list of operators
                let mut new_add_op = item.0.clone();
                new_add_op.push(Operator::Add);
                op_queue.push_back((new_add_op, item.1, item.2));

                let mut new_mult_op = item.0.clone();
                new_mult_op.push(Operator::Multiply);
                op_queue.push_back((new_mult_op, item.1, item.2));

                let mut new_concat_op = item.0.clone();
                new_concat_op.push(Operator::Concat);
                op_queue.push_back((new_concat_op, item.1, item.2));
            }
        }
        None
    }
}

fn main() {
    let file = File::open("../inputs/day7.txt");
    let reader = match file {
        Ok(f) => BufReader::new(f),
        Err(_) => return,
    };

    let mut calibrations: Vec<Calibration> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        calibrations.push(Calibration::from_str(line));
     }

    let mut total_result = 0;
    for c in calibrations {
        if let Some(result) = c.result() {
            total_result += result;
        }
    }

    println!("{}", total_result);
}

