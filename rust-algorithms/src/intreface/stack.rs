pub trait Stack<T> {
    /// 创建一个空栈，它不需要参数，返回一个空栈。
    fn new() -> Self;

    /// 将数据项 item 添加到栈顶，它需要 item 做参数，不返回任何内容。
    fn push(&mut self, item: T);

    /// 从栈中删除顶部数据项，它不需要参数，返回数据项，栈被修改。
    fn pop(&mut self) -> Option<T>;

    ///  从栈返回顶部数据项，但不会删除它，不需要参数，不修改栈。
    fn peek(&self) -> Option<&T>;

    /// 测试栈是否为空，不需要参数，返回布尔值。
    fn is_empty(&self) -> bool;

    /// 返回栈中数据项的数量，不需要参数，返回一个 usize 型整数。
    fn size(&self) -> usize;
}
