enum Node {
	Cons(u32, Box<Node>),
	Nil,
}

struct List {
	head: Node,
	count: u32,
}

impl List {
	fn new() -> List {
		List {
			head: Node::Nil,
			count: 0,
		}
	}

	fn push(&mut self, elem: u32) {
		self.head = Node::Cons(elem, Box::new(self.head));
	}

	fn len(&self) -> u32 {
		self.count
	}
}


