pub fn setupGenericPractice() {
    summarizing();
}

fn summarizing() {
    let data = Data {
        contents: String::from("These are the contents"),
    };

    let summary = data.summarize();
    println!("{summary}");
}

trait Summary {
    fn summarize(&self) -> String;
}

struct Data<T> {
    contents: T,
}

impl<T: std::fmt::Display> Summary for Data<T> {
    fn summarize(&self) -> String {
        format!("contents: {}", self.contents)
    }
}