CREATE TABLE IF NOT EXISTS users (
  id VARCHAR(255)          PRIMARY KEY,
  username VARCHAR(255)    NOT NULL,
  displayName VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS credentials (
  id            VARCHAR(255) PRIMARY KEY,
  publickey     TEXT NOT NULL,
  name          VARCHAR(255) NOT NULL,  
  transports    TEXT[] NOT NULL,
  user_id       VARCHAR(255), 
  CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id)
);


