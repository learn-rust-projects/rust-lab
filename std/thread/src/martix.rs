use core::fmt;
use std::{
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
    vec::Vec,
};

use anyhow::Result;
const N: usize = 4;
use super::vector::Vector;
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}
impl<T> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: impl Into<Vec<T>>) -> Self {
        Self {
            rows,
            cols,
            data: data.into(),
        }
    }
}
pub struct MsgInput<T> {
    row: Vector<T>,
    idx: usize,
    col: Vector<T>,
}
impl<T> MsgInput<T> {
    pub fn new(row: Vector<T>, idx: usize, col: Vector<T>) -> Self {
        Self { row, idx, col }
    }
}
pub struct MsgOutput<T> {
    idx: usize,
    data: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    // sender to send the result back
    sender: oneshot::Sender<MsgOutput<T>>,
}
impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

impl<T> std::fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    // display matrix data 2*3 {123,456}
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ ")?;
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{}", self.data[i * self.cols + j])?;
                if j < self.cols - 1 {
                    write!(f, " ")?;
                }
            }
            if i < self.rows - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, " }}")
    }
}
impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Matrix ({{ rows: {}, cols: {}, data: {} }})",
            self.rows, self.cols, self
        )
    }
}
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Add<Output = T> + AddAssign + Mul<Output = T> + Default + Send + 'static,
{
    // generate 4 theads which receive msg and do dot product
    assert_eq!(a.cols, b.rows);
    // map reduce map phase
    let senders = (0..N)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = super::vector::dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        data: value,
                    }) {
                        eprintln!("Error sending message: {:?}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();
    let message_len = a.rows * b.cols;
    let mut result = Matrix::new(a.rows, b.cols, vec![T::default(); message_len]);
    let mut receivers = Vec::with_capacity(message_len);
    for i in 0..a.rows {
        for j in 0..b.cols {
            let row = Vector::new(&a.data[i * a.cols..(i + 1) * a.cols]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.cols)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            let idx = i * b.cols + j;
            let (tx, rx) = oneshot::channel::<MsgOutput<T>>();
            let msg = Msg::new(MsgInput::new(row, idx, col), tx);
            if let Err(e) = senders[i % N].send(msg) {
                eprintln!("Error sending message: {:?}", e);
            }
            receivers.push(rx);
        }
        // map reduce reduce phase
        // wait for all messages to be processed
        for rx in receivers.drain(..) {
            let msg_output = rx.recv()?;
            result.data[msg_output.idx] = msg_output.data;
        }
    }
    Ok(result)
}

impl<T> Mul for Matrix<T>
where
    T: Copy + Add<Output = T> + AddAssign + Mul<Output = T> + Default + Send + 'static,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("multiply failed")
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        let a = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let b = Matrix::new(3, 2, vec![7, 8, 9, 10, 11, 12]);
        assert_eq!(
            format!("{:?}", a * b),
            "Matrix ({ rows: 2, cols: 2, data: { 58 64, 139 154 } })"
        );
    }
    #[test]
    #[should_panic]
    fn test_multiply_failed() {
        let a = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let b = Matrix::new(2, 2, vec![7, 8, 9, 10]);
        let _ = a * b;
    }
}
