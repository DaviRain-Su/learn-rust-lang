use std::collections::LinkedList;
use crate::intreface::stack::Stack;

#[derive(Debug)]
pub struct StackVec<T> {
    /// 栈顶
    top: usize,
    /// 栈数据容器
    data: Vec<T>,
}

impl<T> Stack<T> for StackVec<T> {
    fn new() -> Self {
        Self {
            top: 0,
            data: Vec::new(),
        }
    }

    fn push(&mut self, item: T) {
        // 数据 保存 在 Vec 末尾
        self.data.push(item);
        self.top += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        // 栈顶减 1 后 再 弹 出 数 据
        self.top -= 1;
        self.data.pop()
    }

    // 数 据 不 能 移 动 ， 只 能 返 回 引 用
    fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        self.data.get(self.top - 1)
    }

    fn is_empty(&self) -> bool {
        0 == self.top
    }

    fn size(&self) -> usize {
        // 栈 顶 恰 好 就 是 栈 中 元 素 个 数
        self.top
    }
}


#[derive(Debug)]
pub struct StackLinkList<T> {
    top: usize,
    data: LinkedList<T>,
}

impl<T> Stack<T> for StackLinkList<T> {
    fn new() -> Self {
        Self {
            top: 0,
            data: LinkedList::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.data.push_back(item);
        self.top += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        // 栈顶减 1 后 再 弹 出 数 据
        self.top -= 1;
        self.data.pop_back()
    }

    fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        self.data.back()
    }

    fn is_empty(&self) -> bool {
        0 == self.top
    }

    fn size(&self) -> usize {
        self.top
    }
}
// 这里的balance是什么意思不是很懂
pub fn par_check(data: &str) -> bool {
    let char_list = data.chars().map(|c| c).collect::<Vec<_>>();

    let mut index = 0;
    // 括 号 是 否 匹 配 (平衡)标示
    let mut balance = true;
    // 使 用 前 面 实 现 的 栈
    let mut stack = StackVec::new();
    while index < char_list.len() && balance {
        if let Some(c) = char_list.get(index) {
            if '(' == *c || '[' == *c || '{' == *c {
                // 如 果 为 开 符 号 ， 入栈
                stack.push(*c);
            } else {
                // 如 果 为 闭 符 号 ， 判 断 栈 是 否 为 空
                if stack.is_empty() {
                    balance = false; // 为 空 则 不 平 衡
                } else {
                    let s_c = stack.peek();
                    match (s_c, c) {
                        (Some(&'('), ')') | (Some(&'['), ']') | (Some(&'{'), '}') => {
                            let _r = stack.pop();
                        }
                        _ => {}
                    }
                }
            }
        }
        index += 1;
    }

    // 平 衡 且 栈 为 空 ， 括 号 表 达 式 才 是 匹 配 的
    balance && stack.is_empty()
}


pub fn divide_by_two(dec_num: u32) -> String {
    base_converter(dec_num, 2)
}

pub fn base_converter(mut dec_num: u32, base: u32) -> String {
    // digits 对 应 各 种 余 数 的 字 符 形 式 ， 尤其是 10 - 15
    let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
    let mut rem_stack = StackVec::new();

    // 余数入栈
    while dec_num > 0 {
        let rem = dec_num % base;
        rem_stack.push(rem);
        dec_num /= base;
    }

    // 余 数 出 栈 并 取 对 应 字 符 来 拼 接 成 字 符 串
    let mut bin_str = String::new();
    while !rem_stack.is_empty() {
        if let Some(rem) = rem_stack.pop() {
            bin_str.push_str(&digits[rem as usize].to_string());
        }
    }

    bin_str
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intreface::stack::Stack;

    #[test]
    fn test_stackvec() {
        let mut stack = StackVec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        // test peek
        assert_eq!(Some(&3), stack.peek());
        // test size
        assert_eq!(3, stack.size());
        // test pop
        stack.pop();
        assert_eq!(Some(&2), stack.peek());
        assert_eq!(2, stack.size());

        // test is_empty
        assert_eq!(false, stack.is_empty());

        stack.pop();
        stack.pop();
        assert_eq!(true, stack.is_empty());
    }

    #[test]
    fn test_stack_list() {
        let mut stack = StackLinkList::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        // test peek
        assert_eq!(Some(&3), stack.peek());
        // test size
        assert_eq!(3, stack.size());
        // test pop
        stack.pop();
        assert_eq!(Some(&2), stack.peek());
        assert_eq!(2, stack.size());

        // test is_empty
        assert_eq!(false, stack.is_empty());

        stack.pop();
        stack.pop();
        assert_eq!(true, stack.is_empty());
    }

    #[test]
    fn test_par_check() {
        let sa = "()(())";
        let sb = "()((()";
        let res1 = par_check(sa);
        let res2 = par_check(sb);
        println!("sa balanced:{res1}, sb balanced:{res2}");

        let sa = "[]({})";
        let sb = "{}((()";
        let res1 = par_check(sa);
        let res2 = par_check(sb);
        println!("sa balanced:{res1}, sb balanced:{res2}");

        let sa = "(2+3){func}[abc]";
        let sb = "(2+3)*(3-1";
        let res1 = par_check(sa);
        let res2 = par_check(sb);
        println!("sa balanced:{res1}, sb balanced:{res2}");
    }

    #[test]
    fn test_divide_by_two() {
        let bin_str = divide_by_two(10);
        println!("10 is b{bin_str}");
    }

    #[test]
    fn test_base_converter() {
        let bin_str: String = base_converter(10, 2);
        let hex_str: String = base_converter(43, 16);
        println!("10 is b{bin_str}, 43 is x{hex_str}");
    }
}
