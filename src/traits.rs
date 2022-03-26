trait Readable {
    fn read(&self) -> ();
    fn how_long_it_takes(&self) -> i32 {
        0
    }
}

struct Book {
    page: i32,
    content: String
}

impl Book {
    fn num_of_pages(&self) -> i32 {
        self.page
    }
}

impl Readable for Book {
    fn read(&self) -> () {
        println!("{}", self.content);
    }
    fn how_long_it_takes(&self) -> i32 {
        32
    }
}

pub fn try_traits() {
    let content = String::from("This is the moby dick");
    let moby_dick = Book{
        page: 300,
        content,
    };
    
    println!("Pages: {}", moby_dick.num_of_pages());
    moby_dick.read();
    println!("To read it, it takes {} hours", moby_dick.how_long_it_takes());
}
