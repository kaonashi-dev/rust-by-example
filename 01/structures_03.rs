#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool
}

#[derive(Debug)]
struct Library {
    name: String,
    books: Vec<Book>
}

fn add_book(library: &mut Library, book: Book) {
    library.books.push(book);
}

fn find_book_by_title<'a>(library: &'a Library, title: &str) -> Option<&'a Book> {
    library.books.iter().find(|b| b.title == title)
}

fn borrow_book(library: &mut Library, title: &str) -> bool {
    // Try to find a mutable reference to the book with the given title
    if let Some(book) = library.books.iter_mut().find(|b| b.title == title) {
        // If found and available, mark as borrowed
        if book.available {
            book.available = false;
            return true; // Successfully borrowed
        }
    }
    false // Book not found or already borrowed
}

fn return_book(library: &mut Library, title: &str) -> bool {
    // Find the book and mark it as available again
    if let Some(book) = library.books.iter_mut().find(|b| b.title == title) {
        if !book.available { // Only if it was borrowed
            book.available = true;
            return true; // Successfully returned
        }
    }
    false // Book not found or wasn't borrowed
}

fn count_available_books(library: &Library) -> usize {
    library.books
        .iter()
        .filter(|book| book.available)
        .count()
}

fn display_library(library: &Library) {
    println!("\n📚 Library: {}", library.name);
    println!("{}", "=".repeat(50));

    for (index, book) in library.books.iter().enumerate() {
        let status = if book.available { "✅ Available" } else { "❌ Borrowed" };
        println!("{}. '{}' by {} ({} pages) - {}",
            index + 1,
            book.title,
            book.author,
            book.pages,
            status
        );
    }
    println!();
}

fn main() {
    let mut library = Library {
        name: String::from("City Central Library"),
        books: Vec::new()
    };

    // Add books to the library
    add_book(&mut library, Book {
        title: String::from("Don Quixote"),
        author: String::from("Miguel de Cervantes"),
        pages: 863,
        available: true
    });

    add_book(&mut library, Book {
        title: String::from("1984"),
        author: String::from("George Orwell"),
        pages: 328,
        available: true
    });

    add_book(&mut library, Book {
        title: String::from("The Great Gatsby"),
        author: String::from("F. Scott Fitzgerald"),
        pages: 180,
        available: true
    });

    // Display initial state
    display_library(&library);
    println!("Available books: {}", count_available_books(&library));

    // Borrow some books
    println!("\n📖 Borrowing books...");
    println!("Borrowed 'Don Quixote': {}", borrow_book(&mut library, "Don Quixote"));
    println!("Borrowed '1984': {}", borrow_book(&mut library, "1984"));
    println!("Try to borrow '1984' again: {}", borrow_book(&mut library, "1984")); // Should fail

    display_library(&library);
    println!("Available books: {}", count_available_books(&library));

    // Return a book
    println!("\n📥 Returning '1984'...");
    println!("Returned '1984': {}", return_book(&mut library, "1984"));

    display_library(&library);
    println!("Available books: {}", count_available_books(&library));

    // Search for a book
    println!("\n🔍 Searching for 'The Great Gatsby'...");
    if let Some(book) = find_book_by_title(&library, "The Great Gatsby") {
        println!("Found: '{}'  by {} - {} pages",
            book.title, book.author, book.pages);
    } else {
        println!("Book not found");
    }
}
