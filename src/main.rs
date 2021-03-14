mod generics {
    fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn example_largest() {
        let number_list = vec![1,345,53,62,25,253];
        let result = largest(&number_list);

        println!("The largest number is {}", result);

        let char_list = vec!['a', 'b', 'd', 'p', 'ü'];
        let result = largest(&char_list);
        println!("The largest chart is {}", result);
    }

    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    #[derive(Debug)]
    struct PointTwoTypes<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> PointTwoTypes<T, U> {
        fn mixup<V, W>(self, other_point: PointTwoTypes<V, W>) -> PointTwoTypes<T, W> {
            PointTwoTypes {
                x: self.x,
                y: other_point.y
            }
        }
    }

    fn example_point() {
        let int_point = Point{x:2, y: 5};
        let float_point = Point{ x: 4.5, y: 4.2 };
        println!("Point 1: {:?}", int_point);
        println!("Point 2: {:?}", float_point);

        let two_types_point = PointTwoTypes{x: 4, y: 4.1};
        println!("Point 3: {:?}", two_types_point);
        println!("X of point 2: {}", float_point.x());

        let float32_point = Point { x: 4.5 as f32, y: 19.1 as f32 };
        println!("Distance from origin is {}", float32_point.distance_from_origin());
        example_mixup();
    }

    fn example_mixup() {
        let first_point = PointTwoTypes {x: "123", y: 2};
        let second_point = PointTwoTypes { x: 'a', y: 45.5};
        let mixed_up_point = first_point.mixup(second_point);
        println!("Mixed up point: {:?}", mixed_up_point);
    }

    pub fn example() {
        example_largest();
        example_point();
    }
}

mod traits {
    use std::fmt::Debug;
    use std::iter::Sum;

    trait Summary {
        fn summarize(&self) -> String;
    }

    trait AutoSummary {
        fn summarize_author(&self) -> String;

        fn auto_summarize(&self) -> String {
            String::from("(Read more)...")
        }

        // In default implementations it's possible to use functions that don't have a default implementation.
        fn auto_summarize_author(&self) -> String {
            format!("(Read more from ${}", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        headline: String,
        location: String,
        author: String,
        content: String,
    }

    impl Summary for NewsArticle {

        fn summarize(&self) -> String {
            format!("{}, by {}, ({})", self.headline, self.author, self.location)
        }
    }

    impl AutoSummary for NewsArticle {
        fn summarize_author(&self) -> String {
            self.author.clone()
        }
    }

    #[derive(Debug)]
    pub struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    fn example_summary() {
        let article = NewsArticle {
            headline: String::from("What a game!"),
            location: String::from("London"),
            author: String::from("Karla Kolumna"),
            content: String::from("After a stunning game from both teams Liverpool beats Chelsea 2:1. Matchwinner of this game is once again Sadio Mané with one goal and one assist."),
        };
        let tweet = Tweet {
            username: String::from("PinkPanda"),
            content: String::from("Sadio Mané! We're so lucky having you here at the club! YNAW!"),
            reply: false,
            retweet: true,
        };
        let summarizables: Vec<&dyn Summary> = vec![&article as &dyn Summary, &tweet as &dyn Summary];
        summarizables.iter().for_each(|summarizable| println!("Summary: {}", summarizable.summarize()));
        println!("Auto summarize: {}", article.auto_summarize());
        println!("Author summary: {}", article.auto_summarize_author());
        // continue here https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations
    }

    // This is syntactical sugar for notify2
    fn notify(summary: &impl Summary) {
        println!("Notifying about summary: {}", summary.summarize());
    }

    fn notify2<T: Summary>(summary: &T) {
        println!("Notifying about summary: {}", summary.summarize());
    }

    fn notify3(debuggable_summary: &(impl Summary + Debug)) {
        println!("Debug summary: {}", debuggable_summary.summarize());
        println!("Debug: {:?}", debuggable_summary);
    }

    fn example_traits_as_parameters() {
        let tweet = Tweet {
            username: String::from("PinkPanda"),
            content: String::from("Sadio Mané! We're so lucky having you here at the club! YNAW!"),
            reply: false,
            retweet: true,
        };
        notify(&tweet);
        notify2(&tweet);
        notify3(&tweet);
    }

    fn create_notification<T, U>(summary1: T, summary2: U) -> String
    where T: Summary + Debug,
    U: AutoSummary {
        format!("Summary 1: {:?}. In short: {}. But summary 2: {}", summary1, summary1.summarize(), summary2.auto_summarize())
    }

    fn example_where_clauses() {
        let article = NewsArticle {
            headline: String::from("What a game!"),
            location: String::from("London"),
            author: String::from("Karla Kolumna"),
            content: String::from("After a stunning game from both teams Liverpool beats Chelsea 2:1. Matchwinner of this game is once again Sadio Mané with one goal and one assist."),
        };
        let tweet = Tweet {
            username: String::from("PinkPanda"),
            content: String::from("Sadio Mané! We're so lucky having you here at the club! YNAW!"),
            reply: false,
            retweet: true,
        };
        println!("{}", create_notification(tweet, article));
    }

    fn get_summary() -> impl Summary {
        Tweet {
            username: String::from("PinkPanda"),
            content: String::from("Sadio Mané! We're so lucky having you here at the club! YNAW!"),
            reply: false,
            retweet: true,
        }

        // The following would not working (returning Tweet or NewsArticle) because
        // the compiler needs to know at compile time what type is returned:

        /*

        fn returns_summarizable(switch: bool) -> impl Summary {
            if switch {
                NewsArticle {
                headline: String::from(
                    "Penguins win the Stanley Cup Championship!",
                ),
                location: String::from("Pittsburgh, PA, USA"),
                author: String::from("Iceburgh"),
                content: String::from(
                    "The Pittsburgh Penguins once again are the best \
                    hockey team in the NHL.",
                ),
            }
        } else {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from(
                    "of course, as you probably already know, people",
                ),
                reply: false,
                retweet: false,
            }
        }
    }
         */
    }

    fn example_trait_as_return() {
        let summary = get_summary();
        println!(summary.summarize());
    }

    pub fn example() {
        example_summary();
        example_traits_as_parameters();
        example_where_clauses();
        example_trait_as_return();
    }
}

fn main() {
    generics::example();
    traits::example();
}
