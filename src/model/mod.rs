#[cfg(test)]
mod tests;

use error::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct HuffmanCode {
	code: Vec<bool>,
	pos: usize,
}

impl From<Vec<bool>> for HuffmanCode {
	fn from(code: Vec<bool>) -> Self {
		HuffmanCode {
			code,
			pos: 0,
		}
	}
}

impl From<Vec<u8>> for HuffmanCode {
	fn from(bytes: Vec<u8>) -> Self {
		let mut code = Vec::with_capacity(bytes.len() * 8);

		for byte in bytes {
			for bit in 0..7 {
				code.push((byte << bit) & 0b10000000 == 0b10000000);
			}
		}

		code.into()
	}
}

impl HuffmanCode {
	pub fn new() -> HuffmanCode {
		HuffmanCode {
			code: vec![],
			pos: 0,
		}
	}

	pub fn extend(&mut self, code: Vec<bool>) {
		self.code.extend(code);
	}

	pub fn next(&mut self) -> Result<bool> {
		if !self.exhausted() {
			self.pos += 1;
			Ok(self.code[self.pos - 1])
		} else {
			Err(HuffcodeError(1))
		}
	}

	pub fn exhausted(&self) -> bool {
		self.pos >= self.code.len()
	}

	pub fn vec(self) -> Vec<bool> {
		self.code
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HuffmanTree {
	root: Node
}

impl HuffmanTree {
	pub fn construct(text: String) -> HuffmanTree {
		Self::construct_multi(vec![text])
	}

	pub fn construct_multi(text: Vec<String>) -> HuffmanTree {
		// Node / Frequency tuple vector
		let mut nodes = {
			// Character / frequency hashmap
			let mut map = HashMap::with_capacity(0xFF); // Start with 256 character support and reallocate if necessary

			for line in text {
				for ch in line.chars() {
					map.entry(ch)
						.and_modify(|b| *b += 1)
						.or_insert(1);
				}
			}

			// TBH I am questioning myself why I'm reallocating as well... I guess the entry api is just so useful...
			let mut nodes: Vec<(Node, usize)> = Vec::with_capacity(map.len());

			for (ch, freq) in map {
				nodes.push((Node::Leaf(ch), freq));
			}

			nodes
		};

		while nodes.len() > 1 {
			nodes.sort_unstable_by(|(_, a), (_, b)| b.cmp(a));

			// Pop off the two least common entries and join them together
			let a = nodes.pop().unwrap();
			let b = nodes.pop().unwrap();

			nodes.push((Node::Branch(Box::new(a.0), Box::new(b.0)), a.1 + b.1));
		}

		HuffmanTree { root: nodes.pop().unwrap_or((Node::Leaf(0x00 as char), 0)).0 }
	}

	pub fn encode(&self, str: String) -> Result<HuffmanCode> {
		let mut code = HuffmanCode::new();
		for ch in str.chars() {
			code.extend(self.find(ch)?.vec());
		}

		Ok(code)
	}

	pub fn find(&self, ch: char) -> Result<HuffmanCode> {
		Ok(self.root.get(ch)?.into())
	}

	pub fn decode(&self, mut code: &mut HuffmanCode) -> Result<String> {
		let mut buffer = String::new();

		loop {
			if code.exhausted() {
				break;
			}
			let ch = self.root.traverse(&mut code)?;
			buffer.push(ch);
		}

		return Ok(buffer);
	}
}

#[derive(Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub(crate) enum Node {
	Branch(Box<Node>, Box<Node>),
	Leaf(char),
}

impl fmt::Debug for Node {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Node::Branch(ref l, ref r) => f.write_str(&format!("({:?}, {:?})", l, r)),
			Node::Leaf(ch) => f.write_str(&format!("('{}')", ch)),
		}
	}
}

impl Node {
	pub fn get(&self, ch: char) -> Result<Vec<bool>> {
		match self {
			Node::Branch(ref l, ref r) => {
				if let Ok(search) = l.get(ch) {
					let mut vec = vec![true];
					vec.extend(search);

					Ok(vec)
				} else if let Ok(search) = r.get(ch) {
					let mut vec = vec![false];
					vec.extend(search);

					Ok(vec)
				} else {
					Err(HuffcodeError(0))
				}
			}
			Node::Leaf(c) => if *c == ch { Ok(vec![]) } else { Err(HuffcodeError(0)) }
		}
	}

	pub fn traverse(&self, code: &mut HuffmanCode) -> Result<char> {
		match self {
			Node::Branch(ref l, ref r) => {
				if code.next()? { l } else { r }.traverse(code)
			}
			Node::Leaf(c) => Ok(*c)
		}
	}
}
