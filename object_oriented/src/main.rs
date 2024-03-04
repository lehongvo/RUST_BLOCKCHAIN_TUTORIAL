// #[derive(Debug)]
// struct Car {
//     model: String,
//     year: u32,
//     is_run: bool
// }

// trait Vihecal {
//     fn new() -> Self where Self: Sized;

//     fn get_info(&self);

//     fn is_move(&self) -> bool;
// }

// impl Car {
//     fn new(name: &str) -> Car {
//         let new_car = Car {
//             model: String::from(name.to_string()),
//             year: 1878,
//             is_run: true
//         };
//         return new_car;
//     }

//     fn get_info(&self) {
//         println!("Model: {}, Year: {}", self.model, self.year);
//     }

//     fn is_move(&self) -> bool {
//         return self.is_run;
//     }
// }

// #[derive(Debug)]
// pub struct AveragedCollection {
//     list: Vec<i32>,
//     average: f64
// }

// impl AveragedCollection {
//     pub fn add(&mut self, value: i32 ){
//         self.list.push(value);
//         self.update_average();
//         println!("Already added")
//     }

//     pub fn remove(&mut self, value: i32) {
//         self.list.retain(|&x| x != value);
//         self.update_average();
//         println!("Already removed");
//     }

//     fn update_average(&mut self) {
//         let total: i32 = self.list.iter().sum();
//         let len = self.list.len();
//         self.average = total as f64 / len as f64;
//     }

//     fn get_info(&self) {
//         println!("Average is {}", self.average);
//         println!("List value is {:?}", self.list);
//     }
// }

// fn main() {
//     let mut averagedCollection = AveragedCollection{
//         list: vec![],
//         average: 0.0
//     };

//     averagedCollection.add(10);
//     averagedCollection.add(11);
//     averagedCollection.add(12);
//     averagedCollection.add(13);

//     averagedCollection.remove(11);

//     averagedCollection.get_info();
// }

// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

// File: src/lib.rs

// Định nghĩa trait State để biểu diễn các trạng thái của bài viết
// trait State {
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
//     fn approve(self: Box<Self>) -> Box<dyn State>;
//     fn content<'a>(&self, post: &'a Post) -> &'a str;
// }

// // Định nghĩa struct Draft để biểu diễn trạng thái 'draft'
// struct Draft;

// // Implement trait State cho trạng thái Draft
// impl State for Draft {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview {})
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self // Draft không thể được duyệt trực tiếp
//     }

//     fn content<'a>(&self, _: &'a Post) -> &'a str {
//         "" // Nội dung của Draft là chuỗi trống
//     }
// }

// // Định nghĩa struct PendingReview để biểu diễn trạng thái 'waiting for review'
// struct PendingReview;

// // Implement trait State cho trạng thái PendingReview
// impl State for PendingReview {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self // Đang chờ review, không thể yêu cầu lại
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Published {})
//     }

//     fn content<'a>(&self, _: &'a Post) -> &'a str {
//         "" // Nội dung của PendingReview là chuỗi trống
//     }
// }

// // Định nghĩa struct Published để biểu diễn trạng thái 'published'
// struct Published;

// // Implement trait State cho trạng thái Published
// impl State for Published {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self // Đã xuất bản, không thể yêu cầu review lại
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self // Đã xuất bản, không thể duyệt lại
//     }

//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content // Nội dung của Published là nội dung thực tế của bài viết
//     }
// }

// // Định nghĩa struct Post để biểu diễn bài viết với trạng thái
// pub struct Post {
//     state: Option<Box<dyn State>>, // Trạng thái hiện tại của bài viết
//     content: String, // Nội dung của bài viết
// }

// impl Post {
//     // Tạo một bài viết mới với trạng thái là Draft
//     pub fn new() -> Post {
//         Post {
//             state: Some(Box::new(Draft {})),
//             content: String::new(),
//         }
//     }

//     // Yêu cầu review cho bài viết
//     pub fn request_review(&mut self) {
//         if let Some(state) = self.state.take() {
//             self.state = Some(state.request_review());
//         }
//     }

//     // Duyệt bài viết nếu đang ở trạng thái PendingReview
//     pub fn approve(&mut self) {
//         if let Some(state) = self.state.take() {
//             self.state = Some(state.approve());
//         }
//     }

//     // Thêm nội dung vào bài viết nếu đang ở trạng thái Draft
//     pub fn add_text(&mut self, text: &str) {
//         if let Some(state) = &self.state {
//             self.content.push(state.content(self));
//         }
//         self.content.push_str(text);
//     }

//     // Lấy nội dung của bài viết
//     pub fn content(&self) -> &str {
//         if let Some(state) = &self.state {
//             state.content(self)
//         } else {
//             "" // Trạng thái không xác định, trả về chuỗi trống
//         }
//     }
// }

// Ví dụ sử dụng thư viện
// fn main() {
//     let mut post = Post::new();

//     post.add_text("I ate a salad for lunch today");
//     assert_eq!("", post.content());

//     post.request_review();
//     assert_eq!("", post.content());

//     post.approve();
//     assert_eq!("I ate a salad for lunch today", post.content());
// }

// #[derive(Debug)]
// pub struct Post {
//     state: Option<Box<dyn State>>,
//     content: String,
// }

// impl Post {
//     pub fn new() -> Post {
//         Post {
//             state: Some(Box::new(Draft {})),
//             content: String::new(),
//         }
//     }
//     pub fn add_text(&mut self, text: &str) {
//         self.content.push_str(text);
//     }

//     pub fn content(&self) -> &str {
//         ""
//     }

//     pub fn request_review(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.request_review())
//         }
//     }
// }

// trait State {
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
// }

// #[derive(Debug)]
// struct Draft {}

// impl State for Draft {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview {})
//     }
// }

// struct PendingReview {}

// impl State for PendingReview {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
// }

// fn main() {
//     let mut post = Post::new();
//     post.add_text("I ate a salad for lunch today");
//     println!("Value is {}", post.content);
// }

// src/lib.rs

// pub struct Post {
//     content: String,
// }

// pub struct DraftPost {
//     content: String,
// }

// pub struct PendingReviewPost {
//     content: String,
// }

// impl Post {
//     pub fn new() -> DraftPost {
//         DraftPost {
//             content: String::new(),
//         }
//     }

//     pub fn content(&self) -> &str {
//         &self.content
//     }
// }

// impl DraftPost {
//     pub fn add_text(&mut self, text: &str) {
//         self.content.push_str(text);
//     }

//     pub fn request_review(self) -> PendingReviewPost {
//         PendingReviewPost {
//             content: self.content,
//         }
//     }
// }

// impl PendingReviewPost {
//     pub fn approve(self) -> Post {
//         Post {
//             content: self.content,
//         }
//     }
// }

// fn main() {
//     let mut post = Post::new();

//     post.add_text("I ate a salad for lunch today");

//     let post = post.request_review();

//     let post = post.approve();

//     assert_eq!("I ate a salad for lunch today", post.content());
// }

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String
}

impl Post {
    pub fn new() -> DraftPost {
        let value = DraftPost {
            content: String::new()
        };
        return value;
    }

    pub fn content(&self) -> &str {
        let value = &self.content;
        return value;
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    let content = post.content;
    println!("Content is {:?}", content);
}