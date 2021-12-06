pub struct Mat<T: Sized, const M: usize, const N: usize> {
    data: [[T; N]; M],
}

pub struct RowIter<'a, T: Sized, const M: usize, const N: usize> {
    mat: &'a Mat<T, M, N>,
    col: usize,
    row: usize,
}

impl<'a, T: Sized, const M: usize, const N: usize> Iterator for RowIter<'a, T, M, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < N && self.row < M {
            unsafe {
                let ret = Some(
                    self.mat
                        .data
                        .get_unchecked(self.row)
                        .get_unchecked(self.col),
                );
                self.col += 1;
                ret
            }
        } else {
            None
        }
    }
}

pub struct RowIterMut<'a, T: Sized, const M: usize, const N: usize> {
    mat: &'a mut Mat<T, M, N>,
    col: usize,
    row: usize,
}

impl<'a, T: Sized, const M: usize, const N: usize> Iterator for RowIterMut<'a, T, M, N> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < N && self.row < M {
            unsafe {
                let elem = self
                    .mat
                    .data
                    .get_unchecked_mut(self.row)
                    .get_unchecked_mut(self.col) as *mut T;
                self.col += 1;
                Some(elem.as_mut().unwrap())
            }
        } else {
            None
        }
    }
}

pub struct MultiRowIter<'a, T: Sized, const M: usize, const N: usize> {
    mat: &'a Mat<T, M, N>,
    row: usize,
}

impl<'a, T: Sized, const M: usize, const N: usize> Iterator for MultiRowIter<'a, T, M, N> {
    type Item = RowIter<'a, T, M, N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < M {
            let ret = Some(RowIter {
                mat: self.mat,
                row: self.row,
                col: 0,
            });
            self.row += 1;
            ret
        } else {
            None
        }
    }
}

pub struct MultiRowIterMut<'a, T: Sized, const M: usize, const N: usize> {
    mat: &'a mut Mat<T, M, N>,
    row: usize,
}

impl<'a, T: Sized, const M: usize, const N: usize> Iterator for MultiRowIterMut<'a, T, M, N> {
    type Item = RowIterMut<'a, T, M, N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < M {
            unsafe {
                let mat = self.mat as *mut Mat<T, M, N>;
                let iter = RowIterMut {
                    mat: mat.as_mut().unwrap(),
                    row: self.row,
                    col: 0,
                };
                self.row += 1;
                Some(iter)
            }
        } else {
            None
        }
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
            unsafe {
                let ret = Some(
                    self.mat
                        .data
                        .get_unchecked(self.row)
                        .get_unchecked(self.col),
                );
                self.row += 1;
                ret
            }
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

impl<T: Sized, const M: usize, const N: usize> Mat<T, M, N> {
    pub fn width(&self) -> usize {
        N
    }

    pub fn height(&self) -> usize {
        M
    }

    /// Gives an iterator over a single row in the matrix
    /// Panics if r >= M
    pub fn iter_row<'a>(&'a self, r: usize) -> RowIter<T, M, N> {
        if r >= M {
            panic!("Row `{}` is out of bounds for `Mat<T, {}, {}>`", r, M, N);
        }

        RowIter {
            mat: &self,
            row: r,
            col: 0,
        }
    }

    /// Gives an iterator over iterators of rows in the matrix
    pub fn iter_rows<'a>(&'a self) -> MultiRowIter<T, M, N> {
        MultiRowIter { mat: &self, row: 0 }
    }

    /// Gives an iterator over a single row in the matrix
    /// Panics if r >= M
    pub fn iter_row_mut<'a>(&'a mut self, r: usize) -> RowIterMut<T, M, N> {
        if r >= M {
            panic!("Row `{}` is out of bounds for `Mat<T, {}, {}>`", r, M, N);
        }

        RowIterMut {
            mat: self,
            row: r,
            col: 0,
        }
    }

    /// Gives an iterator over iterators of rows in the matrix
    pub fn iter_rows_mut<'a>(&'a mut self) -> MultiRowIterMut<T, M, N> {
        MultiRowIterMut { mat: self, row: 0 }
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
                ret[r][c] = self
                    .iter_row(r)
                    .zip(rhs.iter_column(c))
                    .map(|(&u, &v)| u * v)
                    .sum();
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
                ret[r][c] = self
                    .iter_row(r)
                    .zip(rhs.iter_column(c))
                    .map(|(&u, &v)| (u * v) % modr)
                    .sum::<T>()
                    % modr;
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
    fn multi_row_iter_mut() {
        use crate::Mat;
        let mut a = Mat::from([[0; 2]; 2]);
        let data = [0, 1, 2, 3];
        a.iter_rows_mut()
            .flat_map(|r| r)
            .zip(data.iter())
            .for_each(|(a, b)| *a = *b);

        assert_eq!(a[0][0], 0);
        assert_eq!(a[0][1], 1);
        assert_eq!(a[1][0], 2);
        assert_eq!(a[1][1], 3);
    }
}
