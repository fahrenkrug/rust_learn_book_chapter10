pub fn example() {
    // First example won't run. As you can see when commenting in.
    // The inner r is never used (code highlighting) and the outer r is only declared but not
    // initialized when trying to use it in println.

    // Hint: Lifetimes are annotated with 'a

    /*
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
     */
    example2();
    example3();
    example4();
}

fn example2() {
    let string = String::from("abcdefg");
    let string2 = "xyz";
    let result = longest(string.as_str(), string2);
    println!("The longer string is {}", result);
}

fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    if string1.len() > string2.len() {
        string1
    } else {
        string2
    }
}

fn example3() {
    let string1 = String::from("Long stirngi s long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longer string is {}", result);
    }

    // this would not compile:

    /*
    let string1 = String::from("Long stirngi s long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longer string is {}", result);
     */
    {
        println!("The longer string is {}", longest2(&string1, &"asd"));
    }
}

// If we would return y here. We would get a compile time error, because y has no lifetime definition.
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// Creating a new string inside the longest function will not compile when the return values
// needs a value from the specific lifetime:
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // Third elision rule
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}!", announcement);
        self.part
    }
}

fn example4() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Expected to have a first sentence in the novel.");
    let i = ImportantExcerpt {
        part: first_sentence
    };
    println!("{:?}", i);
    println!("First word: {}", first_word("Das ist es ja quasi."));
    let s: &'static str = "I am so static!";
    println!("{}", s);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}