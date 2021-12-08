pub trait SwapRemovePred<T> {
    fn swap_remove_pred<P: Fn(&T) -> bool>(&mut self, pred: P) -> T;
}

impl<T> SwapRemovePred<T> for Vec<T> {
    fn swap_remove_pred<P: Fn(&T) -> bool>(&mut self, pred: P) -> T {
        self.swap_remove(self.iter().position(pred).unwrap())
    }
}
