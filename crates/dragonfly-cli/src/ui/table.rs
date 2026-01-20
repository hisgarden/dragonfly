//! Table formatting utilities for CLI output

/// Simple table representation
#[derive(Debug)]
pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(headers: Vec<&str>) -> Self {
        Self {
            headers: headers.iter().map(|h| h.to_string()).collect(),
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Vec<&str>) {
        self.rows.push(row.iter().map(|r| r.to_string()).collect());
    }

    pub fn print(&self) {
        // Simple text table printing
        let mut col_widths = vec![0; self.headers.len()];

        for (i, header) in self.headers.iter().enumerate() {
            col_widths[i] = col_widths[i].max(header.len());
        }

        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                col_widths[i] = col_widths[i].max(cell.len());
            }
        }

        // Print header
        for (i, header) in self.headers.iter().enumerate() {
            print!("{:width$} ", header, width = col_widths[i]);
        }
        println!();

        // Print separator
        for width in &col_widths {
            print!("{} ", "─".repeat(*width));
        }
        println!();

        // Print rows
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                print!("{:width$} ", cell, width = col_widths[i]);
            }
            println!();
        }
    }
}
