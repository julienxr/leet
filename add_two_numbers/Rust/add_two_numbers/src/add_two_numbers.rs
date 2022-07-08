#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self {
            val,
            next: None,
        }
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>,
                       l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> 
{
    let mut temp    = ListNode::new(0);
    let mut curr    = &mut temp;
    let mut ptr1    = l1.as_ref();
    let mut ptr2    = l2.as_ref();
    let mut carry   = 0;

    while ptr1.is_some() || ptr2.is_some() {
        let ptr1_val= ptr1.map_or(0, |v| v.val);
        let ptr2_val= ptr2.map_or(0, |v| v.val);
        let sum     = ptr1_val + ptr2_val + carry;

        carry       = sum / 10;
        curr.next   = Some(Box::new(ListNode::new(sum % 10)));
        curr        = curr.next.as_mut().unwrap();

        ptr1        = ptr1.and_then(|n| n.next.as_ref());
        ptr2        = ptr2.and_then(|n| n.next.as_ref());
    }
    if carry > 0 {
        curr.next = Some(Box::new(ListNode::new(carry)));
    }
    curr.next
}

