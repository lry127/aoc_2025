use aoc_2025::assets::read_to_string;
use std::collections::{BTreeMap, HashMap};

#[derive(Eq, Hash, PartialEq, Debug)]
struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
}

impl JunctionBox {
    fn new(data: &str) -> Self {
        let pos: Vec<u64> = data.split(",").map(|pos| pos.parse().unwrap()).collect();
        JunctionBox {
            x: pos[0],
            y: pos[1],
            z: pos[2],
        }
    }

    fn distance(&self, other: &JunctionBox) -> u64 {
        let pow2 = |v1: u64, v2: u64| v1.abs_diff(v2).pow(2);
        pow2(self.x, other.x) + pow2(self.y, other.y) + pow2(self.z, other.z)
    }
}


// todo: read CLRS 4th e.d. chapter 19 and rewrite this function with better algo.
fn connect_junction_boxes<'a, 'b>(
    box_1: &'a JunctionBox,
    box_2: &'a JunctionBox,
    junction_box_to_circuits_map: &mut HashMap<&'a JunctionBox, usize>,
    circuits: &'b mut Vec<Vec<&'a JunctionBox>>,
) -> Option<&'b Vec<&'a JunctionBox>> {
    let circuit_1_idx = junction_box_to_circuits_map.get(box_1);
    let circuit_2_idx = junction_box_to_circuits_map.get(box_2);
    if let Some(circuit_1_idx) = circuit_1_idx
        && let Some(circuit_2_idx) = circuit_2_idx
    {
        let (circuit_1_idx, circuit_2_idx) = if circuit_1_idx < circuit_2_idx {
            (*circuit_1_idx, *circuit_2_idx)
        } else {
            (*circuit_2_idx, *circuit_1_idx)
        };

        if circuit_1_idx != circuit_2_idx {
            // move all junction boxes in circuit 2 to circuit 1
            let (first, second) = circuits.split_at_mut(circuit_1_idx + 1);

            let circuit_1 = first.last_mut().unwrap();
            let circuit_2 = &mut second[circuit_2_idx - circuit_1_idx - 1];

            circuit_2
                .iter()
                .for_each(|junction_box| circuit_1.push(*junction_box));

            junction_box_to_circuits_map
                .iter_mut()
                .filter(|(_, circuit_idx)| **circuit_idx == circuit_2_idx)
                .for_each(|(_, idx)| {
                    *idx = circuit_1_idx;
                });
            circuit_2.clear();
            return Some(circuit_1);
        }
    } else if let Some(circuit_1_idx) = circuit_1_idx {
        let circuit_1 = &mut circuits[*circuit_1_idx];
        circuit_1.push(box_2);
        junction_box_to_circuits_map.insert(box_2, *circuit_1_idx);
        return Some(circuit_1);
    } else if let Some(circuit2_idx) = circuit_2_idx {
        let circuit_2 = &mut circuits[*circuit2_idx];
        circuit_2.push(box_1);
        junction_box_to_circuits_map.insert(box_1, *circuit2_idx);
        return Some(circuit_2);
    } else {
        let new_circuit = vec![box_1, box_2];
        circuits.push(new_circuit);
        junction_box_to_circuits_map.insert(box_1, circuits.len() - 1);
        junction_box_to_circuits_map.insert(box_2, circuits.len() - 1);
    }
    None
}

fn run_problem_1(
    junction_boxes: &[JunctionBox],
    distance_map: &BTreeMap<u64, (usize, usize)>,
    connection_times: u32,
) -> u32 {
    let mut circuits: Vec<Vec<&JunctionBox>> = Vec::new();
    let mut junction_box_to_circuits_map: HashMap<&JunctionBox, usize> = HashMap::new();
    distance_map
        .iter()
        .take(connection_times as usize)
        .for_each(|(_distance, (b1_idx, b2_idx))| {
            connect_junction_boxes(
                &junction_boxes[*b1_idx],
                &junction_boxes[*b2_idx],
                &mut junction_box_to_circuits_map,
                &mut circuits,
            );
        });
    circuits.sort_by(|left, right| right.len().cmp(&left.len()));
    // dbg!(&circuits);
    circuits
        .iter()
        .take(3)
        .fold(1, |acc, circuit| acc * circuit.len() as u32)
}

fn run_problem_2(
    junction_boxes: &[JunctionBox],
    distance_map: &BTreeMap<u64, (usize, usize)>,
) -> u64 {
    let mut circuits: Vec<Vec<&JunctionBox>> = Vec::new();
    let mut junction_box_to_circuits_map: HashMap<&JunctionBox, usize> = HashMap::new();

    let mut result = None;
    distance_map
        .iter()
        .for_each(|(_distance, (b1_idx, b2_idx))| {
            let box_1 = &junction_boxes[*b1_idx];
            let box_2 = &junction_boxes[*b2_idx];
            if let Some(merged_circuit) = connect_junction_boxes(
                box_1,
                box_2,
                &mut junction_box_to_circuits_map,
                &mut circuits,
            ) {
                if merged_circuit.len() == junction_boxes.len() {
                    result = Some(box_1.x * box_2.x);
                }
            }
        });

    result.expect("can't complete the merge")
}

fn main() {
    let data = read_to_string("day8.txt").unwrap();
    let junction_boxes: Vec<JunctionBox> = data.lines().map(|l| JunctionBox::new(&l)).collect();
    let mut distance_map = BTreeMap::new();
    let boxes_len = junction_boxes.len();
    junction_boxes
        .iter()
        .enumerate()
        .for_each(|(idx, junction_box)| {
            (idx + 1..boxes_len).for_each(|other_box_id| {
                let distance = junction_box.distance(&junction_boxes[other_box_id]);
                distance_map.insert(distance, (idx, other_box_id));
            })
        });

    println!(
        "problem 1: {}",
        run_problem_1(&junction_boxes, &distance_map, 1000)
    );

    println!(
        "problem 2: {}",
        run_problem_2(&junction_boxes, &distance_map)
    );
}
