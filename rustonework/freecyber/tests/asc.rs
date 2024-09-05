use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

// 定义结构体，其中包含一个值
#[derive(Debug, Eq, PartialEq)]
struct MyStruct {
    value: i32,
    // 可以添加其他字段
}

// 为了能将MyStruct放入BinaryHeap，需要实现PartialOrd和Ord trait
impl PartialOrd for MyStruct {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
impl Ord for MyStruct {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

#[test]
fn main_asc() {
    let mut heap = BinaryHeap::new();

    // rust自带的数据结构中有堆（BinaryHeap），默认实现是大顶堆。实际应用中，小顶堆的场景也不在少数，官方文档BinaryHeap给了两种方法实现小顶堆.
    // https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
    // 添加元素
    heap.push(Reverse(MyStruct { value: 20 }));
    heap.push(Reverse(MyStruct { value: 10 }));
    heap.push(Reverse(MyStruct { value: 30 }));

    // 获取并打印顶部元素，它将是值最小的元素（因为我们按值升序排列）
    while let Some(top) = heap.pop() {
        println!("{:?}", top);
    }
}
