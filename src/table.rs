use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};

enum StmtKind {
    INSERT,
    SELECT,
}

struct Stmt {
    kind: StmtKind,
    row: Option<Row>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Row {
    id: u64,
    username: [u8; 32],
    email: [u8; 32],
}

#[derive(Debug)]
enum Error {
    SyntaxError,
    UnknownCommand,
    InternalError,
}

pub const ROW_SIZE: usize = 72;

pub struct Table {
    pub len: usize,
    pub memory: [u8; 4096 * ROW_SIZE],
}

impl Table {
    fn insert(&mut self, row: &Row) -> Result<(), Error> {
        let row_bytes = serialize(row).unwrap();
        let mem_index = self.len;
        let mem_offset = mem_index * ROW_SIZE;
        self.memory[mem_offset..mem_offset + ROW_SIZE].copy_from_slice(&row_bytes);
        self.len += 1;
        Ok(())
    }

    fn select(&self) -> Result<Vec<Row>, Error> {
        let mut rows = Vec::new();
        for i in 0..self.len {
            let page_index = i;
            let page_offset = page_index * ROW_SIZE;
            let row_bytes = &self.memory[page_offset..page_offset + ROW_SIZE];
            let row = deserialize(row_bytes);
            match row {
                Ok(row) => rows.push(row),
                Err(e) => {
                    println!("{:?}", e);
                    return Err(Error::InternalError);
                }
            }
        }
        Ok(rows)
    }
}

fn parse(stmt: &str) -> Result<Stmt, Error> {
    let stmt = stmt.trim();
    let splitted = stmt.split_whitespace().collect::<Vec<_>>();
    if splitted.len() == 0 {
        return Err(Error::SyntaxError);
    }
    let kind = match splitted[0] {
        "insert" => StmtKind::INSERT,
        "select" => StmtKind::SELECT,
        _ => return Err(Error::UnknownCommand),
    };
    match kind {
        StmtKind::INSERT => {
            if splitted.len() != 4 {
                return Err(Error::SyntaxError);
            }
            let id = splitted[1].parse::<u64>();
            let id = match id {
                Ok(id) => id,
                Err(_) => return Err(Error::SyntaxError),
            };
            let username_bytes_vec = splitted[2].to_string().into_bytes();
            let mut username = [0; 32];
            for (i, b) in username_bytes_vec.iter().enumerate() {
                username[i] = *b;
            }
            let email_bytes_vec = splitted[3].to_string().into_bytes();
            let mut email = [0; 32];
            for (i, b) in email_bytes_vec.iter().enumerate() {
                email[i] = *b;
            }
            Ok(Stmt {
                kind: StmtKind::INSERT,
                row: Some(Row {
                    id,
                    username,
                    email,
                }),
            })
        }
        StmtKind::SELECT => {
            if splitted.len() != 1 {
                return Err(Error::SyntaxError);
            }
            Ok(Stmt {
                kind: StmtKind::SELECT,
                row: None,
            })
        }
    }
}

pub fn execute(command: &str, table: &mut Table) {
    let stmt = parse(command);
    match stmt {
        Ok(stmt) => match stmt.kind {
            StmtKind::INSERT => {
                let row = stmt.row.unwrap();
                let res = table.insert(&row);
                let err = match res {
                    Ok(_) => {
                        "OK"
                    }
                    Err(e) => match e {
                        Error::SyntaxError => "Syntax error",
                        Error::UnknownCommand => "Unknown command",
                        Error::InternalError => "Internal error",
                    },
                };
                println!("{}", err);
            }
            StmtKind::SELECT => {
                let rows = table.select();
                for row in rows.unwrap() {
                    print!("{}", row.id);
                    print!(" {}", String::from_utf8_lossy(&row.username));
                    print!(" {}", String::from_utf8_lossy(&row.email));
                    println!("");
                }
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
