/// 实现栈
#[derive(Debug)]
struct Stack<T: Clone> {
    /// 栈顶
    top: usize,
    /// 栈数据容器
    data: Vec<T>
}

impl<T: Clone> Stack<T> {

    /// 创建一个空栈，它不需要参数，返回一个空栈。
    fn new() -> Stack<T> {
        Stack { top: 0, data: vec![] }
    }

    /// 将数据项 item 添加到栈顶，它需要 item 做参数，不返回任何内容。
    fn push(&mut self, item: T) {
        self.data.push(item);
        self.top += 1;
    }

    /// 从栈中删除顶部数据项，它不需要参数，返回数据项，栈被修改。
    fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }

        self.top -= 1;
        self.data.pop()
    }

    /// 从栈返回顶部数据项，但不会删除它，不需要参数，不修改栈。
    fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }

        self.data.get(self.top - 1)
    }

    /// 测试栈是否为空，不需要参数，返回布尔值。
    fn is_empty(&self) -> bool {
        self.top == 0
    }

    /// 返回栈中数据项的数量，不需要参数，返回一个 usize 型整数。
    fn size(&self) -> usize {
        self.top
    }
}

/// 括号匹配
pub fn par_checker(par: &str) -> bool {
    let mut stack = Stack::new();
    // 判断括号是否匹配
    let par_match = |open: char, close: char| {
        let opens = "([{";
        let closers = ")]}";
        opens.find(open) == closers.find(close)
    };

    for char in par.chars() {
        if char == '(' || char == '[' || char == '{' {
            stack.push(char);
        } else {
            if stack.is_empty() {
                return false;
            } else {
                if char != ')' && char != ']' && char != '}' {
                    continue;
                }
                let top = stack.pop().unwrap();
                if !par_match(top, char) {
                    return false;
                }
            }
        }
    }

    stack.is_empty()
}