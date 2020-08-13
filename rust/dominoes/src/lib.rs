pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let input = input.to_vec();
    depth_first_chain(input, Vec::new())
}

fn depth_first_chain(input: Vec<(u8, u8)>, chain: Vec<(u8, u8)>) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() {
        if chain.is_empty() || chain[0].0 == chain[chain.len() - 1].1 {
            return Some(chain);
        } else {
            return None;
        }
    }
    for i in 0..input.len() {
        let mut input2 = input.clone();
        let mut chain2 = chain.clone();
        if chain.is_empty() || input[i].0 == chain[chain.len() - 1].1 {
            let dom = input2.swap_remove(i);
            chain2.push(dom);
        } else if input[i].1 == chain[chain.len() - 1].1 {
            let dom = input2.swap_remove(i);
            chain2.push(rev(dom));
        } else if input[i].0 == chain[0].0 {
            let dom = input2.swap_remove(i);
            chain2.insert(0, dom);
        } else if input[i].1 == chain[0].0 {
            let dom = input2.swap_remove(i);
            chain2.insert(0, rev(dom));
        } else {
            continue;
        }
        let o = depth_first_chain(input2, chain2);
        if o.is_some() {
            return o;
        }
    }
    None
}

fn rev((a, b): (u8, u8)) -> (u8, u8) {
    (b, a)
}
