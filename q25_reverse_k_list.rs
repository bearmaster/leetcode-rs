if k == 0 {
    return head
}
// 3 components:
//  - head: remaining list
//  - k_group: current accumulating group, targeting k elements
//  - prev_tail: tail of already processed part of list
//
// Basic idea:
// Take nodes from head, prepend the node to k_group, so after doing it k times,
// k_group will be the reversed list of size k taken from head.
// Then we append k_group to prev_tail, and search for the new prev_tail.
// If there is not enough nodes to form a k_group of size k, that means the current k_group is the last group
// and its size is smaller than k. In this case, we reverse the k_group again to revert the change, and append
// it to prev_tail, then return.
let mut p_head: Option<Box<ListNode>> = None;
let mut prev_tail = &mut p_head;
let mut k_group: Option<Box<ListNode>> = None;
loop {
    for k_group_len in 0..k {
        if let Some(mut node) = head {
            head = node.next.take();
            node.next = k_group;
            k_group = Some(node);
        } else {
            let mut reverted_k_group: Option<Box<ListNode>> = None;
            for _ in 0..k_group_len {
                let mut node = k_group.unwrap();
                k_group = node.next.take();
                node.next = reverted_k_group;
                reverted_k_group = Some(node);
            }
            *prev_tail = reverted_k_group;
            return p_head
        }
    }
    *prev_tail = k_group;
    for _ in 0..k {
        prev_tail = &mut prev_tail.as_mut().unwrap().next;
    }
    k_group = None;
}