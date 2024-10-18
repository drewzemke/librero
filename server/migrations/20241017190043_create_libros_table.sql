-- Add migration script here
CREATE TABLE libros (
    isbn VARCHAR(255) PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL,
    cover_path VARCHAR(255)
)
