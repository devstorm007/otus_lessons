#[derive(Debug)]
struct MyStruct {
    s: String,
    c: String,
}

impl MyStruct {
    fn owned_self(mut self) /*-> &MyStruct*/
    {
        let b = &self.s;
        let b_mut = &mut self.s;
        let moved = self.s; // partial move
                            //println!("{:?}", self.s)
        println!("{:?}", self.c);
        //&self
        //println!("{:?}", self);
    }
    fn shared_self(&self) {
        let b = &self.s;
        //let b_mut = &mut self.s;
        //let moved = self.s;
    }
    fn unique_self(&mut self) {
        let b = &self.s;
        let b_mut = &mut self.s;
        b_mut.remove(2);
        //let moved = self.s; // to move we have to own value
    }

    fn test() {
        let ms_owned = MyStruct {
            s: String::from(""),
            c: String::from(""),
        };
        ms_owned.owned_self();
        //println!("{:?}", ms_owned)

        let ms_shared = MyStruct {
            s: String::from(""),
            c: String::from(""),
        };
        ms_shared.shared_self();
        println!("{:?}", ms_shared.s)
    }
}
