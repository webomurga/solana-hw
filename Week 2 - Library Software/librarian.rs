enum Publication {
    // Declare types of publications as "Book" and "Magazine".
    Book(Book),
    Magazine(Magazine)
}

struct Book {
    // Declare the properties of struct "Book".
    title: String,
    author: String,
    page_count: u32
}

struct Magazine {
    // Declare the properties of struct "Magazine".
    title: String,
    issue: u32,
    topic: String
}

fn print_pub(pubs: Vec<Publication>) {
    // Iterate every publication in the vector collection...
    for p in pubs {
        match p {
            // and match them based on their type & print out relative information.
            Publication::Book(ref book) => println!("Kitap: {} - Yazar: {}, Sayfa: {}", book.title, book.author, book.page_count),
            Publication::Magazine(ref mag) => println!("Dergi: {} - Sayı: {}, Konu: {}", mag.title, mag.issue, mag.topic)
        }
    }
}

// For trial purposes, create instances of structs in enum "Publication".
fn main() {
    // Declare a new instance of struct "Book". (For example, a novel in this case.) 
    let novel = Book {
        title: "Anna Karenina".to_string(),
        author: "Leo Tolstoy".to_string(),
        page_count: 864
    };

    // Declare a new instance of struct "Magazine". (For example, CHIP Online TR.)
    let trmag = Magazine {
        title: "CHIP Online Türkiye".to_string(),
        issue: 1,
        topic: "Bugünün Masalları...".to_string()
    };

    // Assemble a vector collection of the instances declared above...
    let my_favs = vec!(Publication::Book(novel), Publication::Magazine(trmag));

    // ... and call function "print_pub" to print them.
    print_pub(my_favs);
}
