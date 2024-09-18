use std::{io::{self, BufRead}};

#[derive(PartialEq, Debug, Clone)]
enum OperationType {
    Remove,
    Replace
}

#[derive(PartialEq, Debug, Clone)]
struct Operation {
    op_type: OperationType,
    label: String,
    focal_length: Option<u8>
}

#[derive(PartialEq, Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u8
}

fn hash_string(s: &str) -> u16 {
    let mut result: u16 = 0;
    for char in s.bytes() {
        result += char as u16;
        result *= 17;
        result %= 256;
    }

    result
}

fn parse_operation(s: &str) -> Operation {
    let mut label = String::new();
    for (index, char) in s.char_indices() {
        if char != '-' && char != '=' {
            label.push(char);
        } else if char == '-' {
            return Operation {
                op_type: OperationType::Remove,
                label,
                focal_length: None
            };
        } else {
            let focal_length = s[index + 1..].parse::<u64>().unwrap();
            return Operation {
                op_type: OperationType::Replace,
                label,
                focal_length: Some(focal_length as u8)
            };
            
        }
    }
    panic!("Invalid operation string");
}

fn perform_operation(boxes: &mut Vec<Vec<Lens>>, op: &Operation) {
    let box_id = hash_string(&op.label);
    let box_vec = &mut boxes[box_id as usize];

    match op.op_type {
        OperationType::Remove => {
            for i in 0..box_vec.len() {
                let lens = &box_vec[i];
                if lens.label == op.label {
                    box_vec.remove(i);
                    break;
                }
            }
        },
        OperationType::Replace => {
            let mut found = false;
            for i in 0..box_vec.len() {
                let lens = &box_vec[i];
                if lens.label == op.label {
                    found = true;
                    box_vec[i].focal_length = op.focal_length.unwrap();
                    break;
                }
            }

            if !found {
                box_vec.push(Lens {
                    label: op.label.clone(),
                    focal_length: op.focal_length.unwrap()
                });
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    // Create a vector of 256 elements for the boxes
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    for line in stdin.lock().lines() {
        for seq in line.unwrap().split(",").map(|x| x.trim()) {
            let op = parse_operation(seq);

            perform_operation(&mut boxes, &op);
        }
    }

    let mut total_power = 0;
    for i in 0..boxes.len() {
        for j in 0..boxes[i].len() {
            total_power += (i + 1) * (j + 1) * boxes[i][j].focal_length as usize;
        }
    }

    println!("{}", total_power);
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_string() {
        assert_eq!(hash_string("HASH"), 52);
    }

    #[test]
    fn test_parse_operation() {
        let op = parse_operation("cm-");
        assert_eq!(op.op_type, OperationType::Remove);
        assert_eq!(op.label, "cm");
        assert_eq!(op.focal_length, None);

        let op = parse_operation("rn=1");
        assert_eq!(op.op_type, OperationType::Replace);
        assert_eq!(op.label, "rn");
        assert_eq!(op.focal_length, Some(1));
    }

    #[test]
    fn test_perform_operation() {
        let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
        let op = Operation {
            op_type: OperationType::Remove,
            label: "cm".to_string(),
            focal_length: None
        };

        perform_operation(&mut boxes, &op);
        assert_eq!(boxes[hash_string(&op.label) as usize].len(), 0);

        let op = Operation {
            op_type: OperationType::Replace,
            label: "rn".to_string(),
            focal_length: Some(1)
        };
        perform_operation(&mut boxes, &op);
        assert_eq!(boxes[hash_string(&op.label) as usize].len(), 1);
        assert_eq!(boxes[hash_string(&op.label) as usize][0].focal_length, 1);

        let op = Operation {
            op_type: OperationType::Replace,
            label: "cm".to_string(),
            focal_length: Some(2)
        };
        perform_operation(&mut boxes, &op);
        assert_eq!(boxes[hash_string(&op.label) as usize].len(), 2);
        assert_eq!(boxes[hash_string(&op.label) as usize][0].focal_length, 1);
        assert_eq!(boxes[hash_string(&op.label) as usize][1].focal_length, 2);

        let op = Operation {
            op_type: OperationType::Remove,
            label: "rn".to_string(),
            focal_length: None
        };
        perform_operation(&mut boxes, &op);
        assert_eq!(boxes[hash_string(&op.label) as usize].len(), 1);
        assert_eq!(boxes[hash_string(&op.label) as usize][0].focal_length, 2);
        assert_eq!(boxes[hash_string(&op.label) as usize][0].label, "cm");
    }
}