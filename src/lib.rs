use std::{
    marker::PhantomData,
    slice::{Iter, IterMut},
};

pub struct Mat<T, const M: usize, const N: usize> {
    data: [[T; N]; M],
}

pub struct RowIter<'m, T, const M: usize> {
    row: Iter<'m, T>,
}

impl<'a, T, const M: usize> Iterator for RowIter<'a, T, M> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.row.next()
    }
}

pub struct MultiRowIter<'m, T, const M: usize, const N: usize> {
    rows: Iter<'m, [T; N]>,
}

impl<'a, T, const M: usize, const N: usize> Iterator for MultiRowIter<'a, T, M, N> {
    type Item = RowIter<'a, T, N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows.next().map(|row| RowIter { row: row.iter() })
    }
}

pub struct ColumnIter<'a, T: Sized, const M: usize, const N: usize> {
    mat: &'a Mat<T, M, N>,
    col: usize,
    row: usize,
}

impl<'a, T: Sized, const M: usize, const N: usize> Iterator for ColumnIter<'a, T, M, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < N && self.row < M {
            let ret = Some(&self.mat.data[self.row][self.col]);
            self.row += 1;
            ret
        } else {
            None
        }
    }
}

pub struct MultiColumnIter<'a, T: Sized, const M: usize, const N: usize> {
    mat: &'a Mat<T, M, N>,
    col: usize,
}

impl<'a, T: Sized, const M: usize, const N: usize> Iterator for MultiColumnIter<'a, T, M, N> {
    type Item = ColumnIter<'a, T, M, N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < N {
            let ret = Some(ColumnIter {
                mat: self.mat,
                row: 0,
                col: self.col,
            });
            self.col += 1;
            ret
        } else {
            None
        }
    }
}

pub struct RowIterMut<'m, T: Sized, const N: usize> {
    row: IterMut<'m, T>,
}

impl<'m, T: Sized, const N: usize> Iterator for RowIterMut<'m, T, N> {
    type Item = &'m mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.row.next()
    }
}

pub struct MultiRowIterMut<'m, T: Sized, const M: usize, const N: usize> {
    rows: IterMut<'m, [T; N]>,
}

impl<'m, T: Sized, const M: usize, const N: usize> Iterator for MultiRowIterMut<'m, T, M, N> {
    type Item = RowIterMut<'m, T, N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows.next().map(|row| RowIterMut {
            row: row.iter_mut(),
        })
    }
}

pub struct ColumnIterMut<'m, T: Sized, const M: usize, const N: usize> {
    phantom: PhantomData<&'m T>,
    mat: *mut Mat<T, M, N>,
    col: usize,
    row: usize,
}

impl<'m, T: 'm + Sized, const M: usize, const N: usize> Iterator for ColumnIterMut<'m, T, M, N> {
    type Item = &'m mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < N && self.row < M {
            let ret = unsafe {
                Some(&mut self.mat.as_mut().unwrap().data[self.row][self.col] as &'m mut T)
            };
            self.row += 1;
            ret
        } else {
            None
        }
    }
}

pub struct MultiColumnIterMut<'m, T: Sized, const M: usize, const N: usize> {
    phantom: PhantomData<&'m T>,
    mat: *mut Mat<T, M, N>,
    col: usize,
}

impl<'m, T: Sized, const M: usize, const N: usize> Iterator for MultiColumnIterMut<'m, T, M, N> {
    type Item = ColumnIterMut<'m, T, M, N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < N {
            let ret = Some(ColumnIterMut {
                phantom: self.phantom,
                mat: self.mat,
                col: self.col,
                row: 0,
            });
            self.col += 1;
            ret
        } else {
            None
        }
    }
}

impl<T: Sized, const M: usize, const N: usize> Mat<T, M, N> {
    pub fn width(&self) -> usize {
        N
    }

    pub fn height(&self) -> usize {
        M
    }

    /// Gives an iterator over a single row in the matrix
    /// Panics if r >= M
    pub fn iter_row<'a>(&'a self, r: usize) -> RowIter<T, N> {
        if r >= M {
            panic!("Row `{}` is out of bounds for `Mat<T, {}, {}>`", r, M, N);
        }

        RowIter {
            row: self.data[r].iter(),
        }
    }

    /// Gives an iterator over iterators of rows in the matrix
    pub fn iter_rows<'a>(&'a self) -> MultiRowIter<T, M, N> {
        MultiRowIter {
            rows: self.data.iter(),
        }
    }

    /// Gives an iterator over a single row in the matrix
    /// Panics if r >= M
    pub fn iter_column<'a>(&'a self, c: usize) -> ColumnIter<T, M, N> {
        if c >= N {
            panic!("Row `{}` is out of bounds for `Mat<T, {}, {}>`", c, M, N);
        }

        ColumnIter {
            mat: &self,
            row: 0,
            col: c,
        }
    }

    /// Gives an iterator over iterators of rows in the matrix
    pub fn iter_columns<'a>(&'a self) -> MultiColumnIter<T, M, N> {
        MultiColumnIter { mat: &self, col: 0 }
    }

    pub fn iter_row_mut<'m>(&'m mut self, r: usize) -> RowIterMut<T, N> {
        RowIterMut {
            row: self.data[r].iter_mut(),
        }
    }

    pub fn iter_rows_mut<'m>(&'m mut self) -> MultiRowIterMut<T, M, N> {
        MultiRowIterMut {
            rows: self.data.iter_mut(),
        }
    }

    pub fn iter_column_mut<'m>(&'m mut self, c: usize) -> ColumnIterMut<T, M, N> {
        ColumnIterMut {
            phantom: PhantomData,
            mat: self as *mut Self,
            col: c,
            row: 0,
        }
    }

    pub fn iter_columns_mut<'m>(&'m mut self) -> MultiColumnIterMut<T, M, N> {
        MultiColumnIterMut {
            phantom: PhantomData,
            mat: self as *mut Self,
            col: 0,
        }
    }

    pub fn from(data: [[T; N]; M]) -> Mat<T, M, N> {
        Mat { data }
    }
}

impl<T, const M: usize, const N: usize> Mat<T, M, N>
where
    T: Sized + std::ops::Mul<Output = T> + std::iter::Sum + Copy,
{
    pub fn mat_mul<const N2: usize>(&self, rhs: &Mat<T, N, N2>) -> Mat<T, N, N2> {
        let mut ret = Mat::from([[rhs[0][0]; N2]; N]);
        for r in 0..N {
            for c in 0..N2 {
                ret[r][c] = self.iter_row(r).dot(rhs.iter_column(c))
            }
        }
        ret
    }
}

impl<T, const M: usize, const N: usize> Mat<T, M, N>
where
    T: Sized + std::ops::Mul<Output = T> + std::iter::Sum + Copy + std::ops::Rem<Output = T>,
{
    pub fn mat_mul_mod<const N2: usize>(&self, rhs: &Mat<T, N, N2>, modr: T) -> Mat<T, N, N2> {
        let mut ret = Mat::from([[rhs[0][0]; N2]; N]);
        for r in 0..N {
            for c in 0..N2 {
                ret[r][c] = self.iter_row(r).dot_mod(rhs.iter_column(c), modr)
            }
        }
        ret
    }
}

impl<T: Sized, const M: usize, const N: usize> std::ops::Index<usize> for Mat<T, M, N> {
    type Output = [T; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Sized, const M: usize, const N: usize> std::ops::IndexMut<usize> for Mat<T, M, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<'r, T, const N: usize> RowIter<'r, T, N>
where
    T: Sized + std::ops::Mul<Output = T> + std::iter::Sum + Copy,
{
    pub fn dot(self, rhs: impl Iterator<Item = &'r T>) -> T {
        self.row.zip(rhs).map(|(&u, &v)| u * v).sum()
    }
}

impl<'r, T, const N: usize> RowIter<'r, T, N>
where
    T: Sized + std::ops::Mul<Output = T> + std::iter::Sum + Copy + std::ops::Rem<Output = T>,
{
    pub fn dot_mod(self, rhs: impl Iterator<Item = &'r T>, modr: T) -> T {
        self.row
            .zip(rhs)
            .map(|(&u, &v)| ((u % modr) * (v % modr)) % modr)
            .sum::<T>()
            % modr
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::Mat;
        let mut a = Mat::from([[0; 2]; 2]);
        a[0][0] = 1;
        a[0][1] = 2;
        a[1][0] = 3;
        a[1][1] = 4;

        let mut b = Mat::from([[0; 2]; 2]);
        b[0][0] = 5;
        b[0][1] = 6;
        b[1][0] = 7;
        b[1][1] = 8;

        let c = a.mat_mul(&b);

        assert_eq!(c[0][0], 19);
        assert_eq!(c[0][1], 22);
        assert_eq!(c[1][0], 43);
        assert_eq!(c[1][1], 50);
    }

    #[test]
    fn row_wise_mut() {
        use crate::Mat;
        let mut a = Mat::from([[0; 2]; 2]);
        let data = [0, 1, 2, 3];
        a.iter_rows_mut()
            .flatten()
            .zip(data.iter())
            .for_each(|(a, b)| *a = *b);

        assert_eq!(a[0][0], 0);
        assert_eq!(a[0][1], 1);
        assert_eq!(a[1][0], 2);
        assert_eq!(a[1][1], 3);
    }

    #[test]
    fn column_wise_mut() {
        use crate::Mat;
        let mut a = Mat::from([[0; 2]; 2]);
        let data = [0, 1, 2, 3];
        a.iter_columns_mut()
            .flatten()
            .zip(data.iter())
            .for_each(|(a, b)| *a = *b);

        assert_eq!(a[0][0], 0);
        assert_eq!(a[0][1], 2);
        assert_eq!(a[1][0], 1);
        assert_eq!(a[1][1], 3);
    }

    #[test]
    fn test_dot() {
        use crate::Mat;
        let mut a = Mat::from([[0; 3]; 3]);
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        a.iter_rows_mut()
            .flatten()
            .zip(data.iter())
            .for_each(|(a, b)| *a = *b);

        let res = a.iter_row(0).dot(a.iter_row(1));

        assert_eq!(res, 32);
    }
}
