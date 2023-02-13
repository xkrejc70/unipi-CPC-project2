use std::cmp;

pub struct Node {
    v: usize,
}

impl Copy for Node {}

impl Clone for Node {
    fn clone(&self) -> Node {
        Node { v: self.v }
    }
}

impl Node {
    fn new(value: usize) -> Node {
        Node { v: value }
    }

    fn value(&self) -> usize {
        self.v
    }

    fn update(&mut self, value: usize) {
        self.v = value;
    }
}

pub fn create_segment_tree(arr: Vec<usize>) -> Vec<Node> {
    let init_empty_node: Node = Node::new(0);

    let arr_len = arr.len() - 1;
    let n = (usize::MAX >> arr_len.leading_zeros()) + 1;
    // N*2-1
    let len: usize = n * 2 - 1;

    let mut seg_tree: Vec<Node> = vec![init_empty_node; len];

    construct_seqment_tree(&arr, &mut seg_tree, 0, arr_len, 0);

    seg_tree
}

fn construct_seqment_tree(
    arr: &Vec<usize>,
    seg_tree: &mut Vec<Node>,
    low: usize,
    high: usize,
    idx: usize,
) {
    if low == high {
        seg_tree[idx].update(arr[low]);
        return;
    }
    let mid = mid(low, high);
    construct_seqment_tree(arr, seg_tree, low, mid, r_child(idx));
    construct_seqment_tree(arr, seg_tree, mid + 1, high, l_child(idx));

    let max = cmp::max(
        seg_tree[l_child(idx)].value(),
        seg_tree[r_child(idx)].value(),
    );
    seg_tree[idx].update(max);
}

// Right child
fn r_child(idx: usize) -> usize {
    2 * idx + 1
}

// Left child
fn l_child(idx: usize) -> usize {
    2 * idx + 2
}

// Calc mid
fn mid(low: usize, high: usize) -> usize {
    (low + high) / 2
}

// Execute queries
pub fn execute_query(seg_tree: &mut Vec<Node>, qrs: Vec<Vec<usize>>, arr_len: usize) {
    for query in qrs {
        if query[0] == 1 {
            // Max(i,j)
            let max = max(seg_tree, query[1] - 1, query[2] - 1, 0, arr_len, 0);
            println!("{}", max.value());
        } else {
            // Update(i,j,T)
            update(
                seg_tree,
                query[1] - 1,
                query[2] - 1,
                0,
                arr_len,
                0,
                query[3],
            );
        }
    }
}

fn update(
    seg_tree: &mut Vec<Node>,
    from: usize,
    to: usize,
    low: usize,
    high: usize,
    idx: usize,
    value: usize,
) -> Node {
    // Elem of array in update range
    if low == high && from <= low && to >= high {
        let min: usize = cmp::min(seg_tree[idx].value(), value);
        seg_tree[idx].update(min);
        return seg_tree[idx];
    }

    // No overlap
    if from > high || to < low {
        return seg_tree[idx];
    }

    // Partitial overlap
    let mid = mid(low, high);
    let r_child = update(seg_tree, from, to, low, mid, r_child(idx), value);
    let l_child = update(seg_tree, from, to, mid + 1, high, l_child(idx), value);
    let max = cmp::max(r_child.value(), l_child.value());

    seg_tree[idx].update(max);

    Node::new(max)
}

fn max(
    seg_tree: &mut Vec<Node>,
    from: usize,
    to: usize,
    low: usize,
    high: usize,
    idx: usize,
) -> Node {
    // Total overlap
    if from <= low && to >= high {
        return seg_tree[idx];
    }

    // No overlap
    if from > high || to < low {
        return Node::new(0);
    }

    // Partitial overlap
    let mid = mid(low, high);
    let r_child = max(seg_tree, from, to, low, mid, r_child(idx));
    let l_child = max(seg_tree, from, to, mid + 1, high, l_child(idx));
    let max = cmp::max(r_child.value(), l_child.value());

    Node::new(max)
}
