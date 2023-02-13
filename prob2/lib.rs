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

    pub fn value(&self) -> usize {
        self.v
    }

    fn add(&mut self, value: usize) {
        self.v += value;
    }

    fn update(&mut self, value: usize) {
        self.v = value;
    }
}

pub struct NodeOp {
    op: usize,
    sum: usize,
}

impl Copy for NodeOp {}

impl Clone for NodeOp {
    fn clone(&self) -> NodeOp {
        NodeOp {
            op: self.op,
            sum: self.sum,
        }
    }
}

impl NodeOp {
    fn new(operation: usize) -> NodeOp {
        NodeOp { op: operation, sum: 0 }
    }

    fn inc(&mut self) {
        self.sum += 1;
    }

    pub fn sum(&self) -> usize {
        self.sum
    }
}

pub fn create_segment_tree_ops(ops_num: usize) -> Vec<NodeOp> {
    let init_empty_node: NodeOp = NodeOp::new(0);
    let len: usize;

    if ops_num == 0 {
        len = 1;
    } else {
        let n = (usize::MAX >> ops_num.leading_zeros()) + 1;
        // N*2-1
        len = n * 2 - 1;
    }

    let mut seg_tree_ops: Vec<NodeOp> = vec![init_empty_node; len];

    construct_seqment_tree_ops(&mut seg_tree_ops, 0, ops_num, 0);

    seg_tree_ops
}

pub fn create_segment_tree_arr(arr: Vec<usize>) -> Vec<Node> {
    let init_empty_node: Node = Node::new(0);
    let len: usize;

    let arr_len = arr.len() - 1;

    if arr_len == 0 {
        len = 1;
    } else {
        let n = (usize::MAX >> arr_len.leading_zeros()) + 1;
        // N*2-1
        len = n * 2 - 1;
    }

    let mut seg_tree: Vec<Node> = vec![init_empty_node; len];

    construct_seqment_tree_arr(&arr, &mut seg_tree, 0, arr_len, 0);

    seg_tree
}

fn construct_seqment_tree_ops(seg_tree: &mut Vec<NodeOp>, low: usize, high: usize, idx: usize) {
    if low == high {
        seg_tree[idx] = NodeOp::new(low + 1);
        return;
    }
    let mid = mid(low, high);
    construct_seqment_tree_ops(seg_tree, low, mid, r_child(idx));
    construct_seqment_tree_ops(seg_tree, mid + 1, high, l_child(idx));

    seg_tree[idx] = NodeOp::new(0);
}

fn construct_seqment_tree_arr(
    arr: &Vec<usize>,
    seg_tree: &mut Vec<Node>,
    low: usize,
    high: usize,
    idx: usize,
) {
    if low == high {
        seg_tree[idx] = Node::new(arr[low]);
        return;
    }
    let mid = mid(low, high);
    construct_seqment_tree_arr(arr, seg_tree, low, mid, r_child(idx));
    construct_seqment_tree_arr(arr, seg_tree, mid + 1, high, l_child(idx));

    seg_tree[idx] = Node::new(0);
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
pub fn execute_queries(
    seg_tree_ops: &mut Vec<NodeOp>,
    qrs: Vec<Vec<usize>>,
    ops_len: usize,
) -> Vec<usize> {
    for query in qrs {
        update_range_qr(seg_tree_ops, query[0] - 1, query[1] - 1, 0, ops_len, 0);
    }

    let mut num_op: Vec<usize> = Vec::new();

    for i in 0..(ops_len + 1) {
        // Sums values on the path root-leaf
        num_op.push(sum_seg_tree(seg_tree_ops, i, 0, ops_len, 0));
    }

    num_op
}

// Execute operations
pub fn execute_operations(
    seg_tree: &mut Vec<Node>,
    ops: Vec<Vec<usize>>,
    arr_len: usize,
) -> Vec<usize> {
    for op in ops {
        update_range_op(seg_tree, op[0] - 1, op[1] - 1, 0, arr_len, 0, op[2]);
    }

    let mut arr_res: Vec<usize> = Vec::new();

    for i in 0..(arr_len + 1) {
        // Sums values on the path root-leaf
        arr_res.push(sum_seg_tree_arr(seg_tree, i, 0, arr_len, 0));
    }

    arr_res
}

fn update_range_qr(
    seg_tree: &mut Vec<NodeOp>,
    from: usize,
    to: usize,
    low: usize,
    high: usize,
    idx: usize,
) {
    // Total overlap
    if from <= low && to >= high {
        seg_tree[idx].inc();
        return;
    }

    // No overlap
    if from > high || to < low {
        return;
    }

    // Partitial overlap
    let mid = mid(low, high);
    update_range_qr(seg_tree, from, to, low, mid, r_child(idx));
    update_range_qr(seg_tree, from, to, mid + 1, high, l_child(idx));
}

fn sum_seg_tree(
    seg_tree_ops: &mut Vec<NodeOp>,
    leaf: usize,
    low: usize,
    high: usize,
    idx: usize,
) -> usize {
    // Total overlap
    if leaf <= low && leaf >= high {
        return seg_tree_ops[idx].sum();
    }

    // No overlap
    if leaf > high || leaf < low {
        return 0;
    }

    // Partitial overlap
    let mid = mid(low, high);
    let mut sum = 0;
    sum += sum_seg_tree(seg_tree_ops, leaf, low, mid, r_child(idx));
    sum += sum_seg_tree(seg_tree_ops, leaf, mid + 1, high, l_child(idx));

    seg_tree_ops[idx].sum() + sum
}

fn sum_seg_tree_arr(
    seg_tree: &mut Vec<Node>,
    leaf: usize,
    low: usize,
    high: usize,
    idx: usize,
) -> usize {
    // Total overlap
    if leaf <= low && leaf >= high {
        return seg_tree[idx].value();
    }

    // No overlap
    if leaf > high || leaf < low {
        return 0;
    }

    // Partitial overlap
    let mid = mid(low, high);
    let mut sum = 0;
    sum += sum_seg_tree_arr(seg_tree, leaf, low, mid, r_child(idx));
    sum += sum_seg_tree_arr(seg_tree, leaf, mid + 1, high, l_child(idx));

    seg_tree[idx].value() + sum
}

fn update_range_op(
    seg_tree: &mut Vec<Node>,
    from: usize,
    to: usize,
    low: usize,
    high: usize,
    idx: usize,
    value: usize,
) {
    // Total overlap
    if from <= low && to >= high {
        seg_tree[idx].add(value);
        return;
    }

    // No overlap
    if from > high || to < low {
        return;
    }

    // Partitial overlap
    let mid = mid(low, high);
    update_range_op(seg_tree, from, to, low, mid, r_child(idx), value);
    update_range_op(seg_tree, from, to, mid + 1, high, l_child(idx), value);
}
