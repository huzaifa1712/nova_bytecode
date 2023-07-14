use std::{fmt::{Display}, vec};


#[derive(Debug)]
// Instruction
pub enum Inst {
    OpReturn,
    OpConstant(usize), // idx in const pool
}

impl Display for Inst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


// #[derive(Debug)]
// // all objects go through here - easier to return out objects
// pub enum Obj {
//     ObjString(String),
// }

// impl Display for Obj {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let repr = match self {
//             Self::ObjString(s) => s
//         };
//         write!(f, "{}", repr)
//     }
// }

#[derive(Debug,Clone,Copy)]
// Value on the Stack (size known at compile-time)
    // can't do Rc<Obj> because Rc doesn't impl Copy
pub enum Value<'obj> {
    Number(usize),
    Bool(bool),
    ObjString(&'obj String)
}

impl<'obj>  Value<'obj>  {
    pub fn num(n:usize)->Value<'obj>  {
        Self::Number(n)
    }
}

impl<'obj>  Display for Value<'obj>  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr=match &self {
            Self::Number(n) => n.to_string(),
            Self::Bool(b) => b.to_string(),
            Self::ObjString(s) => s.to_string()
        };

        write!(f, "{}", repr)
    }
}

#[derive(Debug)]
struct LineEncoding(usize,usize); // (line number, occurences)
#[derive(Debug)]
struct Lines {
    lines:Vec<LineEncoding>
}

impl Lines {
    pub fn new()->Lines {
        Lines {
            lines:vec![]
        }
    }

    pub fn add_line(&mut self, line_num:usize) {
        let last=self.lines.last_mut();

        match last {
            Some(le) if le.0.eq(&line_num) =>  le.1 += 1,
            _ => self.lines.push(LineEncoding(line_num, 1))
        }
    }

    // idx 2
    // index if we uncompress e.g  (12,2), (14,3), (12,1)..
    pub fn get_line(&self, idx:usize)->Option<usize> {
        // idx < start+occurences => end
        let mut start=0;
        for le in self.lines.iter() {
            let (line,occur)=(le.0,le.1);
            if idx < start + occur {
                return Some(line)
            }

            start+=occur;
        }
        None
    }
}
// represents a series of bytecode instructions along with context
#[derive(Debug)]
pub struct Chunk<'val>  {
    ops:Vec<Inst>,
    constants:Vec<Value<'val>>, // pool of constants
    op_lines:Lines, // line numbers
    constant_lines:Lines // two arrs because index goes along with the enum (less confusing)
}

impl<'val> Chunk<'val> {
    pub fn new()->Self {
        Chunk {
            ops:vec![], constants:vec![], op_lines:Lines::new(), constant_lines:Lines::new()
        }
    }

     // easier to use idx even though slightly more overhead
        // we have no easy way to get the next Inst unlike in C besides idx
    pub fn get_op(&self, idx:usize)->Option<&Inst> {
        self.ops.get(idx)
    }

    pub fn write_op(&mut self, op:Inst, line:usize) {
        self.ops.push(op);
        self.op_lines.add_line(line);
    }

    pub fn get_constant(&self, idx:usize)->Option<Value> {
        self.constants.get(idx).map(|v| v.to_owned())
    }

    // Returns index where constant was added
    pub fn add_constant(&mut self, value:Value<'val>, line:usize)->usize {
        let constants=&mut self.constants;
        constants.push(value);

        self.constant_lines.add_line(line);
        constants.len()-1
    }
}

impl<'val> Display for Chunk<'val> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut code=String::from("Ops:\n");
        for (idx,op) in self.ops.iter().enumerate() {
            let fmt=format!("{} (line {})\n", op.to_string().as_str(), self.op_lines.get_line(idx).unwrap());
            code.push_str(fmt.as_str());
        }
        code.push_str("\n\nConstants:\n");

        for (idx,c) in self.constants.iter().enumerate() {
            let fmt=format!("{} (line {})\n", c.to_string().as_str(), self.constant_lines.get_line(idx).unwrap());
            code.push_str(fmt.as_str());
        }

        write!(f, "{}", code)
    }
}

#[test]
fn test_lines() {
    let mut lines=Lines::new();
    assert_eq!(None, lines.get_line(0));

    lines.add_line(12);
    lines.add_line(12);

    lines.add_line(14);
    lines.add_line(14);
    lines.add_line(14);

    lines.add_line(15);

    assert_eq!(Some(12), lines.get_line(0));
    assert_eq!(Some(14), lines.get_line(2));
    assert_eq!(Some(14), lines.get_line(4));
    assert_eq!(Some(15),lines.get_line(5));
    assert_eq!(None, lines.get_line(6));

}