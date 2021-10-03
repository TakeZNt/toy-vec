/*!
 * 簡易Vectorクラス
 */

/**
 * 簡易Vectorクラスを表す構造体
 */
pub struct ToyVec<T> {
    /// データはBox化されたスライスに格納する
    elements: Box<[T]>,
    /// 現在の要素数(キャパシティ以下)
    length: usize,
}

impl<T: Default> ToyVec<T> {
    /// キャパシティ0のToyVecインスタンスを作成する
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// 指定されたキャパシティのToyVecインスタンスを作成する
    pub fn with_capacity(size: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(size),
            length: 0,
        }
    }

    /// 格納された要素数を返す
    pub fn len(&self) -> usize {
        self.length
    }

    /// キャパシティを返す
    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}

#[cfg(test)]
mod tests {
    use crate::ToyVec;

    #[test]
    fn new() {
        let toyvec = ToyVec::<usize>::new();
        assert_eq!(0, toyvec.len());
        assert_eq!(0, toyvec.capacity());
    }

    #[test]
    fn with_capacity() {
        let toyvec = ToyVec::<usize>::with_capacity(3);
        assert_eq!(0, toyvec.len());
        assert_eq!(3, toyvec.capacity());
    }
}
