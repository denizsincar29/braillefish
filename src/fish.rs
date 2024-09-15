use rand::{rngs::ThreadRng, Rng};

/// Braille display version of the turbo.fish website.
/// turbo.fish is a website that generates a fish made of the turbofish operator ::<>().
/// The website animates the fish swimming across the screen.
/// This program is a Braille display version of the website.




/// Main struct for the BrailleFish program.
pub struct BrailleFish {
    /// Number of cells in the Braille display.
    pub num_cells: usize,
    /// Content to be displayed inside the turbofish bubble.
    pub fish_content: String,
    /// current field / state of the swimming fish
    pub field: String,
    /// cursor position of the fish
    pub cursor: usize,
    /// random number generator
    rng: ThreadRng
}

impl BrailleFish {
    /// Create a new BrailleFish struct.
    pub fn new(num_cells: usize, fish_content: String) -> Self {
        let mut bf = BrailleFish {
            num_cells,
            fish_content: format!("::<{}>()", fish_content),
            field: String::with_capacity(num_cells*5),
            cursor: 0,
            rng: rand::thread_rng()
        };
        bf.init_field();
        bf
    }

    /// initialize the field with randomly generated fish
    pub fn init_field(&mut self) {
        let fish_len = self.fish_content.len();
        while self.field.len() < self.field.capacity() - fish_len {
            // generate the random number of spaces between 5 and num_cells, but after this, the fish should fit in the field
            // let remaining_space = self.field.capacity() - self.field.len() - fish_len;  // oh it can be negative! Lets use the method that returns option instead
            let remaining_space = if let Some(val) = self.field.capacity().checked_sub(self.field.len() + fish_len) {
                val  // if we can subtract, we do it
            } else { break; };  // away!
            let can_use = std::cmp::min(remaining_space, self.num_cells);
            if can_use < 5 {
                break;
            }
            let num_spaces = self.rng.gen_range(5..can_use);
            // ok, now we can freely put a fish without overflowing the field
            self.field.push_str(&" ".repeat(num_spaces));
            self.field.push_str(&self.fish_content);
            // if we overflowed the field, we did something horribly wrong
            assert!(self.field.len() <= self.field.capacity(), "field of {} size overflowed and is now {}", self.field.capacity(), self.field.len());
        }
        // fill the rest of the field with spaces
        self.field.push_str(&" ".repeat(self.field.capacity() - self.field.len()));
        self.cursor = self.field.len();
    }  // init_field

    /// move the cursor to the left and return the new optional subfield
    pub fn move_left(&mut self) -> Option<&str> {
        self.cursor = self.cursor.checked_sub(1)?;  // if we can subtract, we do it
        let right_limit = if self.cursor + self.num_cells > self.field.len() {
            self.field.len()
        } else {
            self.cursor + self.num_cells
        };
        Some(&self.field[self.cursor as usize..right_limit])
    }
}

impl Iterator for BrailleFish {
    // Attention! The iterator is infinite! Use take() to limit the number of iterations.
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.move_left() {
            Some(val.to_string())
        } else {
            self.init_field();
            Some(self.field.clone())
        }
    }
}
