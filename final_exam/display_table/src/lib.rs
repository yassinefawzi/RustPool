use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.headers.is_empty() && self.body.is_empty() {
            return Ok(());
        }

        let mut col_widths: Vec<usize> = self.headers
            .iter()
            .map(|h| h.len())
            .collect();
        for row in &self.body {
            for (i, cell) in row.iter().enumerate() {
                col_widths[i] = col_widths[i].max(cell.len());
            }
        }

        // Helper to center text
        fn center(text: &str, width: usize) -> String {
            let space = width - text.len();
            let left = space / 2;
            let right = space - left;
            format!("{}{}{}", " ".repeat(left), text, " ".repeat(right))
        }

        // Step 2: Print headers
        write!(f, "|")?;	
        for (i, h) in self.headers.iter().enumerate() {
            write!(f, " {} |", center(h, col_widths[i]))?;
        }
        writeln!(f)?;

        // Step 3: Separator
        write!(f, "|")?;
        for (i, w) in col_widths.iter().enumerate() {
            write!(f, "{}", "-".repeat(w + 2))?;
            if i < col_widths.len() - 1 {
                write!(f, "+")?;
            }
        }
        writeln!(f, "|")?;

        // Step 4: Print rows
        for row in &self.body {
            write!(f, "|")?;
            for (i, cell) in row.iter().enumerate() {
                write!(f, " {} |", center(cell, col_widths[i]))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
