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

    /// 指定インデックスの要素を返す
    pub fn get(&self, index: usize) -> Option<&T> {
        if self.length <= index {
            return None;
        }
        Some(&self.elements[index])
    }

    /// 要素を追加する
    pub fn push(&mut self, element: T) {
        if self.length == self.capacity() {
            self.grow();
        }
        self.elements[self.length] = element;
        self.length += 1;
    }

    /// イテレータを返す
    pub fn iter(&self) -> Iter<T> {
        Iter {
            elements: &self.elements,
            len: self.length,
            pos: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn grow(&mut self) {
        if self.capacity() == 0 {
            self.elements = Self::allocate_in_heap(1);
            return;
        }
        // 容量が2倍の配列を確保する
        let new_elements = Self::allocate_in_heap(self.length * 2);
        let old_elements = std::mem::replace(&mut self.elements, new_elements); // 配列を置き換える場合に使う
        for (i, element) in old_elements.into_vec().into_iter().enumerate() {
            self.elements[i] = element;
        }
    }
}

/*
 * 簡易Vectorクラスのイテレータ
 */
pub struct Iter<'vec, T> {
    elements: &'vec Box<[T]>,
    len: usize,
    pos: usize,
}

impl<'vec, T> Iterator for Iter<'vec, T> {
    type Item = &'vec T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.len {
            None
        } else {
            let res = Some(&self.elements[self.pos]);
            self.pos += 1;
            res
        }
    }
}

impl<'vec, T: Default> IntoIterator for &'vec ToyVec<T> {
    type Item = &'vec T;
    type IntoIter = Iter<'vec, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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

    #[test]
    fn get() {
        let toyvec = ToyVec::<usize>::with_capacity(3);
        assert_eq!(None, toyvec.get(0));
        assert_eq!(None, toyvec.get(3));
    }

    #[test]
    fn push_without_grow() {
        let mut toyvec = ToyVec::<usize>::with_capacity(3);
        toyvec.push(100);
        assert_eq!(Some(&100), toyvec.get(0));
        assert_eq!(None, toyvec.get(3));
    }

    #[test]
    fn push_with_grow01() {
        let mut toyvec = ToyVec::<usize>::new();
        toyvec.push(100);

        assert_eq!(Some(&100), toyvec.get(0));
        assert_eq!(None, toyvec.get(1));
    }

    #[test]
    fn push_with_grow02() {
        let mut toyvec = ToyVec::<usize>::with_capacity(2);
        toyvec.push(100);
        toyvec.push(200);
        toyvec.push(300);

        assert_eq!(Some(&100), toyvec.get(0));
        assert_eq!(Some(&200), toyvec.get(1));
        assert_eq!(Some(&300), toyvec.get(2));
        assert_eq!(None, toyvec.get(3));
    }

    #[test]
    fn iterator() {
        let mut toyvec = ToyVec::<usize>::with_capacity(2);
        toyvec.push(100);
        toyvec.push(200);

        let mut iter = toyvec.iter();
        assert_eq!(Some(&100), iter.next());
        assert_eq!(Some(&200), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn into_iterator() {
        let mut toyvec = ToyVec::<usize>::with_capacity(2);
        toyvec.push(100);
        toyvec.push(200);

        let mut i = 1;
        for element in &toyvec {
            assert_eq!(&(i * 100), element);
            i += 1;
        }
    }
}
