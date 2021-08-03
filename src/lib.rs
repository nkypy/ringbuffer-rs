// Copyright 2021 [Jack Shih <i@kshih.com>]
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[derive(Debug)]
pub struct RingBuffer<const N: usize> {
    size: usize,
    buffer: [u8; N],
    head: usize,
    tail: usize,
}

impl<const N: usize> RingBuffer<N> {
    pub fn new() -> Self {
        if N < 2 {
            panic!("size should be greater than 2");
        }
        RingBuffer {
            size: N,
            buffer: [0u8; N],
            head: 0,
            tail: 0,
        }
    }
    pub fn empty(&self) -> bool {
        self.head == self.tail
    }
    pub fn full(&self) -> bool {
        (self.tail + 1) % self.size == self.head
    }
    pub fn len(&self) -> usize {
        (self.tail + self.size - self.head) % self.size
    }
    pub fn write(&mut self, src: u8) -> bool {
        if self.full() {
            return false;
        }
        self.buffer[self.tail] = src;
        self.tail = (self.tail + 1) % self.size;
        true
    }
    pub fn read(&mut self) -> Option<u8> {
        if self.empty() {
            return None;
        }
        let src = self.buffer[self.head];
        self.head = (self.head + 1) % self.size;
        return Some(src);
    }
}

#[cfg(test)]
mod tests {
    use crate::RingBuffer;

    #[test]
    fn it_works() {
        let mut ring = RingBuffer::<32>::new();
        assert_eq!(ring.empty(), true);
        for i in [1u8; 31] {
            assert_eq!(ring.write(i), true);
        }
        assert_eq!(ring.empty(), false);
        assert_eq!(ring.full(), true);
        assert_eq!(ring.write(4u8), false);
        for _i in 0..31 {
            assert_eq!(ring.read(), Some(1u8));
        }
        assert_eq!(ring.empty(), true);
        assert_eq!(ring.full(), false);
    }
}
