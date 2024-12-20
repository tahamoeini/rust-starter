pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() {
        return Some(vec![]);
    }

    for i in 0..input.len() {
        let mut used = vec![false; input.len()];
        let mut current_chain = vec![input[i]];
        used[i] = true;

        if build_chain(input, &mut used, &mut current_chain) {
            return Some(current_chain);
        }
    }

    None
}

fn build_chain(
    input: &[(u8, u8)],
    used: &mut Vec<bool>,
    current_chain: &mut Vec<(u8, u8)>,
) -> bool {
    if current_chain.len() == input.len() {
        return current_chain[0].0 == current_chain.last().unwrap().1;
    }

    let current_end = current_chain.last().unwrap().1;

    for (i, &domino) in input.iter().enumerate() {
        if !used[i] && (domino.0 == current_end || domino.1 == current_end) {
            let next_domino = if domino.0 == current_end { domino } else { (domino.1, domino.0) };
            current_chain.push(next_domino);
            used[i] = true;

            if build_chain(input, used, current_chain) {
                return true;
            }

            current_chain.pop();
            used[i] = false;
        }
    }

    false
}

pub fn main() {
    let input = [(1, 2), (2, 3), (3, 1)];
    let result = chain(&input);
    println!("{:?}", result);
}