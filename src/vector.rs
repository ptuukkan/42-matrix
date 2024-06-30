use std::fmt;

#[derive(Debug)]
pub struct Vector<K, const N: usize>(pub(crate) [K; N]);

impl<K: fmt::Debug, const N: usize> fmt::Display for Vector<K, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..N {
            if i == 0 {
                write!(f, "{:?}", self.0[i])?;
            } else {
                write!(f, ", {:?}", self.0[i])?;
            }
        }
        write!(f, "]")
    }
}

impl<K, const N: usize> From<[K; N]> for Vector<K, N> {
    fn from(value: [K; N]) -> Self {
        Self(value)
    }
}

impl<K, const N: usize> PartialEq for Vector<K, N>
where
    K: std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            if self.0[i] != other.0[i] {
                return false;
            }
        }
        true
    }
}
