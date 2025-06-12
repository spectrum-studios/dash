-- Reference from User struct in types/src/user.rs
CREATE TABLE IF NOT EXISTS users (
  id INTEGER PRIMARY KEY UNIQUE,
  uuid VARCHAR(36) UNIQUE,
  username VARCHAR(24) UNIQUE,
  email VARCHAR(254) UNIQUE,
  password VARCHAR(60),
  is_admin BOOLEAN
);
